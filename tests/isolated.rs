use futures::{AsyncRead, AsyncWrite};
use futures_util::TryStreamExt;
use names::{Generator, Name};
use once_cell::sync::Lazy;
use std::cell::RefCell;
use std::env;
use std::sync::Once;

use tiberius::FromSql;
use tiberius::{numeric::Numeric, xml::XmlData, ColumnType, Query, QueryItem, Result};
use uuid::Uuid;

use runtimes_macro::test_on_runtimes;


#[tokio::test]
async fn connect_once() -> Result<()>
{
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap_or(());
    log::debug!("connect_once");
    let conn_str = "server=tcp:localhost,1433;IntegratedSecurity=true;TrustServerCertificate=true;Encrypt=No";
    let config = tiberius::Config::from_ado_string(conn_str)?;
    let tcp = async_std::net::TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;
    let mut _client = tiberius::Client::connect(config, tcp).await?;
    // let row = conn
    //     .query("SELECT @P1", &[&-4i32])
    //     .await?
    //     .into_row()
    //     .await?
    //     .unwrap();
    //
    // assert_eq!(Some(-4i32), row.get(0));

    Ok(())
}
