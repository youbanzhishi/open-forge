use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ForgeError {
    #[error("not found: {0}")]
    NotFound(String),

    #[error("validation error: {0}")]
    Validation(String),

    #[error("unauthorized")]
    Unauthorized,

    #[error("build failed: {0}")]
    BuildFailed(String),

    #[error("runtime error: {0}")]
    RuntimeError(String),

    #[error("internal error: {0}")]
    Internal(String),
}

impl IntoResponse for ForgeError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self {
            Self::NotFound(_) => (StatusCode::NOT_FOUND, "NOT_FOUND", self.to_string()),
            Self::Validation(_) => (StatusCode::BAD_REQUEST, "VALIDATION_ERROR", self.to_string()),
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED", self.to_string()),
            Self::BuildFailed(_) => (StatusCode::INTERNAL_SERVER_ERROR, "BUILD_FAILED", self.to_string()),
            Self::RuntimeError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "RUNTIME_ERROR", self.to_string()),
            Self::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", self.to_string()),
        };

        let body = json!({
            "error": {
                "code": code,
                "message": message,
            }
        });

        (status, Json(body)).into_response()
    }
}
