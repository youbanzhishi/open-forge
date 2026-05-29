//! Scene CRUD routes

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateSceneRequest {
    pub project_id: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSceneRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/scenes", get(list_scenes))
        .route("/api/v1/scenes", post(create_scene))
        .route("/api/v1/scenes/:id", get(get_scene))
        .route("/api/v1/scenes/:id", put(update_scene))
        .route("/api/v1/scenes/:id", delete(delete_scene))
        .route("/api/v1/projects/:project_id/scenes", get(list_scenes_by_project))
        .route("/api/v1/projects/:project_id/scenes", post(create_scene_for_project))
}

pub async fn list_scenes(
    State(state): State<AppState>,
) -> Result<Json<Vec<Scene>>, StatusCode> {
    let scenes = state.scenes.read().await;
    let list: Vec<Scene> = scenes.values().cloned().collect();
    Ok(Json(list))
}

pub async fn list_scenes_by_project(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
) -> Result<Json<Vec<Scene>>, StatusCode> {
    let scenes = state.scenes.read().await;
    let list: Vec<Scene> = scenes
        .values()
        .filter(|s| s.project_id == project_id)
        .cloned()
        .collect();
    Ok(Json(list))
}

pub async fn create_scene(
    State(state): State<AppState>,
    Json(payload): Json<CreateSceneRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let scene = Scene {
        id: uuid::Uuid::new_v4().to_string(),
        project_id: payload.project_id,
        name: payload.name,
        description: payload.description,
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
    };

    let mut scenes = state.scenes.write().await;
    scenes.insert(scene.id.clone(), scene.clone());

    Ok((StatusCode::CREATED, Json(scene)))
}

pub async fn create_scene_for_project(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
    Json(payload): Json<CreateSceneRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let scene = Scene {
        id: uuid::Uuid::new_v4().to_string(),
        project_id,
        name: payload.name,
        description: payload.description,
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
    };

    let mut scenes = state.scenes.write().await;
    scenes.insert(scene.id.clone(), scene.clone());

    Ok((StatusCode::CREATED, Json(scene)))
}

pub async fn get_scene(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let scenes = state.scenes.read().await;
    match scenes.get(&id) {
        Some(scene) => Ok(Json(scene.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn update_scene(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateSceneRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut scenes = state.scenes.write().await;
    match scenes.get_mut(&id) {
        Some(scene) => {
            if let Some(name) = payload.name {
                scene.name = name;
            }
            if let Some(description) = payload.description {
                scene.description = description;
            }
            scene.updated_at = chrono::Utc::now().to_rfc3339();
            Ok(Json(scene.clone()))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn delete_scene(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut scenes = state.scenes.write().await;
    match scenes.remove(&id) {
        Some(_) => Ok(StatusCode::NO_CONTENT),
        None => Err(StatusCode::NOT_FOUND),
    }
}
