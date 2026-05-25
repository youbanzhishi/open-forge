import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom';
import { Layout } from './components/Layout';
import { ProjectList } from './pages/ProjectList';
import { Editor } from './pages/Editor';
import { AssetManager } from './pages/AssetManager';
import { BuildPanel } from './pages/BuildPanel';

export function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Layout />}>
          <Route index element={<ProjectList />} />
          <Route path="project/:id" element={<Editor />} />
          <Route path="project/:id/assets" element={<AssetManager />} />
          <Route path="project/:id/build" element={<BuildPanel />} />
          <Route path="*" element={<Navigate to="/" replace />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}
