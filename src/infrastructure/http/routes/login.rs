use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use tokio::sync::Mutex;

use crate::{
    application::{models::login::LoginModel, usecases::login::LoginUseCase},
    infrastructure::{
        database::{MSSQLClient, repositories::login::LoginMSSQL},
        jwt_auth::Passport,
        secret::JWTSecret,
    },
};

#[derive(Clone)]
pub struct LoginState {
    login_use_case: Arc<LoginUseCase<LoginMSSQL>>,
    jwt_secret: Arc<JWTSecret>,
}

pub fn routes(db_pool: Arc<Mutex<MSSQLClient>>, jwt_secret: Arc<JWTSecret>) -> Router {
    let login_repository = LoginMSSQL::new(db_pool);
    let login_use_case = LoginUseCase::new(Arc::new(login_repository));

    let state = LoginState {
        login_use_case: Arc::new(login_use_case),
        jwt_secret,
    };

    Router::new().route("/", post(login)).with_state(state)
}

#[utoipa::path(
    post,
    path = "/login",
    request_body = LoginModel,
    responses(
        (status = 200, description = "Login successful", body = Passport),
        (status = 401, description = "Invalid credentials", body = String),
        (status = 500, description = "Error", body = String)
    ),
    tag = "Login"
)]
pub async fn login(
    State(state): State<LoginState>,
    Json(login_model): Json<LoginModel>,
) -> impl IntoResponse {
    match state
        .login_use_case
        .login(
            login_model,
            state.jwt_secret.secret.clone(),
            state.jwt_secret.expiration.clone(),
        )
        .await
    {
        Ok(passport_result) => (StatusCode::OK, Json(passport_result)).into_response(),
        Err(e) => {
            if e.to_string().contains("Invalid username or password") {
                (StatusCode::UNAUTHORIZED, e.to_string()).into_response()
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
        }
    }
}
