use serde::{Deserialize, Serialize};

/// 触发器：当条件满足时执行动作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trigger {
    /// 条件表达式
    pub condition: ConditionExpr,
    /// 满足时执行的动作列表
    pub action: Vec<TriggerAction>,
    /// 是否只触发一次
    #[serde(default)]
    pub once: bool,
}

/// 条件表达式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionExpr {
    /// 条件类型
    #[serde(rename = "type")]
    pub condition_type: String,
    /// 条件参数
    #[serde(flatten)]
    pub params: serde_json::Value,
}

/// 触发动作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerAction {
    /// 动作类型
    #[serde(rename = "type")]
    pub action_type: String,
    /// 动作目标（实体 ID）
    #[serde(default)]
    pub target: Option<String>,
    /// 动作参数
    #[serde(flatten)]
    pub params: serde_json::Value,
}
