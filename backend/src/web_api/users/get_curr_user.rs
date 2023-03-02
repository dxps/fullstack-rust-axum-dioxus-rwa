use super::responses::respond_with_user_dto;
use crate::{
    domain::model::UserId,
    web_api::{respond_internal_server_error, respond_unauthorized},
    AppError, AppState, AppUseCase,
};
use axum::{extract::State, http::StatusCode, Json};
use serde_json::Value;

pub async fn get_current_user(
    State(state): State<AppState>,
    curr_user_id: UserId,
) -> (StatusCode, Json<Value>) {
    //
    match state
        .user_repo
        .get_by_id(&curr_user_id, AppUseCase::AnyTokenProtectedOperation)
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
            AppError::Unauthorized(_) => respond_unauthorized(err),
            _ => respond_internal_server_error(err),
        },
    }
}
