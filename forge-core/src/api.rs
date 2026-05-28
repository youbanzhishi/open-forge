//! Forge Core API 路由

use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use chrono::Utc;
use uuid::Uuid;

use crate::bus::BusMessage;
use crate::error::ForgeError;
use crate::models::*;
use crate::state::AppState;

/// 构建 API 路由
pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        // 项目管理
        .route("/api/v1/projects", post(create_project))
        .route("/api/v1/projects", get(list_projects))
        .route("/api/v1/projects/:id", get(get_project))
        .route("/api/v1/projects/:id", put(update_project))
        .route("/api/v1/projects/:id", delete(delete_project))
        // 场景管理
        .route("/api/v1/projects/:id/scenes", post(create_scene))
        .route("/api/v1/projects/:id/scenes", get(list_scenes))
        .route("/api/v1/scenes/:id", get(get_scene))
        .route("/api/v1/scenes/:id", put(update_scene))
        .route("/api/v1/scenes/:id", delete(delete_scene))
        // 构建
        .route("/api/v1/projects/:id/build", post(start_build))
        .route("/api/v1/builds/:id", get(get_build))
        // 实体管理
        .route("/api/v1/scenes/:id/entities", post(create_entity))
        .route("/api/v1/scenes/:id/entities", get(list_entities))
        .route("/api/v1/entities/:id", put(update_entity))
        .route("/api/v1/entities/:id", delete(delete_entity))
        // 资产管理
        .route("/api/v1/assets", post(upload_asset))
        .route("/api/v1/assets", get(list_assets))
        .route("/api/v1/assets/:id", get(get_asset))
        .route("/api/v1/assets/:id", delete(delete_asset))
}

// === 项目 CRUD ===

async fn create_project(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateProjectRequest>,
) -> Result<impl IntoResponse, ForgeError> {
    if req.name.is_empty() || req.name.len() > 100 {
        return Err(ForgeError::Validation("name must be 1-100 chars".into()));
    }

    let id = format!("proj_{}", Uuid::new_v4());
    let now = Utc::now();
    let project = Project {
        id: id.clone(),
        name: req.name,
        description: req.description.unwrap_or_default(),
        created_at: now,
        updated_at: now,
        settings: req.settings.unwrap_or_else(|| ProjectSettings {
            default_scene: None,
            resolution: Resolution {
                width: 1280,
                height: 720,
            },
            target_platforms: vec!["web".to_string()],
        }),
    };

    state.projects.write().await.insert(id.clone(), project.clone());
    
    // 发布消息
    state.bus.publish(BusMessage::ProjectCreated {
        project_id: id.clone(),
    });
    
    Ok((StatusCode::CREATED, Json(project)))
}

async fn list_projects(State(state): State<Arc<AppState>>) -> Json<Vec<Project>> {
    let projects = state.projects.read().await;
    let list: Vec<Project> = projects.values().cloned().collect();
    Json(list)
}

async fn get_project(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Project>, ForgeError> {
    let projects = state.projects.read().await;
    projects
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or_else(|| ForgeError::NotFound(format!("project {}", id)))
}

async fn update_project(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(req): Json<CreateProjectRequest>,
) -> Result<Json<Project>, ForgeError> {
    let mut projects = state.projects.write().await;
    let project = projects
        .get_mut(&id)
        .ok_or_else(|| ForgeError::NotFound(format!("project {}", id)))?;

    project.name = req.name;
    project.description = req.description.unwrap_or_default();
    project.updated_at = Utc::now();
    if let Some(settings) = req.settings {
        project.settings = settings;
    }

    Ok(Json(project.clone()))
}

async fn delete_project(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<StatusCode, ForgeError> {
    let mut projects = state.projects.write().await;
    projects
        .remove(&id)
        .map(|_| StatusCode::NO_CONTENT)
        .ok_or_else(|| ForgeError::NotFound(format!("project {}", id)))
}

// === 场景 CRUD ===

async fn create_scene(
    State(state): State<Arc<AppState>>,
    Path(project_id): Path<String>,
    Json(req): Json<CreateSceneRequest>,
) -> Result<impl IntoResponse, ForgeError> {
    // 验证项目存在
    {
        let projects = state.projects.read().await;
        if !projects.contains_key(&project_id) {
            return Err(ForgeError::NotFound(format!("project {}", project_id)));
        }
    }

    // 验证 YAML 合法
    script_forge::SceneCompiler::new()
        .compile_yaml(&req.yaml_content)
        .map_err(|e| ForgeError::Validation(format!("invalid YAML: {}", e)))?;

    let id = format!("scene_{}", Uuid::new_v4());
    let now = Utc::now();
    let scene = Scene {
        id: id.clone(),
        project_id,
        name: req.name,
        yaml_content: req.yaml_content,
        created_at: now,
        updated_at: now,
    };

    state.scenes.write().await.insert(id.clone(), scene.clone());
    
    // 发布消息
    state.bus.publish(BusMessage::SceneCreated {
        project_id: scene.project_id.clone(),
        scene_id: id.clone(),
    });
    
    Ok((StatusCode::CREATED, Json(scene)))
}

async fn list_scenes(
    State(state): State<Arc<AppState>>,
    Path(project_id): Path<String>,
) -> Json<Vec<Scene>> {
    let scenes = state.scenes.read().await;
    let list: Vec<Scene> = scenes
        .values()
        .filter(|s| s.project_id == project_id)
        .cloned()
        .collect();
    Json(list)
}

async fn get_scene(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Scene>, ForgeError> {
    let scenes = state.scenes.read().await;
    scenes
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or_else(|| ForgeError::NotFound(format!("scene {}", id)))
}

async fn update_scene(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(req): Json<CreateSceneRequest>,
) -> Result<Json<Scene>, ForgeError> {
    // 验证 YAML
    script_forge::SceneCompiler::new()
        .compile_yaml(&req.yaml_content)
        .map_err(|e| ForgeError::Validation(format!("invalid YAML: {}", e)))?;

    let mut scenes = state.scenes.write().await;
    let scene = scenes
        .get_mut(&id)
        .ok_or_else(|| ForgeError::NotFound(format!("scene {}", id)))?;

    scene.name = req.name;
    scene.yaml_content = req.yaml_content;
    scene.updated_at = Utc::now();

    Ok(Json(scene.clone()))
}

async fn delete_scene(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<StatusCode, ForgeError> {
    let mut scenes = state.scenes.write().await;
    scenes
        .remove(&id)
        .map(|_| StatusCode::NO_CONTENT)
        .ok_or_else(|| ForgeError::NotFound(format!("scene {}", id)))
}

// === 构建 ===

async fn start_build(
    State(state): State<Arc<AppState>>,
    Path(project_id): Path<String>,
) -> Result<impl IntoResponse, ForgeError> {
    // 验证项目存在
    {
        let projects = state.projects.read().await;
        if !projects.contains_key(&project_id) {
            return Err(ForgeError::NotFound(format!("project {}", project_id)));
        }
    }

    let id = format!("build_{}", Uuid::new_v4());
    let build = Build {
        id: id.clone(),
        project_id,
        status: BuildStatus::Pending,
        progress: 0,
        output_url: None,
        created_at: Utc::now(),
        completed_at: None,
        error: None,
    };

    state.builds.write().await.insert(id, build.clone());

    // TODO: 异步启动构建任务
    // tokio::spawn(run_build(state.clone(), build.id.clone()));

    Ok((StatusCode::ACCEPTED, Json(build)))
}

async fn get_build(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Build>, ForgeError> {
    let builds = state.builds.read().await;
    builds
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or_else(|| ForgeError::NotFound(format!("build {}", id)))
}


// === 实体管理 ===

async fn create_entity(
    State(state): State<Arc<AppState>>,
    Path(scene_id): Path<String>,
    Json(req): Json<CreateEntityRequest>,
) -> Result<impl IntoResponse, ForgeError> {
    let scenes = state.scenes.read().await;
    if !scenes.contains_key(&scene_id) {
        return Err(ForgeError::NotFound(format!("scene {}", scene_id)));
    }
    drop(scenes);

    let id = format!("entity_{}", Uuid::new_v4());
    let entity = Entity {
        id: id.clone(),
        scene_id,
        name: req.name,
        entity_type: req.entity_type.unwrap_or_else(|| "default".to_string()),
        components: req.components.unwrap_or_default(),
        position: req.position,
        rotation: req.rotation,
        scale: req.scale,
        parent_id: req.parent_id,
    };

    state.entities.write().await.insert(id.clone(), entity.clone());
    
    state.bus.publish(BusMessage::EntityCreated {
        scene_id: entity.scene_id.clone(),
        entity_id: id,
    });

    Ok((StatusCode::CREATED, Json(entity)))
}

async fn list_entities(
    State(state): State<Arc<AppState>>,
    Path(scene_id): Path<String>,
) -> Json<Vec<Entity>> {
    let entities = state.entities.read().await;
    let list: Vec<Entity> = entities
        .values()
        .filter(|e| e.scene_id == scene_id)
        .cloned()
        .collect();
    Json(list)
}

async fn update_entity(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(req): Json<CreateEntityRequest>,
) -> Result<Json<Entity>, ForgeError> {
    let mut entities = state.entities.write().await;
    let entity = entities
        .get_mut(&id)
        .ok_or_else(|| ForgeError::NotFound(format!("entity {}", id)))?;

    entity.name = req.name;
    if let Some(t) = req.entity_type { entity.entity_type = t; }
    if let Some(c) = req.components { entity.components = c; }
    entity.position = req.position;
    entity.rotation = req.rotation;
    entity.scale = req.scale;
    entity.parent_id = req.parent_id;

    Ok(Json(entity.clone()))
}

async fn delete_entity(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<StatusCode, ForgeError> {
    let mut entities = state.entities.write().await;
    entities
        .remove(&id)
        .map(|_| StatusCode::NO_CONTENT)
        .ok_or_else(|| ForgeError::NotFound(format!("entity {}", id)))
}

// === 资产管理 ===

async fn upload_asset(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateAssetRequest>,
) -> Result<impl IntoResponse, ForgeError> {
    let id = format!("asset_{}", Uuid::new_v4());
    let asset = Asset {
        id: id.clone(),
        name: req.name,
        asset_type: req.asset_type,
        url: req.url,
        size: req.size.unwrap_or(0),
        created_at: Utc::now(),
    };

    state.assets.write().await.insert(id.clone(), asset.clone());
    Ok((StatusCode::CREATED, Json(asset)))
}

async fn list_assets(State(state): State<Arc<AppState>>) -> Json<Vec<Asset>> {
    let assets = state.assets.read().await;
    Json(assets.values().cloned().collect())
}

async fn get_asset(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Asset>, ForgeError> {
    let assets = state.assets.read().await;
    assets.get(&id).cloned().map(Json)
        .ok_or_else(|| ForgeError::NotFound(format!("asset {}", id)))
}

async fn delete_asset(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<StatusCode, ForgeError> {
    let mut assets = state.assets.write().await;
    assets.remove(&id).map(|_| StatusCode::NO_CONTENT)
        .ok_or_else(|| ForgeError::NotFound(format!("asset {}", id)))
}
