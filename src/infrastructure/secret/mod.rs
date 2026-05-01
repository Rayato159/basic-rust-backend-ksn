use anyhow::Result;
use serde::Deserialize;
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};
use vaultrs::kv2;

#[derive(Deserialize, Debug, Clone)]
pub struct DbCreds {
    pub host: String,
    pub port: String,
    pub database: String,
    pub username: String,
    pub password: String,
}

pub async fn get_db_credentials(token: &str, address: &str) -> Result<DbCreds> {
    let client = VaultClient::new(
        VaultClientSettingsBuilder::default()
            .address(address)
            .token(token)
            .build()?,
    )?;

    let secret = kv2::read::<DbCreds>(&client, "secret", "mssql").await?;

    Ok(secret)
}

#[derive(Deserialize, Debug, Clone)]
pub struct JWTSecret {
    pub secret: String,
    pub issuer: String,
    pub expiration: String,
}

pub async fn get_jwt_secret(token: &str, address: &str) -> Result<JWTSecret> {
    let client = VaultClient::new(
        VaultClientSettingsBuilder::default()
            .address(address)
            .token(token)
            .build()?,
    )?;

    let secret = kv2::read::<JWTSecret>(&client, "secret", "jwt").await?;
    Ok(secret)
}
