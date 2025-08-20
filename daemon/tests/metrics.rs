use axum::{
    body::Body,
    http::{header, Request, StatusCode},
};
use lightshuttle_core::api::routes::router;
use tower::ServiceExt;

#[tokio::test]
async fn metrics_works() {
    let app = router();
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/metrics")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers().get(header::CONTENT_TYPE).unwrap(),
        "text/plain; version=0.0.4"
    );
}
