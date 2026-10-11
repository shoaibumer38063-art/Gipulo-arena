/**
 * PropertyRow — docs/design-system/06_components_surfaces.md §8.
 * 28px row, label column + control slot. The ROW is anatomy + states; the properties panel
 * feature owns which rows appear per selection type. HOT component: flat paint only.
 */
import type { ReactNode } from 'react';

import './PropertyRow.css';

interface PropertyRowProps {
  label: string;
  children: ReactNode;
  disabled?: boolean;
  className?: string;
}

export function PropertyRow({ label, children, disabled = false, className }: PropertyRowProps) {
  return (
    <div
      className={['gip-prop-row', disabled ? 'is-disabled' : '', className ?? '']
        .filter(Boolean)
        .join(' ')}
    >
      <span className="gip-prop-row__label">{label}</span>
      <div className="gip-prop-row__control">{children}</div>
    </div>
  );
}
