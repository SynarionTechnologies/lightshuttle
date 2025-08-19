use axum::{body::Body, http::Request};
use lightshuttle_core::api::routes::router;
use tempfile::NamedTempFile;
use tower::ServiceExt; // for oneshot

#[tokio::test]
async fn rejects_missing_api_key() {
    let file = NamedTempFile::new().unwrap();
    std::fs::write(
        file.path(),
        r#"{"secret":{"name":"test","read":true,"write":true}}"#,
    )
    .unwrap();
    std::env::set_var("API_KEYS_FILE", file.path());

    let app = router();
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), axum::http::StatusCode::UNAUTHORIZED);
    std::env::remove_var("API_KEYS_FILE");
}

#[tokio::test]
async fn accepts_valid_api_key() {
    let file = NamedTempFile::new().unwrap();
    std::fs::write(
        file.path(),
        r#"{"secret":{"name":"test","read":true,"write":true}}"#,
    )
    .unwrap();
    std::env::set_var("API_KEYS_FILE", file.path());

    let app = router();
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/health")
                .header("x-api-key", "secret")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), axum::http::StatusCode::OK);
    std::env::remove_var("API_KEYS_FILE");
}
