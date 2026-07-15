import { useMemo } from 'react';
import { Line } from '@react-three/drei';
import type { CommitNode } from '../bindings';

// 각 커밋에서 부모 커밋으로 선을 그어 DAG(분기·머지)를 드러낸다.
export function Edges({ commits }: { commits: CommitNode[] }) {
  const pos = useMemo(() => {
    const m = new Map<string, [number, number, number]>();
    for (const c of commits) m.set(c.id, c.position);
    return m;
  }, [commits]);

  const segments = useMemo(() => {
    const segs: { key: string; points: [number, number, number][]; color: string }[] = [];
    for (const c of commits) {
      for (const p of c.parents) {
        const parentPos = pos.get(p);
        if (!parentPos) continue; // 부모가 (샘플링 등으로) 없으면 건너뜀
        segs.push({ key: `${c.id}-${p}`, points: [c.position, parentPos], color: c.color });
      }
    }
    return segs;
  }, [commits, pos]);

  return (
    <>
      {segments.map((s) => (
        <Line key={s.key} points={s.points} color={s.color} lineWidth={1.2} transparent opacity={0.5} />
      ))}
    </>
  );
}
