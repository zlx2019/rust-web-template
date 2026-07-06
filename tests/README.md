# Tests

Integration tests that exercise the crate as a whole (each `tests/*.rs` file is
compiled as its own crate and can only reach the crate's public API).

Suggested scope:

- End-to-end HTTP flows: drive `router()` with `tower::ServiceExt::oneshot` (no
  network port required), or run the server and hit it with an HTTP client.
- Full-feature scenarios spanning multiple handlers, routes and middleware.

Tip: to reach `router()` and other internals from here, expose them through a
library target (`src/lib.rs`) and add the test-only deps (e.g. `tower`) under
`[dev-dependencies]`.

Run with `cargo nextest run` (or `cargo test`).
