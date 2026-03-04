#![cfg(test)]

use axum::{
    body::Body, 
    http::{Request, StatusCode},
    Router,
};
use http_body_util::BodyExt;
use tower::ServiceExt; // for `call`, `oneshot`, and `ready`
use sqlx::{PgPool, Executor};
use crate::{config::AppConfig, db::setup_db_pool, routes::api_router, handlers::health_check};

// Helper to create a test app instance
async fn app_for_test(pool: PgPool, upstream_url: String) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .nest("/api", api_router(pool, upstream_url))
}

// Helper to set up a test database
async fn setup_test_db() -> PgPool {
    let config = AppConfig::load().expect("Failed to load config for test DB");
    let test_db_url = format!("{}_test", config.database_url);

    // Connect to the default postgres database to create the test database
    let mut root_conn = PgPool::connect("postgres://user:password@localhost:5432/postgres").await
        .expect("Failed to connect to root postgres DB");

    // Drop and create test database
    root_conn.execute(format!("DROP DATABASE IF EXISTS {}", "va_gateway_db_test").as_str()).await
        .expect("Failed to drop test DB");
    root_conn.execute(format!("CREATE DATABASE {}", "va_gateway_db_test").as_str()).await
        .expect("Failed to create test DB");

    // Connect to the newly created test database
    setup_db_pool(&test_db_url).await.expect("Failed to connect to test database")
}

#[tokio::test]
async fn health_check_returns_200_ok() {
    let pool = setup_test_db().await;
    let config = AppConfig::load().expect("Failed to load config");
    let app = app_for_test(pool, config.upstream_service_url).await;

    let response = app
        .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"OK");
}

#[tokio::test]
async fn proxy_route_returns_bad_gateway_for_unreachable_upstream() {
    let pool = setup_test_db().await;
    // Use a non-existent upstream URL to simulate an unreachable service
    let app = app_for_test(pool, "http://127.0.0.1:9999".to_string()).await;

    let response = app
        .oneshot(Request::builder().uri("/api/some/path").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_GATEWAY);
}

// Note: To test a successful proxy, you would need to run a mock upstream service
// alongside your tests, which is beyond a basic scaffold.
