use crate::routes::{
    apps::{
        create_app, delete_app, get_app, get_app_logs, get_app_status, list_apps, recreate_app,
        start_app, stop_app,
    },
    health, metrics, version,
};
use axum::{
    body::Body,
    http::{header, HeaderValue, StatusCode},
    middleware::{from_fn, Next},
    routing::{get, post},
    Router,
};
use std::{convert::Infallible, env};
use tower_http::cors::{Any, CorsLayer};

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
    let allowed_origins = env::var("ALLOWED_ORIGINS")
        .ok()
        .map(|val| {
            val.split(',')
                .filter_map(|s| HeaderValue::from_str(s.trim()).ok())
                .collect::<Vec<_>>()
        })
        .filter(|v| !v.is_empty());

    let cors = {
        let base = CorsLayer::new()
            .allow_headers(Any)
            .allow_methods(Any)
            .expose_headers(Any);

        match &allowed_origins {
            Some(origins) => base.allow_origin(origins.clone()),
            None => base.allow_origin(Any),
        }
    };

    let router = Router::new()
        .route("/apps", get(list_apps).post(create_app))
        .route("/apps/:name", get(get_app).delete(delete_app))
        .route("/apps/:name/start", post(start_app))
        .route("/apps/:name/stop", post(stop_app))
        .route("/apps/:name/recreate", post(recreate_app))
        .route("/apps/:name/logs", get(get_app_logs))
        .route("/apps/:name/status", get(get_app_status))
        .route("/health", get(health))
        .route("/version", get(version))
        .route("/metrics", get(metrics))
        .layer(cors);

    if let Some(origins) = allowed_origins {
        router.layer(from_fn(
            move |req: axum::http::Request<Body>, next: Next| {
                let origins = origins.clone();
                async move {
                    if let Some(origin) = req.headers().get(header::ORIGIN) {
                        if !origins.contains(origin) {
                            return Ok::<_, Infallible>(
                                axum::response::Response::builder()
                                    .status(StatusCode::FORBIDDEN)
                                    .body(Body::empty())
                                    .unwrap(),
                            );
                        }
                    }
                    Ok::<_, Infallible>(next.run(req).await)
                }
            },
        ))
    } else {
        router
    }
}
