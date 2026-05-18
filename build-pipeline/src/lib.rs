//! Build Pipeline — 构建发布管线
//!
//! 将游戏场景编译为目标平台的可运行产物。
//! Phase 1: HTML5 Web 包
//! Phase 2: 微信/抖音小游戏
//! Phase 3: 桌面/移动端

pub mod builder;
pub mod error;
pub mod manifest;

pub use builder::BuildPipeline;
pub use error::BuildError;
pub use manifest::{BuildManifest, BuildTarget, BuildResult};
