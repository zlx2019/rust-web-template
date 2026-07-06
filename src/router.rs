//! 路由与请求处理。
//!
//! 集中定义 HTTP 路由表与各 handler。新增接口时在 [`router`] 中注册路由，
//! 并在下方实现对应的 handler 函数。

use axum::{Router, routing::get};

/// 构建应用的路由表。
pub fn router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health", get(health))
}

/// 根路由：返回欢迎信息。
async fn root() -> &'static str {
    "Hello, rust-web-template!"
}

/// 健康检查：供负载均衡 / 探针使用，服务正常时返回 `ok`。
async fn health() -> &'static str {
    "ok"
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 健康检查 handler 应返回 `ok`。
    #[tokio::test]
    async fn health_returns_ok() {
        assert_eq!(health().await, "ok");
    }
}
