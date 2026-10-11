/**
 * Icon — the Gipulo icon language (docs/design-system/04_icons.md).
 *
 * Spec: 16px grid · stroke 1.25 · round caps/joins · currentColor · fill none
 * (solid exceptions: `move`, `comp`, `more`). Sizes are DERIVED from the 16 master — never a
 * second hand-drawn variant. Feature icons are added with their features (04 §6); this file
 * currently carries the reference set + the minimum additions the DS components need.
 */
import type { ReactElement } from 'react';

export type IconName =
  // reference set (04 §5)
  | 'move'
  | 'frame'
  | 'rect'
  | 'pen'
  | 'text'
  | 'hand'
  | 'eye'
  | 'lock'
  | 'chev'
  | 'check'
  | 'plus'
  | 'comp'
  | 'inst'
  // node types needed by layer rows / toolbars (04 §6 inventory)
  | 'group'
  | 'image'
  | 'vector'
  // component-support additions (all listed in 04 §6 inventory)
  | 'minus'
  | 'cross'
  | 'search'
  | 'more';

export type IconSize = 12 | 14 | 16 | 20 | 24;

interface IconProps {
  name: IconName;
  size?: IconSize;
  className?: string;
}

const STROKE = 1.25;

/** Icons drawn as strokes; `move`, `comp` and `more` are the solid exceptions. */
function stroked(path: ReactElement, size: IconSize, className: string | undefined) {
  return (
    <svg
      className={className}
      width={size}
      height={size}
      viewBox="0 0 16 16"
      fill="none"
      stroke="currentColor"
      strokeWidth={STROKE}
      strokeLinecap="round"
      strokeLinejoin="round"
      aria-hidden="true"
      focusable="false"
    >
      {path}
    </svg>
  );
}

function solid(children: ReactElement, size: IconSize, className: string | undefined) {
  return (
    <svg
      className={className}
      width={size}
      height={size}
      viewBox="0 0 16 16"
      fill="currentColor"
      stroke="none"
      aria-hidden="true"
      focusable="false"
    >
      {children}
    </svg>
  );
}

export function Icon({ name, size = 16, className }: IconProps) {
  switch (name) {
    case 'move':
      return solid(<path d="M3.5 2.4 L13 7.4 L8.7 8.6 L7.5 12.9 Z" />, size, className);
    case 'comp':
      return solid(
        <>
          <path d="M8 1.3 L10.7 4 L8 6.7 L5.3 4 Z" />
          <path d="M8 9.3 L10.7 12 L8 14.7 L5.3 12 Z" />
          <path d="M1.3 8 L4 5.3 L6.7 8 L4 10.7 Z" />
          <path d="M9.3 8 L12 5.3 L14.7 8 L12 10.7 Z" />
        </>,
        size,
        className,
      );
    case 'more':
      return solid(
        <>
          <circle cx="3.2" cy="8" r="1.1" />
          <circle cx="8" cy="8" r="1.1" />
          <circle cx="12.8" cy="8" r="1.1" />
        </>,
        size,
        className,
      );
    case 'frame':
      return stroked(
        <>
          <path d="M5 2v12" />
          <path d="M11 2v12" />
          <path d="M2 5h12" />
          <path d="M2 11h12" />
        </>,
        size,
        className,
      );
    case 'rect':
      return stroked(<rect x="2.5" y="3.5" width="11" height="9" rx="1.5" />, size, className);
    case 'group':
      return stroked(
        <rect x="2.5" y="3.5" width="11" height="9" rx="1.5" strokeDasharray="2.8 2.2" />,
        size,
        className,
      );
    case 'image':
      return stroked(
        <>
          <rect x="2.5" y="3.5" width="11" height="9" rx="1.5" />
          <circle cx="6" cy="6.7" r="1.05" />
          <path d="M3.6 12 L6.9 9.3 L9 11.1 L11 9.5 L12.6 11" />
        </>,
        size,
        className,
      );
    case 'vector':
      return stroked(
        <>
          <path d="M4.4 11.4 C5.6 5.8 10.4 5.8 11.6 11.4" />
          <rect x="2.6" y="10.6" width="3" height="3" rx="0.7" />
          <rect x="10.4" y="10.6" width="3" height="3" rx="0.7" />
        </>,
        size,
        className,
      );
    case 'pen':
      return stroked(
        <>
          <path d="M8 2.3 L12.4 6.9 L8 13.7 L3.6 6.9 Z" />
          <path d="M8 7.2v6.5" />
        </>,
        size,
        className,
      );
    case 'text':
      return stroked(
        <>
          <path d="M3.5 4V2.5h9V4" />
          <path d="M8 2.5v11" />
          <path d="M5.8 13.5h4.4" />
        </>,
        size,
        className,
      );
    case 'hand':
      return stroked(
        <>
          <path d="M5.4 8.6V4.9a1.05 1.05 0 0 1 2.1 0V8" />
          <path d="M7.5 8V3.7a1.05 1.05 0 0 1 2.1 0V8" />
          <path d="M9.6 8V4.4a1.05 1.05 0 0 1 2.1 0V8.2" />
          <path d="M11.7 8.2V6a1 1 0 0 1 2 0v4a4.3 4.3 0 0 1-4.3 4.3H8.2A4.3 4.3 0 0 1 3.9 10V7.2a1.05 1.05 0 0 1 1.5 0" />
        </>,
        size,
        className,
      );
    case 'eye':
      return stroked(
        <>
          <path d="M1.6 8S4.1 4.6 8 4.6 14.4 8 14.4 8 11.9 11.4 8 11.4 1.6 8 1.6 8Z" />
          <circle cx="8" cy="8" r="1.7" />
        </>,
        size,
        className,
      );
    case 'lock':
      return stroked(
        <>
          <rect x="4.4" y="7" width="7.2" height="6" rx="1.4" />
          <path d="M5.8 7V5.3a2.2 2.2 0 0 1 4.4 0V7" />
        </>,
        size,
        className,
      );
    case 'chev':
      return stroked(<path d="M6 3.5 L10.5 8 L6 12.5" />, size, className);
    case 'check':
      return stroked(<path d="M3.4 8.6 L6.4 11.6 L12.6 4.4" />, size, className);
    case 'plus':
      return stroked(
        <>
          <path d="M8 3.5v9" />
          <path d="M3.5 8h9" />
        </>,
        size,
        className,
      );
    case 'minus':
      return stroked(<path d="M3.5 8h9" />, size, className);
    case 'cross':
      return stroked(
        <>
          <path d="M4.2 4.2 L11.8 11.8" />
          <path d="M11.8 4.2 L4.2 11.8" />
        </>,
        size,
        className,
      );
    case 'search':
      return stroked(
        <>
          <circle cx="7" cy="7" r="3.75" />
          <path d="M9.9 9.9 L13 13" />
        </>,
        size,
        className,
      );
    case 'inst':
      return stroked(<path d="M8 1.8 L11 4.8 L8 7.8 L5 4.8 Z" />, size, className);
  }
}
