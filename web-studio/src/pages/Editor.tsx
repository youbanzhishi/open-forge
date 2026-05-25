import { useEffect, useRef, useState } from 'react';
import { useParams, NavLink } from 'react-router-dom';
import { useProjectStore } from '../stores/projectStore';
import YAML from 'yaml';
import styles from './Editor.module.css';

export function Editor() {
  const { id } = useParams<{ id: string }>();
  const { currentProject, selectProject, currentScene, scenes, selectScene, selectedEntityId, selectEntity, updateEntity } = useProjectStore();
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [yamlText, setYamlText] = useState('');
  const [editMode, setEditMode] = useState<'visual' | 'yaml'>('visual');

  useEffect(() => {
    if (id) selectProject(id);
  }, [id, selectProject]);

  useEffect(() => {
    if (currentScene) setYamlText(YAML.stringify(currentScene, null, 2));
  }, [currentScene]);

  // Canvas渲染
  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas || !currentScene) return;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    const render = () => {
      ctx.fillStyle = '#1a1a2e';
      ctx.fillRect(0, 0, canvas.width, canvas.height);

      // 网格
      ctx.strokeStyle = '#2a2a4a';
      ctx.lineWidth = 0.5;
      for (let x = 0; x < canvas.width; x += 32) {
        ctx.beginPath(); ctx.moveTo(x, 0); ctx.lineTo(x, canvas.height); ctx.stroke();
      }
      for (let y = 0; y < canvas.height; y += 32) {
        ctx.beginPath(); ctx.moveTo(0, y); ctx.lineTo(canvas.width, y); ctx.stroke();
      }

      // 渲染实体
      currentScene.entities.forEach(entity => {
        const x = (entity.position?.x ?? 0) * 32 + canvas.width / 2;
        const y = (entity.position?.y ?? 0) * 32 + canvas.height / 2;
        const isSelected = entity.id === selectedEntityId;

        ctx.fillStyle = isSelected ? '#e94560' : '#4a90d9';
        ctx.fillRect(x - 16, y - 16, 32, 32);
        ctx.strokeStyle = isSelected ? '#ff6b81' : '#6ab0ff';
        ctx.lineWidth = isSelected ? 2 : 1;
        ctx.strokeRect(x - 16, y - 16, 32, 32);

        ctx.fillStyle = '#e4e4e7';
        ctx.font = '10px Inter';
        ctx.textAlign = 'center';
        ctx.fillText(entity.name, x, y + 28);
      });
    };
    render();
  }, [currentScene, selectedEntityId]);

  if (!currentProject) return <div className={styles.loading}>Loading project...</div>;

  return (
    <div className={styles.editor}>
      <div className={styles.toolbar}>
        <h2>{currentProject.name}</h2>
        <div className={styles.tabs}>
          <NavLink to={`/project/${id}/assets`} className={styles.tab}>📦 Assets</NavLink>
          <NavLink to={`/project/${id}/build`} className={styles.tab}>🚀 Build</NavLink>
        </div>
        <div className={styles.modeToggle}>
          <button className={editMode === 'visual' ? styles.activeMode : ''} onClick={() => setEditMode('visual')}>Visual</button>
          <button className={editMode === 'yaml' ? styles.activeMode : ''} onClick={() => setEditMode('yaml')}>YAML</button>
        </div>
      </div>

      <div className={styles.content}>
        <div className={styles.sceneList}>
          {scenes.map(s => (
            <div key={s.id}
              className={`${styles.sceneItem} ${s.id === currentScene?.id ? styles.activeScene : ''}`}
              onClick={() => selectScene(s)}>
              {s.name}
            </div>
          ))}
        </div>

        {editMode === 'visual' ? (
          <canvas ref={canvasRef} width={800} height={600} className={styles.canvas}
            onClick={(e) => {
              // 简化的实体选择
              const canvas = canvasRef.current;
              if (!canvas || !currentScene) return;
              const rect = canvas.getBoundingClientRect();
              const cx = e.clientX - rect.left;
              const cy = e.clientY - rect.top;
              for (const entity of currentScene.entities) {
                const ex = (entity.position?.x ?? 0) * 32 + canvas.width / 2;
                const ey = (entity.position?.y ?? 0) * 32 + canvas.height / 2;
                if (cx >= ex - 16 && cx <= ex + 16 && cy >= ey - 16 && cy <= ey + 16) {
                  selectEntity(entity.id);
                  return;
                }
              }
              selectEntity(null);
            }} />
        ) : (
          <textarea className={styles.yamlEditor} value={yamlText}
            onChange={e => setYamlText(e.target.value)} />
        )}

        <div className={styles.inspector}>
          <h3>Inspector</h3>
          {selectedEntityId ? (
            <EntityInspector entityId={selectedEntityId} onUpdate={updateEntity} />
          ) : (
            <p className={styles.hint}>Select an entity to inspect</p>
          )}
        </div>
      </div>
    </div>
  );
}

function EntityInspector({ entityId, onUpdate }: { entityId: string; onUpdate: (id: string, u: Partial<import('../types').Entity>) => Promise<void> }) {
  const { currentScene } = useProjectStore();
  const entity = currentScene?.entities.find(e => e.id === entityId);
  if (!entity) return null;

  return (
    <div className={styles.inspectorContent}>
      <div className={styles.field}>
        <label>Name</label>
        <input value={entity.name} onChange={e => onUpdate(entityId, { name: e.target.value })} />
      </div>
      <div className={styles.field}>
        <label>X</label>
        <input type="number" value={entity.position?.x ?? 0}
          onChange={e => onUpdate(entityId, { position: { ...entity.position!, x: +e.target.value } })} />
      </div>
      <div className={styles.field}>
        <label>Y</label>
        <input type="number" value={entity.position?.y ?? 0}
          onChange={e => onUpdate(entityId, { position: { ...entity.position!, y: +e.target.value } })} />
      </div>
    </div>
  );
}
