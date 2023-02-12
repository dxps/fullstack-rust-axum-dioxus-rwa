use axum::{
    extract::State,
    response::IntoResponse,
    routing::{get, post, put},
    Json, Router,
};
use axum_extra::routing::SpaRouter;
use backend::{
    config::get_config,
    db::{init_db_pool, ping_db},
    web_api::{
        create_article, delete_article, follow_user, get_articles, get_current_user,
        get_user_profile, login_user, register_user, unfollow_user, update_article,
        update_current_user,
    },
    AppState,
};
use clap::Parser;
use serde_json::json;
use std::{
    net::{IpAddr, Ipv6Addr, SocketAddr},
    process::exit,
    str::FromStr,
};
use tokio::signal::{self, unix::SignalKind};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

#[tokio::main]
async fn main() {
    let opt = Opt::parse();

    // Logging init.
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var(
            "RUST_LOG",
            format!(
                "{},hyper=info,mio=info,sqlx=warn,tower_http=warn",
                opt.log_level
            ),
        )
    }
    tracing_subscriber::fmt::init();

    // Load the config.
    let app_cfg = get_config().expect("Failed to load the app config.");

    // Init the database connection pool.
    let db_conn_pool = init_db_pool(&app_cfg)
        .await
        .expect("Failed to connect to database.");
    match ping_db(&db_conn_pool).await {
        true => log::info!(
            "Connected to the database (with {} conns).",
            db_conn_pool.size()
        ),
        false => {
            log::error!("Failed to ping the database. Exiting now.");
            exit(1);
        }
    }

    let state = AppState::new(db_conn_pool);

    let routes = routes(state, opt.assets_dir);

    let sock_addr = SocketAddr::from((
        IpAddr::from_str(opt.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        opt.port,
    ));
    log::info!("Listening for requests on http://{} ...", sock_addr);

    axum::Server::bind(&sock_addr)
        .serve(routes.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Unable to start server");
}

fn routes(state: AppState, assets_dir: String) -> Router {
    let tracing_layer = TraceLayer::new_for_http();
    let cors_layer = CorsLayer::new().allow_origin(Any);

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
        // .with_state(Arc::new(state))
        .with_state(state)
        .merge(SpaRouter::new("/assets", assets_dir))
}

async fn health_check(State(state): State<AppState>) -> impl IntoResponse {
    match ping_db(&state.dbcp).await {
        true => Json(json!({ "database": "ok" })),
        false => Json(json!({ "database": "err" })),
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to init Ctrl+C handler")
    };
    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(SignalKind::terminate())
            .expect("Failed to init signal handler")
            .recv()
            .await
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
    log::info!("Shutting down gracefully ...")
}

#[derive(Parser, Debug)]
#[clap(
    name = "server",
    about = "The server side of Fullstack Rust RealWorld App project."
)]
struct Opt {
    /// The HTTP listening address.
    #[clap(short = 'a', long = "addr", default_value = "::1")]
    addr: String,

    /// The HTTP listening port.
    #[clap(short = 'p', long = "port", default_value = "8080")]
    port: u16,

    /// The logging level.
    #[clap(short = 'l', long = "log", default_value = "info")]
    log_level: String,

    /// The directory where assets (static) files are served from (for `/assets/*` requests).
    #[clap(short = 's', long = "assets-dir", default_value = "../dist")]
    assets_dir: String,
}
