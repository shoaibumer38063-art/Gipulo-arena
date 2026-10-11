/**
 * Segmented control — docs/design-system/05_components_controls.md §2.
 * 2–5 mutually exclusive options; arrow-key navigation; active segment carries the panel
 * surface + ring (the reference's "raised" treatment).
 */
import type { KeyboardEvent } from 'react';

import { Icon, type IconName } from './Icon';
import './Segmented.css';

export interface SegmentedOption<T extends string> {
  value: T;
  label: string;
  icon?: IconName;
}

interface SegmentedProps<T extends string> {
  options: ReadonlyArray<SegmentedOption<T>>;
  value: T;
  onChange: (value: T) => void;
  disabled?: boolean;
  ariaLabel: string;
}

export function Segmented<T extends string>({
  options,
  value,
  onChange,
  disabled = false,
  ariaLabel,
}: SegmentedProps<T>) {
  function onKeyDown(event: KeyboardEvent<HTMLDivElement>) {
    if (disabled) return;
    const index = options.findIndex((option) => option.value === value);
    if (index < 0) return;
    let next = index;
    if (event.key === 'ArrowRight' || event.key === 'ArrowDown') next = (index + 1) % options.length;
    else if (event.key === 'ArrowLeft' || event.key === 'ArrowUp') next = (index - 1 + options.length) % options.length;
    else return;
    event.preventDefault();
    const option = options[next];
    if (option) onChange(option.value);
  }

  return (
    <div
      className={`gip-segmented${disabled ? ' is-disabled' : ''}`}
      role="radiogroup"
      aria-label={ariaLabel}
      onKeyDown={onKeyDown}
    >
      {options.map((option) => {
        const active = option.value === value;
        return (
          <button
            key={option.value}
            type="button"
            role="radio"
            aria-checked={active}
            tabIndex={active ? 0 : -1}
            disabled={disabled}
            className={`gip-segmented__segment${active ? ' is-active' : ''}`}
            onClick={() => onChange(option.value)}
          >
            {option.icon && <Icon name={option.icon} size={14} />}
            {option.label}
          </button>
        );
      })}
    </div>
  );
}
