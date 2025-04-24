use axum::Router;
use lightshuttle_core::app::build_router;

pub fn test_app() -> Router {
    build_router()
}
