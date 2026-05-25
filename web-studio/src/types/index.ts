// Forge Core API Types

export interface Project {
  id: string;
  name: string;
  description: string;
  created_at: string;
  updated_at: string;
  scene_count: number;
  asset_count: number;
  status: ProjectStatus;
}

export type ProjectStatus = 'draft' | 'building' | 'ready' | 'error';

export interface Scene {
  id: string;
  project_id: string;
  name: string;
  entities: Entity[];
  yaml_source: string;
}

export interface Entity {
  id: string;
  name: string;
  components: Record<string, unknown>;
  children?: Entity[];
  position?: { x: number; y: number; z?: number };
  rotation?: { x: number; y: number; z?: number };
  scale?: { x: number; y: number; z?: number };
}

export interface Asset {
  id: string;
  name: string;
  type: AssetType;
  url: string;
  size: number;
  created_at: string;
}

export type AssetType = 'image' | 'audio' | 'model3d' | 'script' | 'other';

export interface Build {
  id: string;
  project_id: string;
  status: BuildStatus;
  progress: number;
  platform: BuildPlatform;
  created_at: string;
  completed_at?: string;
  download_url?: string;
  error?: string;
}

export type BuildStatus = 'queued' | 'building' | 'success' | 'failed';
export type BuildPlatform = 'web' | 'windows' | 'macos' | 'linux';

export interface WSMessage {
  type: 'build_progress' | 'scene_update' | 'entity_update' | 'error';
  payload: unknown;
}
