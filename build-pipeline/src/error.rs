use thiserror::Error;

#[derive(Debug, Error)]
pub enum BuildError {
    #[error("compilation failed: {0}")]
    CompilationFailed(String),

    #[error("packaging failed: {0}")]
    PackagingFailed(String),

    #[error("unsupported target: {0}")]
    UnsupportedTarget(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
