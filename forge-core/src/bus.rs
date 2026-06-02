//! Forge Bus - 消息总线

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use parking_lot::RwLock;
use std::sync::Arc;
use tracing::{info, debug};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum BusMessage {
    ProjectCreated { project_id: String },
    ProjectUpdated { project_id: String },
    ProjectDeleted { project_id: String },
    EntityCreated {
        project_id: String,
        scene_id: String,
        entity_id: String,
    },
    SceneCreated { project_id: String, scene_id: String },
    SceneUpdated { project_id: String, scene_id: String },
    SceneDeleted { project_id: String, scene_id: String },
    NodeAdded { scene_id: String, node_id: String, parent_id: Option<String> },
    NodeUpdated { scene_id: String, node_id: String },
    NodeDeleted { scene_id: String, node_id: String },
    AssetCreated { project_id: String, asset_id: String },
    AssetUpdated { project_id: String, asset_id: String },
    AssetDeleted { project_id: String, asset_id: String },
    AssetReady { project_id: String, asset_id: String, url: String },
    ScriptCreated { project_id: String, script_id: String },
    ScriptUpdated { project_id: String, script_id: String },
    ScriptDeleted { project_id: String, script_id: String },
    ScriptCompiled { project_id: String, script_id: String, success: bool },
    BuildStarted { project_id: String, build_id: String },
    BuildProgress { project_id: String, build_id: String, percent: u8, step: String },
    BuildCompleted { project_id: String, build_id: String, download_url: String },
    BuildFailed { project_id: String, build_id: String, error: String },
    RuntimeInit { project_id: String, scene_id: String },
    RuntimeUpdate { project_id: String, frame: u64, delta_time: f64 },
    RuntimeError { project_id: String, code: String, message: String },
    ExtensionRegistered { extension_type: String, name: String },
}

pub trait BusMessageHandler: Send + Sync {
    fn handle(&self, message: &BusMessage);
}

struct Subscription {
    handler: Arc<dyn BusMessageHandler>,
}

pub struct ForgeBus {
    subscribers: RwLock<HashMap<String, Vec<Subscription>>>,
}

impl Default for ForgeBus {
    fn default() -> Self {
        Self::new()
    }
}

impl ForgeBus {
    pub fn new() -> Self {
        Self {
            subscribers: RwLock::new(HashMap::new()),
        }
    }

    pub fn subscribe(&self, message_type: impl Into<String>, handler: Arc<dyn BusMessageHandler>) {
        let message_type = message_type.into();
        debug!("Subscribing to: {}", message_type);
        let mut subscribers = self.subscribers.write();
        subscribers
            .entry(message_type)
            .or_default()
            .push(Subscription { handler });
    }

    pub fn unsubscribe(&self, message_type: impl Into<String>) {
        self.subscribers.write().remove(&message_type.into());
    }

    pub fn publish(&self, message: BusMessage) {
        let message_type = message.type_name();
        info!("Publishing: {}", message_type);

        let subscribers = self.subscribers.read();
        let handlers: Vec<Arc<dyn BusMessageHandler>> = subscribers
            .get(&message_type.to_string())
            .map(|v| v.iter().map(|s| s.handler.clone()).collect())
            .unwrap_or_default();

        let wildcard_handlers: Vec<Arc<dyn BusMessageHandler>> = subscribers
            .get("*")
            .map(|v| v.iter().map(|s| s.handler.clone()).collect())
            .unwrap_or_default();

        for handler in handlers.into_iter().chain(wildcard_handlers) {
            handler.handle(&message);
        }
    }
}

impl BusMessage {
    pub fn type_name(&self) -> &'static str {
        match self {
            BusMessage::ProjectCreated { .. } => "project_created",
            BusMessage::ProjectUpdated { .. } => "project_updated",
            BusMessage::ProjectDeleted { .. } => "project_deleted",
            BusMessage::EntityCreated { .. } => "entity_created",
            BusMessage::SceneCreated { .. } => "scene_created",
            BusMessage::SceneUpdated { .. } => "scene_updated",
            BusMessage::SceneDeleted { .. } => "scene_deleted",
            BusMessage::NodeAdded { .. } => "node_added",
            BusMessage::NodeUpdated { .. } => "node_updated",
            BusMessage::NodeDeleted { .. } => "node_deleted",
            BusMessage::AssetCreated { .. } => "asset_created",
            BusMessage::AssetUpdated { .. } => "asset_updated",
            BusMessage::AssetDeleted { .. } => "asset_deleted",
            BusMessage::AssetReady { .. } => "asset_ready",
            BusMessage::ScriptCreated { .. } => "script_created",
            BusMessage::ScriptUpdated { .. } => "script_updated",
            BusMessage::ScriptDeleted { .. } => "script_deleted",
            BusMessage::ScriptCompiled { .. } => "script_compiled",
            BusMessage::BuildStarted { .. } => "build_started",
            BusMessage::BuildProgress { .. } => "build_progress",
            BusMessage::BuildCompleted { .. } => "build_completed",
            BusMessage::BuildFailed { .. } => "build_failed",
            BusMessage::RuntimeInit { .. } => "runtime_init",
            BusMessage::RuntimeUpdate { .. } => "runtime_update",
            BusMessage::RuntimeError { .. } => "runtime_error",
            BusMessage::ExtensionRegistered { .. } => "extension_registered",
        }
    }
}
