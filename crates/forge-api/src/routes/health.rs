//! Health check route

use axum::{routing::get, Router};
use serde::Serialize;

use crate::AppState;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health))
}

async fn health() -> &'static str {
    r#"{"status":"ok","version":"0.1.0"}"#
}
