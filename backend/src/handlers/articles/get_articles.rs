use std::sync::Arc;

use axum::{http::StatusCode, Extension, Json};
use serde_json::{json, Value};

use crate::{
    handlers::{respond_internal_server_error, respond_unauthorized},
    AppError, AppState,
};

pub async fn get_articles(Extension(state): Extension<Arc<AppState>>) -> (StatusCode, Json<Value>) {
    match state.articles_mgr.get_articles().await {
        Ok(articles) => (
            StatusCode::OK,
            Json(json!({
                "articles": articles,
                "articles_count": articles.len()
            })),
        ),
        Err(err) => match err {
            AppError::AuthUnauthorized => respond_unauthorized(err),
            _ => respond_internal_server_error(err),
        },
    }
}
