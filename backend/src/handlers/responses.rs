use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

use crate::AppError;

/// Utility function for responding with `500 Internal Server Error` code and an error description.
pub fn respond_internal_server_error<E>(err: E) -> (StatusCode, Json<Value>)
where
    E: std::error::Error,
{
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({
            "error": err.to_string()
        })),
    )
}

/// Utility function for responding with `400 Bad Request` code and an error description.
pub fn respond_bad_request<E>(err: E) -> (StatusCode, Json<Value>)
where
    E: std::error::Error,
{
    (
        StatusCode::BAD_REQUEST,
        Json(json!({
            "error": err.to_string()
        })),
    )
}

/// Utility function for responding with `401 Unauthorized` code and an error description.
pub fn respond_unauthorized<E>(err: E) -> (StatusCode, Json<Value>)
where
    E: std::error::Error,
{
    (
        StatusCode::UNAUTHORIZED,
        Json(json!({
            "error": err.to_string()
        })),
    )
}

/// Utility function for responding with `400 Bad Request` code and an error description.
pub fn respond_not_found<E>(err: E) -> (StatusCode, Json<Value>)
where
    E: std::error::Error,
{
    (
        StatusCode::NOT_FOUND,
        Json(json!({
            "error": err.to_string()
        })),
    )
}

// Implementation of Axum's `IntoResponse` trait, so that an `AppError` can be converted into an HTTP response.
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let body = Json(json!({
            "error": self.to_string()
        }));
        let status_code = match self {
            AppError::AuthUnauthorizedErr => (StatusCode::UNAUTHORIZED, Json(Value::default())),
            AppError::InvalidTokenErr(_) => (StatusCode::UNAUTHORIZED, body),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, body),
        };
        status_code.into_response()
    }
}
