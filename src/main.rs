//! Application entry point.
//!
//! Loads `.env` if present, initializes logging, builds the router, binds the
//! configured address, and serves the app with graceful shutdown on Ctrl+C /
//! SIGTERM.

mod error;
mod handlers;
mod routes;
mod shutdown;

use std::net::SocketAddr;

use anyhow::Context;
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

/// Bind address used when `BIND_ADDR` is not set in the environment.
const DEFAULT_BIND_ADDR: &str = "0.0.0.0:3000";

/// Program entry point: loads configuration, sets up logging, then runs the
/// server with graceful shutdown.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load variables from a local `.env` file when present; in production the
    // real process environment is used and this is a no-op.
    dotenvy::dotenv().ok();
    init_tracing();

    let app = routes::router();
    let bind_addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| DEFAULT_BIND_ADDR.to_string());
    let listener = TcpListener::bind(&bind_addr)
        .await
        .with_context(|| format!("failed to bind listen address: {bind_addr}"))?;
    let local_addr: SocketAddr = listener
        .local_addr()
        .context("failed to get local listen address")?;
    tracing::info!("server started, listening on http://{local_addr}");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown::shutdown_signal())
        .await
        .context("web server error")?;

    tracing::info!("server shut down gracefully");
    Ok(())
}

/// Initializes the tracing subscriber, reading `RUST_LOG` and defaulting to `info`.
fn init_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt().with_env_filter(filter).init();
}
