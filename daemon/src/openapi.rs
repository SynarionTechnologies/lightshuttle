use utoipa::OpenApi;

use crate::{
    api::error::ApiError,
    docker::models::{AppInstance, AppStatus},
    routes::{
        apps,
        health::{self, HealthResponse},
        metrics,
        models::{
            AppListResponse, ContainerIdResponse, CreateAppRequest, CreateAppResponse, Pagination,
            StatusResponse,
        },
        version::{self, VersionResponse},
    },
};

/// OpenAPI documentation for LightShuttle API.
#[derive(OpenApi)]
#[openapi(
    paths(
        apps::create_app,
        apps::start_app,
        apps::stop_app,
        apps::recreate_app,
        apps::list_apps,
        apps::get_app,
        apps::get_app_logs,
        apps::get_app_status,
        apps::delete_app,
        health::health,
        metrics::metrics,
        version::version,
    ),
    components(schemas(
        CreateAppRequest,
        Pagination,
        AppListResponse,
        CreateAppResponse,
        ContainerIdResponse,
        StatusResponse,
        HealthResponse,
        VersionResponse,
        AppInstance,
        AppStatus,
        ApiError,
    )),
    tags(
        (name = "Apps", description = "Application management"),
        (name = "Health", description = "Health check"),
        (name = "Metrics", description = "Service metrics"),
        (name = "Version", description = "Service version"),
    )
)]
pub struct ApiDoc;
