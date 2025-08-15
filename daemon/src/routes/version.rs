use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct VersionResponse {
    version: &'static str,
}

#[cfg_attr(
    feature = "openapi",
    utoipa::path(
        get,
        path = "/version",
        tag = "Version",
        responses((status = 200, description = "Service version", body = VersionResponse))
    )
)]
pub async fn version() -> Json<VersionResponse> {
    Json(VersionResponse {
        version: env!("CARGO_PKG_VERSION"),
    })
}
