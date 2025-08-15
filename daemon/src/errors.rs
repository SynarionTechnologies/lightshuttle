use crate::api::error::{ApiError, TRACE_ID};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

/// Defines all the possible errors for the LightShuttle daemon service.
#[derive(Error, Debug)]
pub enum Error {
    #[error("Docker command execution failed")]
    DockerCommandFailed,

    #[error("Container not found")]
    ContainerNotFound,

    #[error("Docker output parsing failed: {0}")]
    DockerOutputParse(String),

    #[error("Unexpected error: {0}")]
    Unexpected(String),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Invalid input: {0}")]
    BadRequest(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, message, details) = match self {
            Error::ContainerNotFound => (
                StatusCode::NOT_FOUND,
                "Container not found".to_string(),
                None,
            ),
            Error::DockerCommandFailed => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Docker command execution failed".to_string(),
                None,
            ),
            Error::DockerOutputParse(detail) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Docker output parsing failed".to_string(),
                Some(detail),
            ),
            Error::Unexpected(detail) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unexpected error".to_string(),
                Some(detail),
            ),
            Error::InvalidRequest(detail) => (
                StatusCode::BAD_REQUEST,
                "Invalid request".to_string(),
                Some(detail),
            ),
            Error::BadRequest(detail) => (
                StatusCode::BAD_REQUEST,
                "Invalid input".to_string(),
                Some(detail),
            ),
        };

        let trace_id = TRACE_ID.with(|id| id.clone());
        ApiError {
            trace_id,
            code: status.as_u16(),
            message,
            details,
        }
        .into_response()
    }
}
