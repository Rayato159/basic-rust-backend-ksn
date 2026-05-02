use axum::{
    Json, Router,
    extract::{FromRef, State},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    application::{
        models::transfer::{TransferModel, TransferResult},
        usecases::transfer::TransferUseCase,
    },
    infrastructure::{
        database::{MSSQLClient, repositories::transfer::TransferMSSQL},
        http::middleware::AuthCheck,
        secret::JWTSecret,
    },
};

#[derive(Clone)]
pub struct TransferState {
    transfer_use_case: Arc<TransferUseCase<TransferMSSQL>>,
    jwt_secret: Arc<JWTSecret>,
}

impl FromRef<TransferState> for Arc<JWTSecret> {
    fn from_ref(state: &TransferState) -> Arc<JWTSecret> {
        state.jwt_secret.clone()
    }
}

pub fn routes(db_pool: Arc<Mutex<MSSQLClient>>, jwt_secret: Arc<JWTSecret>) -> Router {
    let transfer_repository = TransferMSSQL::new(db_pool);
    let transfer_use_case = TransferUseCase::new(Arc::new(transfer_repository));

    let state = TransferState {
        transfer_use_case: Arc::new(transfer_use_case),
        jwt_secret,
    };

    Router::new()
        .route("/", post(create_transfer))
        .with_state(state)
}

#[utoipa::path(
    post,
    path = "/transfer",
    request_body = TransferModel,
    responses(
        (status = 200, description = "Transfer created successfully", body = TransferResult),
        (status = 401, description = "Missing or invalid token", body = String),
        (status = 500, description = "Internal server error", body = String)
    ),
    tag = "Transfer",
    security(("bearer_auth" = []))
)]
pub async fn create_transfer(
    State(state): State<TransferState>,
    user_id: AuthCheck,
    Json(transfer_model): Json<TransferModel>,
) -> impl IntoResponse {
    match state
        .transfer_use_case
        .create_transfer(user_id.0, transfer_model)
        .await
    {
        Ok(transfer_result) => (StatusCode::OK, Json(transfer_result)).into_response(),
        Err(e) => {
            tracing::error!(error = %e, user_id = %user_id.0, "Failed to create transfer");
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}
