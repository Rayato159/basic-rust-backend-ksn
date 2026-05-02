use std::sync::Arc;

use crate::{
    application::models::login::LoginModel,
    domain::repositories::login::LoginRepository,
    infrastructure::{
        argon2_hashing,
        jwt_auth::{Claims, Passport, generate_token},
    },
};
use anyhow::Result;
use chrono::{Duration, Utc};

pub struct LoginUseCase<T>
where
    T: LoginRepository + Send + Sync + 'static,
{
    login_repository: Arc<T>,
}

impl<T> LoginUseCase<T>
where
    T: LoginRepository + Send + Sync + 'static,
{
    pub fn new(login_repository: Arc<T>) -> Self {
        Self { login_repository }
    }

    pub async fn login(
        &self,
        login_model: LoginModel,
        jwt_secret: String,
        jwt_expiration: String,
    ) -> Result<Passport> {
        // Get user from repository
        let login_dto = self
            .login_repository
            .get_user(&login_model.username)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Invalid username or password"))?;

        // Verify password
        let is_valid = argon2_hashing::verify(login_model.password, login_dto.password)?;

        if !is_valid {
            return Err(anyhow::anyhow!("Invalid username or password"));
        }

        // Parse expiration duration (format: "24h", "1d", "3600s", etc.)
        let expiration_hours = parse_expiration_to_hours(&jwt_expiration)?;

        // Generate JWT token
        let claims = Claims {
            sub: login_dto.id.to_string(),
            exp: (Utc::now() + Duration::hours(expiration_hours)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };

        let access_token = generate_token(jwt_secret, &claims)?;

        Ok(Passport { access_token })
    }
}

/// Parse expiration string to hours
/// Supports formats: "24h", "1d", "3600s", "86400" (assumed seconds)
fn parse_expiration_to_hours(expiration: &str) -> Result<i64> {
    let expiration = expiration.trim().to_lowercase();

    if expiration.ends_with('h') {
        // Hours format: "24h"
        let num: i64 = expiration[..expiration.len() - 1]
            .trim()
            .parse()
            .map_err(|e| anyhow::anyhow!("Invalid expiration format: {}", e))?;
        Ok(num)
    } else if expiration.ends_with('d') {
        // Days format: "1d"
        let num: i64 = expiration[..expiration.len() - 1]
            .trim()
            .parse()
            .map_err(|e| anyhow::anyhow!("Invalid expiration format: {}", e))?;
        Ok(num * 24)
    } else if expiration.ends_with('s') {
        // Seconds format: "3600s"
        let num: i64 = expiration[..expiration.len() - 1]
            .trim()
            .parse()
            .map_err(|e| anyhow::anyhow!("Invalid expiration format: {}", e))?;
        Ok(num / 3600)
    } else {
        // Assume it's seconds if no unit specified
        let num: i64 = expiration
            .trim()
            .parse()
            .map_err(|e| anyhow::anyhow!("Invalid expiration format: {}", e))?;
        Ok(num / 3600)
    }
}
