use std::sync::Arc;

use axum::{http::StatusCode, Extension, Json};
use serde::Deserialize;
use serde_json::Value;

use crate::{
    domain::model::User,
    handlers::{respond_bad_request, respond_internal_server_error},
    jwt::sign,
    AppError::UserRepoSaveEmailAlreadyExistsErr,
    AppState,
};

use super::{UserAuthnOutputDTO, UserInfoDTO};

#[derive(Debug, Deserialize)]
pub struct RegisterUserInput {
    pub user: RegisterUserInputUserKey,
}

impl Into<User> for RegisterUserInput {
    fn into(self) -> User {
        User {
            id: 0, // not relevant
            email: self.user.email,
            username: self.user.username,
            bio: "".to_string(),
            image: None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct RegisterUserInputUserKey {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub async fn register_user(
    Json(input): Json<RegisterUserInput>,
    Extension(state): Extension<Arc<AppState>>,
) -> (StatusCode, Json<Value>) {
    let pwd = input.user.password.clone();
    let user: User = input.into();
    match &state.auth_mgr.register_user(&user, pwd).await {
        Ok(id) => match sign(*id) {
            Ok(token) => {
                let out = UserAuthnOutputDTO {
                    user: UserInfoDTO {
                        email: user.email,
                        token: Some(token),
                        username: user.username,
                        bio: "".to_string(),
                        image: None,
                    },
                };
                (StatusCode::OK, Json(serde_json::to_value(out).unwrap()))
            }
            Err(_) => todo!(),
        },
        Err(err) => match err {
            UserRepoSaveEmailAlreadyExistsErr => respond_bad_request(err),
            _ => respond_internal_server_error(err),
        },
    }
}
