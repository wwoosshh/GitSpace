import { useMemo } from 'react';
import { Line } from '@react-three/drei';
import type { Star } from '../bindings';

const BASE_RADIUS = 6;
const RADIUS_STEP = 4;
const INCLINATION_STEP = 0.18;

function orbitPoints(lane: number): [number, number, number][] {
  const radius = BASE_RADIUS + lane * RADIUS_STEP;
  const incl = lane * INCLINATION_STEP;
  const pts: [number, number, number][] = [];
  for (let i = 0; i <= 160; i++) {
    const a = (i / 160) * Math.PI * 2;
    const px = Math.cos(a) * radius;
    const pz = Math.sin(a) * radius;
    const y = -pz * Math.sin(incl);
    const z = pz * Math.cos(incl);
    pts.push([px, y, z]);
  }
  return pts;
}

export function OrbitLines({ branches }: { branches: Star[] }) {
  // 브랜치를 백엔드와 동일 순서(default 먼저, 이름순)로 정렬해 lane 인덱스 부여
  const ordered = useMemo(() => {
    return [...branches].sort(
      (a, b) => Number(b.isDefault) - Number(a.isDefault) || a.name.localeCompare(b.name),
    );
  }, [branches]);

  return (
    <>
      {ordered.map((b, lane) => (
        <Line key={b.id} points={orbitPoints(lane)} color={b.color} lineWidth={1} transparent opacity={0.35} />
      ))}
    </>
  );
}
