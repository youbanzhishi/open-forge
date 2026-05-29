//! WebSocket 实时事件流
//!
//! 端点: WS /ws/v1/events?token=<jwt>[&project_id=<id>]
//!
//! 连接后推送 ForgeBus 消息（可按 project_id 过滤）

use std::sync::Arc;

use axum::extract::{Query, State, WebSocketUpgrade, ws::{Message, WebSocket}};
use axum::response::IntoResponse;
use futures::{SinkExt, StreamExt};
use serde::Deserialize;
use tokio::sync::broadcast;
use tracing::{info, warn, error};

use crate::auth::{verify_jwt, AuthConfig};
use crate::bus::BusMessage;
use crate::state::AppState;

/// WebSocket 连接参数
#[derive(Debug, Deserialize)]
pub struct WsParams {
    /// JWT Token（必填）
    pub token: String,
    /// 项目 ID（可选，过滤只推送该项目的事件）
    pub project_id: Option<String>,
}

/// WebSocket 升级处理
pub async fn ws_events_handler(
    ws: WebSocketUpgrade,
    Query(params): Query<WsParams>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, crate::error::ForgeError> {
    // 验证 JWT
    let claims = verify_jwt(&state.auth_config, &params.token)?;
    let project_filter = params.project_id.clone();

    info!(
        "WebSocket auth ok: sub={}, project_filter={:?}",
        claims.sub, project_filter
    );

    // 升级为 WebSocket 连接
    Ok(ws.on_upgrade(move |socket| handle_ws(socket, state, project_filter)))
}

/// 处理 WebSocket 连接
async fn handle_ws(socket: WebSocket, state: Arc<AppState>, project_filter: Option<String>) {
    let (mut sender, mut receiver) = socket.split();

    // 订阅 ForgeBus 事件广播
    let mut rx = state.event_tx.subscribe();

    info!("WebSocket client connected, filter={:?}", project_filter);

    // 双向任务：接收客户端消息 + 推送服务端事件
    let mut recv_task = tokio::spawn(async move {
        while let Some(msg) = receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    // 客户端发来的文本消息，当前仅做 ping 响应
                    if text == "ping" {
                        // handled by sender task
                    }
                }
                Ok(Message::Close(_)) => break,
                Err(e) => {
                    warn!("WebSocket recv error: {}", e);
                    break;
                }
                _ => {}
            }
        }
    });

    let filter = project_filter.clone();
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // 按 project_id 过滤
            if let Some(ref pid) = filter {
                if !msg_matches_project(&msg, pid) {
                    continue;
                }
            }

            let json = match serde_json::to_string(&msg) {
                Ok(j) => j,
                Err(e) => {
                    error!("Serialize event failed: {}", e);
                    continue;
                }
            };

            if sender.send(Message::Text(json.into())).await.is_err() {
                break;
            }
        }
    });

    // 任一方向结束则关闭
    tokio::select! {
        _ = (&mut recv_task) => send_task.abort(),
        _ = (&mut send_task) => recv_task.abort(),
    }

    info!("WebSocket client disconnected, filter={:?}", project_filter);
}

/// 判断事件是否匹配项目 ID
fn msg_matches_project(msg: &BusMessage, project_id: &str) -> bool {
    match msg {
        BusMessage::ProjectCreated { project_id: pid } => pid == project_id,
        BusMessage::ProjectUpdated { project_id: pid } => pid == project_id,
        BusMessage::ProjectDeleted { project_id: pid } => pid == project_id,
        BusMessage::EntityCreated { project_id: pid, .. } => pid == project_id,
        BusMessage::SceneCreated { project_id: pid, .. } => pid == project_id,
        BusMessage::SceneUpdated { project_id: pid, .. } => pid == project_id,
        BusMessage::SceneDeleted { project_id: pid, .. } => pid == project_id,
        BusMessage::AssetCreated { project_id: pid, .. } => pid == project_id,
        BusMessage::AssetUpdated { project_id: pid, .. } => pid == project_id,
        BusMessage::AssetDeleted { project_id: pid, .. } => pid == project_id,
        BusMessage::AssetReady { project_id: pid, .. } => pid == project_id,
        BusMessage::ScriptCreated { project_id: pid, .. } => pid == project_id,
        BusMessage::ScriptUpdated { project_id: pid, .. } => pid == project_id,
        BusMessage::ScriptDeleted { project_id: pid, .. } => pid == project_id,
        BusMessage::ScriptCompiled { project_id: pid, .. } => pid == project_id,
        BusMessage::BuildStarted { project_id: pid, .. } => pid == project_id,
        BusMessage::BuildProgress { project_id: pid, .. } => pid == project_id,
        BusMessage::BuildCompleted { project_id: pid, .. } => pid == project_id,
        BusMessage::BuildFailed { project_id: pid, .. } => pid == project_id,
        BusMessage::RuntimeInit { project_id: pid, .. } => pid == project_id,
        BusMessage::RuntimeUpdate { project_id: pid, .. } => pid == project_id,
        BusMessage::RuntimeError { project_id: pid, .. } => pid == project_id,
        // 全局事件不过滤
        BusMessage::NodeAdded { .. }
        | BusMessage::NodeUpdated { .. }
        | BusMessage::NodeDeleted { .. }
        | BusMessage::ExtensionRegistered { .. } => true,
    }
}
