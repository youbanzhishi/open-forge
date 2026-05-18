use thiserror::Error;

#[derive(Debug, Error)]
pub enum RegistryError {
    #[error("extension '{0}' not found")]
    NotFound(String),

    #[error("extension '{0}' already registered")]
    Duplicate(String),

    #[error("invalid extension config: {0}")]
    InvalidConfig(String),
}
