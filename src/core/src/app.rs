use crate::routes::{apps::get_app, apps::list_apps, health, metrics, version};
use axum::{routing::get, Router};
use tower_http::cors::CorsLayer;

pub fn create_app() -> Router {
    Router::new()
        .route("/apps", get(list_apps))
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
        let app = create_app();
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
        let app = create_app();
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
        let app = create_app();
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
