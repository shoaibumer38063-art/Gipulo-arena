/**
 * Slider — docs/design-system/05_components_controls.md §7.
 * Custom-drawn (no native range): 2px track, 12px knob, keyboard + pointer support.
 * Drag emits live changes; a drag is ONE interaction (callers commit appropriately —
 * undo-redo.md one-gesture-one-entry).
 */
import { useRef } from 'react';
import type { KeyboardEvent as ReactKeyboardEvent, PointerEvent as ReactPointerEvent } from 'react';

import './Slider.css';

interface SliderProps {
  min?: number;
  max?: number;
  step?: number;
  value: number;
  onChange: (value: number) => void;
  disabled?: boolean;
  ariaLabel: string;
}

function clamp(value: number, min: number, max: number): number {
  return Math.min(max, Math.max(min, value));
}

/** Pointer capture is progressive (jsdom / older browsers skip it — gestures still work). */
function capturePointer(element: Element, pointerId: number): void {
  const target = element as Element & { setPointerCapture?: (id: number) => void };
  if (typeof target.setPointerCapture === 'function') target.setPointerCapture(pointerId);
}

function releasePointer(element: Element, pointerId: number): void {
  const target = element as Element & { releasePointerCapture?: (id: number) => void };
  if (typeof target.releasePointerCapture === 'function') target.releasePointerCapture(pointerId);
}

export function Slider({
  min = 0,
  max = 100,
  step = 1,
  value,
  onChange,
  disabled = false,
  ariaLabel,
}: SliderProps) {
  const trackRef = useRef<HTMLDivElement>(null);
  const dragging = useRef(false);

  const ratio = clamp((value - min) / (max - min || 1), 0, 1);

  function valueFromClientX(clientX: number): number {
    const track = trackRef.current;
    if (!track) return value;
    const rect = track.getBoundingClientRect();
    if (rect.width === 0) return value;
    const raw = min + ((clientX - rect.left) / rect.width) * (max - min);
    const stepped = Math.round(raw / step) * step;
    return clamp(Math.round(stepped * 1000) / 1000, min, max);
  }

  function onPointerDown(event: ReactPointerEvent<HTMLDivElement>) {
    if (disabled) return;
    dragging.current = true;
    capturePointer(event.currentTarget, event.pointerId);
    onChange(valueFromClientX(event.clientX));
  }

  function onPointerMove(event: ReactPointerEvent<HTMLDivElement>) {
    if (!dragging.current || disabled) return;
    onChange(valueFromClientX(event.clientX));
  }

  function onPointerUp(event: ReactPointerEvent<HTMLDivElement>) {
    dragging.current = false;
    releasePointer(event.currentTarget, event.pointerId);
  }

  function onKeyDown(event: ReactKeyboardEvent<HTMLDivElement>) {
    if (disabled) return;
    const multiplier = event.shiftKey ? 10 : 1;
    let next = value;
    if (event.key === 'ArrowRight' || event.key === 'ArrowUp') next = value + step * multiplier;
    else if (event.key === 'ArrowLeft' || event.key === 'ArrowDown') next = value - step * multiplier;
    else if (event.key === 'Home') next = min;
    else if (event.key === 'End') next = max;
    else return;
    event.preventDefault();
    onChange(clamp(Math.round(next * 1000) / 1000, min, max));
  }

  return (
    <div
      ref={trackRef}
      className={`gip-slider${disabled ? ' is-disabled' : ''}`}
      role="slider"
      aria-label={ariaLabel}
      aria-valuemin={min}
      aria-valuemax={max}
      aria-valuenow={value}
      tabIndex={disabled ? -1 : 0}
      onPointerDown={onPointerDown}
      onPointerMove={onPointerMove}
      onPointerUp={onPointerUp}
      onKeyDown={onKeyDown}
    >
      <div className="gip-slider__track">
        <div className="gip-slider__fill" style={{ width: `${ratio * 100}%` }} />
      </div>
      <div className="gip-slider__knob" style={{ left: `${ratio * 100}%` }} />
    </div>
  );
}
