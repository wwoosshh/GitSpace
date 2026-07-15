import type { SceneModel, CommitNode, Star, MergeEvent } from '../bindings';

export interface VisibleScene {
  currentTime: number;
  commits: CommitNode[];
  branches: Star[];
  merges: MergeEvent[];
}

/** t01: 0..1 스크럽 값 → 실제 시각으로 환산해 그 시점까지의 요소만 반환 */
export function selectVisible(scene: SceneModel, t01: number): VisibleScene {
  const { start, end } = scene.timeline;
  const clamped = Math.max(0, Math.min(1, t01));
  const currentTime = start + (end - start) * clamped;
  return {
    currentTime,
    commits: scene.commits.filter((c) => c.timestamp <= currentTime),
    branches: scene.branches.filter((b) => b.bornAt <= currentTime),
    merges: scene.merges.filter((m) => m.timestamp <= currentTime),
  };
}
