use crate::{
    domain::model::UserId,
    web_api::{
        respond_bad_request, respond_internal_server_error, respond_not_found, respond_unauthorized,
    },
    AppError, AppState, AppUseCase,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};

pub async fn get_user_profile(
    State(state): State<AppState>,
    curr_user_id: UserId,
    Path(username): Path<String>,
) -> (StatusCode, Json<Value>) {
    //
    let profile = state
        .user_repo
        .get_profile_by_username(&curr_user_id, &username, AppUseCase::GetUserProfile)
        .await;
    match profile {
        Ok(profile) => (StatusCode::OK, Json(json!({ "profile": profile }))),
        Err(err) => match err {
            AppError::NotFound(_) => respond_not_found(err),
            AppError::InvalidRequest(_) => respond_bad_request(err),
            AppError::Unauthorized(_) => respond_unauthorized(err),
            _ => respond_internal_server_error(err),
        },
    }
}
