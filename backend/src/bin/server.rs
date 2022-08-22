use std::{
    net::{IpAddr, Ipv6Addr, SocketAddr},
    str::FromStr,
};

use axum::{response::IntoResponse, routing::get, Router};
use axum_extra::routing::SpaRouter;
use clap::Parser;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    let opt = Opt::parse();

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", format!("{},hyper=info,mio=info", opt.log_level))
    }
    tracing_subscriber::fmt::init();
    let tracing_layer = TraceLayer::new_for_http();

    let http_svc = Router::new()
        .route("/api/healthcheck", get(health_check))
        .merge(SpaRouter::new("/assets", opt.static_dir))
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

    /// The directory where static files are served from.
    #[clap(long = "static-dir", default_value = "../dist")]
    static_dir: String,
}
