use crate::routes::{apps::{create_app, get_app, list_apps}, health, metrics, version};
use axum::{routing::get, Router};
use tower_http::cors::CorsLayer;

pub fn build_router() -> Router {
    Router::new()
        .route("/apps", get(list_apps).post(create_app))
        .route("/apps/:id", get(get_app))
        .route("/health", get(health))
        .route("/version", get(version))
        .route("/metrics", get(metrics))
        .layer(CorsLayer::permissive())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn health_works() {
        let app = build_router();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn version_works() {
        let app = build_router();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/version")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn metrics_works() {
        let app = build_router();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/metrics")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
