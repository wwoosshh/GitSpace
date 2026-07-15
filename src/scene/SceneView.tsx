import { useMemo, useState } from 'react';
import { Canvas } from '@react-three/fiber';
import { OrbitControls } from '@react-three/drei';
import type { SceneModel } from '../bindings';
import { selectVisible } from '../lib/timeline';
import { Starfield } from './Starfield';
import { OrbitLines } from './OrbitLines';
import { BranchStars } from './BranchStars';
import { CommitNodes } from './CommitNodes';
import { MergeFlashes } from './MergeFlashes';
import { Effects } from './Effects';

export function SceneView({ scene }: { scene: SceneModel }) {
  const [t] = useState(1); // 다음 태스크에서 스크러버로 연결
  const visible = useMemo(() => selectVisible(scene, t), [scene, t]);

  return (
    <>
      <Canvas camera={{ position: [0, 12, 34], fov: 50, near: 0.1, far: 3000 }} dpr={[1, 2]} gl={{ antialias: true }}>
        <color attach="background" args={['#04050c']} />
        <ambientLight intensity={0.25} />
        <pointLight position={[0, 0, 0]} intensity={2} />
        <Starfield />
        <OrbitLines branches={scene.branches} />
        <BranchStars branches={visible.branches} />
        <CommitNodes commits={visible.commits} />
        <MergeFlashes merges={visible.merges} />
        <OrbitControls makeDefault enableDamping dampingFactor={0.06} minDistance={6} maxDistance={800} autoRotate autoRotateSpeed={0.25} />
        <Effects />
      </Canvas>
      {scene.meta.sampled && (
        <div style={{ position: 'absolute', bottom: 12, left: 12, color: '#ffd166', fontSize: 13 }}>
          ⚠ 커밋이 많아 최신 {scene.meta.totalCommits}개만 표시됩니다.
        </div>
      )}
    </>
  );
}
