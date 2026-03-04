use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
    http::{StatusCode, Request, HeaderMap},
    body::Body,
};
use http::Uri;
use reqwest::Client;
use tracing::debug;
use crate::error::AppError;

// State struct to hold shared resources like the HTTP client and upstream URL
#[derive(Clone)]
pub struct AppState {
    pub db_pool: sqlx::PgPool,
    pub http_client: Client,
    pub upstream_url: String,
}

pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

pub async fn proxy_handler(
    State(state): State<AppState>,
    Path(path): Path<String>,
    mut req: Request<Body>,
) -> Result<Response, AppError> {
    let client = &state.http_client;
    let upstream_base = &state.upstream_url;

    let uri_string = format!("{}/{}", upstream_base, path);
    let uri = uri_string.parse::<Uri>()
        .map_err(|e| AppError::ProxyError(format!("Invalid upstream URI: {}", e)))?;

    // Reconstruct the request for the upstream service
    *req.uri_mut() = uri;

    // Remove headers that might cause issues or are not relevant for proxying
    let headers = req.headers_mut();
    headers.remove(http::header::HOST);
    headers.remove(http::header::CONNECTION);
    headers.remove("x-forwarded-for"); // Or handle it properly if needed

    debug!("Proxying request to: {:?} {}", req.method(), req.uri());

    let response = client
        .execute(req.try_into().map_err(|e| AppError::ProxyError(format!("Failed to convert Axum request to reqwest: {}", e)))?)
        .await
        .map_err(|e| AppError::ProxyError(format!("Failed to send request to upstream: {}", e)))?;

    // Convert reqwest::Response to axum::response::Response
    let mut builder = Response::builder()
        .status(response.status());

    // Copy headers from upstream response to our response
    let mut headers = HeaderMap::new();
    for (key, value) in response.headers() {
        headers.insert(key, value.clone());
    }
    builder = builder.header_map(headers);

    let body = Body::from(response.bytes().await.map_err(|e| AppError::ProxyError(format!("Failed to read upstream response body: {}", e)))?);

    builder.body(body).map_err(|e| AppError::ProxyError(format!("Failed to build Axum response: {}", e)))
}

// Example of a database interaction handler (not directly gateway related, but for completeness)
pub async fn get_example_data(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    // Example: Query the database
    let row: (i32,) = sqlx::query_as("SELECT 1")
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| AppError::DbError(e.to_string()))?;

    Ok(format!("Database returned: {}", row.0))
}
