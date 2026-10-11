import { useEffect, useState } from 'react';

import { Gallery } from './design-system/gallery/Gallery';
import { loadEngine } from './engine/client';
import './App.css';

type EngineState =
  | { readonly kind: 'starting' }
  | { readonly kind: 'ready'; readonly protocolVersion: number }
  | { readonly kind: 'error'; readonly message: string };

// The shell currently hosts the Design System gallery — the foundation preview surface.
// Real editor UI arrives only after the foundation is accepted (no editor features yet).
// The status line proves the shell ↔ core handshake end to end (engine-core.md §10).
export function App() {
  const [engine, setEngine] = useState<EngineState>({ kind: 'starting' });

  useEffect(() => {
    let alive = true;
    loadEngine().then(
      ({ protocolVersion }) => {
        if (alive) setEngine({ kind: 'ready', protocolVersion });
      },
      (error: unknown) => {
        if (alive) setEngine({ kind: 'error', message: error instanceof Error ? error.message : String(error) });
      },
    );
    return () => {
      alive = false;
    };
  }, []);

  return (
    <>
      <output className="engine-status" data-engine-status={engine.kind} aria-live="polite">
        {engine.kind === 'starting' && 'Engine: starting…'}
        {engine.kind === 'ready' && `Engine ready · protocol ${engine.protocolVersion}`}
        {engine.kind === 'error' && `Engine error: ${engine.message}`}
      </output>
      <Gallery />
    </>
  );
}
