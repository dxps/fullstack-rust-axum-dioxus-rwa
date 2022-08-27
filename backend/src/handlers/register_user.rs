use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use serde::Deserialize;

use crate::AppState;

#[derive(Deserialize, Debug)]
pub struct RegisterUserInput {
    pub user: RegisterUserInputUserKey,
}

#[derive(Deserialize, Debug)]
pub struct RegisterUserInputUserKey {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub async fn register_user(
    Extension(_state): Extension<Arc<AppState>>,
    Json(register_user): Json<RegisterUserInput>,
) -> impl IntoResponse {
    log::info!("Registering user: {:?}", register_user);
    // TODO: to be cont'd
    StatusCode::OK
}
