/**
 * Toggle — docs/design-system/05_components_controls.md §5.
 * 28×16 switch, 120ms knob, applies immediately (no confirm page). Native `button` keeps
 * Space/Enter activation for free; role=switch + aria-checked carry the state.
 */
import './Toggle.css';

interface ToggleProps {
  checked: boolean;
  onChange: (checked: boolean) => void;
  disabled?: boolean;
  ariaLabel: string;
}

export function Toggle({ checked, onChange, disabled = false, ariaLabel }: ToggleProps) {
  return (
    <button
      type="button"
      role="switch"
      aria-checked={checked}
      aria-label={ariaLabel}
      disabled={disabled}
      className={`gip-toggle${checked ? ' is-on' : ''}`}
      onClick={() => onChange(!checked)}
    >
      <span className="gip-toggle__knob" />
    </button>
  );
}
