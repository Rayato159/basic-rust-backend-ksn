use std::fmt;

use anyhow::Result;

#[derive(Debug, Clone)]
pub struct DotEnvyConfig {
    pub server: Server,
    pub vault: Vault,
}

#[derive(Debug, Clone)]
pub struct Server {
    pub port: u16,
    pub body_limit: u64,
    pub timeout: u64,
}

#[derive(Debug, Clone)]
pub struct Vault {
    pub address: String,
    pub token: String,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Stage {
    #[default]
    Dev,
    Prod,
}

impl fmt::Display for Stage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let stage = match self {
            Stage::Dev => "Dev",
            Stage::Prod => "Prod",
        };
        write!(f, "{}", stage)
    }
}

impl Stage {
    pub fn try_from(stage: &str) -> Result<Self> {
        match stage {
            "Dev" => Ok(Self::Dev),
            "Prod" => Ok(Self::Prod),
            _ => Err(anyhow::anyhow!("Invalid stage")),
        }
    }

    pub fn get() -> Self {
        dotenvy::dotenv().ok();

        let stage_str = std::env::var("STAGE").unwrap_or("".to_string());
        Self::try_from(&stage_str).unwrap_or_default()
    }
}

pub fn load() -> Result<DotEnvyConfig> {
    dotenvy::dotenv().ok();

    let server = Server {
        port: std::env::var("SERVER_PORT")
            .expect("SERVER_PORT is invalid")
            .parse()?,
        body_limit: std::env::var("SERVER_BODY_LIMIT")
            .expect("SERVER_BODY_LIMIT is invalid")
            .parse()?,
        timeout: std::env::var("SERVER_TIMEOUT")
            .expect("SERVER_TIMEOUT is invalid")
            .parse()?,
    };

    let vault = Vault {
        address: std::env::var("VAULT_ADDRESS").expect("VAULT_ADDRESS is invalid"),
        token: std::env::var("VAULT_TOKEN").expect("VAULT_TOKEN is invalid"),
    };

    Ok(DotEnvyConfig { server, vault })
}
