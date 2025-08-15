use axum::{
    body,
    body::Body,
    http::{Request, StatusCode},
};
use lightshuttle_core::api::routes::router;
use serde_json::Value;
use tower::ServiceExt;

#[tokio::test]
async fn create_app_invalid_volume_returns_error_message() {
    let app = router();

    let payload = serde_json::json!({
        "name": "bad-volume",
        "image": "nginx:latest",
        "ports": [8080],
        "container_port": 80,
        "volumes": ["invalid"],
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/apps")
        .header("Content-Type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body_bytes = body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body_bytes).unwrap();
    assert_eq!(json["code"].as_i64().unwrap(), 400);
    assert!(json["trace_id"].as_str().is_some());
    assert_eq!(json["message"].as_str().unwrap(), "Invalid input");
    assert!(json["details"]
        .as_str()
        .unwrap()
        .contains("Invalid volume format"));
}
