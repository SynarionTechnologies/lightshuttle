use axum::{
    body,
    body::Body,
    http::{Request, StatusCode},
};
use lightshuttle_core::app::build_router;
use serde_json::json;
use tower::ServiceExt;

#[tokio::test]
async fn post_apps_should_succeed() {
    if std::env::var("DOCKER_TEST").is_err() {
        eprintln!("⏭️ Skipping Docker test (DOCKER_TEST not set)");
        return;
    }

    let container_name = "test-nginx";

    let _ = std::process::Command::new("docker")
        .args(["rm", "-f", container_name])
        .output();

    let app = build_router();

    let payload = json!({
        "name": container_name,
        "image": "nginx:latest",
        "ports": [8080],
        "container_port": 80
    });

    let request = Request::builder()
        .method("POST")
        .uri("/apps")
        .header("Content-Type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    let status = response.status();
    let body_bytes = body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body = String::from_utf8_lossy(&body_bytes);

    println!("Status: {}", status);
    println!("Body: {}", body);

    let _ = std::process::Command::new("docker")
        .args(["rm", "-f", container_name])
        .output();

    assert_eq!(status, StatusCode::CREATED);
}
