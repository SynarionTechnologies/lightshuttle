use axum::{body::Body, http::{Request, StatusCode}};
use http_body_util::BodyExt;
use lightshuttle_core::app::build_router;
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
        .oneshot(Request::builder().uri("/apps?page=2&limit=5").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body_bytes).unwrap();

    assert_eq!(json["page"], 2);
    assert_eq!(json["limit"], 5);
    assert_eq!(json["items"].as_array().unwrap().len(), 5);
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
async fn apps_get_by_id_works() {
    let app = build_router();
    let response = app
        .oneshot(Request::builder().uri("/apps/1").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body_bytes).unwrap();

    assert_eq!(json["id"], 1);
    assert_eq!(json["name"], "app-1");
}

#[tokio::test]
async fn apps_get_by_id_not_found() {
    let app = build_router();
    let response = app
        .oneshot(Request::builder().uri("/apps/9999").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
