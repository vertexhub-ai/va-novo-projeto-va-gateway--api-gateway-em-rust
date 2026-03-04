mod config;
mod db;
mod handlers;
mod routes;
mod error;

use axum::{
    routing::get,
    Router,
};
use tower_http::{
    trace::{DefaultMakeSpan, TraceLayer},
    cors::{CorsLayer, Any},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::net::SocketAddr;
use crate::config::AppConfig;
use crate::db::setup_db_pool;
use crate::routes::api_router;

#[tokio::main]
async fn main() {
    // Initialize tracing (logging)
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "va_novo_projeto_va_gateway__api_gateway_em_rust=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting va-gateway API server...");

    // Load configuration
    let config = match AppConfig::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            tracing::error!("Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    };
    tracing::info!("Configuration loaded: {:?}", config);

    // Setup database connection pool
    let pool = match setup_db_pool(&config.database_url).await {
        Ok(p) => p,
        Err(e) => {
            tracing::error!("Failed to connect to database: {}", e);
            std::process::exit(1);
        }
    };
    tracing::info!("Database connection pool established.");

    // Define CORS policy
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build our application with a route
    let app = Router::new()
        .route("/health", get(handlers::health_check))
        .nest("/api", api_router(pool, config.upstream_service_url))
        .layer(cors)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        );

    // Run our app with hyper
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}
