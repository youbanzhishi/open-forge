use serde::{Deserialize, Serialize};

/// YAML 场景定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneDefinition {
    /// 场景名称
    pub scene: String,
    /// 实体列表
    #[serde(default)]
    pub entities: Vec<EntityDefinition>,
    /// 触发器列表
    #[serde(default)]
    pub triggers: Vec<crate::trigger::Trigger>,
}

/// 实体定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityDefinition {
    /// 实体 ID（唯一标识）
    pub id: String,
    /// 实体类型（sprite / text / tilemap / ...）
    #[serde(rename = "type")]
    pub entity_type: String,
    /// 初始位置 [x, y]
    #[serde(default)]
    pub position: [f64; 2],
    /// 旋转角度（度）
    #[serde(default)]
    pub rotation: f64,
    /// 缩放 [x, y]
    #[serde(default = "default_scale")]
    pub scale: [f64; 2],
    /// 组件列表
    #[serde(default)]
    pub components: Vec<ComponentDefinition>,
}

/// 组件定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentDefinition {
    /// 组件类型
    #[serde(rename = "type")]
    pub component_type: String,
    /// 组件参数（自由结构）
    #[serde(flatten)]
    pub params: serde_json::Value,
}

fn default_scale() -> [f64; 2] {
    [1.0, 1.0]
}
