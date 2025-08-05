use axum::{
    extract::{Json, Path, Query},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};

use crate::{
    docker::{self, container},
    errors::Error,
};

use super::{CreateAppRequest, PaginatedResponse, Pagination};

use crate::docker::ContainerConfig;

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
    let config = ContainerConfig {
        name: &payload.name,
        image: &payload.image,
        host_ports: &payload.ports,
        container_port: payload.container_port,
        labels: payload.labels.as_ref(),
        env: payload.env.as_ref(),
        volumes: payload.volumes.as_ref(),
        restart_policy: payload.restart_policy.as_deref(),
    };

    let container_id = docker::create_and_run_container(config)?;
    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({
            "status": "success",
            "container_id": container_id
        })),
    ))
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
    container::start_container(&name)?;
    Ok(StatusCode::OK)
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
    container::stop_container(&name)?;
    Ok(StatusCode::OK)
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
    let container_id = container::recreate_container(&name)?;
    Ok((
        StatusCode::OK,
        Json(serde_json::json!({ "container_id": container_id })),
    ))
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
/// - `200 OK` with an empty list if Docker is unavailable.
/// - `500 Internal Server Error` on unexpected errors.
pub async fn list_apps(Query(pagination): Query<Pagination>) -> Result<impl IntoResponse, Error> {
    let all_apps = match docker::get_running_containers() {
        Ok(apps) => apps,
        Err(Error::DockerCommandFailed) => Vec::new(),
        Err(e) => return Err(e),
    };

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
    let app = docker::get_container_by_name(&name)?.ok_or(Error::ContainerNotFound)?;
    Ok((StatusCode::OK, Json(app)))
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
    let logs = docker::get_container_logs(&name)?;
    Ok((StatusCode::OK, [(header::CONTENT_TYPE, "text/plain")], logs).into_response())
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
    let state = container::get_container_status(&name)?;
    Ok((StatusCode::OK, Json(serde_json::json!({ "status": state }))))
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
    docker::remove_container(&name)?;
    Ok(StatusCode::NO_CONTENT)
}
