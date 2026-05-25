import { useEffect, useRef, useState } from 'react';
import { assetsApi } from '../services/api';
import type { Asset } from '../types';
import styles from './AssetManager.module.css';

export function AssetManager() {
  const [assets, setAssets] = useState<Asset[]>([]);
  const [uploading, setUploading] = useState(false);
  const fileRef = useRef<HTMLInputElement>(null);

  const loadAssets = async () => {
    try {
      const list = await assetsApi.list();
      setAssets(list);
    } catch { /* empty */ }
  };

  useEffect(() => { loadAssets(); }, []);

  const handleUpload = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;
    setUploading(true);
    try {
      await assetsApi.upload(file);
      await loadAssets();
    } finally {
      setUploading(false);
    }
  };

  const handleDelete = async (id: string) => {
    if (!confirm('Delete this asset?')) return;
    await assetsApi.delete(id);
    setAssets(a => a.filter(x => x.id !== id));
  };

  const typeIcon: Record<string, string> = { image: '🖼️', audio: '🔊', model3d: '🎭', script: '📜', other: '📎' };

  return (
    <div className={styles.page}>
      <div className={styles.header}>
        <h2>Asset Manager</h2>
        <button onClick={() => fileRef.current?.click()} disabled={uploading}>
          {uploading ? 'Uploading...' : '📤 Upload'}
        </button>
        <input ref={fileRef} type="file" hidden onChange={handleUpload} />
      </div>
      <div className={styles.grid}>
        {assets.map(a => (
          <div key={a.id} className={styles.card}>
            <div className={styles.icon}>{typeIcon[a.type] ?? '📎'}</div>
            <div className={styles.info}>
              <strong>{a.name}</strong>
              <span>{(a.size / 1024).toFixed(1)} KB</span>
            </div>
            <button className={styles.deleteBtn} onClick={() => handleDelete(a.id)}>×</button>
          </div>
        ))}
        {assets.length === 0 && <div className={styles.empty}>No assets yet</div>}
      </div>
    </div>
  );
}
