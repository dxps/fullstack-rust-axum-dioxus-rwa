use super::responses::respond_with_user_dto;
use crate::{
    domain::model::UserId,
    web_api::{
        extractors::InputJson, respond_bad_request, respond_internal_server_error,
        respond_unauthorized,
    },
    AppError, AppState,
};
use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use serde_json::Value;

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
    State(state): State<AppState>,
    user_id: UserId,
    InputJson(input): InputJson<UpdateUserInputDTO>,
) -> (StatusCode, Json<Value>) {
    //
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
            AppError::InvalidRequest(_) => respond_bad_request(err),
            AppError::Unauthorized(_) => respond_unauthorized(err),
            _ => respond_internal_server_error(err),
        },
    }
}
