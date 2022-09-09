use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, Extension, Json};
use serde_json::{json, Value};

use crate::{
    domain::model::UserId,
    handlers::{get_user_profile, respond_not_found},
    token::Claims,
    AppError, AppState,
};

use super::{respond_bad_request, respond_internal_server_error, respond_unauthorized};

pub async fn follow_user(
    Path(username): Path<String>,
    user_claims: Claims,
    Extension(state): Extension<Arc<AppState>>,
) -> (StatusCode, Json<Value>) {
    let profile = state
        .user_repo
        .follow_user(
            &UserId::from(user_claims.sub),
            &user_claims.username,
            &username,
        )
        .await;

    match profile {
        Ok(profile) => (StatusCode::OK, Json(json!({ "profile": profile }))),
        Err(err) => match err {
            AppError::Ignorable => {
                get_user_profile(Path(user_claims.username), Extension(state)).await
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
