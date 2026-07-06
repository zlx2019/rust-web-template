# {{project-name}}

[![CI](https://github.com/{{gh_username}}/{{project-name}}/actions/workflows/ci.yml/badge.svg)](https://github.com/{{gh_username}}/{{project-name}}/actions/workflows/ci.yml)
[![License: {{license}}](https://img.shields.io/badge/license-{{ license | replace: "-", "--" }}-blue.svg)](./LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.96.0%2B-orange.svg)](https://www.rust-lang.org)

> {{description}}

一个基于 [axum](https://docs.rs/axum) 的 Web 服务，内置服务启动、路由与优雅停机。

## Features

- axum + tokio Web 服务脚手架：路由按域拆分（`src/routes/`）、handler 独立（`src/handlers/`）
- 内置 404 兜底与健康检查（`GET /health`、`GET /ready`）
- 统一错误类型 `AppError`（`src/error.rs`）：实现 `IntoResponse`，handler 可返回 `Result`，`?` 直接冒泡 `anyhow::Error`
- 优雅停机：收到 Ctrl+C / SIGTERM 时停止接收新连接并等待在途请求完成（`src/shutdown.rs`）
- 基于 `tracing` 的结构化日志（`RUST_LOG` 可调级别，缺省 `info`）
- 环境变量配置：监听地址 `BIND_ADDR` 可覆盖，支持 `.env`（`dotenvy` 自动加载）
- 可选 tower-http 中间件（生成时选择，在 `src/routes/mod.rs` 注册）

## 快速开始

### 1. 安装开发工具

项目通过 `rust-toolchain.toml` 锁定 Rust 版本，进入目录后 rustup 会自动安装对应工具链。另需安装以下工具（与 CI 检查保持一致）：

```bash
cargo install --locked cargo-deny     # 依赖安全 / 许可证审计
cargo install cargo-nextest --locked  # 测试运行器
cargo install typos-cli               # 拼写检查
cargo install git-cliff               # Changelog 生成
pip install pre-commit                # Git 提交前检查
```

### 2. 启用 pre-commit 钩子

```bash
pre-commit install
```

启用后每次 `git commit` 会自动运行格式化、Lint、测试等检查，全部通过才会提交成功。

### 3. 构建与运行

```bash
cargo run                          # 启动服务，默认监听 0.0.0.0:3000
curl http://localhost:3000/        # Hello, ...
curl http://localhost:3000/health  # ok
```

按 `Ctrl+C`（或向进程发送 `SIGTERM`）触发优雅停机。

## 开发

常用命令：

```bash
cargo nextest run    # 运行测试
cargo clippy         # 静态检查
cargo fmt            # 格式化
```

提交规范与完整开发流程见 [CONTRIBUTING.md](./CONTRIBUTING.md)。

## 项目结构

```text
src/
├── main.rs        服务入口：加载 .env → 初始化日志 → 构建路由 → 启动服务（优雅停机）
├── error.rs       统一错误类型 AppError（实现 IntoResponse）
├── routes/        路由：按业务域拆分，router() 汇总并注册中间件
│   ├── mod.rs     router()：merge 各域路由 + 404 兜底 + 中间件层
│   └── health.rs  健康检查路由（GET /health、GET /ready）
├── handlers/      请求处理器（与路由分离）
│   ├── mod.rs     根路由 index() 等通用 handler
│   └── health.rs  健康检查 handler
└── shutdown.rs    优雅停机信号（Ctrl+C / SIGTERM）
```

新增接口：在 `src/handlers/` 实现 handler，在 `src/routes/` 对应域注册路由，并在 `routes::router()` 中 `.merge()`。handler 需要返回错误时用 `Result<T, AppError>`。

## 配置

通过环境变量配置，本地开发可复制 `.env.example` 为 `.env`（启动时由 `dotenvy` 自动加载）：

| 变量 | 说明 | 默认 |
|------|------|------|
| `BIND_ADDR` | 服务监听地址 `host:port` | `0.0.0.0:3000` |
| `RUST_LOG` | 日志级别（`tracing_subscriber` EnvFilter 语法） | `info` |

## 中间件

生成时可选启用 [tower-http](https://docs.rs/tower-http) 中间件，注册在 `src/routes/mod.rs` 的 `router()`：

| 选项 | 层 | 说明 |
|------|-----|------|
| `trace` | `TraceLayer` | 请求/响应日志（`RUST_LOG=tower_http=debug` 可见） |
| `cors` | `CorsLayer::permissive` | 跨域（示例为宽松策略，生产需按需收紧） |
| `timeout` | `TimeoutLayer` | 请求超时（默认 30s，超时返回 408） |
| `compression` | `CompressionLayer` | 响应压缩（gzip/deflate） |

## License

本项目采用 [{{license}}](./LICENSE) 许可证。
