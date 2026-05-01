use anyhow::Result;
use tiberius::{AuthMethod, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

use crate::infrastructure::secret::DbCreds;

pub type MSSQLClient = Client<Compat<TcpStream>>;

pub async fn mssql_connect(db_creds: &DbCreds) -> Result<MSSQLClient> {
    let mut config = Config::new();

    let port: u16 = db_creds
        .port
        .parse()
        .map_err(|e| anyhow::anyhow!("Parse port to u16 error: {:?}", e))?;

    config.host(db_creds.host.to_string());
    config.port(port);
    config.database(db_creds.database.to_string());
    config.authentication(AuthMethod::sql_server(
        db_creds.username.to_string(),
        db_creds.password.to_string(),
    ));
    config.trust_cert();

    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    let client = Client::connect(config, tcp.compat_write()).await?;

    Ok(client)
}
