//! Asset Forge — 资产生成管线
//!
//! 管理游戏资产（图片、音频、字体等）的上传、存储和索引。

pub mod asset;
pub mod error;
pub mod store;

pub use asset::{Asset, AssetType};
pub use error::AssetError;
pub use store::AssetStore;
