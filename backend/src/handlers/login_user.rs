use std::sync::Arc;

use axum::{http::StatusCode, Extension, Json};
use serde::Deserialize;
use serde_json::Value;

use crate::{AppError::LoginWrongCredentialsErr, AppState};

use super::{respond_internal_server_error, respond_unauthorized, UserAuthnOutputDTO, UserInfoDTO};

#[derive(Debug, Deserialize)]
pub struct LoginUserInput {
    pub user: LoginUserInputUserKey,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserInputUserKey {
    pub email: String,
    pub password: String,
}

pub async fn login_user(
    Json(input): Json<LoginUserInput>,
    Extension(state): Extension<Arc<AppState>>,
) -> (StatusCode, Json<Value>) {
    match state
        .auth_mgr
        .login_user(input.user.email, input.user.password)
        .await
    {
        Ok(user) => {
            let out = UserAuthnOutputDTO {
                user: UserInfoDTO {
                    email: user.email,
                    token: "TODO".to_string(),
                    username: user.username,
                    bio: user.bio,
                    image: user.image,
                },
            };
            (StatusCode::OK, Json(serde_json::to_value(out).unwrap()))
        }
        Err(err) => match err {
            LoginWrongCredentialsErr => respond_unauthorized(err),
            _ => respond_internal_server_error(err),
        },
    }
}
