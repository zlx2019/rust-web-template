//! Web 服务入口。
//!
//! 负责初始化日志、构建路由、绑定监听地址并启动 HTTP 服务；
//! 服务以优雅停机方式运行，收到 Ctrl+C 或 SIGTERM 时停止接收新连接并等待在途请求完成。

mod router;
mod shutdown;

use std::net::SocketAddr;

use anyhow::Context;
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

/// 服务默认监听地址。
const LISTEN_ADDR: &str = "0.0.0.0:3000";

/// 程序主入口：初始化日志后启动带优雅停机的 Web 服务。
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();

    let app = router::router();
    let listener = TcpListener::bind(LISTEN_ADDR)
        .await
        .with_context(|| format!("绑定监听地址失败: {LISTEN_ADDR}"))?;
    let local_addr: SocketAddr = listener.local_addr().context("获取本地监听地址失败")?;
    tracing::info!("服务已启动，监听于 http://{local_addr}");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown::shutdown_signal())
        .await
        .context("Web 服务运行异常")?;

    tracing::info!("服务已优雅停机");
    Ok(())
}

/// 初始化 tracing 日志：优先读取 `RUST_LOG` 环境变量，缺省为 `info` 级别。
fn init_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt().with_env_filter(filter).init();
}
