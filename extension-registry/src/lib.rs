//! Extension Registry — 四柱扩展注册中心
//!
//! 所有游戏能力通过注册扩展接入，架构永远不改。
//! 四柱：Action / Condition / Hook / Component

pub mod action;
pub mod component;
pub mod condition;
pub mod error;
pub mod hook;
pub mod registry;

pub use error::RegistryError;
pub use registry::ExtensionRegistry;
