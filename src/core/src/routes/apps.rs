use axum::{
    extract::{Json, Path, Query},
    http::StatusCode,
    response::IntoResponse,
};

use serde::{Deserialize, Serialize};

use crate::docker::{get_container_by_name, get_running_containers, launch_container};

/// Represents an application instance (a running Docker container).
#[derive(Serialize, Deserialize, Clone)]
pub struct AppInstance {
    pub id: u32,
    pub name: String,
    pub status: AppStatus,
    pub image: String,
    pub ports: Vec<u16>,
    pub created_at: String,
}

/// Represents the status of an application.
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AppStatus {
    Running,
    Stopped,
    Error,
}

/// Pagination parameters for listing applications.
#[derive(Deserialize)]
pub struct Pagination {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

/// Standard response format for paginated lists.
#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    pub total: usize,
    pub page: usize,
    pub limit: usize,
    pub items: Vec<T>,
}

/// Request payload for creating a new application/container.
#[derive(Deserialize)]
pub struct CreateAppRequest {
    pub name: String,
    pub image: String,
    pub ports: Vec<u16>,
    pub container_port: u16,
}

/// Handles POST /apps
///
/// Launches a new container based on the provided configuration.
///
/// # Arguments
/// - `payload`: JSON body containing app creation parameters.
///
/// # Returns
/// - `201 Created` with container ID if successful.
/// - `400 Bad Request` with error message if failed.
pub async fn create_app(Json(payload): Json<CreateAppRequest>) -> impl IntoResponse {
    match launch_container(&payload.name, &payload.image, &payload.ports, payload.container_port) {
        Ok(container_id) => (
            StatusCode::CREATED,
            Json(serde_json::json!({
                "status": "success",
                "container_id": container_id
            }))
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "status": "error",
                "message": e
            }))
        ),
    }
}

/// Handles GET /apps
///
/// Lists running containers, paginated.
///
/// # Arguments
/// - `pagination`: Query parameters `page` and `limit`.
///
/// # Returns
/// - `200 OK` with paginated list of applications.
/// - `500 Internal Server Error` if Docker command fails.
pub async fn list_apps(Query(pagination): Query<Pagination>) -> (StatusCode, Json<PaginatedResponse<AppInstance>>) {
    match get_running_containers() {
        Ok(all_apps) => {
            let page = pagination.page.unwrap_or(1);
            let limit = pagination.limit.unwrap_or(10);
            let total = all_apps.len();

            let start = (page - 1).saturating_mul(limit);
            let end = (start + limit).min(total);
            let items = if start >= total { vec![] } else { all_apps[start..end].to_vec() };

            let response = PaginatedResponse { total, page, limit, items };
            (StatusCode::OK, Json(response))
        }
        Err(_) => {
            let empty = PaginatedResponse {
                total: 0,
                page: 1,
                limit: 10,
                items: vec![],
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(empty))
        }
    }
}


/// Retrieve a specific running app by its container name.
///
/// # Path Parameters
/// - `name`: The Docker container name.
///
/// # Returns
/// - `200 OK` with app details if found
/// - `404 Not Found` if the app does not exist
pub async fn get_app(Path(name): Path<String>) -> (StatusCode, Json<Option<AppInstance>>) {
    match get_container_by_name(&name) {
        Ok(Some(app)) => (StatusCode::OK, Json(Some(app))),
        Ok(None) => (StatusCode::NOT_FOUND, Json(None)),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(None),
        ),
    }
}
