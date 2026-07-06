//! Route composition and global middleware.
//!
//! `router()` merges the per-domain routers, installs the 404 fallback, and
//! registers the middleware layers selected at generation time. To add a new
//! domain, declare its route module here and `.merge()` it in `router()`.

pub mod health;

use axum::{Router, routing::get};

use crate::{error::AppError, handlers};

/// Builds the application router: merges the domain routers, installs the 404
/// fallback, and registers the selected middleware layers.
///
/// Layers are applied via `.layer()`; a layer added later wraps the earlier ones
/// (it sees the request first and the response last).
pub fn router() -> Router {
    Router::new()
        .route("/", get(handlers::index))
        .merge(health::routes())
        .fallback(fallback)
    {%- if ask_middlewares contains "compression" %}
        .layer(tower_http::compression::CompressionLayer::new())
    {%- endif %}
    {%- if ask_middlewares contains "timeout" %}
        .layer(tower_http::timeout::TimeoutLayer::with_status_code(
            axum::http::StatusCode::REQUEST_TIMEOUT,
            std::time::Duration::from_secs(30),
        ))
    {%- endif %}
    {%- if ask_middlewares contains "cors" %}
        .layer(tower_http::cors::CorsLayer::permissive())
    {%- endif %}
    {%- if ask_middlewares contains "trace" %}
        .layer(tower_http::trace::TraceLayer::new_for_http())
    {%- endif %}
}

/// Fallback handler for unmatched routes: responds with 404 via [`AppError`].
async fn fallback() -> AppError {
    AppError::NotFound
}
