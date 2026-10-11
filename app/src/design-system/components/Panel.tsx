/**
 * Panel & Section — docs/design-system/06_components_surfaces.md §7.
 * Panel = the docked surface (bg-panel, 1px border, pinned headers, scrolling content).
 * Section = the properties-section rhythm (padding 8/16/12, 1px top border) with optional
 * collapse via the chevron header.
 */
import { useState } from 'react';
import type { ReactNode } from 'react';

import { Icon } from './Icon';
import './Panel.css';

interface PanelProps {
  children: ReactNode;
  className?: string;
  /** 240px default; see 12_responsive.md §2 for the 200–480 range policy. */
  width?: number;
}

export function Panel({ children, className, width }: PanelProps) {
  return (
    <div
      className={className ? `gip-panel ${className}` : 'gip-panel'}
      style={width === undefined ? undefined : { width }}
    >
      {children}
    </div>
  );
}

interface SectionProps {
  title: string;
  children: ReactNode;
  collapsible?: boolean;
  defaultOpen?: boolean;
}

export function Section({ title, children, collapsible = true, defaultOpen = true }: SectionProps) {
  const [open, setOpen] = useState(defaultOpen);
  const bodyId = `gip-section-${title.replace(/\s+/g, '-').toLowerCase()}`;

  return (
    <section className="gip-section">
      {collapsible ? (
        <button
          type="button"
          className="gip-section__header is-button"
          aria-expanded={open}
          aria-controls={bodyId}
          onClick={() => setOpen((value) => !value)}
        >
          <span className={`gip-section__chev${open ? ' is-open' : ''}`}>
            <Icon name="chev" size={12} />
          </span>
          {title}
        </button>
      ) : (
        <div className="gip-section__header">{title}</div>
      )}
      {open && (
        <div className="gip-section__body" id={bodyId}>
          {children}
        </div>
      )}
    </section>
  );
}
