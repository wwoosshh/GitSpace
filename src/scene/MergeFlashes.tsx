import type { MergeEvent } from '../bindings';

export function MergeFlashes({ merges }: { merges: MergeEvent[] }) {
  return (
    <>
      {merges.map((m) => (
        <mesh key={m.commitId} position={m.position}>
          <sphereGeometry args={[0.9, 24, 24]} />
          <meshStandardMaterial color="#000000" emissive="#ffe08a" emissiveIntensity={6} toneMapped={false} />
        </mesh>
      ))}
    </>
  );
}
