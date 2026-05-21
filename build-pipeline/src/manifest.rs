use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 构建目标平台
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BuildTarget {
    /// HTML5 Web 包
    Web,
    /// 微信小游戏
    WechatMini,
    /// 抖音小游戏
    DouyinMini,
    /// 桌面应用
    Desktop,
}

/// 构建清单
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildManifest {
    pub build_id: String,
    pub project_id: String,
    pub project_name: String,
    pub target: BuildTarget,
    pub scene_ids: Vec<String>,
    pub scenes: Vec<SceneData>,
    pub config: HashMap<String, serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

/// 场景数据（用于构建）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneData {
    pub id: String,
    pub name: String,
    pub entities: Vec<EntityData>,
}

/// 实体数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityData {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub visible: bool,
    pub components: HashMap<String, serde_json::Value>,
}

/// 构建结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildResult {
    pub build_id: String,
    pub success: bool,
    pub output_path: Option<String>,
    pub output_size: Option<u64>,
    pub error: Option<String>,
    pub duration_ms: u64,
}
