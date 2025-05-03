use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Request payload for creating a new application/container.
#[derive(Deserialize)]
pub struct CreateAppRequest {
    pub name: String,
    pub image: String,
    pub ports: Vec<u16>,
    pub container_port: u16,
    pub labels: Option<HashMap<String, String>>,
    pub env: Option<HashMap<String, String>>,
    pub restart_policy: Option<String>,
}

/// Pagination parameters for listing applications.
#[derive(Deserialize)]
pub struct Pagination {
    pub page: Option<usize>,
    pub limit: Option<usize>,
    pub search: Option<String>,
}

/// Standard response format for paginated lists.
#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    pub total: usize,
    pub page: usize,
    pub limit: usize,
    pub items: Vec<T>,
}
