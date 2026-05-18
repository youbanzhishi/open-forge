use async_trait::async_trait;
use serde_json::Value;

use crate::action::GameContext;
use crate::error::RegistryError;

/// 条件判断器
///
/// 用于触发器系统：判断某个条件是否满足。
/// 例如：按键按下、分数超过阈值、时间到达。
#[async_trait]
pub trait ConditionHandler: Send + Sync {
    /// 条件名称（唯一标识）
    fn name(&self) -> &str;

    /// 评估条件
    async fn evaluate(&self, ctx: &GameContext, params: &Value) -> Result<bool, RegistryError>;
}
