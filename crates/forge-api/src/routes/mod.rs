//! Route handlers

pub mod health;
pub mod projects;
pub mod scenes;

use axum::Router;
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(health::router())
        .merge(projects::router())
        .merge(scenes::router())
}
