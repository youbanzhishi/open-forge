use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 项目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub settings: ProjectSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSettings {
    pub default_scene: Option<String>,
    pub resolution: Resolution,
    pub target_platforms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

/// 创建项目请求
#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub description: Option<String>,
    pub settings: Option<ProjectSettings>,
}

/// 场景
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub yaml_content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 创建场景请求
#[derive(Debug, Deserialize)]
pub struct CreateSceneRequest {
    pub name: String,
    pub yaml_content: String,
}

/// 构建
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Build {
    pub id: String,
    pub project_id: String,
    pub status: BuildStatus,
    pub progress: u8,
    pub output_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BuildStatus {
    Pending,
    Building,
    Completed,
    Failed,
}


// === 实体 ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub id: String,
    pub scene_id: String,
    pub name: String,
    pub entity_type: String,
    pub components: serde_json::Value,
    pub position: Option<Vec3>,
    pub rotation: Option<Vec3>,
    pub scale: Option<Vec3>,
    pub parent_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEntityRequest {
    pub name: String,
    pub entity_type: Option<String>,
    pub components: Option<serde_json::Value>,
    pub position: Option<Vec3>,
    pub rotation: Option<Vec3>,
    pub scale: Option<Vec3>,
    pub parent_id: Option<String>,
}

// === 资产 ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub id: String,
    pub name: String,
    pub asset_type: String,
    pub url: String,
    pub size: u64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAssetRequest {
    pub name: String,
    pub asset_type: String,
    pub url: String,
    pub size: Option<u64>,
}
