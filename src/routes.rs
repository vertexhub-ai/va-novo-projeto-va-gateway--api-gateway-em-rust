use axum::{
    routing::{get, any},
    Router,
};
use sqlx::PgPool;
use reqwest::Client;
use crate::handlers::{self, AppState};

pub fn api_router(pool: PgPool, upstream_url: String) -> Router {
    let http_client = Client::new();
    let app_state = AppState {
        db_pool: pool,
        http_client,
        upstream_url,
    };

    Router::new()
        // Example route that interacts with the database
        .route("/data", get(handlers::get_example_data))
        // Catch-all route for proxying requests to the upstream service
        .route("/*path", any(handlers::proxy_handler))
        .with_state(app_state)
}
