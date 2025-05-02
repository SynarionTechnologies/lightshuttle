use crate::routes::{
    apps::{create_app, delete_app, get_app, get_app_logs, list_apps, start_app, stop_app},
    health, metrics, version,
};
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;

/// Builds the main application router.
///
/// This router wires all the available HTTP routes and applies middleware (like CORS).
///
/// # Routes
/// - `GET /apps` — List running applications.
/// - `POST /apps` — Create (launch) a new application.
/// - `GET /apps/:name` — Get a single application by its name.
/// - `GET /apps/:name/logs` — Get logs for a specific application.
/// - `GET /health` — Healthcheck endpoint.
/// - `GET /version` — Application version information.
/// - `GET /metrics` — Prometheus-compatible metrics.
///
/// # Returns
/// A configured `axum::Router` instance ready to be served.
pub fn build_router() -> Router {
    Router::new()
        .route("/apps", get(list_apps).post(create_app))
        .route("/apps/:name", get(get_app).delete(delete_app))
        .route("/apps/:name/start", post(start_app))
        .route("/apps/:name/stop", post(stop_app))
        .route("/apps/:name/logs", get(get_app_logs))
        .route("/health", get(health))
        .route("/version", get(version))
        .route("/metrics", get(metrics))
        .layer(CorsLayer::permissive())
}
