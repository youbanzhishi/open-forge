import { Outlet, NavLink } from 'react-router-dom';
import styles from './Layout.module.css';

export function Layout() {
  return (
    <div className={styles.layout}>
      <nav className={styles.sidebar}>
        <div className={styles.logo}>
          <h2>🔨 OpenForge</h2>
          <span className={styles.subtitle}>Web Studio</span>
        </div>
        <div className={styles.nav}>
          <NavLink to="/" className={({ isActive }) => isActive ? styles.activeLink : styles.link}>
            📁 Projects
          </NavLink>
        </div>
      </nav>
      <main className={styles.main}>
        <Outlet />
      </main>
    </div>
  );
}
