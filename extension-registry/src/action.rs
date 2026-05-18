use async_trait::async_trait;
use serde_json::Value;

use crate::error::RegistryError;

/// 游戏逻辑动作执行器
///
/// 所有游戏行为（移动、碰撞、播放声音、切换场景）都通过 Action 实现。
/// 新能力 = 注册新 Action，架构永远不改。
#[async_trait]
pub trait ActionHandler: Send + Sync {
    /// 动作名称（唯一标识）
    fn name(&self) -> &str;

    /// 执行动作
    async fn execute(&self, ctx: &mut GameContext, params: &Value) -> Result<ActionResult, RegistryError>;
}

/// 动作执行结果
#[derive(Debug, Clone)]
pub enum ActionResult {
    /// 继续执行下一个动作
    Continue,
    /// 停止后续动作
    Stop,
    /// 跳转到指定场景
    SwitchScene(String),
}

/// 游戏运行上下文
pub struct GameContext {
    /// 当前帧号
    pub frame: u64,
    /// 帧间隔（秒）
    pub delta: f64,
    /// 实体状态存储
    pub entities: std::collections::HashMap<String, EntityState>,
    /// 全局变量
    pub globals: std::collections::HashMap<String, Value>,
}

/// 实体状态
#[derive(Debug, Clone)]
pub struct EntityState {
    pub id: String,
    pub entity_type: String,
    pub position: [f64; 2],
    pub rotation: f64,
    pub scale: [f64; 2],
    pub components: std::collections::HashMap<String, Value>,
}
