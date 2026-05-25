import type { Project, Scene, Entity, Asset, Build, BuildPlatform } from '../types';

const API_BASE = '/api/v1';

async function request<T>(path: string, options?: RequestInit): Promise<T> {
  const res = await fetch(`${API_BASE}${path}`, {
    headers: { 'Content-Type': 'application/json' },
    ...options,
  });
  if (!res.ok) {
    const body = await res.text().catch(() => 'Unknown error');
    throw new Error(`API ${res.status}: ${body}`);
  }
  return res.json();
}

// Projects
export const projectsApi = {
  list: () => request<Project[]>('/projects'),
  get: (id: string) => request<Project>(`/projects/${id}`),
  create: (data: { name: string; description: string }) =>
    request<Project>('/projects', { method: 'POST', body: JSON.stringify(data) }),
  update: (id: string, data: Partial<Project>) =>
    request<Project>(`/projects/${id}`, { method: 'PUT', body: JSON.stringify(data) }),
  delete: (id: string) =>
    request<void>(`/projects/${id}`, { method: 'DELETE' }),
};

// Scenes
export const scenesApi = {
  list: (projectId: string) => request<Scene[]>(`/projects/${projectId}/scenes`),
  get: (id: string) => request<Scene>(`/scenes/${id}`),
  create: (projectId: string, data: { name: string; yaml_source: string }) =>
    request<Scene>(`/projects/${projectId}/scenes`, { method: 'POST', body: JSON.stringify(data) }),
  update: (id: string, data: Partial<Scene>) =>
    request<Scene>(`/scenes/${id}`, { method: 'PUT', body: JSON.stringify(data) }),
  delete: (id: string) =>
    request<void>(`/scenes/${id}`, { method: 'DELETE' }),
};

// Entities
export const entitiesApi = {
  list: (sceneId: string) => request<Entity[]>(`/scenes/${sceneId}/entities`),
  create: (sceneId: string, data: Partial<Entity>) =>
    request<Entity>(`/scenes/${sceneId}/entities`, { method: 'POST', body: JSON.stringify(data) }),
  update: (id: string, data: Partial<Entity>) =>
    request<Entity>(`/entities/${id}`, { method: 'PUT', body: JSON.stringify(data) }),
  delete: (id: string) =>
    request<void>(`/entities/${id}`, { method: 'DELETE' }),
};

// Assets
export const assetsApi = {
  list: () => request<Asset[]>('/assets'),
  upload: (file: File) => {
    const form = new FormData();
    form.append('file', file);
    return fetch(`${API_BASE}/assets`, { method: 'POST', body: form }).then(r => {
      if (!r.ok) throw new Error(`Upload failed: ${r.status}`);
      return r.json() as Promise<Asset>;
    });
  },
  get: (id: string) => request<Asset>(`/assets/${id}`),
  delete: (id: string) =>
    request<void>(`/assets/${id}`, { method: 'DELETE' }),
};

// Builds
export const buildsApi = {
  trigger: (projectId: string, platform: BuildPlatform) =>
    request<Build>(`/projects/${projectId}/build`, {
      method: 'POST',
      body: JSON.stringify({ platform }),
    }),
  getStatus: (id: string) => request<Build>(`/builds/${id}`),
  download: (id: string) => `${API_BASE}/builds/${id}/download`,
};
