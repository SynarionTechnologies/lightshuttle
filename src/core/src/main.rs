mod app;
mod docker;
mod routes;

use app::build_router;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Entry point for the LightShuttle Core server.
/// 
/// Initializes logging, sets up the HTTP server and routes,
/// then starts serving incoming requests.
#[tokio::main]
async fn main() {
    // Initialize the tracing subscriber for structured logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::new("info"))
        .init();

    // Define the listening address
    let addr: SocketAddr = "127.0.0.1:7878".parse().expect("Invalid bind address");
    tracing::info!("LightShuttle Core starting on http://{}", addr);

    // Bind TCP listener
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind address");

    // Start serving using axum
    axum::serve(listener, build_router())
        .await
        .expect("Server failed");
}
