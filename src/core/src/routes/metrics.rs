use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct MetricsResponse {
    uptime: &'static str,
    requests_handled: u32,
}

pub async fn metrics() -> Json<MetricsResponse> {
    Json(MetricsResponse {
        uptime: "42s",
        requests_handled: 8,
    })
}
