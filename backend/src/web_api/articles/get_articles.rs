use crate::{web_api::respond_internal_server_error, AppState};
use axum::{extract::State, http::StatusCode, Json};
use serde_json::{json, Value};

pub async fn get_articles(State(state): State<AppState>) -> (StatusCode, Json<Value>) {
    //
    match state.articles_mgr.get_articles().await {
        Ok(articles) => (
            StatusCode::OK,
            Json(json!({
                "articles": articles,
                "articles_count": articles.len()
            })),
        ),
        Err(err) => respond_internal_server_error(err),
    }
}
