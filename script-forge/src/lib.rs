//! Script Forge — YAML/JSON → 可运行游戏逻辑
//!
//! 将 YAML/JSON 描述的游戏场景编译为可执行的逻辑指令。
//! 参考 VCMix 的 YAML→渲染思路。

pub mod compiler;
pub mod error;
pub mod scene;
pub mod trigger;

pub use compiler::SceneCompiler;
pub use error::ScriptError;
pub use scene::{SceneDefinition, EntityDefinition, ComponentDefinition};
pub use trigger::{Trigger, TriggerAction};
