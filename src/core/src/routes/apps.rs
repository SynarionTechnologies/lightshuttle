use axum::{
    extract::{Json, Path, Query},
    http::StatusCode,
    response::IntoResponse,
};

use serde::{Deserialize, Serialize};

use crate::docker::{get_running_containers, launch_container};

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


/// Handles GET /apps/:id
///
/// Fetches an application by its local ID (mocked).
///
/// # Arguments
/// - `id`: Application ID.
///
/// # Returns
/// - `200 OK` with the application if found.
/// - `404 Not Found` if not found.
pub async fn get_app(Path(id): Path<u32>) -> (StatusCode, Json<Option<AppInstance>>) {
    let app = get_mock_apps().into_iter().find(|a| a.id == id);

    match app {
        Some(app) => (StatusCode::OK, Json(Some(app))),
        None => (StatusCode::NOT_FOUND, Json(None)),
    }
}

/// Generates a mocked list of applications (for testing purposes).
///
/// # Returns
/// - `Vec<AppInstance>` mocked apps with random statuses and ports.
pub fn get_mock_apps() -> Vec<AppInstance> {
    let statuses = vec![
        AppStatus::Running,
        AppStatus::Stopped,
        AppStatus::Error,
    ];

    (1..=50)
        .map(|id| AppInstance {
            id,
            name: format!("app-{}", id),
            status: statuses[(id as usize) % statuses.len()].clone(),
            image: format!("image-{}:latest", id),
            ports: vec![8000 + id as u16],
            created_at: "2025-04-22T15:00:00Z".to_string(),
        })
        .collect()
}
