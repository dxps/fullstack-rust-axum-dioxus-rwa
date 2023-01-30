use crate::{domain::model::Article, AppState};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::DateTime;
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

    todo!()
    // (
    //     StatusCode::OK,
    //     Json(json!({
    //         "article": article
    //     })),
    // )
}
