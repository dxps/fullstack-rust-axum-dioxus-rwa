use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use serde_json::Value;

use crate::{token::create_jwt, AppError::AuthLoginFailed, AppState};

use super::{respond_internal_server_error, respond_unauthorized, respond_with_user_dto};

#[derive(Debug, Deserialize)]
pub struct LoginUserInput {
    pub user: LoginUserInputUserKey,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserInputUserKey {
    pub email: String,
    pub password: String,
}

#[axum_macros::debug_handler]
pub async fn login_user(
    State(state): State<Arc<AppState>>,
    Json(input): Json<LoginUserInput>,
) -> (StatusCode, Json<Value>) {
    match state
        .auth_mgr
        .login_user(input.user.email, input.user.password)
        .await
    {
        Ok(user) => match create_jwt(user.id, user.email.clone(), user.username.clone()) {
            Ok(token) => {
                respond_with_user_dto(user.email, Some(token), user.username, user.bio, user.image)
            }
            Err(err) => {
                log::error!("Failed to create jwt: {err}");
                respond_internal_server_error(err)
            }
        },
        Err(err) => match err {
            AuthLoginFailed => respond_unauthorized(err),
            _ => respond_internal_server_error(err),
        },
    }
}
