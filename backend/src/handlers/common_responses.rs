use axum::{http::StatusCode, Json};
use serde_json::Value;

/// Utility function for mapping any error into the classic `500 Internal Server Error` response.
pub fn internal_error_response<E>(err: E) -> (StatusCode, Json<Value>)
where
    E: std::error::Error,
{
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(Value::from(err.to_string())),
    )
}
