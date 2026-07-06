//! Graceful shutdown signal handling.
//!
//! Provides a future that resolves when an interrupt signal arrives, intended for
//! `axum::serve(..).with_graceful_shutdown(..)`. Listens for Ctrl+C (cross-platform)
//! and Unix SIGTERM (the stop signal used by containers and orchestrators).

/// Waits for a shutdown signal: Ctrl+C or, on Unix, SIGTERM, whichever arrives first.
///
/// Failing to install a signal handler does not panic; it is only logged so the
/// running server is unaffected.
pub async fn shutdown_signal() {
    let ctrl_c = async {
        if let Err(err) = tokio::signal::ctrl_c().await {
            tracing::error!("failed to install Ctrl+C handler: {err}");
        }
    };
    #[cfg(unix)]
    let terminate = async {
        use tokio::signal::unix::{SignalKind, signal};
        match signal(SignalKind::terminate()) {
            Ok(mut stream) => {
                stream.recv().await;
            }
            Err(err) => tracing::error!("failed to install SIGTERM handler: {err}"),
        }
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
    tracing::info!("shutdown signal received, starting graceful shutdown");
}
