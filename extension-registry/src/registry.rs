use std::collections::HashMap;
use std::sync::Arc;

use crate::action::ActionHandler;
use crate::component::ComponentHandler;
use crate::condition::ConditionHandler;
use crate::error::RegistryError;
use crate::hook::HookHandler;

/// 四柱扩展注册中心
///
/// 新能力 = 注册扩展，架构永远不改。
pub struct ExtensionRegistry {
    actions: HashMap<String, Arc<dyn ActionHandler>>,
    conditions: HashMap<String, Arc<dyn ConditionHandler>>,
    hooks: HashMap<String, Arc<dyn HookHandler>>,
    components: HashMap<String, Arc<dyn ComponentHandler>>,
}

impl ExtensionRegistry {
    pub fn new() -> Self {
        Self {
            actions: HashMap::new(),
            conditions: HashMap::new(),
            hooks: HashMap::new(),
            components: HashMap::new(),
        }
    }

    // === Action ===

    pub fn register_action(&mut self, handler: Arc<dyn ActionHandler>) -> Result<(), RegistryError> {
        let name = handler.name().to_string();
        if self.actions.contains_key(&name) {
            return Err(RegistryError::Duplicate(name));
        }
        self.actions.insert(name, handler);
        Ok(())
    }

    pub fn get_action(&self, name: &str) -> Option<&Arc<dyn ActionHandler>> {
        self.actions.get(name)
    }

    pub fn action_names(&self) -> Vec<&str> {
        self.actions.keys().map(|s| s.as_str()).collect()
    }

    // === Condition ===

    pub fn register_condition(&mut self, handler: Arc<dyn ConditionHandler>) -> Result<(), RegistryError> {
        let name = handler.name().to_string();
        if self.conditions.contains_key(&name) {
            return Err(RegistryError::Duplicate(name));
        }
        self.conditions.insert(name, handler);
        Ok(())
    }

    pub fn get_condition(&self, name: &str) -> Option<&Arc<dyn ConditionHandler>> {
        self.conditions.get(name)
    }

    // === Hook ===

    pub fn register_hook(&mut self, handler: Arc<dyn HookHandler>) -> Result<(), RegistryError> {
        let name = handler.name().to_string();
        if self.hooks.contains_key(&name) {
            return Err(RegistryError::Duplicate(name));
        }
        self.hooks.insert(name, handler);
        Ok(())
    }

    pub fn get_hooks_by_phase(&self, phase: crate::hook::HookPhase) -> Vec<&Arc<dyn HookHandler>> {
        let mut hooks: Vec<_> = self.hooks.values().filter(|h| h.phase() == phase).collect();
        hooks.sort_by_key(|h| h.priority());
        hooks
    }

    // === Component ===

    pub fn register_component(&mut self, handler: Arc<dyn ComponentHandler>) -> Result<(), RegistryError> {
        let name = handler.name().to_string();
        if self.components.contains_key(&name) {
            return Err(RegistryError::Duplicate(name));
        }
        self.components.insert(name, handler);
        Ok(())
    }

    pub fn get_component(&self, name: &str) -> Option<&Arc<dyn ComponentHandler>> {
        self.components.get(name)
    }
}

impl Default for ExtensionRegistry {
    fn default() -> Self {
        Self::new()
    }
}
