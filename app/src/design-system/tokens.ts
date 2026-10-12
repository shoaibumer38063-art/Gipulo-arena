/**
 * Gipulo Design System — typed token access.
 *
 * Mirrors `tokens.css` (the numeric source of truth). `tokens.test.ts` fails if the two drift
 * apart, so adding a token is a three-step chore: tokens.css → here → docs/design-system/02.
 */

/** Every CSS custom property defined in tokens.css (theme-invariant names; dark overrides
 *  share the same names). Keep sorted by section for reviewability. */
export const TOKEN_NAMES = [
  // type
  'font-ui',
  'font-mono',
  'type-title',
  'type-section',
  'type-body',
  'type-body-medium',
  'type-caption',
  'type-code',
  'type-pill',
  'ls-caption',
  // surfaces
  'bg-canvas',
  'bg-panel',
  'bg-input',
  'bg-hover',
  'bg-selected',
  'frame-bg',
  'toolbar-bg',
  'tip-bg',
  'menu-bg',
  'menu-text',
  'menu-sep',
  'menu-border',
  // text
  'text-primary',
  'text-secondary',
  'text-tertiary',
  // borders
  'border-default',
  'border-strong',
  'border-focus',
  // identity + status
  'accent',
  'accent-hi',
  'accent-hover-fuel',
  'accent-lo',
  'danger',
  'danger-text',
  'success',
  'warning',
  'component',
  'selection-fill',
  'brand-gradient',
  // guide language (closed)
  'guide-geometry',
  'guide-spacing',
  'guide-snap',
  'guide-ruler',
  // spacing
  'space-1',
  'space-2',
  'space-3',
  'space-4',
  'space-5',
  'space-6',
  'space-7',
  'space-8',
  // radii
  'radius-xs',
  'radius-s',
  'radius-m',
  'radius-logo',
  'radius-toast',
  'radius-pill',
  'radius-l',
  // strokes
  'stroke-hair',
  'stroke-default',
  'stroke-strong',
  'stroke-canvas-path',
  'stroke-handle-line',
  'stroke-halo',
  // canvas
  'handle-size',
  // elevation
  'shadow-elevation',
  'shadow-menu',
  'shadow-button',
  'shadow-segment',
  'shadow-knob',
  // opacity
  'opacity-disabled',
  'opacity-hidden-row',
  'opacity-muted',
  'opacity-shortcut',
  'opacity-inactive',
  // motion
  'ease-standard',
  'motion-hover',
  'motion-toggle',
  'motion-menu',
  'motion-snap-release',
  // z-layers
  'z-overlay',
  'z-pill',
  'z-floating',
  'z-panel',
  'z-popover',
  'z-menu',
  'z-dialog',
  'z-toast',
  'z-tooltip',
  // density + metrics
  'density-ultra',
  'density-compact',
  'density-comfortable',
  'hit-min',
  'control-h',
  'field-h',
  'row-h',
  'canvas-host-h',
  'menu-item-h',
  'tab-h',
  'toolbar-h',
  'tool-h',
  'logo-size',
  'panel-w',
  'panel-w-min',
  'panel-w-max',
  'indent-step',
  'indent-deep-step',
  'scrollbar-size',
  'icon-size',
  'icon-size-sm',
] as const;

export type TokenName = (typeof TOKEN_NAMES)[number];

/** Returns a `var(...)` reference for a token — the only sanctioned way to consume values
 *  from TypeScript (ui.md UI1). */
export function cssVar(name: TokenName): string {
  return `var(--${name})`;
}

export type ThemePreference = 'light' | 'dark' | 'system';

/** Applies a theme preference to the document root. `system` removes the override so the
 *  `prefers-color-scheme` rules in tokens.css take effect. */
export function applyTheme(preference: ThemePreference, root: HTMLElement = document.documentElement): void {
  if (preference === 'system') {
    root.removeAttribute('data-theme');
  } else {
    root.setAttribute('data-theme', preference);
  }
}
