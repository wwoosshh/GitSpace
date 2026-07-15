import { useState, useEffect } from 'react';
import { Html } from '@react-three/drei';
import type { Star } from '../bindings';

function BranchStar({ star }: { star: Star }) {
  const [hovered, setHovered] = useState(false);
  useEffect(() => () => { document.body.style.cursor = 'auto'; }, []);
  const r = star.isDefault ? 1.4 : 1.0;
  return (
    <mesh
      position={star.position}
      onPointerOver={(e) => { e.stopPropagation(); setHovered(true); document.body.style.cursor = 'pointer'; }}
      onPointerOut={() => { setHovered(false); document.body.style.cursor = 'auto'; }}
    >
      <sphereGeometry args={[r, 32, 32]} />
      <meshStandardMaterial
        color="#000000"
        emissive={star.color}
        emissiveIntensity={hovered ? 5 : 3}
        toneMapped={false}
      />
      {hovered && (
        <Html distanceFactor={16} position={[0, r + 0.6, 0]} center occlude style={{ pointerEvents: 'none' }}>
          <div style={{ background: 'rgba(0,0,0,0.82)', color: '#fff', padding: '4px 8px', borderRadius: 6, fontSize: 12, whiteSpace: 'nowrap' }}>
            🌟 {star.name}{star.isDefault ? ' (기본)' : ''}
          </div>
        </Html>
      )}
    </mesh>
  );
}

export function BranchStars({ branches }: { branches: Star[] }) {
  return <>{branches.map((b) => <BranchStar key={b.id} star={b} />)}</>;
}
