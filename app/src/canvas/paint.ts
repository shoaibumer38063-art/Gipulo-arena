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

/** Flag bits in a rect record (see the core's `Engine.rects`). */
export const RECT_FLAG_SELECTED = 1;
export const RECT_FLAG_HOVERED = 2;
export const RECT_FLAG_PREVIEW = 4;

/** Object and selection counts for the status readout. */
export function objectReadout(objects: number, selected: number): string {
  const noun = objects === 1 ? 'object' : 'objects';
  return `${objects} ${noun} · ${selected} selected`;
}

/** Colors and hover opacity for rectangles, read from tokens by the caller. */
export interface RectPaintStyle {
  readonly fill: string;
  readonly accent: string;
  readonly hoverAlpha: number;
}

/**
 * Paints the core's draw list. Each rectangle is a polygon of core-computed viewport corners.
 * Selection is a 1 px outline with no handles (Slice 0, CAN-SEL-5). Hover is the same outline at
 * reduced opacity (CAN-SEL-6). A creation preview is a dashed outline.
 */
export function paintRects(
  ctx: CanvasRenderingContext2D,
  rects: Float64Array,
  stride: number,
  style: RectPaintStyle,
): void {
  ctx.lineWidth = 1;
  for (let i = 0; i + stride <= rects.length; i += stride) {
    const flags = rects[i + 8] ?? 0;
    ctx.beginPath();
    ctx.moveTo(rects[i] ?? 0, rects[i + 1] ?? 0);
    ctx.lineTo(rects[i + 2] ?? 0, rects[i + 3] ?? 0);
    ctx.lineTo(rects[i + 4] ?? 0, rects[i + 5] ?? 0);
    ctx.lineTo(rects[i + 6] ?? 0, rects[i + 7] ?? 0);
    ctx.closePath();
    const preview = (flags & RECT_FLAG_PREVIEW) !== 0;
    if (!preview) {
      ctx.fillStyle = style.fill;
      ctx.fill();
    }
    ctx.globalAlpha = 1;
    if (preview) {
      ctx.setLineDash([4, 3]);
      ctx.strokeStyle = style.accent;
      ctx.stroke();
      ctx.setLineDash([]);
    } else if ((flags & RECT_FLAG_SELECTED) !== 0) {
      ctx.strokeStyle = style.accent;
      ctx.stroke();
    } else if ((flags & RECT_FLAG_HOVERED) !== 0) {
      ctx.globalAlpha = style.hoverAlpha;
      ctx.strokeStyle = style.accent;
      ctx.stroke();
      ctx.globalAlpha = 1;
    }
  }
}

/** Readout for the selected object's size, in whole world units. */
export function sizeReadout(bounds: Float64Array): string {
  if (bounds.length < 4) return '';
  return `${Math.round(bounds[2] ?? 0)} × ${Math.round(bounds[3] ?? 0)}`;
}

/**
 * Paints the resize handles as squares centered on the core's handle points. `size` is the
 * visual edge length in screen pixels, read from the `--handle-size` token by the caller.
 */
export function paintHandles(
  ctx: CanvasRenderingContext2D,
  handles: Float64Array,
  size: number,
  style: { readonly fill: string; readonly accent: string },
): void {
  const half = size / 2;
  ctx.lineWidth = 1;
  for (let i = 0; i + 2 <= handles.length; i += 2) {
    const x = handles[i] ?? 0;
    const y = handles[i + 1] ?? 0;
    ctx.fillStyle = style.fill;
    ctx.fillRect(x - half, y - half, size, size);
    ctx.strokeStyle = style.accent;
    ctx.strokeRect(x - half, y - half, size, size);
  }
}
