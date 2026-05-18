use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("initialization failed: {0}")]
    InitFailed(String),

    #[error("update failed: {0}")]
    UpdateFailed(String),

    #[error("render failed: {0}")]
    RenderFailed(String),

    #[error("event handling failed: {0}")]
    EventFailed(String),

    #[error("shutdown failed: {0}")]
    ShutdownFailed(String),

    #[error("unsupported operation: {0}")]
    Unsupported(String),
}
