use axum::Router;
use lightshuttle_core::api::routes::router;

pub fn test_app() -> Router {
    router()
}
