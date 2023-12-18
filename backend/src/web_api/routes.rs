use axum::Router;
use axum::routing::{get, post, put};
use axum_extra::routing::SpaRouter;
use tower_http::cors::{AllowHeaders, Any, CorsLayer};
use tower_http::trace::TraceLayer;
use crate::AppState;
use crate::web_api::{create_article, update_article, delete_article, follow_user, get_articles,
                     get_current_user, get_user_profile, login_user, register_user, unfollow_user, update_current_user};
use crate::web_api::healthcheck::health_check;

pub fn routes(state: AppState, assets_dir: String) -> Router {
    //
    let tracing_layer = TraceLayer::new_for_http();
    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(AllowHeaders::any());

    Router::new()
        .route("/api/healthcheck", get(health_check))
        .route("/api/users/login", post(login_user))
        .route("/api/users", post(register_user))
        .route("/api/user", get(get_current_user).put(update_current_user))
        .route("/api/profiles/:username", get(get_user_profile))
        .route(
            "/api/profiles/:username/follow",
            post(follow_user).delete(unfollow_user),
        )
        .route("/api/articles", get(get_articles).post(create_article))
        .route(
            "/api/articles/:slug",
            put(update_article).delete(delete_article),
        )
        .layer(tracing_layer)
        .layer(cors_layer)
        .with_state(state)
        .merge(SpaRouter::new("/assets", assets_dir))
}
