use axum::{
    body::Body,
    http::{header, Request, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::Deserialize;
use std::env;

use crate::api::error::{ApiError, TRACE_ID};

/// Claims carried by a JWT.
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Claims {
    /// Subject the token refers to.
    sub: String,
    /// Expiration timestamp as seconds since epoch.
    exp: usize,
}

/// Middleware validating JWT bearer tokens.
///
/// When the `JWT_SECRET` environment variable is set, every request must
/// include a valid `Authorization: Bearer <token>` header. If the variable is
/// unset, the middleware allows all requests.
pub async fn jwt_auth_middleware(req: Request<Body>, next: Next) -> Result<Response, ApiError> {
    let secret = match env::var("JWT_SECRET") {
        Ok(s) if s.len() >= 32 => s,
        Ok(_) => return Err(misconfigured("JWT_SECRET must be at least 32 characters")),
        _ => return Ok(next.run(req).await),
    };

    let header_value = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer ").map(str::to_owned));

    let token = match header_value {
        Some(t) => t,
        None => return Err(unauthorized("Missing bearer token")),
    };

    let decoding_key = DecodingKey::from_secret(secret.as_bytes());
    let validation = Validation::new(Algorithm::HS256);

    decode::<Claims>(&token, &decoding_key, &validation)
        .map_err(|_| unauthorized("Invalid token"))?;

    Ok(next.run(req).await)
}

fn unauthorized(message: &str) -> ApiError {
    let trace_id = TRACE_ID.with(|id| id.clone());
    ApiError {
        trace_id,
        code: StatusCode::UNAUTHORIZED.as_u16(),
        message: message.to_string(),
        details: None,
    }
}

fn misconfigured(message: &str) -> ApiError {
    let trace_id = TRACE_ID.with(|id| id.clone());
    ApiError {
        trace_id,
        code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        message: message.to_string(),
        details: None,
    }
}
