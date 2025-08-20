use lightshuttle_core::api::routes::router;
use lightshuttle_core::metrics;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[cfg(unix)]
fn ensure_not_root() {
    if users::get_current_uid() == 0 {
        tracing::error!("Refusing to run as root");
        std::process::exit(1);
    }
}

#[cfg(not(unix))]
fn ensure_not_root() {}

/// Entry point for the LightShuttle daemon service.
///
/// Initializes logging, sets up the HTTP server and routes,
/// then starts serving incoming requests.
#[tokio::main]
async fn main() {
    ensure_not_root();

    // Initialize the tracing subscriber for structured logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::new("info"))
        .init();

    // Prepare metrics recorder and startup timestamp
    metrics::init();

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
