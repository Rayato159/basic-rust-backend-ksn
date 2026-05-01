use anyhow::Result;
use tiberius::Client;
use tokio::net::TcpStream;
use tokio_util::compat::Compat;

pub const SCRIPTS: [(&str, &str); 1] = [("001_init_db", include_str!("./scripts/001_init_db.sql"))];

pub async fn run_migrations(client: &mut Client<Compat<TcpStream>>) -> Result<()> {
    for (name, sql) in SCRIPTS {
        println!("  - Running: {}...", name);
        client
            .simple_query(sql)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to run migration {}: {}", name, e))?;
    }

    Ok(())
}
