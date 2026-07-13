# rust-web-template

[简体中文](./README.zh.md)

[![Template CI](https://github.com/zlx2019/rust-web-template/actions/workflows/template-ci.yml/badge.svg)](https://github.com/zlx2019/rust-web-template/actions/workflows/template-ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.96.0%2B-orange.svg)](https://www.rust-lang.org)

A Rust **web service** template powered by [cargo-generate](https://github.com/cargo-generate/cargo-generate). It creates a production-ready [axum](https://docs.rs/axum) + [tokio](https://tokio.rs) service with routing, unified error handling, graceful shutdown, and a complete engineering setup: CI, release automation, formatting, linting, tests, dependency auditing, spell checking, and pre-commit hooks.

## Features

- **Ready-to-run web skeleton**: service entry point, domain-split routers (`routes/`) and handlers (`handlers/`), built-in `GET /`, `GET /health`, and `GET /ready` endpoints with a 404 fallback.
- **Unified error handling**: an `AppError` type implementing `IntoResponse`, so handlers can simply return `Result`.
- **Graceful shutdown**: stops cleanly on Ctrl+C or SIGTERM, the stop signal used by containers and orchestrators.
- **Environment-based configuration**: listen address via `BIND_ADDR` with `.env` support through `dotenvy`.
- **Optional tower-http middlewares**: choose `trace`, `cors`, `timeout`, and `compression` at generation time; the selected layers are registered automatically.
- **Batteries included**: `axum`, `tokio`, `anyhow`, `thiserror`, and `tracing`/`tracing-subscriber` are built in; `serde_json`, `reqwest`, and other common crates are optional presets.
- **Strict but practical lints**: `unsafe_code`, `missing_docs`, `unwrap_used`, `expect_used`, `panic`, and `dbg_macro` warn in production code while tests stay exempt.
- **CI and release workflows**: formatting, Clippy, documentation builds, nextest, cargo-deny, and typos on every push; tag-driven releases with changelog generation, multi-platform binaries, and optional crates.io publishing.
- **Collaboration files**: contribution guide, security policy, issue templates, a pull request template, and Dependabot configuration.

## Quick Start

### 1. Install Prerequisites

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install --locked cargo-generate
```

### 2. Generate A Project

```bash
cargo generate zlx2019/rust-web-template --name my-service
```

The generator will ask for the following values:

| Option | Description |
|--------|-------------|
| `Github username` | Used for README badges and `Cargo.toml` homepage/repository links |
| `description` | Project description written to README and `Cargo.toml` |
| `license` | Open source license, one of `MIT`, `Apache-2.0`, or `GPL-3.0`; the matching `LICENSE` file is generated automatically |
| `ask_common_libs` | Common crate choices: `uuid`, `rand`, `serde_json`, `chrono`, and `reqwest` |
| `ask_middlewares` | tower-http middlewares to enable: `trace`, `cors`, `timeout`, and `compression` |

> `axum`, `tokio`, `anyhow`, and `tracing`/`tracing-subscriber` are essential for a web service, so they are always included and never asked about.

### 3. Run The Service

```bash
cd my-service
cargo run
```

```bash
curl http://localhost:3000/         # Hello, ...
curl http://localhost:3000/health   # ok
curl http://localhost:3000/ready    # ok
```

The listen address defaults to `0.0.0.0:3000` and can be changed with the `BIND_ADDR` environment variable or a `.env` file (see `.env.example`). Add a new domain by creating a router module under `src/routes/` and merging it in `routes::router()`.

### 4. Install Development Tools

```bash
cargo install --locked cargo-deny
cargo install --locked cargo-nextest
cargo install --locked typos-cli
cargo install --locked git-cliff
pip install pre-commit
```

Then enable pre-commit checks:

```bash
pre-commit install
```

## Generated Project Layout

```text
.
├── .github/                 # CI, release, issue, and pull request templates
├── examples/                # Runnable examples
├── fixtures/                # Test data
├── src/
│   ├── main.rs              # Entry point: config, logging, serve with graceful shutdown
│   ├── routes/              # Route composition and per-domain routers
│   ├── handlers/            # Request handlers
│   ├── error.rs             # Unified AppError implementing IntoResponse
│   └── shutdown.rs          # Ctrl+C / SIGTERM signal handling
├── tests/                   # Integration tests
├── .env.example             # Environment variable sample (BIND_ADDR)
├── Cargo.toml               # Package metadata, dependencies, lints, and profile config
├── README.md                # README for the generated project
├── CONTRIBUTING.md          # Contribution guide
├── SECURITY.md              # Security policy
├── deny.toml                # cargo-deny configuration
├── rustfmt.toml             # Formatting style
├── clippy.toml              # Clippy lint parameters
├── rust-toolchain.toml      # Pinned Rust toolchain
└── .pre-commit-config.yaml  # Local pre-commit checks
```

## Maintaining This Template

This repository is itself a template. `Cargo.toml` and some source files contain Liquid placeholders, so the repository cannot be built directly as a normal Rust crate. Template CI first expands the template with `cargo generate --path .`, then runs the full check suite against the generated project.

To validate the template locally:

```bash
cargo generate --path . --name smoke-test --destination /tmp
cd /tmp/smoke-test
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
cargo nextest run --all-features --no-tests pass
cargo deny check
typos
```

## License

This project is licensed under [MIT](./LICENSE).
