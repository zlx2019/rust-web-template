//! Unified application error type.
//!
//! Handlers return `Result<T, AppError>`; the error is turned into an HTTP
//! response through [`IntoResponse`]. Use `?` to bubble up any [`anyhow::Error`]
//! as an internal error, and add variants as the application grows.

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

/// Application-level error returned by handlers.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    /// The requested resource does not exist (maps to 404).
    #[error("resource not found")]
    NotFound,

    /// An unexpected internal failure (maps to 500). Wraps any [`anyhow::Error`],
    /// so handlers can use `?` on fallible operations.
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    /// Maps the error to an HTTP status and a client-facing message.
    ///
    /// Internal errors are logged in full but only report a generic message to
    /// the client, so implementation details are never leaked.
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "not found"),
            AppError::Internal(err) => {
                tracing::error!("internal error: {err:?}");
                (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
            }
        };
        (status, message).into_response()
    }
}
