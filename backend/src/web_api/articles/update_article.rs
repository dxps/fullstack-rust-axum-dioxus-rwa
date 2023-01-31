use crate::{
    web_api::{respond_internal_server_error, respond_not_found},
    AppError, AppState,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use log::error;
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct UpdateArticleInputDTO {
    pub article: UpdateArticleInput,
}

#[derive(Debug, Deserialize)]
pub struct UpdateArticleInput {
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
    #[serde(rename = "tagList")]
    pub tag_list: Option<Vec<String>>,
}

pub async fn update_article(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
    Json(input): Json<UpdateArticleInputDTO>,
) -> (StatusCode, Json<Value>) {
    //
    log::debug!("update_article >> input={:?}", input);
    let res = state.articles_mgr.get_article(slug).await;
    if let Err(err) = res {
        return respond_not_found(err);
    }
    let res = res.unwrap();
    if res.is_none() {
        return respond_not_found(AppError::NothingFound);
    }
    let mut res = res.unwrap();

    if let Some(title) = input.article.title {
        res.title = title;
    }
    if let Some(description) = input.article.description {
        res.description = description;
    }
    if let Some(body) = input.article.body {
        res.body = body;
    }
    if let Some(tag_list) = input.article.tag_list {
        res.tag_list = tag_list;
    }

    // match res {
    //     Ok(article) => (StatusCode::OK, Json(json!({ "article": article }))),
    //     Err(err) => respond_internal_server_error(err),
    // }

    todo!()
}
