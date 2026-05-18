use std::sync::Arc;

use axum::Router;
use tower_http::cors::CorsLayer;
use tracing::info;

use crate::api;
use crate::state::AppState;

/// Forge Core 服务器
pub struct ForgeServer {
    state: Arc<AppState>,
    port: u16,
}

impl ForgeServer {
    pub fn new(port: u16) -> Self {
        Self {
            state: Arc::new(AppState::new()),
            port,
        }
    }

    /// 构建路由
    pub fn router(&self) -> Router {
        api::routes()
            .layer(CorsLayer::permissive())
            .with_state(self.state.clone())
    }

    /// 启动服务器
    pub async fn run(&self) -> anyhow::Result<()> {
        let app = self.router();
        let addr = format!("0.0.0.0:{}", self.port);
        info!("Forge Core listening on {}", addr);

        let listener = tokio::net::TcpListener::bind(&addr).await?;
        axum::serve(listener, app).await?;
        Ok(())
    }
}
