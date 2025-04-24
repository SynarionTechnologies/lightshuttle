use crate::routes::test_app;

use axum::{body::Body, http::{Request, StatusCode}};
use tower::ServiceExt;
use serde_json::json;

#[tokio::test]
async fn post_apps_should_succeed() {
    if std::env::var("DOCKER_TEST").is_err() {
        eprintln!("⏭️ Skipping Docker test (DOCKER_TEST not set)");
        return;
    }

    let app = test_app();

    let payload = json!({
        "name": "test-nginx",
        "image": "nginx:latest",
        "ports": [8080]
    });

    let request = Request::builder()
        .method("POST")
        .uri("/apps")
        .header("Content-Type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
}
