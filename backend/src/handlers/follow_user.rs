use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, Extension, Json};
use serde_json::{json, Value};

use crate::{
    domain::model::UserId,
    handlers::{get_user_profile, respond_not_found},
    AppError, AppState,
};

use super::{respond_bad_request, respond_internal_server_error, respond_unauthorized};

pub async fn follow_user(
    Path(username): Path<String>,
    user_id: UserId,
    Extension(state): Extension<Arc<AppState>>,
) -> (StatusCode, Json<Value>) {
    let profile = state.user_repo.follow_user(&user_id, &username).await;

    match profile {
        Ok(profile) => (StatusCode::OK, Json(json!({ "profile": profile }))),
        Err(err) => match err {
            AppError::IgnorableErr => get_user_profile(Path(username), Extension(state)).await,
            AppError::NothingFound => respond_not_found(err),
            AppError::InvalidInput => respond_bad_request(err),
            AppError::AuthUnauthorizedErr => respond_unauthorized(err),
            AppError::InvalidTokenErr(msg) => respond_unauthorized(AppError::InvalidTokenErr(msg)),
            _ => respond_internal_server_error(err),
        },
    }
}
