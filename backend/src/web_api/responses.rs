use crate::AppError;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::{json, Value};

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

/// Utility function for responding with `404 Not Found` code and an error description.
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

// Implementation of Axum's `IntoResponse` trait, so that
// an `AppError` can be converted into an HTTP response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        //
        let body = Json(json!({
            "error": self.to_string()
        }));
        let response_tuple = match self {
            AppError::Unauthorized(msg) => {
                (StatusCode::UNAUTHORIZED, Json(json!({ "error": msg })))
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, body),
        };
        response_tuple.into_response()
    }
}
