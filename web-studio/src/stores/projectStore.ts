import { create } from 'zustand';
import type { Project, Scene, Entity } from '../types';

interface ProjectState {
  projects: Project[];
  currentProject: Project | null;
  scenes: Scene[];
  currentScene: Scene | null;
  selectedEntityId: string | null;
  isLoading: boolean;
  error: string | null;

  fetchProjects: () => Promise<void>;
  createProject: (name: string, description: string) => Promise<Project>;
  deleteProject: (id: string) => Promise<void>;
  selectProject: (id: string) => Promise<void>;
  fetchScenes: (projectId: string) => Promise<void>;
  selectScene: (scene: Scene) => void;
  selectEntity: (entityId: string | null) => void;
  updateEntity: (entityId: string, updates: Partial<Entity>) => Promise<void>;
  clearError: () => void;
}

export const useProjectStore = create<ProjectState>((set, get) => ({
  projects: [],
  currentProject: null,
  scenes: [],
  currentScene: null,
  selectedEntityId: null,
  isLoading: false,
  error: null,

  fetchProjects: async () => {
    set({ isLoading: true, error: null });
    try {
      const res = await fetch('/api/v1/projects');
      if (!res.ok) throw new Error(`Failed to fetch projects: ${res.status}`);
      const projects: Project[] = await res.json();
      set({ projects, isLoading: false });
    } catch (e) {
      set({ error: (e as Error).message, isLoading: false });
    }
  },

  createProject: async (name, description) => {
    set({ isLoading: true });
    try {
      const res = await fetch('/api/v1/projects', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ name, description }),
      });
      if (!res.ok) throw new Error(`Failed to create project: ${res.status}`);
      const project: Project = await res.json();
      set(s => ({ projects: [...s.projects, project], isLoading: false }));
      return project;
    } catch (e) {
      set({ error: (e as Error).message, isLoading: false });
      throw e;
    }
  },

  deleteProject: async (id) => {
    try {
      const res = await fetch(`/api/v1/projects/${id}`, { method: 'DELETE' });
      if (!res.ok) throw new Error(`Failed to delete project: ${res.status}`);
      set(s => ({ projects: s.projects.filter(p => p.id !== id) }));
    } catch (e) {
      set({ error: (e as Error).message });
    }
  },

  selectProject: async (id) => {
    const project = get().projects.find(p => p.id === id);
    if (project) {
      set({ currentProject: project });
      await get().fetchScenes(id);
    }
  },

  fetchScenes: async (projectId) => {
    try {
      const res = await fetch(`/api/v1/projects/${projectId}/scenes`);
      if (!res.ok) throw new Error(`Failed to fetch scenes: ${res.status}`);
      const scenes: Scene[] = await res.json();
      set({ scenes, currentScene: scenes[0] ?? null });
    } catch (e) {
      set({ error: (e as Error).message });
    }
  },

  selectScene: (scene) => set({ currentScene: scene }),

  selectEntity: (entityId) => set({ selectedEntityId: entityId }),

  updateEntity: async (entityId, updates) => {
    try {
      const res = await fetch(`/api/v1/entities/${entityId}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(updates),
      });
      if (!res.ok) throw new Error(`Failed to update entity: ${res.status}`);
      const updated: Entity = await res.json();
      set(s => ({
        scenes: s.scenes.map(sc =>
          sc.id === s.currentScene?.id
            ? { ...sc, entities: sc.entities.map(e => e.id === entityId ? updated : e) }
            : sc
        ),
        currentScene: s.currentScene?.id === s.currentScene?.id
          ? { ...s.currentScene, entities: s.currentScene.entities.map(e => e.id === entityId ? updated : e) }
          : s.currentScene,
      }));
    } catch (e) {
      set({ error: (e as Error).message });
    }
  },

  clearError: () => set({ error: null }),
}));
