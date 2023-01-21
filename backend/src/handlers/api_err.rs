use axum::{extract::rejection::JsonRejection, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    // The `#[from]` attribute generates `From<JsonRejection> for ApiError`
    // implementation. See `thiserror` docs for more information
    #[error(transparent)]
    JsonExtractorRejection(#[from] JsonRejection),
}

// We implement `IntoResponse`, so that ApiError can be used as a response.
impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let message = self.to_string();
        let mut reason = String::new();

        let code = match self {
            ApiError::JsonExtractorRejection(x) => match x {
                JsonRejection::JsonDataError(e) => {
                    reason = e.to_string();
                    StatusCode::UNPROCESSABLE_ENTITY
                }
                JsonRejection::JsonSyntaxError(_) => StatusCode::BAD_REQUEST,
                JsonRejection::MissingJsonContentType(_) => StatusCode::UNSUPPORTED_MEDIA_TYPE,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
        };
        let payload = json!({
            "error": message,
            "reason": reason
        });
        (code, Json(payload)).into_response()
    }
}
