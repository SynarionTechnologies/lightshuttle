use std::collections::HashMap;

use crate::docker::models::AppInstance;
use serde::{Deserialize, Serialize};

/// Request payload for creating a new application/container.
#[derive(Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct CreateAppRequest {
    pub name: String,
    pub image: String,
    pub ports: Vec<u16>,
    pub container_port: u16,
    pub labels: Option<HashMap<String, String>>,
    pub env: Option<HashMap<String, String>>,
    pub volumes: Option<Vec<String>>,
    pub restart_policy: Option<String>,
}

/// Pagination parameters for listing applications.
#[derive(Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::IntoParams, utoipa::ToSchema))]
pub struct Pagination {
    pub page: Option<usize>,
    pub limit: Option<usize>,
    pub search: Option<String>,
}

/// Standard response format for paginated lists.
#[derive(Serialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct AppListResponse {
    pub total: usize,
    pub page: usize,
    pub limit: usize,
    pub items: Vec<AppInstance>,
}

/// Response body returned when creating a new application.
#[derive(Serialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct CreateAppResponse {
    pub status: String,
    pub container_id: String,
}

/// Response containing only a container identifier.
#[derive(Serialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct ContainerIdResponse {
    pub container_id: String,
}

/// Generic status response body.
#[derive(Serialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct StatusResponse {
    pub status: String,
}
