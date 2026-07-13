# rust-web-template

[English](./README.md)

[![Template CI](https://github.com/zlx2019/rust-web-template/actions/workflows/template-ci.yml/badge.svg)](https://github.com/zlx2019/rust-web-template/actions/workflows/template-ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.96.0%2B-orange.svg)](https://www.rust-lang.org)

一个基于 [cargo-generate](https://github.com/cargo-generate/cargo-generate) 的 Rust **Web 服务**模板。生成的项目基于 [axum](https://docs.rs/axum) + [tokio](https://tokio.rs)，自带路由组织、统一错误处理、优雅停机，以及完整的工程化配置：CI、发布自动化、格式化、Lint、测试、依赖审计、拼写检查和 pre-commit 钩子。

## 特性

- **开箱即用的 Web 骨架**：服务入口、按域拆分的路由（`routes/`）与 handler（`handlers/`），内置 `GET /`、`GET /health`、`GET /ready` 与 404 兜底。
- **统一错误处理**：`AppError` 实现 `IntoResponse`，handler 直接返回 `Result` 即可。
- **优雅停机**：收到 Ctrl+C 或 SIGTERM（容器与编排系统的标准停止信号）时干净退出。
- **环境变量配置**：监听地址由 `BIND_ADDR` 控制，通过 `dotenvy` 支持 `.env` 文件。
- **可选 tower-http 中间件**：生成时选择 `trace` / `cors` / `timeout` / `compression`，选中的 layer 自动注册。
- **必备依赖内置**：`axum`、`tokio`、`anyhow`、`thiserror`、`tracing`/`tracing-subscriber` 已固定内置；`serde_json`、`reqwest` 等常用库按需选择。
- **严格而务实的 Lint 基线**：`unsafe_code`、`missing_docs`、`unwrap_used`、`expect_used`、`panic`、`dbg_macro` 在业务代码中告警，测试代码自动豁免。
- **CI 与 Release 工作流**：每次 push 自动运行格式化、Clippy、文档构建、nextest、cargo-deny 和拼写检查；推送标签自动生成 Changelog、构建多平台二进制并可选发布 crates.io。
- **开源协作文件**：贡献指南、安全策略、Issue 模板、PR 模板和 Dependabot 配置。

## 快速开始

### 1. 安装前置工具

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install --locked cargo-generate
```

### 2. 生成项目

```bash
cargo generate zlx2019/rust-web-template --name my-service
```

生成过程中会依次询问：

| 选项 | 说明 |
|------|------|
| `Github username` | 用于生成 README 徽章与 `Cargo.toml` 仓库链接 |
| `description` | 项目简介，写入 README 与 `Cargo.toml` |
| `license` | 开源许可证（`MIT` / `Apache-2.0` / `GPL-3.0`），自动生成对应 LICENSE 文件 |
| `ask_common_libs` | 常用基础库：`uuid`、`rand`、`serde_json`、`chrono`、`reqwest` |
| `ask_middlewares` | tower-http 中间件：`trace`、`cors`、`timeout`、`compression` |

> `axum`、`tokio`、`anyhow`、`tracing`/`tracing-subscriber` 为 Web 服务必备，已固定内置，无需选择。

### 3. 运行服务

```bash
cd my-service
cargo run
```

```bash
curl http://localhost:3000/         # Hello, ...
curl http://localhost:3000/health   # ok
curl http://localhost:3000/ready    # ok
```

监听地址默认 `0.0.0.0:3000`，可通过 `BIND_ADDR` 环境变量或 `.env` 文件修改（参考 `.env.example`）。在 `src/routes/` 下新建路由模块并在 `routes::router()` 中 `.merge()`，即可添加新的业务域。

### 4. 安装开发工具

```bash
cargo install --locked cargo-deny
cargo install --locked cargo-nextest
cargo install --locked typos-cli
cargo install --locked git-cliff
pip install pre-commit
```

然后启用 pre-commit 钩子：

```bash
pre-commit install
```

## 生成后的目录结构

```text
.
├── .github/                 # CI、Release、Issue 和 PR 模板
├── examples/                # 可运行示例
├── fixtures/                # 测试数据
├── src/
│   ├── main.rs              # 入口：加载配置、初始化日志、带优雅停机启动服务
│   ├── routes/              # 路由组织与各业务域路由
│   ├── handlers/            # 请求处理函数
│   ├── error.rs             # 统一错误类型 AppError（实现 IntoResponse）
│   └── shutdown.rs          # Ctrl+C / SIGTERM 信号处理
├── tests/                   # 集成测试
├── .env.example             # 环境变量示例（BIND_ADDR）
├── Cargo.toml               # 包元数据、依赖、lint 和 profile 配置
├── README.md                # 生成项目的 README
├── CONTRIBUTING.md          # 贡献指南
├── SECURITY.md              # 安全策略
├── deny.toml                # cargo-deny 配置
├── rustfmt.toml             # 格式化风格
├── clippy.toml              # Clippy lint 参数
├── rust-toolchain.toml      # Rust 工具链锁定
└── .pre-commit-config.yaml  # 本地提交前检查
```

## 本模板如何维护

本仓库自身也是一个模板：`Cargo.toml` 与部分源码文件含 Liquid 占位符，无法作为普通 Rust crate 直接构建。Template CI 会先用 `cargo generate --path .` 展开模板，再对生成的项目运行完整检查。

本地验证模板：

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

本项目采用 [MIT](./LICENSE) 许可证。
