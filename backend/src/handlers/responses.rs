use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::{json, Value};

use crate::AppError;

use super::UserOutDTO;

/// Utility (and reused) function for responding with `User` payload in different use cases.
pub fn respond_with_user_dto(
    email: String,
    token: Option<String>,
    username: String,
    bio: String,
    image: Option<String>,
) -> (StatusCode, axum::Json<Value>) {
    let dto = UserOutDTO {
        email,
        token,
        username,
        bio,
        image,
    };
    (StatusCode::OK, axum::Json(json!({ "user": dto })))
}

/// Utility function for responding with `500 Internal Server Error` code and an error description.
pub fn respond_internal_server_error<E>(err: E) -> (StatusCode, axum::Json<Value>)
where
    E: std::error::Error,
{
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        axum::Json(json!({
            "error": err.to_string()
        })),
    )
}

/// Utility function for responding with `400 Bad Request` code and an error description.
pub fn respond_bad_request<E>(err: E) -> (StatusCode, axum::Json<Value>)
where
    E: std::error::Error,
{
    (
        StatusCode::BAD_REQUEST,
        axum::Json(json!({
            "error": err.to_string()
        })),
    )
}

/// Utility function for responding with `401 Unauthorized` code and an error description.
pub fn respond_unauthorized<E>(err: E) -> (StatusCode, axum::Json<Value>)
where
    E: std::error::Error,
{
    (
        StatusCode::UNAUTHORIZED,
        axum::Json(json!({
            "error": err.to_string()
        })),
    )
}

/// Utility function for responding with `404 Not Found` code and an error description.
pub fn respond_not_found<E>(err: E) -> (StatusCode, axum::Json<Value>)
where
    E: std::error::Error,
{
    (
        StatusCode::NOT_FOUND,
        axum::Json(json!({
            "error": err.to_string()
        })),
    )
}

// Implementation of Axum's `IntoResponse` trait, so that an `AppError` can be converted into an HTTP response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let body = axum::Json(json!({
            "error": self.to_string()
        }));
        let response_tuple = match self {
            AppError::AuthUnauthorized => (StatusCode::UNAUTHORIZED, axum::Json(Value::default())),
            AppError::AuthInvalidTokenErr(_) => (StatusCode::UNAUTHORIZED, body),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, body),
        };
        response_tuple.into_response()
    }
}
