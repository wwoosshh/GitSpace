import { useState, useCallback } from 'react';
import { open } from '@tauri-apps/plugin-dialog';
import { loadRepo } from './api';
import type { SceneModel } from './bindings';
import { SceneView } from './scene/SceneView';

export default function App() {
  const [scene, setScene] = useState<SceneModel | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const pickAndLoad = useCallback(async () => {
    setError(null);
    const selected = await open({ directory: true, multiple: false });
    if (typeof selected !== 'string') return; // 취소
    setLoading(true);
    try {
      const model = await loadRepo(selected);
      setScene(model);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }, []);

  return (
    <div style={{ position: 'fixed', inset: 0, background: '#04050c', color: '#e6eefc', overflow: 'hidden' }}>
      <div style={{ position: 'absolute', top: 16, left: '50%', transform: 'translateX(-50%)', zIndex: 10 }}>
        <button
          onClick={pickAndLoad}
          disabled={loading}
          style={{
            padding: '10px 20px', borderRadius: 8, border: '1px solid #3a6df0',
            background: loading ? '#1a2740' : '#0d1a33', color: '#dbe7ff',
            cursor: loading ? 'default' : 'pointer', fontSize: 15,
          }}
        >
          {loading ? '불러오는 중…' : '저장소 불러오기'}
        </button>
        {error && <p style={{ color: '#ff7a90', marginTop: 8 }}>에러: {error}</p>}
      </div>

      {!scene && !loading && (
        <div style={{ position: 'absolute', inset: 0, display: 'grid', placeItems: 'center', color: '#5f7196' }}>
          <p>git 저장소 폴더를 불러오면 우주가 펼쳐집니다.</p>
        </div>
      )}

      {scene && <SceneView scene={scene} />}
    </div>
  );
}
