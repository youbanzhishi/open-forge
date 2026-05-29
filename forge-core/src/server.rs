use std::sync::Arc;

use axum::routing::{get, post};
use axum::{extract::State, Json, Router};
use tower_http::cors::CorsLayer;
use tracing::info;

use crate::api;
use crate::auth::{self, AuthConfig, AuthSubject, TokenRequest, TokenResponse};
use crate::state::AppState;
use crate::ws;

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
        let protected = Router::new()
            // API 路由（需要认证）
            .merge(api::routes())
            // 保存/加载数据
            .route("/api/v1/save", post(save_data))
            .route("/api/v1/load", post(load_data))
            // 认证失败由 AuthSubject extractor 返回 401
            .route_layer(axum::middleware::from_fn_with_state(
                self.state.auth_config.clone(),
                auth_middleware,
            ));

        Router::new()
            // 健康检查（无需认证）
            .route("/health", get(health_check))
            // Token 换取（用 API Key 换 JWT）
            .route("/auth/token", post(exchange_token))
            // WebSocket 事件流（自带 token 认证）
            .route("/ws/v1/events", get(ws::ws_events_handler))
            // 受保护的 API 路由
            .merge(protected)
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
        info!(
            "Auth: API Key={}..., JWT enabled",
            &self.state.auth_config.api_key[..8.min(self.state.auth_config.api_key.len())]
        );

        let listener = tokio::net::TcpListener::bind(&addr).await?;
        axum::serve(listener, app).await?;
        Ok(())
    }
}

/// 认证中间件
async fn auth_middleware(
    axum::extract::State(config): axum::extract::State<Arc<AuthConfig>>,
    mut req: axum::http::Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> Result<axum::response::Response, auth::AuthError> {
    // 提取认证主体
    let subject = AuthSubject::from_request_parts(&mut req.parts, &config).await?;
    // 将认证信息注入请求扩展，供后续 handler 使用
    req.extensions_mut().insert(subject);
    Ok(next.run(req).await)
}

/// 健康检查端点（无需认证）
async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "version": "0.1.0",
        "service": "forge-core"
    }))
}

/// 用 API Key 换取 JWT Token
async fn exchange_token(
    State(state): State<Arc<AppState>>,
    Json(req): Json<TokenRequest>,
) -> Result<Json<TokenResponse>, crate::error::ForgeError> {
    // 验证 API Key
    if req.api_key != state.auth_config.api_key {
        return Err(crate::error::ForgeError::Unauthorized);
    }

    let subject = req.subject.unwrap_or_else(|| "web-studio".to_string());
    let token = auth::create_jwt(&state.auth_config, &subject)?;

    Ok(Json(TokenResponse {
        token,
        expires_in: state.auth_config.jwt_expire_hours * 3600,
        token_type: "Bearer".to_string(),
    }))
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
