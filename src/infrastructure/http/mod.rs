pub mod routes;

use crate::infrastructure::{config::DotEnvyConfig, database::MSSQLClient, secret::JWTSecret};
use anyhow::Result;
use axum::{
    Router,
    http::{Method, StatusCode},
    response::IntoResponse,
    routing::get,
};
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::net::TcpListener;
use tower_http::{
    cors::{Any, CorsLayer},
    limit::RequestBodyLimitLayer,
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(paths(health_check), components())]
struct ApiDoc;

pub async fn start(
    shared_config: Arc<DotEnvyConfig>,
    shared_db_conn: Arc<Mutex<MSSQLClient>>,
    shared_jwt_secret: Arc<JWTSecret>,
) -> Result<()> {
    let app = Router::new()
        .fallback(not_found)
        .route("/health-check", get(health_check))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(shared_config.server.timeout),
        ))
        .layer(RequestBodyLimitLayer::new(
            (shared_config.server.body_limit * 1024 * 1024).try_into()?,
        ))
        .layer(
            CorsLayer::new()
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::PATCH,
                    Method::DELETE,
                ])
                .allow_origin(Any),
        )
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], shared_config.server.port));

    let listener = TcpListener::bind(addr).await?;

    info!("Server running on port {}", shared_config.server.port);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => info!("Received Ctrl+C signal"),
        _ = terminate => info!("Received terminate signal"),
    }
}

pub async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not Found").into_response()
}

#[utoipa::path(
    get,
    path = "/health-check",
    responses(
        (status = 200, description = "Health check successful", body = String)
    )
)]
pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "OK").into_response()
}
