use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use tokio::sync::Mutex;

use crate::{
    application::{
        models::register::{RegisterModel, RegisterResult},
        usecases::register::RegisterUseCase,
    },
    infrastructure::database::{MSSQLClient, repositories::register::RegisterMSSQL},
};

pub fn routes(db_pool: Arc<Mutex<MSSQLClient>>) -> Router {
    let register_repository = RegisterMSSQL::new(db_pool);
    let register_use_case = RegisterUseCase::new(Arc::new(register_repository));

    Router::new()
        .route("/", post(register))
        .with_state(Arc::new(register_use_case))
}

#[utoipa::path(
    post,
    path = "/register",
    request_body = RegisterModel,
    responses(
        (status = 201, description = "Create user success", body = RegisterResult),
        (status = 500, description = "Error", body = String)
    ),
    tag = "Register"
)]
pub async fn register(
    State(register_use_case): State<Arc<RegisterUseCase<RegisterMSSQL>>>,
    Json(register_model): Json<RegisterModel>,
) -> impl IntoResponse {
    match register_use_case.register(register_model).await {
        Ok(register_result) => (StatusCode::CREATED, Json(register_result)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
