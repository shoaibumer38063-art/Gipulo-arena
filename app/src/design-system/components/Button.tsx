/**
 * Button — docs/design-system/05_components_controls.md §1.
 * Variants: primary (ONE per view — gradient law), secondary, ghost, icon, destructive.
 * States: hover, pressed, focus (double ring), disabled, loading (width-locked, no shift).
 */
import type { ReactNode } from 'react';

import { Icon, type IconName, type IconSize } from './Icon';
import './Button.css';

export type ButtonVariant = 'primary' | 'secondary' | 'ghost' | 'icon' | 'destructive';

interface ButtonProps {
  variant?: ButtonVariant;
  children?: ReactNode;
  /** Leading icon. For `icon` variant this is the only content (aria-label required). */
  icon?: IconName;
  iconSize?: IconSize;
  /** Icon-only buttons must name themselves for assistive tech (04 §7). */
  ariaLabel?: string;
  loading?: boolean;
  disabled?: boolean;
  title?: string;
  onClick?: () => void;
  type?: 'button' | 'submit';
}

export function Button({
  variant = 'secondary',
  children,
  icon,
  iconSize = 16,
  ariaLabel,
  loading = false,
  disabled = false,
  title,
  onClick,
  type = 'button',
}: ButtonProps) {
  const classes = ['gip-btn', `gip-btn--${variant}`];
  if (loading) classes.push('is-loading');

  return (
    <button
      type={type}
      className={classes.join(' ')}
      disabled={disabled || loading}
      aria-busy={loading || undefined}
      aria-label={ariaLabel}
      title={title}
      onClick={onClick}
    >
      {loading && <span className="gip-btn__spinner" aria-hidden="true" />}
      {icon && !loading && <Icon name={icon} size={iconSize} />}
      {children && <span className="gip-btn__label">{children}</span>}
    </button>
  );
}
