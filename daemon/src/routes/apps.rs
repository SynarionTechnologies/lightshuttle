use axum::{
    extract::{Json, Path, Query},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};

use crate::{
    docker::{self, container, AppInstance},
    errors::Error,
};

use super::{CreateAppRequest, PaginatedResponse, Pagination};

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
pub async fn create_app(Json(payload): Json<CreateAppRequest>) -> Result<impl IntoResponse, Error> {
    match docker::create_and_run_container(
        &payload.name,
        &payload.image,
        &payload.ports,
        payload.container_port,
        payload.labels.as_ref(),
        payload.env.as_ref(),
        payload.restart_policy.as_deref(),
    ) {
        Ok(container_id) => Ok((
            StatusCode::CREATED,
            Json(serde_json::json!({
                "status": "success",
                "container_id": container_id
            })),
        )),
        Err(e) => Ok((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "status": "error",
                "message": e.to_string()
            })),
        )),
    }
}

/// Handles POST /apps/:name/start
///
/// Starts an existing container by name.
///
/// # Returns
/// - `200 OK` if the container was started
/// - `404 Not Found` if the container doesn't exist
/// - `500 Internal Server Error` otherwise
pub async fn start_app(Path(name): Path<String>) -> Result<impl IntoResponse, Error> {
    match container::start_container(&name) {
        Ok(_) => Ok(StatusCode::OK),
        Err(Error::ContainerNotFound) => Ok(StatusCode::NOT_FOUND),
        Err(_) => Ok(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Handles POST /apps/:name/stop
///
/// Stops a running container by name.
///
/// # Returns
/// - `200 OK` if the container was stopped
/// - `404 Not Found` if the container doesn't exist
/// - `500 Internal Server Error` otherwise
pub async fn stop_app(Path(name): Path<String>) -> Result<impl IntoResponse, Error> {
    match container::stop_container(&name) {
        Ok(_) => Ok(StatusCode::OK),
        Err(Error::ContainerNotFound) => Ok(StatusCode::NOT_FOUND),
        Err(_) => Ok(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Handles POST /apps/:name/recreate
///
/// Recreates a container using its original config (image, ports, labels).
///
/// # Returns
/// - `200 OK` with new container ID
/// - `404 Not Found` if container doesn't exist
/// - `500 Internal Server Error` otherwise
pub async fn recreate_app(Path(name): Path<String>) -> Result<impl IntoResponse, Error> {
    match container::recreate_container(&name) {
        Ok(container_id) => Ok((
            StatusCode::OK,
            Json(serde_json::json!({ "container_id": container_id })),
        )),
        Err(Error::ContainerNotFound) => Ok((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "status": "error",
                "message": "Container not found"
            })),
        )),
        Err(_) => Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "status": "error",
                "message": "Internal error"
            })),
        )),
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
pub async fn list_apps(Query(pagination): Query<Pagination>) -> Result<impl IntoResponse, Error> {
    let all_apps = docker::get_running_containers()?;

    let filtered: Vec<_> = match &pagination.search {
        Some(query) => {
            let q = query.to_lowercase();
            all_apps
                .into_iter()
                .filter(|app| app.name.to_lowercase().contains(&q))
                .collect()
        }
        None => all_apps,
    };

    let page = pagination.page.unwrap_or(1);
    let limit = pagination.limit.unwrap_or(10);
    let total = filtered.len();

    let start = (page - 1).saturating_mul(limit);
    let end = (start + limit).min(total);
    let items = if start >= total {
        vec![]
    } else {
        filtered[start..end].to_vec()
    };

    let response = PaginatedResponse {
        total,
        page,
        limit,
        items,
    };

    Ok((StatusCode::OK, Json(response)))
}

/// Retrieve a specific running app by its container name.
///
/// # Path Parameters
/// - `name`: The Docker container name.
///
/// # Returns
/// - `200 OK` with app details if found
/// - `404 Not Found` if the app does not exist
/// - `500 Internal Server Error` if Docker command fails
pub async fn get_app(Path(name): Path<String>) -> Result<impl IntoResponse, Error> {
    match docker::get_container_by_name(&name)? {
        Some(app) => Ok((StatusCode::OK, Json(Some(app)))),
        None => Ok((StatusCode::NOT_FOUND, Json(None::<AppInstance>))),
    }
}

/// Retrieve the logs of a running container.
///
/// # Path Parameters
/// - `name`: The Docker container name.
///
/// # Returns
/// - `200 OK` with the logs as plain text.
/// - `404 Not Found` if the container does not exist.
/// - `500 Internal Server Error` if fetching logs fails.
pub async fn get_app_logs(Path(name): Path<String>) -> Result<Response, Error> {
    match docker::get_container_logs(&name) {
        Ok(logs) => {
            Ok((StatusCode::OK, [(header::CONTENT_TYPE, "text/plain")], logs).into_response())
        }
        Err(Error::ContainerNotFound) => Ok(StatusCode::NOT_FOUND.into_response()),
        Err(_) => Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response()),
    }
}

/// Handles GET /apps/:name/status
///
/// Returns the status of a container (`running`, `exited`, etc.)
///
/// # Returns
/// - `200 OK` with JSON { status }
/// - `404 Not Found` if the container doesn't exist
/// - `500 Internal Server Error` on error
pub async fn get_app_status(Path(name): Path<String>) -> Result<impl IntoResponse, Error> {
    match container::get_container_status(&name) {
        Ok(state) => Ok((StatusCode::OK, Json(serde_json::json!({ "status": state })))),
        Err(Error::ContainerNotFound) => Ok((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "status": "error",
                "message": "Container not found"
            })),
        )),
        Err(_) => Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "status": "error",
                "message": "Failed to fetch container status"
            })),
        )),
    }
}

/// Deletes an application/container by its name.
///
/// # Arguments
/// - `name`: The container name to delete.
///
/// # Returns
/// - `204 No Content` if deleted successfully
/// - `404 Not Found` if container doesn't exist
/// - `500 Internal Server Error` if something went wrong
pub async fn delete_app(Path(name): Path<String>) -> Result<impl IntoResponse, Error> {
    match docker::remove_container(&name) {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(Error::ContainerNotFound) => Ok(StatusCode::NOT_FOUND),
        Err(_) => Ok(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
