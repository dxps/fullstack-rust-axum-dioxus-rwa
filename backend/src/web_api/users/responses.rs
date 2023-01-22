use super::UserOutDTO;
use axum::http::StatusCode;
use serde_json::{json, Value};

/// Utility (reusable and reused) function for responding with `User` payload in multiple use cases.
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
