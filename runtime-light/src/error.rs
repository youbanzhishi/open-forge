use thiserror::Error;

#[derive(Debug, Error)]
pub enum LightError {
    #[error("render error: {0}")]
    Render(String),

    #[error("scene error: {0}")]
    Scene(String),

    #[error("asset not loaded: {0}")]
    AssetNotLoaded(String),
}
