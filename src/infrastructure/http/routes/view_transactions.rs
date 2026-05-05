use axum::{
    Json, Router,
    extract::{FromRef, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

use crate::{
    application::{
        models::view_transactions::TransactionInfo,
        usecases::view_transactions::ViewTransactionsUseCase,
    },
    infrastructure::{
        database::{MSSQLClient, repositories::view_transactions::ViewTransactionsMSSQL},
        http::middleware::AuthCheck,
        secret::JWTSecret,
    },
};

#[derive(Clone)]
pub struct ViewTransactionsState {
    view_transactions_use_case: Arc<ViewTransactionsUseCase<ViewTransactionsMSSQL>>,
    jwt_secret: Arc<JWTSecret>,
}

impl FromRef<ViewTransactionsState> for Arc<JWTSecret> {
    fn from_ref(state: &ViewTransactionsState) -> Arc<JWTSecret> {
        state.jwt_secret.clone()
    }
}

pub fn routes(db_pool: Arc<Mutex<MSSQLClient>>, jwt_secret: Arc<JWTSecret>) -> Router {
    let view_transactions_repository = ViewTransactionsMSSQL::new(db_pool);
    let view_transactions_use_case =
        ViewTransactionsUseCase::new(Arc::new(view_transactions_repository));

    let state = ViewTransactionsState {
        view_transactions_use_case: Arc::new(view_transactions_use_case),
        jwt_secret,
    };

    Router::new()
        .route("/{user_id}", get(view_transactions))
        .route("/by-id/{transaction_id}", get(view_transaction_by_id))
        .with_state(state)
}

#[utoipa::path(
    get,
    path = "/transactions/{user_id}",
    params(
        ("user_id" = String, Path, description = "User ID (must match authenticated user ID)")
    ),
    responses(
        (status = 200, description = "Transactions retrieved successfully", body = Vec<TransactionInfo>),
        (status = 401, description = "Missing or invalid token", body = String),
        (status = 403, description = "Access denied: user_id does not match authenticated user", body = String),
        (status = 404, description = "User not found", body = String),
        (status = 500, description = "Internal server error", body = String)
    ),
    tag = "ViewTransactions",
    security(("bearer_auth" = []))
)]
pub async fn view_transactions(
    State(state): State<ViewTransactionsState>,
    user_id: AuthCheck,
    Path(path_user_id): Path<String>,
) -> impl IntoResponse {
    let path_user_id_uuid = match uuid::Uuid::parse_str(&path_user_id) {
        Ok(uuid) => uuid,
        Err(e) => {
            tracing::error!(error = %e, user_id_str = %path_user_id, "Invalid user_id format in path");
            return (
                StatusCode::BAD_REQUEST,
                "Invalid user_id format".to_string(),
            )
                .into_response();
        }
    };

    info!("user_id: {:?}", user_id.0);

    match state
        .view_transactions_use_case
        .view_transactions(user_id.0, path_user_id_uuid)
        .await
    {
        Ok(transactions) => {
            tracing::info!(
                user_id = %user_id.0,
                transaction_count = transactions.len(),
                "Successfully retrieved transactions"
            );
            (StatusCode::OK, Json(transactions)).into_response()
        }
        Err(e) => {
            let error_message = e.to_string();

            if error_message.contains("Access denied") {
                tracing::warn!(
                    user_id = %user_id.0,
                    path_user_id = %path_user_id,
                    "Access denied: user_id mismatch"
                );
                (StatusCode::FORBIDDEN, error_message).into_response()
            } else {
                tracing::error!(error = %e, user_id = %user_id.0, "Failed to retrieve transactions");
                (StatusCode::INTERNAL_SERVER_ERROR, error_message).into_response()
            }
        }
    }
}

#[utoipa::path(
        get,
        path = "/transactions/by-id/{transaction_id}",
        params(
            ("transaction_id" = String, Path, description = "Transaction ID")
        ),
        responses(
            (status = 200, description = "Transaction retrieved successfully", body = TransactionInfo),
            (status = 400, description = "Invalid transaction_id format", body = String),
            (status = 401, description = "Missing or invalid token", body = String),
            (status = 403, description = "Access denied: user_id does not match transaction owner", body = String),
            (status = 404, description = "Transaction not found", body = String),
            (status = 500, description = "Internal server error", body = String)
        ),
        tag = "ViewTransactions",
        security(("bearer_auth" = []))
    )]
pub async fn view_transaction_by_id(
    State(state): State<ViewTransactionsState>,
    user_id: AuthCheck,
    Path(transaction_id): Path<String>,
) -> impl IntoResponse {
    let transaction_id_uuid = match uuid::Uuid::parse_str(&transaction_id) {
        Ok(uuid) => uuid,
        Err(e) => {
            tracing::error!(error = %e, transaction_id_str = %transaction_id, "Invalid transaction_id format in path");
            return (
                StatusCode::BAD_REQUEST,
                "Invalid transaction_id format".to_string(),
            )
                .into_response();
        }
    };

    info!(
        user_id = %user_id.0,
        transaction_id = %transaction_id_uuid,
        "Fetching transaction by ID"
    );

    match state
        .view_transactions_use_case
        .view_transaction_by_id(user_id.0, transaction_id_uuid)
        .await
    {
        Ok(transaction_info) => {
            tracing::info!(
                user_id = %user_id.0,
                transaction_id = %transaction_id_uuid,
                "Successfully retrieved transaction by ID"
            );
            (StatusCode::OK, Json(transaction_info)).into_response()
        }
        Err(e) => {
            let error_message = e.to_string();

            if error_message.contains("not found") {
                tracing::warn!(
                    user_id = %user_id.0,
                    transaction_id = %transaction_id_uuid,
                    "Transaction not found"
                );
                (StatusCode::NOT_FOUND, error_message).into_response()
            } else if error_message.contains("Access denied") {
                tracing::warn!(
                    user_id = %user_id.0,
                    transaction_id = %transaction_id_uuid,
                    "Access denied: user_id does not match transaction owner"
                );
                (StatusCode::FORBIDDEN, error_message).into_response()
            } else {
                tracing::error!(
                    error = %e,
                    user_id = %user_id.0,
                    transaction_id = %transaction_id_uuid,
                    "Failed to retrieve transaction by ID"
                );
                (StatusCode::INTERNAL_SERVER_ERROR, error_message).into_response()
            }
        }
    }
}
