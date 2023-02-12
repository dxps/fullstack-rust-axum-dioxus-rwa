use crate::{
    domain::model::UserId,
    web_api::{extractors::InputJson, respond_bad_request, respond_internal_server_error},
    AppError, AppState,
};
use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use serde_json::{json, Value};

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
    State(state): State<AppState>,
    user_id: UserId,
    InputJson(input): InputJson<CreateArticleInput>,
) -> (StatusCode, Json<Value>) {
    //
    match state
        .articles_mgr
        .create_article(
            input.article.title,
            input.article.description,
            input.article.body,
            input.article.tag_list,
            user_id,
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
            AppError::AlreadyExists(_) => respond_bad_request(err),
            _ => respond_internal_server_error(err),
        },
    }
}
