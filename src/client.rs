mod auth;
mod config;
mod connection;

mod tls;
#[cfg(any(
    feature = "rustls",
    feature = "native-tls",
    feature = "vendored-openssl"
))]
mod tls_stream;

pub use auth::*;
pub use config::*;
pub(crate) use connection::*;

use crate::tds::stream::ReceivedToken;
use crate::{
    result::ExecuteResult,
    tds::{
        codec::{self, IteratorJoin},
        stream::{QueryStream, TokenStream},
    },
    BulkLoadRequest, ColumnFlag, SqlReadBytes, ToSql,
};
use codec::{BatchRequest, ColumnData, PacketHeader, RpcParam, RpcProcId, TokenRpcRequest};
use enumflags2::BitFlags;
use futures::{AsyncRead, AsyncWrite};
use futures_util::TryStreamExt;
use std::{borrow::Cow, fmt::Debug};

/// `Client` is the main entry point to the SQL Server, providing query
/// execution capabilities.
///
/// A `Client` is created using the [`Config`], defining the needed
/// connection options and capabilities.
///
/// # Example
///
/// ```no_run
/// # use tiberius::{Config, AuthMethod};
/// use tokio_util::compat::TokioAsyncWriteCompatExt;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let mut config = Config::new();
///
/// config.host("0.0.0.0");
/// config.port(1433);
/// config.authentication(AuthMethod::sql_server("SA", "<Mys3cureP4ssW0rD>"));
///
/// let tcp = tokio::net::TcpStream::connect(config.get_addr()).await?;
/// tcp.set_nodelay(true)?;
/// // Client is ready to use.
/// let client = tiberius::Client::connect(config, tcp.compat_write()).await?;
/// # Ok(())
/// # }
/// ```
///
/// [`Config`]: struct.Config.html
#[derive(Debug)]
pub struct Client<S: AsyncRead + AsyncWrite + Unpin + Send> {
    pub(crate) connection: Connection<S>,
}

impl<S: AsyncRead + AsyncWrite + Unpin + Send> Client<S> {
    /// Uses an instance of [`Config`] to specify the connection
    /// options required to connect to the database using an established
    /// tcp connection
    ///
    /// [`Config`]: struct.Config.html
    pub async fn connect(config: Config, tcp_stream: S) -> crate::Result<Client<S>> {
        Ok(Client {
            connection: Connection::connect(config, tcp_stream).await?,
        })
    }

    /// Executes SQL statements in the SQL Server, returning the number rows
    /// affected. Useful for `INSERT`, `UPDATE` and `DELETE` statements. The
    /// `query` can define the parameter placement by annotating them with
    /// `@PN`, where N is the index of the parameter, starting from `1`. If
    /// executing multiple queries at a time, delimit them with `;` and refer to
    /// [`ExecuteResult`] how to get results for the separate queries.
    ///
    /// For mapping of Rust types when writing, see the documentation for
    /// [`ToSql`]. For reading data from the database, see the documentation for
    /// [`FromSql`].
    ///
    /// This API is not quite suitable for dynamic query parameters. In these
    /// cases using a [`Query`] object might be easier.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use tiberius::Config;
    /// # use tokio_util::compat::TokioAsyncWriteCompatExt;
    /// # use std::env;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let c_str = env::var("TIBERIUS_TEST_CONNECTION_STRING").unwrap_or(
    /// #     "server=tcp:localhost,1433;integratedSecurity=true;TrustServerCertificate=true".to_owned(),
    /// # );
    /// # let config = Config::from_ado_string(&c_str)?;
    /// # let tcp = tokio::net::TcpStream::connect(config.get_addr()).await?;
    /// # tcp.set_nodelay(true)?;
    /// # let mut client = tiberius::Client::connect(config, tcp.compat_write()).await?;
    /// let results = client
    ///     .execute(
    ///         "INSERT INTO ##Test (id) VALUES (@P1), (@P2), (@P3)",
    ///         &[&1i32, &2i32, &3i32],
    ///     )
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`ExecuteResult`]: struct.ExecuteResult.html
    /// [`ToSql`]: trait.ToSql.html
    /// [`FromSql`]: trait.FromSql.html
    /// [`Query`]: struct.Query.html
    pub async fn execute<'a>(
        &mut self,
        query: impl Into<Cow<'a, str>>,
        params: &[&dyn ToSql],
    ) -> crate::Result<ExecuteResult> {
        self.connection.flush_stream().await?;
        let rpc_params = Self::rpc_params(query);

        let params = params.iter().map(|s| s.to_sql());
        self.rpc_perform_query(RpcProcId::ExecuteSQL, rpc_params, params)
            .await?;

        ExecuteResult::new(&mut self.connection).await
    }

    /// Executes SQL statements in the SQL Server, returning resulting rows.
    /// Useful for `SELECT` statements. The `query` can define the parameter
    /// placement by annotating them with `@PN`, where N is the index of the
    /// parameter, starting from `1`. If executing multiple queries at a time,
    /// delimit them with `;` and refer to [`QueryStream`] on proper stream
    /// handling.
    ///
    /// For mapping of Rust types when writing, see the documentation for
    /// [`ToSql`]. For reading data from the database, see the documentation for
    /// [`FromSql`].
    ///
    /// This API can be cumbersome for dynamic query parameters. In these cases,
    /// if fighting too much with the compiler, using a [`Query`] object might be
    /// easier.
    ///
    /// # Example
    ///
    /// ```
    /// # use tiberius::Config;
    /// # use tokio_util::compat::TokioAsyncWriteCompatExt;
    /// # use std::env;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let c_str = env::var("TIBERIUS_TEST_CONNECTION_STRING").unwrap_or(
    /// #     "server=tcp:localhost,1433;integratedSecurity=true;TrustServerCertificate=true".to_owned(),
    /// # );
    /// # let config = Config::from_ado_string(&c_str)?;
    /// # let tcp = tokio::net::TcpStream::connect(config.get_addr()).await?;
    /// # tcp.set_nodelay(true)?;
    /// # let mut client = tiberius::Client::connect(config, tcp.compat_write()).await?;
    /// let stream = client
    ///     .query(
    ///         "SELECT @P1, @P2, @P3",
    ///         &[&1i32, &2i32, &3i32],
    ///     )
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`QueryStream`]: struct.QueryStream.html
    /// [`Query`]: struct.Query.html
    /// [`ToSql`]: trait.ToSql.html
    /// [`FromSql`]: trait.FromSql.html
    pub async fn query<'a, 'b>(
        &'a mut self,
        query: impl Into<Cow<'b, str>>,
        params: &'b [&'b dyn ToSql],
    ) -> crate::Result<QueryStream<'a>>
    where
        'a: 'b,
    {
        log::debug!("query");
        self.connection.flush_stream().await?;
        let rpc_params = Self::rpc_params(query);
        log::debug!(" - rpc_params = {:?}", rpc_params);
        let params = params.iter().map(|p| p.to_sql());
        self.rpc_perform_query(RpcProcId::ExecuteSQL, rpc_params, params)
            .await?;
        log::debug!(" - creating token stream");
        let ts = TokenStream::new(&mut self.connection);
        log::debug!(" - creating query stream");
        let mut result = QueryStream::new(ts.try_unfold());
        log::debug!(" - forwarding to metadata");
        result.forward_to_metadata().await?;
        log::debug!(" - returning query stream");

        Ok(result)
    }

    /// Execute multiple queries, delimited with `;` and return multiple result
    /// sets; one for each query.
    ///
    /// # Example
    ///
    /// ```
    /// # use tiberius::Config;
    /// # use tokio_util::compat::TokioAsyncWriteCompatExt;
    /// # use std::env;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let c_str = env::var("TIBERIUS_TEST_CONNECTION_STRING").unwrap_or(
    /// #     "server=tcp:localhost,1433;integratedSecurity=true;TrustServerCertificate=true".to_owned(),
    /// # );
    /// # let config = Config::from_ado_string(&c_str)?;
    /// # let tcp = tokio::net::TcpStream::connect(config.get_addr()).await?;
    /// # tcp.set_nodelay(true)?;
    /// # let mut client = tiberius::Client::connect(config, tcp.compat_write()).await?;
    /// let row = client.simple_query("SELECT 1 AS col").await?.into_row().await?.unwrap();
    /// assert_eq!(Some(1i32), row.get("col"));
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Warning
    ///
    /// Do not use this with any user specified input. Please resort to prepared
    /// statements using the [`query`] method.
    ///
    /// [`query`]: #method.query
    pub async fn simple_query<'a, 'b>(
        &'a mut self,
        query: impl Into<Cow<'b, str>>,
    ) -> crate::Result<QueryStream<'a>>
    where
        'a: 'b,
    {
        self.connection.flush_stream().await?;

        let req = BatchRequest::new(query, self.connection.context().transaction_descriptor());

        let id = self.connection.context_mut().next_packet_id();
        self.connection.send(PacketHeader::batch(id), req).await?;

        let ts = TokenStream::new(&mut self.connection);

        let mut result = QueryStream::new(ts.try_unfold());
        result.forward_to_metadata().await?;

        Ok(result)
    }

    /// Execute a `BULK INSERT` statement, efficiantly storing a large number of
    /// rows to a specified table. Note: make sure the input row follows the same
    /// schema as the table, otherwise calling `send()` will return an error.
    ///
    /// # Example
    ///
    /// ```
    /// # use tiberius::{Config, IntoRow};
    /// # use tokio_util::compat::TokioAsyncWriteCompatExt;
    /// # use std::env;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let c_str = env::var("TIBERIUS_TEST_CONNECTION_STRING").unwrap_or(
    /// #     "server=tcp:localhost,1433;integratedSecurity=true;TrustServerCertificate=true".to_owned(),
    /// # );
    /// # let config = Config::from_ado_string(&c_str)?;
    /// # let tcp = tokio::net::TcpStream::connect(config.get_addr()).await?;
    /// # tcp.set_nodelay(true)?;
    /// # let mut client = tiberius::Client::connect(config, tcp.compat_write()).await?;
    /// let create_table = r#"
    ///     CREATE TABLE ##bulk_test (
    ///         id INT IDENTITY PRIMARY KEY,
    ///         val INT NOT NULL
    ///     )
    /// "#;
    ///
    /// client.simple_query(create_table).await?;
    ///
    /// // Start the bulk insert with the client.
    /// let mut req = client.bulk_insert("##bulk_test").await?;
    ///
    /// for i in [0i32, 1i32, 2i32] {
    ///     let row = (i).into_row();
    ///
    ///     // The request will handle flushing to the wire in an optimal way,
    ///     // balancing between memory usage and IO performance.
    ///     req.send(row).await?;
    /// }
    ///
    /// // The request must be finalized.
    /// let res = req.finalize().await?;
    /// assert_eq!(3, res.total());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn bulk_insert<'a>(
        &'a mut self,
        table: &'a str,
    ) -> crate::Result<BulkLoadRequest<'a, S>> {
        // Start the bulk request
        self.connection.flush_stream().await?;

        // retrieve column metadata from server
        let query = format!("SELECT TOP 0 * FROM {}", table);

        let req = BatchRequest::new(query, self.connection.context().transaction_descriptor());

        let id = self.connection.context_mut().next_packet_id();
        self.connection.send(PacketHeader::batch(id), req).await?;

        let token_stream = TokenStream::new(&mut self.connection).try_unfold();

        let columns = token_stream
            .try_fold(None, |mut columns, token| async move {
                if let ReceivedToken::NewResultset(metadata) = token {
                    columns = Some(metadata.columns.clone());
                };

                Ok(columns)
            })
            .await?;

        // now start bulk upload
        let columns: Vec<_> = columns
            .ok_or_else(|| {
                crate::Error::Protocol("expecting column metadata from query but not found".into())
            })?
            .into_iter()
            .filter(|column| column.base.flags.contains(ColumnFlag::Updateable))
            .collect();

        self.connection.flush_stream().await?;
        let col_data = columns.iter().map(|c| format!("{}", c)).join(", ");
        let query = format!("INSERT BULK {} ({})", table, col_data);

        let req = BatchRequest::new(query, self.connection.context().transaction_descriptor());
        let id = self.connection.context_mut().next_packet_id();

        self.connection.send(PacketHeader::batch(id), req).await?;

        let ts = TokenStream::new(&mut self.connection);
        ts.flush_done().await?;

        BulkLoadRequest::new(&mut self.connection, columns)
    }

    pub(crate) fn rpc_params<'a>(query: impl Into<Cow<'a, str>>) -> Vec<RpcParam<'a>> {
        log::debug!("rpc_params");
        vec![
            RpcParam {
                name: Cow::Borrowed("stmt"),
                flags: BitFlags::empty(),
                value: ColumnData::String(Some(query.into())),
            },
            RpcParam {
                name: Cow::Borrowed("params"),
                flags: BitFlags::empty(),
                value: ColumnData::I32(Some(0)),
            },
        ]
    }

    pub(crate) async fn rpc_perform_query<'a, 'b>(
        &'a mut self,
        proc_id: RpcProcId,
        mut rpc_params: Vec<RpcParam<'b>>,
        params: impl Iterator<Item = ColumnData<'b>>,
    ) -> crate::Result<()>
    where
        'a: 'b,
    {
        log::debug!("rpc_perform_query");
        let mut param_str = String::new();

        for (i, param) in params.enumerate() {
            log::debug!(" - adding param {:?}, type = {:?}", i, &param.type_name());
            if i > 0 {
                param_str.push(',')
            }
            param_str.push_str(&format!("@P{} ", i + 1));
            param_str.push_str(&param.type_name());

            rpc_params.push(RpcParam {
                name: Cow::Owned(format!("@P{}", i + 1)),
                flags: BitFlags::empty(),
                value: param,
            });
        }
        log::debug!(" - finding 'params' param");
        if let Some(params) = rpc_params.iter_mut().find(|x| x.name == "params") {
            log::debug!(" - updating 'params' param value to {:?}", param_str);
            params.value = ColumnData::String(Some(param_str.into()));
        }
        log::debug!(" - creating RPC request");
        let req = TokenRpcRequest::new(
            proc_id,
            rpc_params,
            self.connection.context().transaction_descriptor(),
        );
        log::debug!(" - getting next packet ID");
        let id = self.connection.context_mut().next_packet_id();
        log::debug!(" - got next packet ID = {:?}", id);
        log::debug!(" - sending RPC packet(s)");
        self.connection.send(PacketHeader::rpc(id), req).await?;
        log::debug!(" - done rpc_perform_query");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use async_std::net::TcpStream;
    use super::*;
    use bytes::{Bytes, BytesMut};
    use crate::{Encode, IntoSql};

    fn encode_query<'b>(
        query: impl Into<Cow<'b, str>>,
        params: &'b [&'b dyn ToSql],
    ) -> anyhow::Result<()> {
        let mut rpc_params: Vec<RpcParam> = Client::<TcpStream>::rpc_params(query);

        let proc_id = RpcProcId::ExecuteSQL;

        let params = params.iter().map(|p| p.to_sql());
        let mut param_str = String::new();
        for (i, param) in params.enumerate() {
            log::debug!(" - adding param {:?}, type = {:?}", i, &param.type_name());
            if i > 0 {
                param_str.push(',')
            }
            param_str.push_str(&format!("@P{} ", i + 1));
            param_str.push_str(&param.type_name());

            rpc_params.push(RpcParam {
                name: Cow::Owned(format!("@P{}", i + 1)),
                flags: BitFlags::empty(),
                value: param,
            });
        }
        log::debug!(" - finding 'params' param");
        if let Some(params) = rpc_params.iter_mut().find(|x| x.name == "params") {
            log::debug!(" - updating 'params' param value to {:?}", param_str);
            params.value = ColumnData::String(Some(param_str.into()));
        }
        let transaction_desc = [0;8];
        log::debug!(" - creating RPC request");
        let req = TokenRpcRequest::new(
            proc_id,
            rpc_params,
            transaction_desc,
        );
        let mut bytes = BytesMut::new();
        req.encode(&mut bytes)?;

        log::warn!("encoded to bytes:");
        for chunk in bytes.chunks(16) {
            log::warn!("{:?}", hex::encode(chunk))
        }
        Ok(())
    }

    #[tokio::test]
    pub async fn test_rpc_bit() -> anyhow::Result<()>{
        log4rs::init_file("log4rs.yaml", Default::default()).unwrap_or(());
        log::warn!("test_rpc_bit");
        let sql = "select @P1";
        let params: [&dyn ToSql; 1] = [
            &false
        ];
        encode_query(sql, &params)?;
        Ok(())
    }

    #[tokio::test]
    pub async fn test_rpc_u8() -> anyhow::Result<()>{
        log4rs::init_file("log4rs.yaml", Default::default()).unwrap_or(());
        log::warn!("test_rpc_u8");
        let sql = "select @P1";
        let params: [&dyn ToSql; 1] = [
            &1u8
        ];
        encode_query(sql, &params)?;
        Ok(())
    }

    #[tokio::test]
    pub async fn test_rpc_i16() -> anyhow::Result<()>{
        log4rs::init_file("log4rs.yaml", Default::default()).unwrap_or(());
        log::warn!("test_rpc_i16");
        let sql = "select @P1";
        let params: [&dyn ToSql; 1] = [
            &i16::MAX
        ];
        encode_query(sql, &params)?;
        Ok(())
    }

    #[tokio::test]
    pub async fn test_rpc_i32() -> anyhow::Result<()>{
        log4rs::init_file("log4rs.yaml", Default::default()).unwrap_or(());
        log::warn!("test_rpc_i32");
        let sql = "select @P1";
        let params: [&dyn ToSql; 1] = [
            &i32::MAX
        ];
        encode_query(sql, &params)?;
        Ok(())
    }

    #[tokio::test]
    pub async fn test_rpc_i64() -> anyhow::Result<()>{
        log4rs::init_file("log4rs.yaml", Default::default()).unwrap_or(());
        log::warn!("test_rpc_i64");
        let sql = "select @P1";
        let params: [&dyn ToSql; 1] = [
            &i64::MAX
        ];
        encode_query(sql, &params)?;
        Ok(())
    }

    #[tokio::test]
    pub async fn test_rpc_f32() -> anyhow::Result<()>{
        log4rs::init_file("log4rs.yaml", Default::default()).unwrap_or(());
        log::warn!("test_rpc_f32");
        let sql = "select @P1";
        let params: [&dyn ToSql; 1] = [
            &f32::MAX
        ];
        encode_query(sql, &params)?;
        Ok(())
    }

    #[tokio::test]
    pub async fn test_rpc_f64() -> anyhow::Result<()>{
        log4rs::init_file("log4rs.yaml", Default::default()).unwrap_or(());
        log::warn!("test_rpc_f64");
        let sql = "select @P1";
        let params: [&dyn ToSql; 1] = [
            &f64::MAX
        ];
        encode_query(sql, &params)?;
        Ok(())
    }

    #[tokio::test]
    pub async fn test_rpc_string() -> anyhow::Result<()>{
        log4rs::init_file("log4rs.yaml", Default::default()).unwrap_or(());
        log::warn!("test_rpc_string");
        let sql = "select @P1";
        let params: [&dyn ToSql; 1] = [
            &("abcd".to_string())
        ];
        encode_query(sql, &params)?;
        Ok(())
    }

    #[tokio::test]
    pub async fn test_rpc_0() -> anyhow::Result<()>{
        log4rs::init_file("log4rs.yaml", Default::default()).unwrap_or(());
        log::warn!("test_rpc_0");
        let sql = "select @P1, @P2, @P3, @P4, @P5, @P6, @P7, @P8, @P9";
        let params: [&dyn ToSql; 9] = [
            &false,
            &2u8,
            &3i16,
            &4i32,
            &5i64,
            &6.123456789f32,
            &7.123456789f64,
            &"8abc",
            &("9def".to_string()),
        ];
        encode_query(sql, &params)?;
        Ok(())
    }

}