use basic_rust_backend_ksn::infrastructure::{
    config::load, database::mssql_connect, secret::get_db_credentials,
};
use migrations::run_migrations;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let dotenvy_env = match load() {
        Ok(env) => {
            info!("ENV has been loaded");
            env
        }
        Err(e) => {
            error!("Failed to load ENV: {}", e);
            std::process::exit(1);
        }
    };

    let db_creds =
        match get_db_credentials(&dotenvy_env.vault.token, &dotenvy_env.vault.address).await {
            Ok(creds) => {
                info!("DB credentials has been loaded");
                creds
            }
            Err(e) => {
                error!("Failed to load DB credentials: {}", e);
                std::process::exit(1);
            }
        };

    let mut mssql_conn = match mssql_connect(&db_creds).await {
        Ok(conn) => {
            info!("MSSQL connection has been established");
            conn
        }
        Err(e) => {
            error!("Failed to connect to MSSQL: {:?}", e);
            std::process::exit(1);
        }
    };

    match run_migrations(&mut mssql_conn).await {
        Ok(_) => info!("Migration process finished successfully"),
        Err(e) => {
            error!("Migration failed: {}", e);
            std::process::exit(1);
        }
    }
}
