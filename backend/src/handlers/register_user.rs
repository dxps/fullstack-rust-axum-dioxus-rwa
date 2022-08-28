use std::sync::Arc;

use axum::{http::StatusCode, Extension, Json};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{domain::model::User, handlers::internal_error_response, AppState};

#[derive(Debug, Deserialize)]
pub struct RegisterUserInput {
    pub user: RegisterUserInputUserKey,
}

impl Into<User> for RegisterUserInput {
    fn into(self) -> User {
        User {
            email: self.user.email,
            username: self.user.username,
            bio: "".to_string(),
            image: "".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct RegisterUserInputUserKey {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterUserOutput {
    pub user: RegisterUserOutputUserKey,
}

#[derive(Debug, Serialize)]
pub struct RegisterUserOutputUserKey {
    pub email: String,
    pub token: String,
    pub username: String,
    pub bio: String,
    pub image: String,
}

pub async fn register_user(
    Extension(state): Extension<Arc<AppState>>,
    Json(input): Json<RegisterUserInput>,
    // ) -> impl IntoResponse {
) -> (StatusCode, Json<Value>) {
    log::debug!("Registering based on {:?}", input);
    let pwd = input.user.password.clone();
    let user: User = input.into();
    let repo = &state.user_repo;
    match repo.save(&user, pwd).await {
        Ok(()) => {
            log::debug!("Registered user with username {:?}.", user.username)
        }
        Err(e) => {
            log::error!(
                "Failed to register user with username {:?}: {}",
                user.username,
                e
            );
            return internal_error_response(e);
        }
    };

    let out = RegisterUserOutput {
        user: RegisterUserOutputUserKey {
            email: user.email,
            token: "TODO".to_string(),
            username: user.username,
            bio: "".to_string(),
            image: "".to_string(),
        },
    };

    (StatusCode::OK, Json(serde_json::to_value(out).unwrap()))
}
