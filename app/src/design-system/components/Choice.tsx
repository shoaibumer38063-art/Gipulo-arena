/**
 * Checkbox & Radio — docs/design-system/05_components_controls.md §6.
 * Native inputs are kept (a11y, forms, keyboard) and rebuilt visually with appearance:none —
 * the "replace, never expose" law (01 §3). The check glyph follows the icon spec (04).
 */
import { useEffect, useRef } from 'react';

import { Icon } from './Icon';
import './Choice.css';

interface CheckboxProps {
  checked: boolean;
  indeterminate?: boolean;
  onChange: (checked: boolean) => void;
  disabled?: boolean;
  label?: string;
  ariaLabel?: string;
}

export function Checkbox({
  checked,
  indeterminate = false,
  onChange,
  disabled = false,
  label,
  ariaLabel,
}: CheckboxProps) {
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    if (inputRef.current) inputRef.current.indeterminate = indeterminate && !checked;
  }, [indeterminate, checked]);

  const input = (
    <span className="gip-choice__control gip-choice__control--checkbox">
      <input
        ref={inputRef}
        type="checkbox"
        checked={checked}
        disabled={disabled}
        aria-label={ariaLabel ?? label}
        onChange={(event) => onChange(event.target.checked)}
      />
      <span className="gip-choice__visual" aria-hidden="true">
        {indeterminate && !checked ? <Icon name="minus" size={12} /> : <Icon name="check" size={12} />}
      </span>
    </span>
  );

  return (
    <label className={`gip-choice${disabled ? ' is-disabled' : ''}`}>
      {input}
      {label && <span className="gip-choice__label">{label}</span>}
    </label>
  );
}

interface RadioProps {
  checked: boolean;
  onChange: () => void;
  name: string;
  value: string;
  disabled?: boolean;
  label?: string;
}

export function Radio({ checked, onChange, name, value, disabled = false, label }: RadioProps) {
  return (
    <label className={`gip-choice${disabled ? ' is-disabled' : ''}`}>
      <span className="gip-choice__control gip-choice__control--radio">
        <input
          type="radio"
          name={name}
          value={value}
          checked={checked}
          disabled={disabled}
          aria-label={label ?? value}
          onChange={onChange}
        />
        <span className="gip-choice__visual" aria-hidden="true" />
      </span>
      {label && <span className="gip-choice__label">{label}</span>}
    </label>
  );
}
