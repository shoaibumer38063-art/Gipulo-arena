/**
 * Tooltip — docs/design-system/06_components_surfaces.md §4.
 * tip/surface (theme-invariant), 400ms hover delay [PROPOSED per canvas §27], shortcut part
 * at .5 opacity. Content is a noun phrase; when the control has a visible label the tooltip
 * adds direction/context and MUST match the aria-label (03 §6).
 */
import { useEffect, useId, useRef, useState } from 'react';
import type { ReactNode } from 'react';

import './Tooltip.css';

interface TooltipProps {
  label: string;
  shortcut?: string;
  children: ReactNode;
  /** Delay before showing on hover (ms). Focus shows immediately (keyboard users). */
  delay?: number;
}

export function Tooltip({ label, shortcut, children, delay = 400 }: TooltipProps) {
  const [visible, setVisible] = useState(false);
  const timer = useRef<number | null>(null);
  const id = useId();

  useEffect(() => {
    return () => {
      if (timer.current !== null) window.clearTimeout(timer.current);
    };
  }, []);

  function show(delayMs: number) {
    if (timer.current !== null) window.clearTimeout(timer.current);
    timer.current = window.setTimeout(() => setVisible(true), delayMs);
  }

  function hide() {
    if (timer.current !== null) window.clearTimeout(timer.current);
    timer.current = null;
    setVisible(false);
  }

  return (
    <span
      className="gip-tooltip-anchor"
      aria-describedby={visible ? id : undefined}
      onMouseEnter={() => show(delay)}
      onMouseLeave={hide}
      onFocus={() => show(0)}
      onBlur={hide}
    >
      {children}
      {visible && (
        <span role="tooltip" id={id} className="gip-tooltip">
          {label}
          {shortcut && <span className="gip-tooltip__shortcut">{shortcut}</span>}
        </span>
      )}
    </span>
  );
}
