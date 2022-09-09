use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, Extension, Json};
use serde_json::{json, Value};

use crate::{handlers::respond_not_found, AppError, AppState, AppUseCase};

use super::{respond_bad_request, respond_internal_server_error, respond_unauthorized};

pub async fn get_user_profile(
    Path(username): Path<String>,
    Extension(state): Extension<Arc<AppState>>,
) -> (StatusCode, Json<Value>) {
    let profile = state
        .user_repo
        .get_profile_by_username(&username, AppUseCase::GetUserProfile)
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
