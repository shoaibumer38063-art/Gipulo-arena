import { useEffect, useState } from 'react';

import { CanvasHost } from './canvas/CanvasHost';
import { Gallery } from './design-system/gallery/Gallery';
import { type EngineHandle, loadEngine } from './engine/client';
import './App.css';

type EngineState =
  | { readonly kind: 'starting' }
  | { readonly kind: 'ready'; readonly engine: EngineHandle }
  | { readonly kind: 'error'; readonly message: string };

// The shell hosts the editor canvas (view only in Slice 0a: pan and zoom, no document edits yet)
// above the Design System gallery. The status line proves the shell ↔ core handshake end to end
// (engine-core.md §10).
export function App() {
  const [engine, setEngine] = useState<EngineState>({ kind: 'starting' });

  useEffect(() => {
    let alive = true;
    loadEngine().then(
      (handle) => {
        if (alive) setEngine({ kind: 'ready', engine: handle });
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
        {engine.kind === 'ready' && `Engine ready · protocol ${engine.engine.protocolVersion}`}
        {engine.kind === 'error' && `Engine error: ${engine.message}`}
      </output>
      {engine.kind === 'ready' && <CanvasHost engine={engine.engine} />}
      <Gallery />
    </>
  );
}
