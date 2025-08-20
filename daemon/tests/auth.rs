use axum::{
    body::Body,
    http::{header, Request, StatusCode},
};
use jsonwebtoken::{encode, EncodingKey, Header};
use lightshuttle_core::api::routes::router;
use serde::{Deserialize, Serialize};
use tower::ServiceExt;

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[tokio::test]
async fn jwt_authentication() {
    // Weak secret should return 500
    std::env::set_var("JWT_SECRET", "short");
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
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    std::env::remove_var("JWT_SECRET");

    // Missing token with strong secret should return 401
    const SECRET: &str = "01234567890123456789012345678901";
    std::env::set_var("JWT_SECRET", SECRET);
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
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    // Valid token should return 200
    let token = encode(
        &Header::default(),
        &Claims {
            sub: "demo".into(),
            exp: 9_999_999_999,
        },
        &EncodingKey::from_secret(SECRET.as_bytes()),
    )
    .unwrap();

    let app = router();
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/health")
                .header(header::AUTHORIZATION, format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    std::env::remove_var("JWT_SECRET");
}
