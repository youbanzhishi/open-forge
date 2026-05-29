use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};

use crate::auth::AuthConfig;
use crate::bus::ForgeBus;
use crate::models::{Asset, Build, Entity, Project, Scene};

const DATA_DIR: &str = "./data";

/// 应用全局状态
///
/// Phase 1 用内存 + 文件持久化
pub struct AppState {
    pub projects: Arc<RwLock<HashMap<String, Project>>>,
    pub scenes: Arc<RwLock<HashMap<String, Scene>>>,
    pub builds: Arc<RwLock<HashMap<String, Build>>>,
    pub entities: Arc<RwLock<HashMap<String, Entity>>>,
    pub assets: Arc<RwLock<HashMap<String, Asset>>>,
    pub bus: Arc<ForgeBus>,
    pub auth_config: Arc<AuthConfig>,
    /// 事件广播通道，供 WebSocket 推送
    pub event_tx: broadcast::Sender<crate::bus::BusMessage>,
    pub data_dir: PathBuf,
}

impl AppState {
    pub fn new() -> Self {
        let data_dir = PathBuf::from(DATA_DIR);

        // 确保数据目录存在
        std::fs::create_dir_all(&data_dir).ok();

        let (event_tx, _) = broadcast::channel(256);

        Self {
            projects: Arc::new(RwLock::new(HashMap::new())),
            scenes: Arc::new(RwLock::new(HashMap::new())),
            builds: Arc::new(RwLock::new(HashMap::new())),
            entities: Arc::new(RwLock::new(HashMap::new())),
            assets: Arc::new(RwLock::new(HashMap::new())),
            bus: Arc::new(ForgeBus::new()),
            auth_config: Arc::new(AuthConfig::from_env()),
            event_tx,
            data_dir,
        }
    }

    /// 加载所有数据
    pub async fn load(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 加载项目
        let projects_file = self.data_dir.join("projects.json");
        if projects_file.exists() {
            let content = tokio::fs::read_to_string(&projects_file).await?;
            let projects: Vec<Project> = serde_json::from_str(&content)?;
            let mut map = self.projects.write().await;
            for p in projects {
                map.insert(p.id.clone(), p);
            }
            tracing::info!("Loaded {} projects", map.len());
        }

        // 加载场景
        let scenes_file = self.data_dir.join("scenes.json");
        if scenes_file.exists() {
            let content = tokio::fs::read_to_string(&scenes_file).await?;
            let scenes: Vec<Scene> = serde_json::from_str(&content)?;
            let mut map = self.scenes.write().await;
            for s in scenes {
                map.insert(s.id.clone(), s);
            }
            tracing::info!("Loaded {} scenes", map.len());
        }

        Ok(())
    }

    /// 保存所有数据
    pub async fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 保存项目
        let projects = self.projects.read().await;
        let project_list: Vec<&Project> = projects.values().collect();
        let content = serde_json::to_string_pretty(&project_list)?;
        tokio::fs::write(self.data_dir.join("projects.json"), content).await?;

        // 保存场景
        let scenes = self.scenes.read().await;
        let scene_list: Vec<&Scene> = scenes.values().collect();
        let content = serde_json::to_string_pretty(&scene_list)?;
        tokio::fs::write(self.data_dir.join("scenes.json"), content).await?;

        tracing::info!("Data saved");
        Ok(())
    }

    /// 发布事件到 ForgeBus + WebSocket 广播
    pub fn emit(&self, msg: crate::bus::BusMessage) {
        // 发布到 ForgeBus（同步订阅者）
        self.bus.publish(msg.clone());
        // 广播到 WebSocket 连接
        let _ = self.event_tx.send(msg);
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
