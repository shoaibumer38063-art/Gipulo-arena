import { useCallback, useEffect, useRef, useState, type PointerEvent } from 'react';

import type { EngineHandle, EngineView, ToolName } from '../engine/client';
import { matchShortcut, type ShortcutAction } from './shortcuts';
import { objectReadout, paintFrame, paintRects, viewReadout } from './paint';
import './CanvasHost.css';

/**
 * Wheel zoom sensitivity per wheel delta unit. A standard mouse notch (deltaY ≈ 100) gives about
 * 1.1× (CAN-CAM-3 "≈1.1× per notch"). Behavior constant, not a visual token.
 */
const WHEEL_ZOOM_SENSITIVITY = 0.00095;

/** Everything the shell paints, read from the core in one pass. */
interface Snapshot {
  readonly view: EngineView;
  readonly rects: Float64Array;
  readonly stride: number;
  readonly objects: number;
  readonly selected: number;
  readonly tool: ToolName;
  readonly canUndo: boolean;
  readonly canRedo: boolean;
}

function readSnapshot(engine: EngineHandle): Snapshot {
  return {
    view: engine.view(),
    rects: engine.rects(),
    stride: engine.rectStride,
    objects: engine.objectCount(),
    selected: engine.selectionCount(),
    tool: engine.tool(),
    canUndo: engine.canUndo(),
    canRedo: engine.canRedo(),
  };
}

const TOOLS: readonly { readonly name: ToolName; readonly label: string; readonly key: string }[] = [
  { name: 'select', label: 'Select', key: 'V' },
  { name: 'rectangle', label: 'Rectangle', key: 'R' },
];

/**
 * The editor canvas. The shell owns the DOM element and forwards raw pointer, wheel, and key input
 * to the engine; the engine answers with a snapshot that this component paints. No coordinate
 * math, hit testing, or gesture logic lives here (CAN-COORD-2, Boundary Duplication Law).
 */
export function CanvasHost({ engine }: { readonly engine: EngineHandle }) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [snap, setSnap] = useState<Snapshot>(() => readSnapshot(engine));

  const refresh = useCallback(() => setSnap(readSnapshot(engine)), [engine]);

  // Keep the engine's viewport in step with the element's client rect (CAN-CAM-5).
  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const sync = () => {
      const rect = canvas.getBoundingClientRect();
      engine.setViewport(rect.left, rect.top, rect.width, rect.height);
      refresh();
    };
    sync();
    const observer = new ResizeObserver(sync);
    observer.observe(canvas);
    return () => observer.disconnect();
  }, [engine, refresh]);

  // Paint whenever the snapshot changes. Colors come from the design tokens.
  useEffect(() => {
    const canvas = canvasRef.current;
    const ctx = canvas?.getContext('2d');
    if (!canvas || !ctx) return;
    const dpr = window.devicePixelRatio || 1;
    canvas.width = Math.max(1, Math.round(snap.view.width * dpr));
    canvas.height = Math.max(1, Math.round(snap.view.height * dpr));
    const styles = getComputedStyle(canvas);
    const accent = styles.getPropertyValue('--accent').trim();
    paintFrame(
      ctx,
      snap.view,
      {
        background: styles.getPropertyValue('--bg-canvas').trim(),
        guide: styles.getPropertyValue('--guide-geometry').trim(),
      },
      dpr,
    );
    paintRects(ctx, snap.rects, snap.stride, {
      fill: styles.getPropertyValue('--frame-bg').trim(),
      accent,
      hoverAlpha: Number.parseFloat(styles.getPropertyValue('--opacity-muted')) || 0.6,
    });
  }, [snap]);

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
      refresh();
    };
    canvas.addEventListener('wheel', onWheel, { passive: false });
    return () => canvas.removeEventListener('wheel', onWheel);
  }, [engine, refresh]);

  // Keyboard: one dispatch through the shortcut registry (window scope, not inside text fields).
  useEffect(() => {
    const run: Record<ShortcutAction, () => void> = {
      'tool.select': () => engine.setTool('select'),
      'tool.rectangle': () => engine.setTool('rectangle'),
      'edit.delete': () => engine.deleteSelection(),
      'edit.undo': () => engine.undo(),
      'edit.redo': () => engine.redo(),
      'edit.cancel': () => engine.escape(),
    };
    const onKeyDown = (event: KeyboardEvent) => {
      const target = event.target as HTMLElement | null;
      if (target && (target.isContentEditable || /^(INPUT|TEXTAREA|SELECT)$/.test(target.tagName))) return;
      const action = matchShortcut(event);
      if (!action) return;
      event.preventDefault();
      run[action]();
      refresh();
    };
    window.addEventListener('keydown', onKeyDown);
    return () => window.removeEventListener('keydown', onKeyDown);
  }, [engine, refresh]);

  const onPointerDown = (event: PointerEvent<HTMLCanvasElement>) => {
    if (event.button !== 0 && event.button !== 1) return;
    event.preventDefault();
    event.currentTarget.setPointerCapture(event.pointerId);
    engine.pointerDown(event.clientX, event.clientY, event.button, event.shiftKey);
    refresh();
  };

  const onPointerMove = (event: PointerEvent<HTMLCanvasElement>) => {
    engine.pointerMove(event.clientX, event.clientY);
    refresh();
  };

  const endPointer = (event: PointerEvent<HTMLCanvasElement>, cancelled: boolean) => {
    if (event.currentTarget.hasPointerCapture(event.pointerId)) {
      event.currentTarget.releasePointerCapture(event.pointerId);
    }
    if (cancelled) {
      engine.cancel();
    } else {
      engine.pointerUp(event.clientX, event.clientY);
    }
    refresh();
  };

  return (
    <section className="canvas-host" aria-label="Editor canvas">
      <div className="canvas-host__tools" role="toolbar" aria-label="Tools">
        {TOOLS.map((tool) => (
          <button
            key={tool.name}
            type="button"
            className="canvas-host__tool"
            aria-pressed={snap.tool === tool.name}
            data-testid={`tool-${tool.name}`}
            title={`${tool.label} (${tool.key})`}
            onClick={() => {
              engine.setTool(tool.name);
              refresh();
            }}
          >
            {tool.label}
          </button>
        ))}
      </div>
      <canvas
        ref={canvasRef}
        className="canvas-host__surface"
        data-testid="editor-canvas"
        onPointerDown={onPointerDown}
        onPointerMove={onPointerMove}
        onPointerUp={(event) => endPointer(event, false)}
        onPointerCancel={(event) => endPointer(event, true)}
        onContextMenu={(event) => event.preventDefault()}
      />
      <output className="canvas-host__readout" data-testid="view-readout">
        {viewReadout(snap.view)}
      </output>
      <output className="canvas-host__counts" data-testid="object-readout">
        {objectReadout(snap.objects, snap.selected)}
      </output>
    </section>
  );
}
