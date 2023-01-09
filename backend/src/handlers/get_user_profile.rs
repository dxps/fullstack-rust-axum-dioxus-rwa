use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};

use crate::{domain::model::UserId, handlers::respond_not_found, AppError, AppState, AppUseCase};

use super::{respond_bad_request, respond_internal_server_error, respond_unauthorized};

pub async fn get_user_profile(
    State(state): State<Arc<AppState>>,
    curr_user_id: UserId,
    Path(username): Path<String>,
) -> (StatusCode, Json<Value>) {
    let profile = state
        .user_repo
        .get_profile_by_username(&curr_user_id, &username, AppUseCase::GetUserProfile)
        .await;
    match profile {
        Ok(profile) => (StatusCode::OK, Json(json!({ "profile": profile }))),
        Err(err) => match err {
            AppError::NothingFound => respond_not_found(err),
            AppError::AuthInvalidInput => respond_bad_request(err),
            AppError::AuthUnauthorized => respond_unauthorized(err),
            AppError::AuthInvalidTokenErr(msg) => {
                respond_unauthorized(AppError::AuthInvalidTokenErr(msg))
            }
            _ => respond_internal_server_error(err),
        },
    }
}
