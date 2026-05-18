use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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
    pub target: BuildTarget,
    pub scene_ids: Vec<String>,
    pub created_at: DateTime<Utc>,
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
