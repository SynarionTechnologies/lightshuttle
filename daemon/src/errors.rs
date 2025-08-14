use axum::response::{IntoResponse, Response};
use axum::{http::StatusCode, Json};
use serde::Serialize;
use thiserror::Error;
use utoipa::ToSchema;

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

/// Standard error response body.
#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = match self {
            Error::ContainerNotFound => StatusCode::NOT_FOUND,
            Error::DockerCommandFailed => StatusCode::INTERNAL_SERVER_ERROR,
            Error::DockerOutputParse(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::InvalidRequest(_) => StatusCode::BAD_REQUEST,
            Error::BadRequest(_) => StatusCode::BAD_REQUEST,
        };
        let body = Json(ErrorResponse {
            error: self.to_string(),
        });

        (status, body).into_response()
    }
}
