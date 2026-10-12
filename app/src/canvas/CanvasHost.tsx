import { useEffect, useRef, useState, type PointerEvent } from 'react';

import type { EngineHandle, EngineView } from '../engine/client';
import { paintFrame, viewReadout } from './paint';
import './CanvasHost.css';

/**
 * Wheel zoom sensitivity per wheel delta unit. A standard mouse notch (deltaY ≈ 100) gives about
 * 1.1× (CAN-CAM-3 "≈1.1× per notch"). Behavior constant, not a visual token.
 */
const WHEEL_ZOOM_SENSITIVITY = 0.00095;

/** Gesture kind for the pointer (Slice 0a: middle-button pan only). Replaced by the core's
 * interaction state machine in Slice 0b (decision D2: gip-interaction). */
type PointerGesture = { readonly kind: 'idle' } | { readonly kind: 'panning'; readonly pointerId: number };

/**
 * The editor canvas. The shell owns the DOM element and forwards raw pointer and wheel input to
 * the engine; the engine answers with a view state that this component paints. No coordinate
 * math lives here (CAN-COORD-2).
 */
export function CanvasHost({ engine }: { readonly engine: EngineHandle }) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [view, setView] = useState<EngineView>(() => engine.view());
  const gestureRef = useRef<PointerGesture>({ kind: 'idle' });

  // Keep the engine's viewport in step with the element's client rect (CAN-CAM-5).
  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const sync = () => {
      const rect = canvas.getBoundingClientRect();
      engine.setViewport(rect.left, rect.top, rect.width, rect.height);
      setView(engine.view());
    };
    sync();
    const observer = new ResizeObserver(sync);
    observer.observe(canvas);
    return () => observer.disconnect();
  }, [engine]);

  // Paint whenever the engine view changes. Colors come from the design tokens.
  useEffect(() => {
    const canvas = canvasRef.current;
    const ctx = canvas?.getContext('2d');
    if (!canvas || !ctx) return;
    const dpr = window.devicePixelRatio || 1;
    canvas.width = Math.max(1, Math.round(view.width * dpr));
    canvas.height = Math.max(1, Math.round(view.height * dpr));
    const styles = getComputedStyle(canvas);
    paintFrame(
      ctx,
      view,
      {
        background: styles.getPropertyValue('--bg-canvas').trim(),
        guide: styles.getPropertyValue('--guide-geometry').trim(),
      },
      dpr,
    );
  }, [view]);

  const refresh = () => setView(engine.view());

  // Wheel needs a NON-passive native listener so preventDefault stops page scrolling.
  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const onWheel = (event: WheelEvent) => {
      event.preventDefault();
      if (event.ctrlKey || event.metaKey) {
        // Ctrl/⌘ + wheel and trackpad pinch zoom around the pointer (CAN-CAM-2, CAN-CAM-4).
        engine.zoomAt(event.clientX, event.clientY, Math.exp(-event.deltaY * WHEEL_ZOOM_SENSITIVITY));
      } else {
        // Plain wheel and two-finger scroll pan: content moves opposite to the scroll.
        engine.panBy(-event.deltaX, -event.deltaY);
      }
      setView(engine.view());
    };
    canvas.addEventListener('wheel', onWheel, { passive: false });
    return () => canvas.removeEventListener('wheel', onWheel);
  }, [engine]);

  const onPointerDown = (event: PointerEvent<HTMLCanvasElement>) => {
    if (event.button !== 1) return; // Slice 0a: middle button pans.
    event.preventDefault();
    event.currentTarget.setPointerCapture(event.pointerId);
    gestureRef.current = { kind: 'panning', pointerId: event.pointerId };
  };

  const onPointerMove = (event: PointerEvent<HTMLCanvasElement>) => {
    const gesture = gestureRef.current;
    if (gesture.kind !== 'panning' || gesture.pointerId !== event.pointerId) return;
    engine.panBy(event.movementX, event.movementY);
    refresh();
  };

  const endGesture = (event: PointerEvent<HTMLCanvasElement>) => {
    const gesture = gestureRef.current;
    if (gesture.kind !== 'panning' || gesture.pointerId !== event.pointerId) return;
    if (event.currentTarget.hasPointerCapture(event.pointerId)) {
      event.currentTarget.releasePointerCapture(event.pointerId);
    }
    gestureRef.current = { kind: 'idle' };
  };

  return (
    <section className="canvas-host" aria-label="Editor canvas">
      <canvas
        ref={canvasRef}
        className="canvas-host__surface"
        data-testid="editor-canvas"
        onPointerDown={onPointerDown}
        onPointerMove={onPointerMove}
        onPointerUp={endGesture}
        onPointerCancel={endGesture}
        onContextMenu={(event) => event.preventDefault()}
      />
      <output className="canvas-host__readout" data-testid="view-readout">
        {viewReadout(view)}
      </output>
    </section>
  );
}
