/**
 * Tabs — docs/design-system/06_components_surfaces.md §6.
 * 40px tablist, 16px gaps, border-bottom rule, 2px accent underline on the active tab
 * [PROPOSED in 06 §6]. ARIA tablist with roving tabindex + arrow keys (11 §5).
 */
import type { KeyboardEvent as ReactKeyboardEvent } from 'react';

import './Tabs.css';

export interface TabDef<T extends string> {
  id: T;
  label: string;
}

interface TabsProps<T extends string> {
  tabs: ReadonlyArray<TabDef<T>>;
  value: T;
  onChange: (value: T) => void;
  ariaLabel: string;
}

export function Tabs<T extends string>({ tabs, value, onChange, ariaLabel }: TabsProps<T>) {
  function onKeyDown(event: ReactKeyboardEvent<HTMLDivElement>) {
    const index = tabs.findIndex((tab) => tab.id === value);
    if (index < 0) return;
    let next = index;
    if (event.key === 'ArrowRight') next = (index + 1) % tabs.length;
    else if (event.key === 'ArrowLeft') next = (index - 1 + tabs.length) % tabs.length;
    else if (event.key === 'Home') next = 0;
    else if (event.key === 'End') next = tabs.length - 1;
    else return;
    event.preventDefault();
    const tab = tabs[next];
    if (tab) onChange(tab.id);
  }

  return (
    <div className="gip-tabs" role="tablist" aria-label={ariaLabel} onKeyDown={onKeyDown}>
      {tabs.map((tab) => {
        const selected = tab.id === value;
        return (
          <button
            key={tab.id}
            type="button"
            role="tab"
            aria-selected={selected}
            tabIndex={selected ? 0 : -1}
            className={`gip-tabs__tab${selected ? ' is-active' : ''}`}
            onClick={() => onChange(tab.id)}
          >
            {tab.label}
          </button>
        );
      })}
    </div>
  );
}
