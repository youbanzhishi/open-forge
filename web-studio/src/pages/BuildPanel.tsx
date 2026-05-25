import { useParams } from 'react-router-dom';
import { useBuildStore } from '../stores/buildStore';
import type { BuildPlatform } from '../types';
import styles from './BuildPanel.module.css';

const platforms: { id: BuildPlatform; label: string; icon: string }[] = [
  { id: 'web', label: 'Web', icon: '🌐' },
  { id: 'windows', label: 'Windows', icon: '🪟' },
  { id: 'macos', label: 'macOS', icon: '🍎' },
  { id: 'linux', label: 'Linux', icon: '🐧' },
];

export function BuildPanel() {
  const { id } = useParams<{ id: string }>();
  const { builds, currentBuild, isBuilding, triggerBuild, fetchBuildStatus } = useBuildStore();

  const handleBuild = async (platform: BuildPlatform) => {
    if (!id) return;
    const build = await triggerBuild(id, platform);
    // 轮询构建状态
    const poll = setInterval(async () => {
      await fetchBuildStatus(build.id);
      if (currentBuild?.status === 'success' || currentBuild?.status === 'failed') clearInterval(poll);
    }, 2000);
  };

  return (
    <div className={styles.page}>
      <h2>Build & Deploy</h2>
      <div className={styles.platforms}>
        {platforms.map(p => (
          <button key={p.id} className={styles.platformBtn}
            onClick={() => handleBuild(p.id)} disabled={isBuilding}>
            <span className={styles.platformIcon}>{p.icon}</span>
            <span>{p.label}</span>
          </button>
        ))}
      </div>
      {currentBuild && (
        <div className={styles.buildInfo}>
          <div className={styles.buildStatus}>
            Status: <span className={styles[currentBuild.status]}>{currentBuild.status}</span>
          </div>
          {currentBuild.status === 'building' && (
            <div className={styles.progress}>
              <div className={styles.progressBar} style={{ width: `${currentBuild.progress}%` }} />
            </div>
          )}
          {currentBuild.status === 'success' && currentBuild.download_url && (
            <a href={currentBuild.download_url} className={styles.download}>⬇️ Download</a>
          )}
          {currentBuild.error && <div className={styles.error}>{currentBuild.error}</div>}
        </div>
      )}
      <div className={styles.history}>
        <h3>Build History</h3>
        {builds.map(b => (
          <div key={b.id} className={styles.buildItem}>
            <span className={styles[b.status]}>{b.status}</span>
            <span>{b.platform}</span>
            <span>{new Date(b.created_at).toLocaleString()}</span>
          </div>
        ))}
      </div>
    </div>
  );
}
