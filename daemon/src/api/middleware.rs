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

/// Mapping of API keys to namespaces loaded from a JSON file at startup.
static KEY_STORE: Lazy<HashMap<String, Namespace>> = Lazy::new(|| {
    let path = match std::env::var("API_KEYS_FILE") {
        Ok(p) => p,
        Err(_) => return HashMap::new(),
    };
    let data = std::fs::read_to_string(path).unwrap_or_default();
    serde_json::from_str(&data).unwrap_or_default()
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
