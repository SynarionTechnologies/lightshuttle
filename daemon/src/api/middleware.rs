use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use tracing::{info, warn};

use crate::models::namespace::Namespace;
use metrics::{counter, histogram};
use std::time::Instant;

/// Mapping of API keys to namespaces loaded from a JSON file at startup.
static KEY_STORE: Lazy<HashMap<String, Namespace>> = Lazy::new(|| {
    let path = match std::env::var("API_KEYS_FILE") {
        Ok(p) => p,
        Err(_) => {
            warn!("API_KEYS_FILE not set; authentication disabled");
            return HashMap::new();
        }
    };

    let data = match std::fs::read_to_string(&path) {
        Ok(d) => d,
        Err(e) => {
            warn!(%path, error = %e, "Failed to read API keys file; authentication disabled");
            return HashMap::new();
        }
    };

    match serde_json::from_str(&data) {
        Ok(map) => map,
        Err(e) => {
            warn!(%path, error = %e, "Failed to parse API keys file; authentication disabled");
            HashMap::new()
        }
    }
});

/// Middleware that validates an `X-API-Key` header against the key store.
pub async fn auth_middleware(mut req: Request<Body>, next: Next) -> Response {
    if KEY_STORE.is_empty() {
        return next.run(req).await;
    }
    match req.headers().get("x-api-key").and_then(|v| v.to_str().ok()) {
        Some(k) => {
            if let Some(ns) = KEY_STORE.get(k) {
                info!(key = k, namespace = %ns.name, "API key authenticated");
                req.extensions_mut().insert(ns.clone());
                next.run(req).await
            } else {
                warn!(key = k, "Invalid API key");
                StatusCode::UNAUTHORIZED.into_response()
            }
        }
        None => {
            warn!("Missing API key");
            StatusCode::UNAUTHORIZED.into_response()
        }
    }
}

/// Middleware to record basic HTTP metrics.
pub async fn metrics_middleware(req: Request<Body>, next: Next) -> Response {
    let method = req.method().to_string();
    let path = req.uri().path().to_owned();
    let start = Instant::now();
    let response = next.run(req).await;
    let status = response.status().as_u16().to_string();
    let elapsed = start.elapsed().as_secs_f64();
    counter!("http_requests_total", 1, "method" => method.clone(), "path" => path.clone(), "status" => status.clone());
    histogram!("http_request_duration_seconds", elapsed, "method" => method, "path" => path, "status" => status);
    response
}
