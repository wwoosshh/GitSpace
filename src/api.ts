import { invoke } from '@tauri-apps/api/core';
import type { SceneModel } from './bindings';

export async function loadRepo(path: string): Promise<SceneModel> {
  return invoke<SceneModel>('load_repo', { path });
}
