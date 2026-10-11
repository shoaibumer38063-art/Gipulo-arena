import react from '@vitejs/plugin-react';
import { defineConfig } from 'vitest/config';

// Gipulo shell — dev/build/test config.
//
// Notes (see docs/architecture/engine-core.md):
// - No COOP/COEP headers yet. They become required only when the core moves to a worker with
//   shared-memory threading (engine-core.md §5); adding them early breaks cross-origin assets.
// - `allowedHosts: true` keeps proxied preview environments working (hosted sandboxes, tunnels).
// - The engine is consumed exclusively through src/engine/client.ts (the hand-written facade).
// - Tests: jsdom for the DS component suites (engine-core.md §9 layer 3 — shell-only logic;
//   engine math is tested natively in cargo).
export default defineConfig({
  plugins: [react()],
  server: { host: true, allowedHosts: true },
  preview: { host: true, allowedHosts: true },
  test: {
    environment: 'jsdom',
    include: ['src/**/*.test.{ts,tsx}'],
    setupFiles: ['src/test-setup.ts'],
  },
});
