use std::sync::Arc;

use axum::routing::post;
use axum::{extract::State, Json, Router};
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
        let state = Arc::new(AppState::new());
        Self { state, port }
    }

    /// 构建路由
    pub fn router(&self) -> Router {
        api::routes()
            .route("/health", axum::routing::get(health_check))
            .route("/api/v1/save", post(save_data))
            .route("/api/v1/load", post(load_data))
            .layer(CorsLayer::permissive())
            .with_state(self.state.clone())
    }

    /// 启动服务器
    pub async fn run(&self) -> anyhow::Result<()> {
        // 加载数据
        if let Err(e) = self.state.load().await {
            tracing::warn!("Failed to load data: {}", e);
        }
        
        let app = self.router();
        let addr = format!("0.0.0.0:{}", self.port);
        info!("Forge Core listening on {}", addr);

        let listener = tokio::net::TcpListener::bind(&addr).await?;
        axum::serve(listener, app).await?;
        Ok(())
    }
}

/// 保存数据
async fn save_data(
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    match state.save().await {
        Ok(_) => Json(serde_json::json!({ "status": "ok", "message": "Data saved" })),
        Err(e) => Json(serde_json::json!({ "status": "error", "message": e.to_string() })),
    }
}

/// 加载数据
async fn load_data(
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    match state.load().await {
        Ok(_) => Json(serde_json::json!({ "status": "ok", "message": "Data loaded" })),
        Err(e) => Json(serde_json::json!({ "status": "error", "message": e.to_string() })),
    }
}


/// 健康检查端点（无需认证）
async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "version": "0.1.0",
        "service": "forge-core"
    }))
}
