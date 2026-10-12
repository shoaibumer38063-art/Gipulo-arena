/**
 * Canvas painting for the editor view (Slice 0a, task 020). Pure functions: no DOM access in the
 * geometry, so the tests can check them directly.
 *
 * Boundary law: this module PAINTS what the core reports. It performs no coordinate conversion
 * (CAN-COORD-2). The only arithmetic is pixel alignment for crisp 1 px lines (CAN-SS-3).
 */
import type { EngineView } from '../engine/client';

/** A line segment in viewport pixels. */
export interface Segment {
  readonly x1: number;
  readonly y1: number;
  readonly x2: number;
  readonly y2: number;
}

/**
 * The world origin marker: a 1 px crosshair through the point the core reports as the origin.
 * Lines are half-pixel aligned so they stay crisp at every zoom (CAN-SS-3, CAN-MICRO guides).
 */
export function originCrosshair(view: EngineView): readonly Segment[] {
  const x = Math.round(view.originX) + 0.5;
  const y = Math.round(view.originY) + 0.5;
  return [
    { x1: 0, y1: y, x2: view.width, y2: y },
    { x1: x, y1: 0, x2: x, y2: view.height },
  ];
}

/** Human-readable readout of the view (zoom percent and origin), for the status output. */
export function viewReadout(view: EngineView): string {
  return `Zoom ${Math.round(view.zoomPercent)}% · origin ${Math.round(view.originX)}, ${Math.round(view.originY)}`;
}

/** Colors read from the design tokens at paint time (never hard-coded here). */
export interface PaintStyle {
  readonly background: string;
  readonly guide: string;
}

/** Paints one frame. `devicePixelRatio` scales the backing store only (device pixels). */
export function paintFrame(
  ctx: CanvasRenderingContext2D,
  view: EngineView,
  style: PaintStyle,
  devicePixelRatio: number,
): void {
  ctx.setTransform(devicePixelRatio, 0, 0, devicePixelRatio, 0, 0);
  ctx.fillStyle = style.background;
  ctx.fillRect(0, 0, view.width, view.height);
  ctx.strokeStyle = style.guide;
  ctx.lineWidth = 1;
  for (const s of originCrosshair(view)) {
    ctx.beginPath();
    ctx.moveTo(s.x1, s.y1);
    ctx.lineTo(s.x2, s.y2);
    ctx.stroke();
  }
}
