import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useProjectStore } from '../stores/projectStore';
import styles from './ProjectList.module.css';

export function ProjectList() {
  const { projects, fetchProjects, createProject, deleteProject, isLoading, error } = useProjectStore();
  const navigate = useNavigate();
  const [showCreate, setShowCreate] = useState(false);
  const [name, setName] = useState('');
  const [desc, setDesc] = useState('');

  useEffect(() => { fetchProjects(); }, [fetchProjects]);

  const handleCreate = async () => {
    if (!name.trim()) return;
    const p = await createProject(name.trim(), desc.trim());
    setShowCreate(false);
    setName('');
    setDesc('');
    navigate(`/project/${p.id}`);
  };

  const handleDelete = async (e: React.MouseEvent, id: string) => {
    e.stopPropagation();
    if (confirm('Delete this project?')) await deleteProject(id);
  };

  if (isLoading && projects.length === 0) return <div className={styles.loading}>Loading...</div>;

  return (
    <div className={styles.page}>
      <div className={styles.header}>
        <h1>Projects</h1>
        <button className={styles.createBtn} onClick={() => setShowCreate(true)}>
          + New Project
        </button>
      </div>

      {error && <div className={styles.error}>{error}</div>}

      {showCreate && (
        <div className={styles.createForm}>
          <input placeholder="Project name" value={name} onChange={e => setName(e.target.value)} />
          <textarea placeholder="Description (optional)" value={desc} onChange={e => setDesc(e.target.value)} />
          <div className={styles.formActions}>
            <button onClick={handleCreate} disabled={!name.trim()}>Create</button>
            <button onClick={() => setShowCreate(false)}>Cancel</button>
          </div>
        </div>
      )}

      <div className={styles.grid}>
        {projects.map(p => (
          <div key={p.id} className={styles.card} onClick={() => navigate(`/project/${p.id}`)}>
            <div className={styles.cardHeader}>
              <h3>{p.name}</h3>
              <span className={`${styles.status} ${styles[p.status]}`}>{p.status}</span>
            </div>
            <p className={styles.cardDesc}>{p.description || 'No description'}</p>
            <div className={styles.cardMeta}>
              <span>🎬 {p.scene_count} scenes</span>
              <span>📦 {p.asset_count} assets</span>
              <span>🕐 {new Date(p.updated_at).toLocaleDateString()}</span>
            </div>
            <button className={styles.deleteBtn} onClick={e => handleDelete(e, p.id)}>×</button>
          </div>
        ))}
        {projects.length === 0 && !isLoading && (
          <div className={styles.empty}>No projects yet. Create one to get started!</div>
        )}
      </div>
    </div>
  );
}
