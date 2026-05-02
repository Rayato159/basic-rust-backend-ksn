use basic_rust_backend_ksn::infrastructure::{
    config::load,
    database::mssql_connect,
    http::start,
    secret::{get_db_credentials, get_jwt_secret},
};

use std::sync::Arc;
use tokio::sync::Mutex;
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

    let jwt_secret =
        match get_jwt_secret(&dotenvy_env.vault.token, &dotenvy_env.vault.address).await {
            Ok(secret) => {
                info!("Secret has been loaded");
                secret
            }
            Err(e) => {
                error!("Failed to load DB credentials: {}", e);
                std::process::exit(1);
            }
        };

    let mssql_conn = match mssql_connect(&db_creds).await {
        Ok(conn) => {
            info!("MSSQL connection has been established");
            conn
        }
        Err(e) => {
            error!("Failed to connect to MSSQL: {:?}", e);
            std::process::exit(1);
        }
    };

    let shared_mssql_conn = Arc::new(Mutex::new(mssql_conn));
    let shared_dotenvy_env = Arc::new(dotenvy_env);
    let shared_jwt_secret = Arc::new(jwt_secret);

    start(shared_dotenvy_env, shared_mssql_conn, shared_jwt_secret)
        .await
        .expect("Failed to start server");
}
