//! Health check routes.

use axum::{Router, routing::get};

use crate::handlers::health;

/// Registers the health check routes: `GET /health` (liveness) and `GET /ready`
/// (readiness).
pub fn routes() -> Router {
    Router::new()
        .route("/health", get(health::health))
        .route("/ready", get(health::ready))
}
