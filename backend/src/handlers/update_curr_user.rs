use std::sync::Arc;

use axum::{http::StatusCode, Extension, Json};
use serde::Deserialize;
use serde_json::Value;

use crate::{domain::model::UserId, AppError, AppState};

use super::{
    respond_bad_request, respond_internal_server_error, respond_unauthorized, UserOutDTO,
    UserOutDTOUserAttrs,
};

#[derive(Debug, Deserialize)]
pub struct UpdateUserInputDTO {
    pub user: UpdateUserInputDTOUserAttrs,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserInputDTOUserAttrs {
    pub email: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
}

pub async fn update_current_user(
    user_id: UserId,
    Json(input): Json<UpdateUserInputDTO>,
    Extension(state): Extension<Arc<AppState>>,
) -> (StatusCode, Json<Value>) {
    match state
        .user_repo
        .update_by_id(user_id, input.user.email, input.user.bio, input.user.image)
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
            AppError::AuthInvalidInput => respond_bad_request(err),
            AppError::AuthUnauthorized => respond_unauthorized(err),
            AppError::AuthInvalidTokenErr(msg) => {
                respond_unauthorized(AppError::AuthInvalidTokenErr(msg))
            }
            _ => respond_internal_server_error(err),
        },
    }
}
