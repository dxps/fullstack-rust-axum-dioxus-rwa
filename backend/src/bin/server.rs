use std::{
    net::{IpAddr, Ipv6Addr, SocketAddr},
    str::FromStr,
};

use axum::{response::IntoResponse, routing::get, Router};
use axum_extra::routing::SpaRouter;
use backend::config::get_config;
use clap::Parser;
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    let opt = Opt::parse();

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", format!("{},hyper=info,mio=info", opt.log_level))
    }

    // Load the config.
    let app_cfg = get_config().expect("Failed to load the app config.");

    // Init db connection pool.
    let db_conn_pool = PgPoolOptions::new()
        .idle_timeout(std::time::Duration::from_secs(3))
        // .connect_lazy(&app_cfg.database.connection_string().expose_secret())
        .connect(&app_cfg.database.connection_string().expose_secret())
        .await
        .unwrap_or_else(|err| {
            log::error!("Failed to connect to database: {}", err);
            panic!("Failed to connect to database: {}", err);
        });
    // .expect("Failed to connect to database");
    log::info!("Connected to the database ({} conns)", db_conn_pool.size());

    tracing_subscriber::fmt::init();
    let tracing_layer = TraceLayer::new_for_http();

    let http_svc = Router::new()
        .route("/api/healthcheck", get(health_check))
        .merge(SpaRouter::new("/assets", opt.assets_dir))
        .layer(tracing_layer)
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

async fn health_check() -> impl IntoResponse {
    "OK"
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
