import type { Timeline } from '../bindings';

function fmt(ts: number): string {
  if (!ts) return '-';
  return new Date(ts * 1000).toISOString().slice(0, 10);
}

export function Scrubber({
  timeline, value, onChange, playing, onTogglePlay,
}: {
  timeline: Timeline; value: number; onChange: (v: number) => void;
  playing: boolean; onTogglePlay: () => void;
}) {
  const current = timeline.start + (timeline.end - timeline.start) * value;
  return (
    <div style={{ position: 'absolute', top: 0, right: 0, height: '100%', width: 72, display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center', gap: 12, background: 'linear-gradient(to left, rgba(6,10,22,0.85), rgba(6,10,22,0))', zIndex: 10 }}>
      <span style={{ color: '#8aa0c8', fontSize: 10 }}>현재</span>
      <span style={{ color: '#dbe7ff', fontSize: 11 }}>{fmt(current)}</span>
      <input
        type="range" min={0} max={1} step={0.001} value={value}
        onChange={(e) => onChange(parseFloat(e.target.value))}
        style={{ writingMode: 'vertical-lr', direction: 'rtl', height: '60%', accentColor: '#4a90ff' }}
        aria-label="타임 스크러버"
      />
      <button onClick={onTogglePlay} style={{ background: '#0d1a33', color: '#dbe7ff', border: '1px solid #3a6df0', borderRadius: 6, padding: '4px 8px', cursor: 'pointer' }}>
        {playing ? '⏸' : '▶'}
      </button>
      <span style={{ color: '#5f7196', fontSize: 9 }}>{fmt(timeline.start)}<br />↕<br />{fmt(timeline.end)}</span>
    </div>
  );
}
