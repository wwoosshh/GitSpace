import { useState } from 'react';
import { Instances, Instance } from '@react-three/drei';
import { Html } from '@react-three/drei';
import type { CommitNode } from '../bindings';

export function CommitNodes({ commits }: { commits: CommitNode[] }) {
  const [hover, setHover] = useState<CommitNode | null>(null);
  if (commits.length === 0) return null;

  return (
    <>
      <Instances limit={Math.max(1, commits.length)} range={commits.length}>
        <sphereGeometry args={[0.22, 12, 12]} />
        <meshStandardMaterial toneMapped={false} emissiveIntensity={1.4} />
        {commits.map((c) => (
          <Instance
            key={c.id}
            position={c.position}
            color={c.color}
            onPointerOver={(e) => { e.stopPropagation(); setHover(c); }}
            onPointerOut={() => setHover(null)}
          />
        ))}
      </Instances>
      {hover && (
        <Html position={hover.position} distanceFactor={16} center style={{ pointerEvents: 'none' }}>
          <div style={{ background: 'rgba(0,0,0,0.82)', color: '#fff', padding: '4px 8px', borderRadius: 6, fontSize: 11, whiteSpace: 'nowrap', maxWidth: 260, overflow: 'hidden', textOverflow: 'ellipsis' }}>
            {hover.id.slice(0, 7)} · {hover.message.split('\n')[0]}
          </div>
        </Html>
      )}
    </>
  );
}
