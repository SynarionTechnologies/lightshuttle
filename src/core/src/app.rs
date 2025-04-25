use crate::routes::{
    apps::{create_app, get_app, list_apps},
    health, metrics, version,
};
use axum::{routing::get, Router};
use tower_http::cors::CorsLayer;

/// Builds the main application router.
///
/// This router wires all the available HTTP routes and applies middleware (like CORS).
///
/// # Routes
/// - `GET /apps` — List running applications.
/// - `POST /apps` — Create (launch) a new application.
/// - `GET /apps/:id` — Get a single application by ID.
/// - `GET /health` — Healthcheck endpoint.
/// - `GET /version` — Application version information.
/// - `GET /metrics` — Prometheus-compatible metrics.
///
/// # Returns
/// A configured `axum::Router` instance ready to be served.
pub fn build_router() -> Router {
    Router::new()
        .route("/apps", get(list_apps).post(create_app))
        .route("/apps/:id", get(get_app))
        .route("/health", get(health))
        .route("/version", get(version))
        .route("/metrics", get(metrics))
        .layer(CorsLayer::permissive())
}
