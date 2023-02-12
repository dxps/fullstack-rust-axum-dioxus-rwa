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
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct UpdateArticleInputDTO {
    pub article: UpdateArticleInput,
}

pub async fn update_article(
    State(state): State<Arc<AppState>>,
    _: UserId,
    Path(slug): Path<String>,
    Json(input): Json<UpdateArticleInputDTO>,
) -> (StatusCode, Json<Value>) {
    //
    match state.articles_mgr.update_article(slug, input.article).await {
        Ok(article) => (StatusCode::OK, Json(json!({ "article": article }))),
        Err(err) => {
            log::debug!("update_article > err: {}", err);
            match err {
                AppError::AuthUnauthorized => respond_unauthorized(err),
                AppError::AuthInvalidTokenErr(_) => respond_unauthorized(err),
                AppError::AuthInvalidInput => respond_bad_request(err),
                AppError::NotFound(_) => respond_not_found(err),
                _ => respond_internal_server_error(err),
            }
        }
    }
}
