use std::sync::Arc;

use axum::{http::StatusCode, Json, extract::State};
use serde_json::Value;

use crate::{domain::model::UserId, AppError, AppState, AppUseCase};

use super::{respond_internal_server_error, respond_unauthorized, respond_with_user_dto};

pub async fn get_current_user(
    State(state): State<Arc<AppState>>,
    curr_user_id: UserId,
) -> (StatusCode, Json<Value>) {
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
            AppError::AuthUnauthorized => respond_unauthorized(err),
            AppError::AuthInvalidTokenErr(msg) => {
                respond_unauthorized(AppError::AuthInvalidTokenErr(msg))
            }
            _ => respond_internal_server_error(err),
        },
    }
}
