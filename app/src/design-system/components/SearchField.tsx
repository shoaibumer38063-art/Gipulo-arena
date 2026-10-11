/**
 * SearchField — docs/design-system/05_components_controls.md §4 [PROPOSED].
 * Field base + leading search icon + clear affordance while non-empty.
 */
import { Icon } from './Icon';
import './SearchField.css';

interface SearchFieldProps {
  value: string;
  onChange: (value: string) => void;
  placeholder?: string;
  ariaLabel: string;
  disabled?: boolean;
}

export function SearchField({
  value,
  onChange,
  placeholder = 'Search…',
  ariaLabel,
  disabled = false,
}: SearchFieldProps) {
  return (
    <div className={`gip-search${disabled ? ' is-disabled' : ''}`}>
      <span className="gip-search__icon">
        <Icon name="search" size={16} />
      </span>
      <input
        type="text"
        className="gip-search__input"
        value={value}
        placeholder={placeholder}
        aria-label={ariaLabel}
        disabled={disabled}
        onChange={(event) => onChange(event.target.value)}
        onKeyDown={(event) => {
          if (event.key === 'Escape') onChange('');
        }}
      />
      {value !== '' && (
        <button
          type="button"
          className="gip-search__clear"
          aria-label="Clear search"
          onClick={() => onChange('')}
        >
          <Icon name="cross" size={12} />
        </button>
      )}
    </div>
  );
}
