//! Health check handlers.

use crate::error::AppError;

/// Liveness check: returns `ok` while the process is running.
pub async fn health() -> &'static str {
    "ok"
}

/// Readiness check: returns `ready` once dependencies are up and the service can
/// serve traffic.
///
/// Returns `Result` to show the error pattern: a real project checks its
/// dependencies here and returns `Err(AppError::..)` when not ready.
pub async fn ready() -> Result<&'static str, AppError> {
    Ok("ready")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Liveness check should return `ok`.
    #[tokio::test]
    async fn health_returns_ok() {
        assert_eq!(health().await, "ok");
    }

    /// Readiness check should succeed with `ready`.
    #[tokio::test]
    async fn ready_returns_ready() {
        assert!(matches!(ready().await, Ok("ready")));
    }
}
