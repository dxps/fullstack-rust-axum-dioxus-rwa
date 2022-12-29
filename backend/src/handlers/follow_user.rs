use std::sync::Arc;

use axum::{extract::{Path, State}, http::StatusCode, Json};
use serde_json::{json, Value};

use crate::{
    domain::model::UserId,
    handlers::{get_user_profile, respond_not_found},
    token::Claims,
    AppError, AppState,
};

use super::{respond_bad_request, respond_internal_server_error, respond_unauthorized};

pub async fn follow_user(
    State(state): State<Arc<AppState>>,
    Path(username): Path<String>,
    user_claims: Claims,
) -> (StatusCode, Json<Value>) {
    let curr_user_id = UserId::from(user_claims.sub);
    let profile = state.user_repo.follow_user(&curr_user_id, &username).await;

    match profile {
        Ok(profile) => (StatusCode::OK, Json(json!({ "profile": profile }))),
        Err(err) => match err {
            AppError::Ignorable => {
                get_user_profile(State(state), curr_user_id, Path(username)).await
            }
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

pub async fn unfollow_user(
    Path(username): Path<String>,
    user_claims: Claims,
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<Value>) {
    let curr_user_id = UserId::from(user_claims.sub);
    let profile = state
        .user_repo
        .unfollow_user(&curr_user_id, &username)
        .await;

    match profile {
        Ok(profile) => (StatusCode::OK, Json(json!({ "profile": profile }))),
        Err(err) => match err {
            AppError::Ignorable => {
                get_user_profile(State(state), curr_user_id, Path(username)).await
            }
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
