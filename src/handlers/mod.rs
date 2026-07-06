//! Request handlers.
//!
//! Each business domain gets its own handler module; this module also hosts
//! shared handlers such as the index route. Handlers only parse input, invoke
//! logic and build the response; route registration lives in `crate::routes`.

pub mod health;

/// Index route: returns a welcome message.
pub async fn index() -> &'static str {
    "Hello, rust-web-template!"
}
