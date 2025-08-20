use axum::{
    http::{header, HeaderMap, HeaderValue},
    response::IntoResponse,
};
use metrics::gauge;

use crate::metrics::{METRICS_HANDLE, START_TIME};

#[cfg_attr(
    feature = "openapi",
    utoipa::path(
        get,
        path = "/metrics",
        tag = "Metrics",
        responses((status = 200, description = "Service metrics", content_type = "text/plain"))
    )
)]
pub async fn metrics() -> impl IntoResponse {
    let uptime = START_TIME.elapsed().as_secs_f64();
    gauge!("uptime_seconds", uptime);
    let body = METRICS_HANDLE.render();
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("text/plain; version=0.0.4"),
    );
    (headers, body)
}
