use crate::{
    domain::model::UserId,
    web_api::{respond_internal_server_error, respond_unauthorized},
    AppState,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::Value;

pub async fn delete_article(
    State(state): State<AppState>,
    _: UserId,
    Path(slug): Path<String>,
) -> (StatusCode, Json<Value>) {
    //
    match state.articles_mgr.delete_article(slug).await {
        Ok(_) => (StatusCode::NO_CONTENT, Json(Value::default())),
        Err(err) => match err {
            crate::AppError::AuthUnauthorized => respond_unauthorized(err),
            crate::AppError::AuthInvalidTokenErr(_) => respond_unauthorized(err),
            _ => respond_internal_server_error(err),
        },
    }
}
