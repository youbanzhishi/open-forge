use async_trait::async_trait;

use crate::config::RuntimeConfig;
use crate::error::RuntimeError;
use crate::event::GameEvent;

/// 统一游戏运行时接口
///
/// 所有游戏引擎（Light / Cocos / Godot / Unreal）实现此接口。
/// 上层代码（Forge Core / Script Forge）只依赖此 trait，
/// 后续桥接新引擎不需要改上层。
#[async_trait]
pub trait GameRuntime: Send + Sync {
    /// 运行时名称
    fn name(&self) -> &str;

    /// 初始化运行时
    async fn init(&mut self, config: RuntimeConfig) -> Result<(), RuntimeError>;

    /// 每帧更新（逻辑 tick）
    async fn update(&mut self, delta: f64) -> Result<(), RuntimeError>;

    /// 渲染当前帧
    async fn render(&self) -> Result<(), RuntimeError>;

    /// 处理输入事件
    async fn handle_event(&mut self, event: GameEvent) -> Result<(), RuntimeError>;

    /// 关闭运行时，释放资源
    async fn shutdown(&mut self) -> Result<(), RuntimeError>;

    /// 是否已初始化
    fn is_initialized(&self) -> bool;
}
