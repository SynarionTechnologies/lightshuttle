use axum::{
    body::Body,
    http::{header, Request, StatusCode},
};
use lightshuttle_core::api::routes::router;
use tower::ServiceExt;

#[tokio::test]
async fn rejects_disallowed_origin() {
    std::env::set_var("ALLOWED_ORIGINS", "https://allowed.example");
    let app = router();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/health")
                .header(header::ORIGIN, "https://notallowed.example")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::FORBIDDEN);
    std::env::remove_var("ALLOWED_ORIGINS");
}

#[tokio::test]
async fn accepts_allowed_origin() {
    std::env::set_var("ALLOWED_ORIGINS", "https://allowed.example");
    let app = router();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/health")
                .header(header::ORIGIN, "https://allowed.example")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    std::env::remove_var("ALLOWED_ORIGINS");
}
