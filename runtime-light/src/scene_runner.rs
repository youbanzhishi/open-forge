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
struct EntityInstance {
    id: String,
    x: f64,
    y: f64,
    rotation: f64,
    scale_x: f64,
    scale_y: f64,
    components: HashMap<String, serde_json::Value>,
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
        // TODO: 更新所有实体逻辑
        // TODO: 运行 HookPhase::OnUpdate 钩子
        let _ = delta;
        Ok(())
    }

    async fn render(&self) -> Result<(), RuntimeError> {
        let renderer = self.renderer.as_ref().ok_or_else(|| {
            RuntimeError::RenderFailed("runtime not initialized".into())
        })?;

        let bg = self.config.as_ref().unwrap().background_color;
        renderer.begin_frame(bg);

        // TODO: 按层级渲染所有实体
        // TODO: 运行 HookPhase::OnRender 钩子

        renderer.end_frame();
        Ok(())
    }

    async fn handle_event(&mut self, event: GameEvent) -> Result<(), RuntimeError> {
        // TODO: 分发事件到实体组件
        // TODO: 运行 HookPhase::OnEvent 钩子
        tracing::debug!("event: {:?}", event);
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
