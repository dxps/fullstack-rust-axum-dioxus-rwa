use crate::{
    domain::model::UserId,
    web_api::{respond_bad_request, respond_internal_server_error, respond_unauthorized},
    AppError, AppState,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::Value;

pub async fn delete_article(
    State(state): State<AppState>,
    curr_user_id: UserId,
    Path(slug): Path<String>,
) -> (StatusCode, Json<Value>) {
    //
    match state.articles_mgr.delete_article(curr_user_id, slug).await {
        Ok(_) => (StatusCode::NO_CONTENT, Json(Value::default())),
        Err(err) => match err {
            AppError::Unauthorized(_) => respond_unauthorized(err),
            AppError::InvalidRequest(_) => respond_bad_request(err),
            _ => respond_internal_server_error(err),
        },
    }
}
