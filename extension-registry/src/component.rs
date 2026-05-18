use async_trait::async_trait;
use serde_json::Value;

use crate::action::GameContext;
use crate::error::RegistryError;

/// 运行时类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeType {
    Light,
    Cocos,
    Godot,
    Unreal,
}

/// 渲染组件处理器
///
/// 每个组件定义一种渲染能力（精灵、音频、物理、UI、粒子）。
/// 组件声明支持哪些运行时，运行时只加载兼容的组件。
#[async_trait]
pub trait ComponentHandler: Send + Sync {
    /// 组件名称（唯一标识）
    fn name(&self) -> &str;

    /// 支持的运行时
    fn supported_runtimes(&self) -> Vec<RuntimeType>;

    /// 初始化组件
    async fn init(&self, ctx: &mut GameContext, config: &Value) -> Result<(), RegistryError>;

    /// 每帧更新
    async fn update(&self, ctx: &mut GameContext, delta: f64) -> Result<(), RegistryError>;

    /// 清理资源
    async fn cleanup(&self, ctx: &mut GameContext) -> Result<(), RegistryError>;
}
