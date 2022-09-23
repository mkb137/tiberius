#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_extern_crates)]

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
async fn connect_once() -> Result<()> {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap_or(());
    log::trace!("connect_once");
    let conn_str = env::var("TIBERIUS_TEST_CONNECTION_STRING").unwrap_or_else(|_| {"server=tcp:localhost,1433;IntegratedSecurity=true;TrustServerCertificate=false;Encrypt=No".to_string()});
    log::warn!(" - connection string = {:?}", conn_str);
    let config = tiberius::Config::from_ado_string(conn_str.as_str())?;
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
