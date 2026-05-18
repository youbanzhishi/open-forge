use async_trait::async_trait;

use crate::action::GameContext;
use crate::error::RegistryError;

/// 生命周期阶段
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HookPhase {
    OnInit,
    OnUpdate,
    OnRender,
    OnEvent,
    OnShutdown,
}

/// 生命周期钩子
///
/// 在游戏运行的各个阶段注入自定义逻辑。
/// 优先级越小越先执行。
#[async_trait]
pub trait HookHandler: Send + Sync {
    /// 钩子名称（唯一标识）
    fn name(&self) -> &str;

    /// 所属阶段
    fn phase(&self) -> HookPhase;

    /// 执行优先级（越小越先）
    fn priority(&self) -> i32;

    /// 执行钩子
    async fn run(&self, ctx: &mut GameContext) -> Result<(), RegistryError>;
}
