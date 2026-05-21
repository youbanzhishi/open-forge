use std::collections::HashMap;

use async_trait::async_trait;
use runtime_abstraction::{
    config::RuntimeConfig, error::RuntimeError, event::GameEvent, runtime::GameRuntime,
};
use tracing::info;

use crate::renderer::Renderer;

/// Light Runtime 实现
///
/// 自研 2D Canvas 运行时，60fps，100+ 精灵。
pub struct LightRuntime {
    config: Option<RuntimeConfig>,
    renderer: Option<Renderer>,
    entities: HashMap<String, EntityInstance>,
    frame: u64,
    running: bool,
}

/// 运行时实体实例
#[derive(Clone)]
pub struct EntityInstance {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub rotation: f64,
    pub scale_x: f64,
    pub scale_y: f64,
    pub visible: bool,
    pub layer: i32,
    pub components: HashMap<String, serde_json::Value>,
}

impl EntityInstance {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            x: 0.0,
            y: 0.0,
            rotation: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            visible: true,
            layer: 0,
            components: HashMap::new(),
        }
    }
    
    pub fn with_position(mut self, x: f64, y: f64) -> Self {
        self.x = x;
        self.y = y;
        self
    }
    
    pub fn with_scale(mut self, sx: f64, sy: f64) -> Self {
        self.scale_x = sx;
        self.scale_y = sy;
        self
    }
    
    pub fn with_layer(mut self, layer: i32) -> Self {
        self.layer = layer;
        self
    }
}

impl LightRuntime {
    pub fn new() -> Self {
        Self {
            config: None,
            renderer: None,
            entities: HashMap::new(),
            frame: 0,
            running: false,
        }
    }
    
    /// 添加实体
    pub fn add_entity(&mut self, entity: EntityInstance) {
        self.entities.insert(entity.id.clone(), entity);
    }
    
    /// 移除实体
    pub fn remove_entity(&mut self, id: &str) -> Option<EntityInstance> {
        self.entities.remove(id)
    }
    
    /// 获取实体
    pub fn get_entity(&self, id: &str) -> Option<&EntityInstance> {
        self.entities.get(id)
    }
    
    /// 获取所有实体（按层级排序）
    pub fn get_entities(&self) -> Vec<&EntityInstance> {
        let mut entities: Vec<&EntityInstance> = self.entities.values().collect();
        entities.sort_by_key(|e| e.layer);
        entities
    }
    
    /// 更新实体位置
    pub fn move_entity(&mut self, id: &str, x: f64, y: f64) -> Option<()> {
        if let Some(entity) = self.entities.get_mut(id) {
            entity.x = x;
            entity.y = y;
            Some(())
        } else {
            None
        }
    }
}

impl Default for LightRuntime {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl GameRuntime for LightRuntime {
    fn name(&self) -> &str {
        "light"
    }

    async fn init(&mut self, config: RuntimeConfig) -> Result<(), RuntimeError> {
        info!("Light Runtime init: {}x{}", config.width, config.height);
        self.renderer = Some(Renderer::new(config.width, config.height));
        self.config = Some(config);
        self.running = true;
        Ok(())
    }

    async fn update(&mut self, delta: f64) -> Result<(), RuntimeError> {
        self.frame += 1;
        
        // 更新所有实体
        for entity in self.entities.values_mut() {
            // 处理移动组件
            if let Some(movement) = entity.components.get("movement") {
                if let (Some(vx), Some(vy)) = (
                    movement.get("vx").and_then(|v| v.as_f64()),
                    movement.get("vy").and_then(|v| v.as_f64()),
                ) {
                    entity.x += vx * delta;
                    entity.y += vy * delta;
                }
            }
        }
        
        let _ = delta;
        Ok(())
    }

    async fn render(&self) -> Result<(), RuntimeError> {
        let renderer = self.renderer.as_ref().ok_or_else(|| {
            RuntimeError::RenderFailed("runtime not initialized".into())
        })?;

        let bg = self.config.as_ref().unwrap().background_color;
        renderer.begin_frame(bg);

        // 按层级渲染所有实体
        for entity in self.get_entities() {
            if !entity.visible {
                continue;
            }
            
            // 渲染精灵
            if let Some(sprite) = entity.components.get("sprite") {
                if let Some(image) = sprite.get("image").and_then(|v| v.as_str()) {
                    renderer.draw_sprite(
                        image,
                        entity.x,
                        entity.y,
                        entity.rotation,
                        entity.scale_x,
                        entity.scale_y,
                    );
                }
            }
            
            // 渲染矩形（调试用）
            if let Some(rect) = entity.components.get("rect") {
                let w = rect.get("width").and_then(|v| v.as_f64()).unwrap_or(32.0);
                let h = rect.get("height").and_then(|v| v.as_f64()).unwrap_or(32.0);
                let color = rect.get("color").and_then(|v| v.as_str()).unwrap_or("#ff0000");
                renderer.draw_rect(entity.x, entity.y, w, h, color);
            }
            
            // 渲染文字
            if let Some(text) = entity.components.get("text") {
                let content = text.get("content").and_then(|v| v.as_str()).unwrap_or("");
                let size = text.get("size").and_then(|v| v.as_f64()).unwrap_or(16.0);
                let color = text.get("color").and_then(|v| v.as_str()).unwrap_or("#ffffff");
                renderer.draw_text(content, entity.x, entity.y, size as f32, color);
            }
        }

        renderer.end_frame();
        Ok(())
    }

    async fn handle_event(&mut self, event: GameEvent) -> Result<(), RuntimeError> {
        match event {
            GameEvent::KeyDown { key } => {
                info!("Key down: {}", key);
                
                // 收集需要执行的实体动作
                let mut actions_to_run: Vec<(String, String)> = Vec::new();
                
                // 触发按键相关实体
                for entity in self.entities.values() {
                    if let Some(on_key) = entity.components.get("on_key") {
                        if let Some(trigger_keys) = on_key.get("keys").and_then(|v| v.as_array()) {
                            for k in trigger_keys {
                                if k.as_str() == Some(&key) {
                                    // 执行动作
                                    if let Some(action) = on_key.get("action").and_then(|v| v.as_str()) {
                                        actions_to_run.push((entity.id.clone(), action.to_string()));
                                    }
                                }
                            }
                        }
                    }
                }
                
                // 执行动作（避免借用冲突）
                for (entity_id, action) in actions_to_run {
                    self.execute_action(&entity_id, &action);
                }
            }
            GameEvent::MouseMove { x, y } => {
                // 收集需要执行的实体动作
                let mut actions_to_run: Vec<(String, String)> = Vec::new();
                
                // 触发鼠标悬停相关实体
                for entity in self.entities.values() {
                    if let Some(on_hover) = entity.components.get("on_hover") {
                        // 简单碰撞检测
                        let w = on_hover.get("width").and_then(|v| v.as_f64()).unwrap_or(32.0);
                        let h = on_hover.get("height").and_then(|v| v.as_f64()).unwrap_or(32.0);
                        if entity.x <= x && x <= entity.x + w && entity.y <= y && y <= entity.y + h {
                            if let Some(action) = on_hover.get("action").and_then(|v| v.as_str()) {
                                actions_to_run.push((entity.id.clone(), action.to_string()));
                            }
                        }
                    }
                }
                
                // 执行动作
                for (entity_id, action) in actions_to_run {
                    self.execute_action(&entity_id, &action);
                }
            }
            _ => {}
        }
        
        Ok(())
    }
    
    async fn shutdown(&mut self) -> Result<(), RuntimeError> {
        info!("Light Runtime shutdown at frame {}", self.frame);
        self.running = false;
        self.entities.clear();
        self.renderer = None;
        Ok(())
    }

    fn is_initialized(&self) -> bool {
        self.config.is_some()
    }
}

impl LightRuntime {
    fn execute_action(&mut self, entity_id: &str, action: &str) {
        match action {
            "hide" => {
                if let Some(entity) = self.entities.get_mut(entity_id) {
                    entity.visible = false;
                }
            }
            "show" => {
                if let Some(entity) = self.entities.get_mut(entity_id) {
                    entity.visible = true;
                }
            }
            "destroy" => {
                self.entities.remove(entity_id);
            }
            _ => {
                tracing::warn!("Unknown action: {}", action);
            }
        }
    }
}
