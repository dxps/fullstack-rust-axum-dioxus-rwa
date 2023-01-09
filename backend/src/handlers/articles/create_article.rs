use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::{
    handlers::{respond_internal_server_error, respond_unauthorized},
    token::Claims,
    AppError, AppState,
};

#[derive(Debug, Deserialize)]
pub struct CreateArticleInput {
    pub article: CreateArticleInputArticleKey,
}

#[derive(Debug, Deserialize)]
pub struct CreateArticleInputArticleKey {
    pub title: String,
    pub description: String,
    pub body: String,
    #[serde(rename = "tagList")]
    pub tag_list: Vec<String>,
}

pub async fn create_article(
    State(state): State<Arc<AppState>>,
    user_claims: Claims,
    Json(input): Json<CreateArticleInput>,
) -> (StatusCode, Json<Value>) {
    //
    match state
        .articles_mgr
        .create_article(
            input.article.title,
            input.article.description,
            input.article.body,
            input.article.tag_list,
            user_claims.sub,
        )
        .await
    {
        Ok(article) => (
            StatusCode::OK,
            Json(json!({
                "article": article,
            })),
        ),
        Err(err) => match err {
            AppError::AuthUnauthorized => respond_unauthorized(err),
            _ => respond_internal_server_error(err),
        },
    }
}
