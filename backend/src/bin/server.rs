use std::{
    net::{IpAddr, Ipv6Addr, SocketAddr},
    process::exit,
    str::FromStr,
    sync::Arc,
};

use axum::{response::IntoResponse, routing::get, Extension, Json, Router};
use axum_extra::routing::SpaRouter;
use backend::{
    config::get_config,
    db::{init_db_pool, ping_db, DbConnPool},
};
use clap::Parser;
use serde_json::json;
use tower_http::trace::TraceLayer;

struct AppState {
    db_conn_pool: DbConnPool,
}

#[tokio::main]
async fn main() {
    let opt = Opt::parse();

    // Logging init.
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var(
            "RUST_LOG",
            format!("{},hyper=info,mio=info,sqlx=warn", opt.log_level),
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
            "Connected to the database (with {} conns)",
            db_conn_pool.size()
        ),
        false => {
            log::error!("Failed to ping the database. Exiting now.");
            exit(1);
        }
    }

    let tracing_layer = TraceLayer::new_for_http();
    let db_cp_layer = Arc::new(AppState { db_conn_pool });

    let http_svc = Router::new()
        .route("/api/healthcheck", get(health_check))
        .merge(SpaRouter::new("/assets", opt.assets_dir))
        .layer(tracing_layer)
        .layer(Extension(db_cp_layer))
        .into_make_service();

    let sock_addr = SocketAddr::from((
        IpAddr::from_str(opt.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        opt.port,
    ));
    log::info!("Listening on http://{}", sock_addr);

    axum::Server::bind(&sock_addr)
        .serve(http_svc)
        .await
        .expect("Unable to start server");
}

async fn health_check(Extension(state): Extension<Arc<AppState>>) -> impl IntoResponse {
    match ping_db(&state.db_conn_pool).await {
        true => Json(json!({ "database": "ok" })),
        false => Json(json!({ "database": "err" })),
    }
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
