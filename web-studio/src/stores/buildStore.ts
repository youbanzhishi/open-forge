import { create } from 'zustand';
import type { Build, BuildPlatform } from '../types';

interface BuildState {
  builds: Build[];
  currentBuild: Build | null;
  isBuilding: boolean;

  triggerBuild: (projectId: string, platform: BuildPlatform) => Promise<Build>;
  fetchBuildStatus: (buildId: string) => Promise<void>;
  connectWS: (projectId: string) => () => void;
}

export const useBuildStore = create<BuildState>((set, get) => ({
  builds: [],
  currentBuild: null,
  isBuilding: false,

  triggerBuild: async (projectId, platform) => {
    set({ isBuilding: true });
    try {
      const res = await fetch(`/api/v1/projects/${projectId}/build`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ platform }),
      });
      if (!res.ok) throw new Error(`Build failed: ${res.status}`);
      const build: Build = await res.json();
      set(s => ({ builds: [...s.builds, build], currentBuild: build }));
      return build;
    } catch (e) {
      set({ isBuilding: false });
      throw e;
    }
  },

  fetchBuildStatus: async (buildId) => {
    try {
      const res = await fetch(`/api/v1/builds/${buildId}`);
      if (!res.ok) throw new Error(`Failed to fetch build: ${res.status}`);
      const build: Build = await res.json();
      set(s => ({
        builds: s.builds.map(b => b.id === buildId ? build : b),
        currentBuild: s.currentBuild?.id === buildId ? build : s.currentBuild,
        isBuilding: build.status === 'building',
      }));
    } catch (e) {
      console.error('Failed to fetch build status:', e);
    }
  },

  connectWS: (projectId) => {
    const ws = new WebSocket(`ws://${window.location.host}/ws?project=${projectId}`);
    ws.onmessage = (event) => {
      try {
        const msg = JSON.parse(event.data);
        if (msg.type === 'build_progress') {
          get().fetchBuildStatus(msg.payload.build_id);
        }
      } catch {
        console.warn('Invalid WS message');
      }
    };
    return () => ws.close();
  },
}));
