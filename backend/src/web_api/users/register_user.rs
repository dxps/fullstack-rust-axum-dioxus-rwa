use super::responses::respond_with_user_dto;
use crate::{
    domain::model::User,
    web_api::{
        extractors::InputJson, respond_bad_request, respond_internal_server_error,
        token::create_jwt,
    },
    AppError::AlreadyExists,
    AppState,
};
use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct RegisterUserInput {
    pub user: RegisterUserInputUserKey,
}

impl Into<User> for RegisterUserInput {
    //
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
    State(state): State<AppState>,
    InputJson(input): InputJson<RegisterUserInput>,
) -> (StatusCode, Json<Value>) {
    //
    let pwd = input.user.password.clone();
    let user: User = input.into();
    match state.auth_mgr.register_user(&user, pwd).await {
        Ok(id) => match create_jwt(id, user.email.clone(), user.username.clone()) {
            Ok(token) => {
                respond_with_user_dto(user.email, Some(token), user.username, "".to_string(), None)
            }
            Err(err) => {
                log::error!("Failed to create JWT: {err}");
                respond_internal_server_error(err)
            }
        },
        Err(err) => match err {
            AlreadyExists(_) => respond_bad_request(err),
            _ => respond_internal_server_error(err),
        },
    }
}
