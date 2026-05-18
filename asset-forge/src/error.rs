use thiserror::Error;

#[derive(Debug, Error)]
pub enum AssetError {
    #[error("asset not found: {0}")]
    NotFound(String),

    #[error("invalid asset type: {0}")]
    InvalidType(String),

    #[error("storage error: {0}")]
    Storage(String),

    #[error("asset too large: {size} bytes (max {max})")]
    TooLarge { size: u64, max: u64 },
}
