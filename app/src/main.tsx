import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';

// Design System first: tokens + global resets load before any component code (ui.md UI1).
import './design-system/tokens.css';
import './design-system/globals.css';

import { App } from './App';

const container = document.getElementById('root');
if (!container) {
  throw new Error('Gipulo shell: #root container missing from index.html');
}

createRoot(container).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
