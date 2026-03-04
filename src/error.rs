use axum::response::{IntoResponse, Response};
use http::StatusCode;
use tracing::error;

#[derive(Debug)]
pub enum AppError {
    DbError(String),
    ProxyError(String),
    // Add other error types as needed
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::DbError(msg) => {
                error!("Database error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string())
            }
            AppError::ProxyError(msg) => {
                error!("Proxy error: {}", msg);
                (StatusCode::BAD_GATEWAY, "Upstream service error".to_string())
            }
        };

        (status, error_message).into_response()
    }
}
