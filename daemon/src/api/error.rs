use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use tokio::task_local;
use uuid::Uuid;

/// Structured API error response.
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct ApiError {
    pub trace_id: String,
    pub code: u16,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let body = Json(self);
        (status, body).into_response()
    }
}

task_local! {
    /// Trace identifier for the current request.
    pub static TRACE_ID: String;
}

/// Middleware that generates a trace ID and stores it in request extensions.
pub async fn trace_id_middleware(mut req: Request<Body>, next: Next) -> Response {
    let trace_id = Uuid::new_v4().to_string();
    TRACE_ID
        .scope(trace_id.clone(), async move {
            req.extensions_mut().insert(trace_id);
            next.run(req).await
        })
        .await
}
