use axum::{
    body::Body,
    http::{header, HeaderValue, StatusCode},
    middleware::{from_fn, Next},
    routing::{get, post},
    Router,
};
use std::{convert::Infallible, env, sync::Arc};
use tower_http::cors::{Any, CorsLayer};

use crate::api::{
    error::trace_id_middleware,
    middleware::{auth_middleware, metrics_middleware},
};
use crate::routes::{
    apps::{
        create_app, delete_app, get_app, get_app_logs, get_app_status, list_apps, recreate_app,
        start_app, stop_app,
    },
    health, metrics, version,
};
use crate::services::docker::{DockerClient, ShellDockerClient};

#[cfg(all(feature = "openapi", debug_assertions))]
use crate::openapi::ApiDoc;
#[cfg(all(feature = "openapi", debug_assertions))]
use utoipa::OpenApi;
#[cfg(all(feature = "openapi", debug_assertions))]
use utoipa_swagger_ui::SwaggerUi;

/// Builds the API router mounted at `/api/v1`.
pub fn router() -> Router {
    let docker: Arc<dyn DockerClient> = Arc::new(ShellDockerClient);
    let allowed_origins = env::var("ALLOWED_ORIGINS")
        .ok()
        .map(|val| {
            val.split(',')
                .filter_map(|s| HeaderValue::from_str(s.trim()).ok())
                .collect::<Vec<_>>()
        })
        .filter(|v| !v.is_empty());

    let cors = {
        let base = CorsLayer::new()
            .allow_headers(Any)
            .allow_methods(Any)
            .expose_headers(Any);

        match &allowed_origins {
            Some(origins) => base.allow_origin(origins.clone()),
            None => base.allow_origin(Any),
        }
    };

    let api = Router::new()
        .route("/apps", get(list_apps).post(create_app))
        .route("/apps/:name", get(get_app).delete(delete_app))
        .route("/apps/:name/start", post(start_app))
        .route("/apps/:name/stop", post(stop_app))
        .route("/apps/:name/recreate", post(recreate_app))
        .route("/apps/:name/logs", get(get_app_logs))
        .route("/apps/:name/status", get(get_app_status))
        .route("/health", get(health))
        .route("/version", get(version))
        .route("/metrics", get(metrics))
        .layer(cors)
        .with_state(docker.clone());

    let api = if let Some(origins) = allowed_origins {
        api.layer(from_fn(
            move |req: axum::http::Request<Body>, next: Next| {
                let origins = origins.clone();
                async move {
                    if let Some(origin) = req.headers().get(header::ORIGIN) {
                        if !origins.contains(origin) {
                            return Ok::<_, Infallible>(
                                axum::response::Response::builder()
                                    .status(StatusCode::FORBIDDEN)
                                    .body(Body::empty())
                                    .unwrap(),
                            );
                        }
                    }
                    Ok::<_, Infallible>(next.run(req).await)
                }
            },
        ))
    } else {
        api
    };

    let app = Router::new().nest("/api/v1", api).with_state(docker);
    let app = app
        .layer(from_fn(metrics_middleware))
        .layer(from_fn(trace_id_middleware))
        .layer(from_fn(auth_middleware));

    #[cfg(all(feature = "openapi", debug_assertions))]
    let app =
        app.merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()));

    app
}
