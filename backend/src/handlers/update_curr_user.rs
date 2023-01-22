use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use serde_json::Value;

use crate::{domain::model::UserId, AppError, AppState};

use super::{
    respond_bad_request, respond_internal_server_error, respond_unauthorized,
    respond_with_user_dto, InputJson,
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
    State(state): State<Arc<AppState>>,
    user_id: UserId,
    InputJson(input): InputJson<UpdateUserInputDTO>,
) -> (StatusCode, Json<Value>) {
    match state
        .user_repo
        .update_by_id(user_id, input.user.email, input.user.bio, input.user.image)
        .await
    {
        Ok(entry) => respond_with_user_dto(
            entry.user.email,
            None,
            entry.user.username,
            entry.user.bio,
            entry.user.image,
        ),
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
