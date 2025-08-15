mod api;
mod docker;
mod errors;
mod routes;
mod services;

#[cfg(feature = "openapi")]
mod openapi;

use api::routes::router;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Entry point for the LightShuttle daemon service.
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

    // Read bind address from environment variable or fallback to default
    let bind_addr = std::env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:7878".to_string());
    let addr: SocketAddr = bind_addr.parse().unwrap_or_else(|_| {
        tracing::error!("Invalid bind address provided: {bind_addr}");
        std::process::exit(1);
    });

    tracing::info!("LightShuttle API starting on http://{addr}");

    // Bind TCP listener
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap_or_else(|e| {
            tracing::error!("Failed to bind address: {e}");
            std::process::exit(1);
        });

    // Start serving using axum
    axum::serve(listener, router()).await.unwrap_or_else(|e| {
        tracing::error!("Server crashed: {e}");
        std::process::exit(1);
    });
}
