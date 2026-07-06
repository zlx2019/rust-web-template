//! 优雅停机信号处理。
//!
//! 提供一个在收到中断信号时完成的 Future，交给 `axum::serve(..).with_graceful_shutdown(..)`。
//! 监听 Ctrl+C（跨平台）与 Unix SIGTERM（容器/编排环境常用的停止信号）。

/// 等待停机信号：Ctrl+C 或（Unix 下）SIGTERM，任一到达即返回。
///
/// 信号处理器安装失败不会 panic，仅记录错误日志，避免影响主服务。
pub async fn shutdown_signal() {
    let ctrl_c = async {
        if let Err(err) = tokio::signal::ctrl_c().await {
            tracing::error!("安装 Ctrl+C 信号处理器失败: {err}");
        }
    };

    #[cfg(unix)]
    let terminate = async {
        use tokio::signal::unix::{SignalKind, signal};
        match signal(SignalKind::terminate()) {
            Ok(mut stream) => {
                stream.recv().await;
            }
            Err(err) => tracing::error!("安装 SIGTERM 信号处理器失败: {err}"),
        }
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("收到停机信号，开始优雅停机");
}
