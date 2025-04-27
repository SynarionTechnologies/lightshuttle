use axum::{body::Body, http::{Request, StatusCode}};
use http_body_util::BodyExt;
use lightshuttle_core::{app::build_router, docker::{launch_container, remove_container}};
use serde_json::Value;
use tower::ServiceExt;

#[tokio::test]
async fn apps_basic_returns_ok() {
    let app = build_router();
    let response = app
        .oneshot(Request::builder().uri("/apps").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn apps_paginated_returns_data() {
    let app = build_router();
    let response = app
        .oneshot(Request::builder().uri("/apps?page=1&limit=5").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body_bytes).unwrap();

    assert_eq!(json["page"], 1);
    assert_eq!(json["limit"], 5);
    assert!(json["items"].is_array());
}


#[tokio::test]
async fn apps_pagination_overflow_returns_empty() {
    let app = build_router();
    let response = app
        .oneshot(Request::builder().uri("/apps?page=1000&limit=10").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body_bytes).unwrap();

    assert_eq!(json["items"].as_array().unwrap().len(), 0);
}

#[tokio::test]
async fn get_existing_app_should_succeed() {
    if std::env::var("DOCKER_TEST").is_err() {
        eprintln!("⏭ Skipping test: set DOCKER_TEST=1 to run it");
        return;
    }

    let container_name = "test-nginx-lightshuttle";
    let _ = remove_container(container_name);
    launch_container(container_name, "nginx:latest", &[8080], 80)
        .expect("Failed to launch test container");

    let app = build_router();
    let response = app
        .oneshot(
            Request::builder()
                .uri(&format!("/apps/{}", container_name))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let status = response.status();
    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: Value = serde_json::from_slice(&body_bytes).unwrap();

    println!("Status: {}", status);
    println!("Body: {}", body);

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["name"], container_name);

    remove_container(container_name).expect("Failed to remove test container");
}

#[tokio::test]
async fn get_non_existing_app_should_return_404() {
    if std::env::var("DOCKER_TEST").is_err() {
        eprintln!("⏭ Skipping test: set DOCKER_TEST=1 to run it");
        return;
    }

    let app = build_router();

    let response = app
        .oneshot(Request::builder().uri("/apps/i-dont-exist").body(Body::empty()).unwrap())
        .await
        .unwrap();

    let status = response.status();
    println!("Status: {}", status);

    assert_eq!(status, StatusCode::NOT_FOUND);
}
