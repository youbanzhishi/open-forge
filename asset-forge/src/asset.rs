use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 资产类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AssetType {
    Image,
    Audio,
    Font,
    Tilemap,
    Animation,
    Script,
    Other(String),
}

/// 资产元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub asset_type: AssetType,
    pub url: String,
    pub size: u64,
    pub created_at: DateTime<Utc>,
    pub metadata: serde_json::Value,
}
