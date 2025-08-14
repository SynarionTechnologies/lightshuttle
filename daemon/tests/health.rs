use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use lightshuttle_core::api::routes::router;
use tower::ServiceExt;

#[tokio::test]
async fn health_works() {
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
    assert_eq!(response.status(), StatusCode::OK);
}
