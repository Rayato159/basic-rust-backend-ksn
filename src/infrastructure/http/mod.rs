pub mod routes;

use crate::{
    application::models::{
        login::LoginModel,
        register::{RegisterModel, RegisterResult},
    },
    infrastructure::{
        config::DotEnvyConfig,
        database::MSSQLClient,
        http::routes::{login, register},
        jwt_auth::Passport,
        secret::JWTSecret,
    },
};
use anyhow::Result;
use axum::{
    Router,
    http::{Method, StatusCode},
    response::IntoResponse,
    routing::get,
};
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::net::TcpListener;
use tokio::sync::Mutex;
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
#[openapi(
    paths(
        crate::infrastructure::http::routes::login::login,
        crate::infrastructure::http::routes::register::register,
        health_check
    ),
    components(
        schemas(LoginModel, Passport, RegisterModel, RegisterResult)
    ),
    tags(
        (name = "Login", description = "Login and get access token"),
        (name = "Register", description = "Register a new user")
    )
)]
struct ApiDoc;

pub async fn start(
    shared_config: Arc<DotEnvyConfig>,
    shared_db_conn: Arc<Mutex<MSSQLClient>>,
    shared_jwt_secret: Arc<JWTSecret>,
) -> Result<()> {
    let app = Router::new()
        .fallback(not_found)
        .route("/health-check", get(health_check))
        .nest(
            "/login",
            login::routes(Arc::clone(&shared_db_conn), Arc::clone(&shared_jwt_secret)),
        )
        .nest("/register", register::routes(Arc::clone(&shared_db_conn)))
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
