use std::sync::Arc;

use axum::{http::StatusCode, Extension, Json};
use serde_json::Value;

use crate::{domain::model::UserId, AppError, AppState, AppUseCase};

use super::{respond_internal_server_error, respond_unauthorized, UserOutDTO, UserOutDTOUserAttrs};

pub async fn get_current_user(
    user_id: UserId,
    Extension(state): Extension<Arc<AppState>>,
) -> (StatusCode, Json<Value>) {
    match state
        .user_repo
        .get_by_id(&user_id, AppUseCase::AnyTokenProtectedOperation)
        .await
    {
        Ok(entry) => {
            let out = UserOutDTO {
                user: UserOutDTOUserAttrs {
                    email: entry.user.email,
                    token: None,
                    username: entry.user.username,
                    bio: entry.user.bio,
                    image: entry.user.image,
                },
            };
            (StatusCode::OK, Json(serde_json::to_value(out).unwrap()))
        }
        Err(err) => match err {
            AppError::AuthUnauthorized => respond_unauthorized(err),
            AppError::AuthInvalidTokenErr(msg) => {
                respond_unauthorized(AppError::AuthInvalidTokenErr(msg))
            }
            _ => respond_internal_server_error(err),
        },
    }
}
