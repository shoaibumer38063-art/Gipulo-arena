/**
 * Field — docs/design-system/05_components_controls.md §3.
 * Text/numeric field with scrubbable label prefix, mixed values, invalid state, and the
 * commit/cancel contract from interactions I6 (Enter commits, Escape restores, focus
 * selects all). Scrubbing emits previews and commits once on release (one gesture = one
 * transaction — undo-redo.md).
 */
import { useEffect, useRef, useState } from 'react';
import type { PointerEvent as ReactPointerEvent } from 'react';

import './Field.css';

interface FieldProps {
  /** Prefix label (W, H, X, Y, R…) — also the scrub handle for numeric fields. */
  label?: string;
  /** `null` renders the Mixed placeholder (readable, text-primary — 07 §8). */
  value: string | number | null;
  /** Enable prefix scrubbing (numeric values). */
  scrub?: boolean;
  /** Scrub step per pixel; hold ⌥ for /10 (interactions I7). */
  scrubStep?: number;
  invalid?: boolean;
  disabled?: boolean;
  placeholder?: string;
  ariaLabel?: string;
  /** Final value (Enter, blur, or scrub release). */
  onCommit?: (value: string) => void;
  /** Live value while scrubbing (transient state — never a document write). */
  onPreview?: (value: string) => void;
}

function toDisplay(value: string | number | null): string {
  if (value === null) return '';
  return typeof value === 'number' ? String(value) : value;
}

/** Pointer capture is progressive: environments without it (older browsers, jsdom) simply
 *  skip capturing — gestures still work via move/up on the element itself. */
function capturePointer(element: Element, pointerId: number): void {
  const target = element as Element & { setPointerCapture?: (id: number) => void };
  if (typeof target.setPointerCapture === 'function') target.setPointerCapture(pointerId);
}

function releasePointer(element: Element, pointerId: number): void {
  const target = element as Element & { releasePointerCapture?: (id: number) => void };
  if (typeof target.releasePointerCapture === 'function') target.releasePointerCapture(pointerId);
}

export function Field({
  label,
  value,
  scrub = false,
  scrubStep = 1,
  invalid = false,
  disabled = false,
  placeholder,
  ariaLabel,
  onCommit,
  onPreview,
}: FieldProps) {
  const inputRef = useRef<HTMLInputElement>(null);
  const [draft, setDraft] = useState<string | null>(null);
  const scrubState = useRef<{ startX: number; startValue: number; latest: string } | null>(null);
  /** Set by Escape so the blur that follows does NOT commit the discarded edit (I6). */
  const cancelNextBlurCommit = useRef(false);

  const display = draft ?? toDisplay(value);
  const mixed = value === null && draft === null;
  const inputId = `gip-field-${label ?? 'value'}`;

  // If the value prop changes while not editing, the draft must not shadow it.
  useEffect(() => {
    if (document.activeElement !== inputRef.current) setDraft(null);
  }, [value]);

  function commitDraft() {
    if (cancelNextBlurCommit.current) {
      cancelNextBlurCommit.current = false;
      setDraft(null);
      return;
    }
    if (draft !== null && onCommit) onCommit(draft);
    setDraft(null);
  }

  function beginScrub(event: ReactPointerEvent<HTMLLabelElement>) {
    if (!scrub || disabled) return;
    const numeric = Number.parseFloat(toDisplay(value));
    if (Number.isNaN(numeric)) return;
    event.preventDefault();
    capturePointer(event.currentTarget, event.pointerId);
    scrubState.current = { startX: event.clientX, startValue: numeric, latest: toDisplay(value) };
  }

  function moveScrub(event: ReactPointerEvent<HTMLLabelElement>) {
    const state = scrubState.current;
    if (!state) return;
    const step = event.altKey ? scrubStep / 10 : scrubStep;
    const next = state.startValue + Math.round((event.clientX - state.startX) / 2) * step;
    const nextString = String(Math.round(next * 100) / 100);
    state.latest = nextString;
    setDraft(nextString);
    if (onPreview) onPreview(nextString);
  }

  function endScrub(event: ReactPointerEvent<HTMLLabelElement>) {
    const state = scrubState.current;
    if (!state) return;
    releasePointer(event.currentTarget, event.pointerId);
    scrubState.current = null;
    if (state.latest !== toDisplay(value) && onCommit) onCommit(state.latest);
    setDraft(null);
  }

  return (
    <div
      className={[
        'gip-field',
        mixed ? 'is-mixed' : '',
        invalid ? 'is-invalid' : '',
        disabled ? 'is-disabled' : '',
        scrub ? 'is-scrubbable' : '',
      ]
        .filter(Boolean)
        .join(' ')}
    >
      {label && (
        <label
          className="gip-field__label"
          htmlFor={inputId}
          onPointerDown={beginScrub}
          onPointerMove={moveScrub}
          onPointerUp={endScrub}
          onPointerCancel={endScrub}
        >
          {label}
        </label>
      )}
      <input
        ref={inputRef}
        id={inputId}
        className="gip-field__input"
        type="text"
        inputMode={scrub ? 'decimal' : undefined}
        value={display}
        placeholder={mixed ? 'Mixed' : placeholder}
        aria-label={ariaLabel ?? label}
        aria-invalid={invalid || undefined}
        disabled={disabled}
        onChange={(event) => setDraft(event.target.value)}
        onFocus={(event) => {
          setDraft(toDisplay(value));
          event.target.select();
        }}
        onBlur={commitDraft}
        onKeyDown={(event) => {
          if (event.key === 'Enter') {
            commitDraft();
            inputRef.current?.blur();
          } else if (event.key === 'Escape') {
            cancelNextBlurCommit.current = true; // restore the committed value, discard the draft
            setDraft(null);
            inputRef.current?.blur();
          }
        }}
      />
    </div>
  );
}
