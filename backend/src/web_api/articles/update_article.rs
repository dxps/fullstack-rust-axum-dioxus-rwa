use crate::{
    domain::{logic::UpdateArticleInput, model::UserId},
    web_api::{
        respond_bad_request, respond_internal_server_error, respond_not_found, respond_unauthorized,
    },
    AppError, AppState,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Debug, Deserialize)]
pub struct UpdateArticleInputDTO {
    pub article: UpdateArticleInput,
}

pub async fn update_article(
    State(state): State<AppState>,
    curr_user_id: UserId,
    Path(slug): Path<String>,
    Json(input): Json<UpdateArticleInputDTO>,
) -> (StatusCode, Json<Value>) {
    //
    match state
        .articles_mgr
        .update_article(curr_user_id, slug, input.article)
        .await
    {
        Ok(article) => (StatusCode::OK, Json(json!({ "article": article }))),
        Err(err) => {
            log::error!("Failed to update article: {}", err);
            match err {
                AppError::Unauthorized(_) => respond_unauthorized(err),
                AppError::InvalidRequest(_) => respond_bad_request(err),
                AppError::NotFound(_) => respond_not_found(err),
                _ => respond_internal_server_error(err),
            }
        }
    }
}
