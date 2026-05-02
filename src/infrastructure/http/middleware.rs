use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
    response::IntoResponse,
};
use std::sync::Arc;
use utoipa::{
    Modify,
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
};
use uuid::Uuid;

use crate::infrastructure::{jwt_auth, secret::JWTSecret};

#[derive(Debug)]
pub enum AuthError {
    MissingToken,
    InvalidToken,
    InternalError,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AuthError::MissingToken => (
                axum::http::StatusCode::UNAUTHORIZED,
                "Missing authorization token",
            ),
            AuthError::InvalidToken => (
                axum::http::StatusCode::UNAUTHORIZED,
                "Invalid or expired token",
            ),
            AuthError::InternalError => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal authentication error",
            ),
        };
        (status, message).into_response()
    }
}

#[derive(Debug, Clone)]
pub struct AuthCheck(pub Uuid);

impl<S> FromRequestParts<S> for AuthCheck
where
    Arc<JWTSecret>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> std::result::Result<Self, Self::Rejection> {
        // Extract JWT secret from state
        let jwt_secret = Arc::<JWTSecret>::from_ref(state);

        // Extract Authorization header
        let auth_header = parts
            .headers
            .get("authorization")
            .ok_or(AuthError::MissingToken)?
            .to_str()
            .map_err(|_| AuthError::InvalidToken)?;

        // Parse Bearer token
        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(AuthError::InvalidToken)?;

        // Verify token and extract claims
        let claims = jwt_auth::verify_token(jwt_secret.secret.clone(), token.to_string())
            .map_err(|_| AuthError::InvalidToken)?;

        // Parse user_id from sub claim
        let user_id = Uuid::parse_str(&claims.sub).map_err(|_| AuthError::InvalidToken)?;

        Ok(AuthCheck(user_id))
    }
}

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        )
    }
}
