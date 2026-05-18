use std::collections::HashMap;

use crate::asset::Asset;
use crate::error::AssetError;

/// 资产存储
///
/// Phase 1: 内存存储 + 本地文件系统
/// Phase 2: 对象存储（S3/MinIO）
pub struct AssetStore {
    assets: HashMap<String, Asset>,
    base_path: String,
}

impl AssetStore {
    pub fn new(base_path: &str) -> Self {
        Self {
            assets: HashMap::new(),
            base_path: base_path.to_string(),
        }
    }

    /// 注册资产元数据
    pub fn register(&mut self, asset: Asset) -> Result<(), AssetError> {
        self.assets.insert(asset.id.clone(), asset);
        Ok(())
    }

    /// 获取资产
    pub fn get(&self, id: &str) -> Result<&Asset, AssetError> {
        self.assets.get(id).ok_or_else(|| AssetError::NotFound(id.to_string()))
    }

    /// 列出项目资产
    pub fn list_by_project(&self, project_id: &str) -> Vec<&Asset> {
        self.assets.values().filter(|a| a.project_id == project_id).collect()
    }

    /// 删除资产
    pub fn remove(&mut self, id: &str) -> Result<Asset, AssetError> {
        self.assets.remove(id).ok_or_else(|| AssetError::NotFound(id.to_string()))
    }
}
