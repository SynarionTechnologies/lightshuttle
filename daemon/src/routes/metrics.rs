use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct MetricsResponse {
    uptime: &'static str,
    requests_handled: u32,
}

#[cfg_attr(
    feature = "openapi",
    utoipa::path(
        get,
        path = "/metrics",
        tag = "Metrics",
        responses((status = 200, description = "Service metrics", body = MetricsResponse))
    )
)]
pub async fn metrics() -> Json<MetricsResponse> {
    Json(MetricsResponse {
        uptime: "42s",
        requests_handled: 8,
    })
}
