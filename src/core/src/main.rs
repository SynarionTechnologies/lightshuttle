mod app;
mod docker;
mod routes;

use app::build_router;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::new("info"))
        .init();

    let addr: SocketAddr = "127.0.0.1:7878".parse().unwrap();
    tracing::info!("LightShuttle Core starting on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, build_router()).await.unwrap();
}
