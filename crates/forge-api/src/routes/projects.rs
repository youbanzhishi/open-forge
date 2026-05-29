//! Project CRUD routes

use axum::{
    extract::{State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProjectRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

// In-memory storage (would be replaced with forge-core in production)
pub type ProjectStore = Arc<RwLock<HashMap<String, Project>>>;

use crate::routes::scenes::Scene;

#[derive(Clone)]
pub struct AppState {
    pub store: ProjectStore,
    pub scenes: Arc<RwLock<HashMap<String, Scene>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
            scenes: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/projects", get(list_projects))
        .route("/api/v1/projects", post(create_project))
        .route("/api/v1/projects/:id", get(get_project))
        .route("/api/v1/projects/:id", put(update_project))
        .route("/api/v1/projects/:id", delete(delete_project))
}

pub async fn list_projects(
    State(state): State<AppState>,
) -> Result<Json<Vec<Project>>, StatusCode> {
    let projects = state.store.read().await;
    let list: Vec<Project> = projects.values().cloned().collect();
    Ok(Json(list))
}

pub async fn create_project(
    State(state): State<AppState>,
    Json(payload): Json<CreateProjectRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let project = Project {
        id: uuid::Uuid::new_v4().to_string(),
        name: payload.name,
        description: payload.description,
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
    };

    let mut projects = state.store.write().await;
    projects.insert(project.id.clone(), project.clone());

    Ok((StatusCode::CREATED, Json(project)))
}

pub async fn get_project(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let projects = state.store.read().await;
    match projects.get(&id) {
        Some(project) => Ok(Json(project.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn update_project(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
    Json(payload): Json<UpdateProjectRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut projects = state.store.write().await;
    match projects.get_mut(&id) {
        Some(project) => {
            if let Some(name) = payload.name {
                project.name = name;
            }
            if let Some(description) = payload.description {
                project.description = description;
            }
            project.updated_at = chrono::Utc::now().to_rfc3339();
            Ok(Json(project.clone()))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn delete_project(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut projects = state.store.write().await;
    match projects.remove(&id) {
        Some(_) => Ok(StatusCode::NO_CONTENT),
        None => Err(StatusCode::NOT_FOUND),
    }
}
