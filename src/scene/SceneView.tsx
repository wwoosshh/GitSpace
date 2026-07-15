import { useMemo, useState, useEffect, useRef } from 'react';
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
import { Scrubber } from './Scrubber';

export function SceneView({ scene }: { scene: SceneModel }) {
  const [t, setT] = useState(1);
  const [playing, setPlaying] = useState(false);
  const raf = useRef<number | null>(null);

  useEffect(() => {
    if (!playing) return;
    let last = performance.now();
    const tick = (now: number) => {
      const dt = (now - last) / 1000;
      last = now;
      setT((prev) => {
        const next = prev + dt * 0.08; // 약 12.5초에 전체 재생
        return next >= 1 ? 1 : next;
      });
      raf.current = requestAnimationFrame(tick);
    };
    raf.current = requestAnimationFrame(tick);
    return () => { if (raf.current) cancelAnimationFrame(raf.current); };
  }, [playing]);

  // 끝에 도달하면 재생 정지
  useEffect(() => { if (t >= 1 && playing) setPlaying(false); }, [t, playing]);

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
        <OrbitControls makeDefault enableDamping dampingFactor={0.06} minDistance={6} maxDistance={800} autoRotate={!playing} autoRotateSpeed={0.25} />
        <Effects />
      </Canvas>
      <Scrubber
        timeline={scene.timeline}
        value={t}
        onChange={(v) => { setPlaying(false); setT(v); }}
        playing={playing}
        onTogglePlay={() => setPlaying((p) => !p)}
      />
      {scene.meta.sampled && (
        <div style={{ position: 'absolute', bottom: 12, left: 12, color: '#ffd166', fontSize: 13 }}>
          ⚠ 커밋이 많아 최신 {scene.meta.totalCommits}개만 표시됩니다.
        </div>
      )}
    </>
  );
}
