use anyhow::Result;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Passport {
    pub access_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

// Example of usage:
// &Claims {
//     sub: sub.to_string(),
//     exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
//     iat: Utc::now().timestamp() as usize,
// }

pub fn generate_token(secret: String, claims: &Claims) -> Result<String> {
    // HSA256
    let token = encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;

    Ok(token)
}

pub fn verify_token(secret: String, token: String) -> Result<Claims> {
    let token = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?;

    Ok(token.claims)
}
