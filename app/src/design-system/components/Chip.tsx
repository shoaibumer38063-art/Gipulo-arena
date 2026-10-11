/**
 * Chip — docs/design-system/06_components_surfaces.md §11.
 * 20px pill for statuses/filters/tags. Error chips exist for values like "Missing font"
 * (danger text on the tinted surface — 11 §6 keeps an icon or wording alongside, never
 * color alone).
 */
import type { ReactNode } from 'react';

import './Chip.css';

interface ChipProps {
  children: ReactNode;
  active?: boolean;
  error?: boolean;
}

export function Chip({ children, active = false, error = false }: ChipProps) {
  return (
    <span
      className={['gip-chip', active ? 'is-active' : '', error ? 'is-error' : '']
        .filter(Boolean)
        .join(' ')}
    >
      {children}
    </span>
  );
}
