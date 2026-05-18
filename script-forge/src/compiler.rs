use crate::error::ScriptError;
use crate::scene::SceneDefinition;

/// 场景编译器
///
/// 将 YAML 场景定义编译为运行时可执行的指令序列。
pub struct SceneCompiler;

impl SceneCompiler {
    pub fn new() -> Self {
        Self
    }

    /// 从 YAML 字符串编译场景定义
    pub fn compile_yaml(&self, yaml: &str) -> Result<SceneDefinition, ScriptError> {
        let scene: SceneDefinition = serde_yaml::from_str(yaml)?;
        self.validate(&scene)?;
        Ok(scene)
    }

    /// 从 JSON 字符串编译场景定义
    pub fn compile_json(&self, json: &str) -> Result<SceneDefinition, ScriptError> {
        let scene: SceneDefinition = serde_json::from_str(json)?;
        self.validate(&scene)?;
        Ok(scene)
    }

    /// 验证场景定义
    fn validate(&self, scene: &SceneDefinition) -> Result<(), ScriptError> {
        if scene.scene.is_empty() {
            return Err(ScriptError::Validation("scene name cannot be empty".into()));
        }

        // 检查实体 ID 唯一性
        let mut ids = std::collections::HashSet::new();
        for entity in &scene.entities {
            if !ids.insert(&entity.id) {
                return Err(ScriptError::Validation(format!(
                    "duplicate entity id: {}",
                    entity.id
                )));
            }
        }

        Ok(())
    }
}

impl Default for SceneCompiler {
    fn default() -> Self {
        Self::new()
    }
}
