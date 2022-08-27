use axum::http::StatusCode;

/// Utility function for mapping any error into the classic `500 Internal Server Error` response.
pub fn internal_error_response<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
