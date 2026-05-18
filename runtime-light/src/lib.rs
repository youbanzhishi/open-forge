//! Light Runtime — 自研 2D Canvas/WebGL 运行时
//!
//! 目标：60fps，100+ 精灵
//! Phase 1 实现：基础 2D 渲染、碰撞检测、音频播放

pub mod canvas;
pub mod error;
pub mod renderer;
pub mod scene_runner;

pub use error::LightError;
pub use scene_runner::LightRuntime;
