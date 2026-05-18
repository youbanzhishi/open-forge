//! Runtime Abstraction — 统一运行时接口
//!
//! 所有游戏引擎通过统一接口接入：
//! init / update / render / event / shutdown
//!
//! 后续桥接 Godot/Unreal 不需要改上层。

pub mod config;
pub mod error;
pub mod event;
pub mod runtime;

pub use config::RuntimeConfig;
pub use error::RuntimeError;
pub use event::GameEvent;
pub use runtime::GameRuntime;
