import type { SceneModel } from '../bindings';

export function SceneView({ scene }: { scene: SceneModel }) {
  return (
    <div style={{ position: 'absolute', top: 12, left: 12, color: '#9fc3ff', fontFamily: 'monospace' }}>
      <div>repo: {scene.repo.name}</div>
      <div>branches: {scene.branches.length} · commits: {scene.commits.length} · merges: {scene.merges.length}</div>
      {scene.meta.sampled && <div style={{ color: '#ffd166' }}>⚠ 커밋이 많아 일부만 표시됩니다 (최신 {scene.meta.totalCommits}개)</div>}
    </div>
  );
}
