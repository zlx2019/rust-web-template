# rust-web-template

[![Template CI](https://github.com/zlx2019/rust-web-template/actions/workflows/template-ci.yml/badge.svg)](https://github.com/zlx2019/rust-web-template/actions/workflows/template-ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.96.0%2B-orange.svg)](https://www.rust-lang.org)

一个 [cargo-generate](https://github.com/cargo-generate/cargo-generate) 项目模板，用于快速初始化带有完整工程化配置的 Rust **Web 服务** 项目。默认基于 [axum](https://docs.rs/axum) + [tokio](https://tokio.rs)，内置程序入口、服务启动与优雅停机。

## 使用

```bash
cargo install cargo-generate
cargo generate zlx2019/rust-web-template
```

生成过程中会依次询问：

| 选项 | 说明 |
|------|------|
| Github username | 用于生成 README 徽章与 Cargo.toml 仓库链接 |
| description | 项目简介 |
| license | 开源许可证（MIT / Apache-2.0 / GPL-3.0），自动生成对应 LICENSE 文件 |
| 常用基础库 | uuid / rand / serde_json / chrono / reqwest |
| 中间件 | 可选 tower-http：trace / cors / timeout / compression（选中才引入并注册） |

> `axum`、`tokio`、`anyhow`、`tracing`/`tracing-subscriber` 为 Web 服务必备，已固定内置，无需选择。

## 模板内容

- `src/` 预置 Web 脚手架：服务入口、按域拆分的路由（`routes/`）与 handler（`handlers/`）、优雅停机
- 内置 `axum` 路由（`GET /`、`GET /health`、`GET /ready`）、404 兜底与基于 `tracing` 的日志
- 统一错误类型 `AppError`（实现 `IntoResponse`），handler 可返回 `Result`
- 环境变量配置监听地址（`BIND_ADDR`）+ `.env` 支持（`dotenvy`）
- 可选 tower-http 中间件（trace / cors / timeout / compression），生成时选择、自动注册
- 收到 Ctrl+C / SIGTERM 时优雅停机
- `rust-toolchain.toml` 锁定 Rust 工具链版本
- Lint 规则预设（`unsafe_code` / `missing_docs` / `unwrap_used` 告警）
- pre-commit 钩子：fmt / cargo-deny / typos / check / clippy / nextest
- GitHub Actions：CI（lint / test / deny / typos）与 Release（git-cliff 生成 Changelog）工作流

## 生成后

进入新项目目录，按项目内 README 指引安装开发工具并启用 pre-commit，即可开始开发：

```bash
cargo run                          # 启动服务，默认监听 0.0.0.0:3000
curl http://localhost:3000/        # Hello, ...
curl http://localhost:3000/health  # ok
```

## License

本项目采用 [MIT](./LICENSE) 许可证。
