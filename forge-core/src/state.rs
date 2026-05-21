use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::bus::ForgeBus;
use crate::models::{Build, Project, Scene};

/// 应用全局状态
///
/// Phase 1 用内存存储，Phase 2 换持久化（SQLite/PostgreSQL）。
pub struct AppState {
    pub projects: Arc<RwLock<HashMap<String, Project>>>,
    pub scenes: Arc<RwLock<HashMap<String, Scene>>>,
    pub builds: Arc<RwLock<HashMap<String, Build>>>,
    pub bus: Arc<ForgeBus>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            projects: Arc::new(RwLock::new(HashMap::new())),
            scenes: Arc::new(RwLock::new(HashMap::new())),
            builds: Arc::new(RwLock::new(HashMap::new())),
            bus: Arc::new(ForgeBus::new()),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
