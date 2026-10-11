import { readFileSync } from 'node:fs';
import { join } from 'node:path';

import { describe, expect, it } from 'vitest';

import { TOKEN_NAMES, applyTheme, cssVar } from './tokens';

/**
 * Token-layer integrity: tokens.css ↔ tokens.ts must never drift (docs/design-system/02_tokens.md).
 * If this test fails, someone added/changed a token in one place only — fix both + the doc.
 * (Vitest runs with cwd = app/; import.meta.url is not a file: URL under the test transform.)
 */
const cssPath = join(process.cwd(), 'src', 'design-system', 'tokens.css');
const css = readFileSync(cssPath, 'utf8');

function variableNames(block: string): Set<string> {
  const names = new Set<string>();
  for (const match of block.matchAll(/--([a-z0-9-]+)\s*:/g)) {
    if (match[1]) names.add(match[1]);
  }
  return names;
}

const rootMatch = /:root \{([\s\S]*?)\n\}/.exec(css);
const darkMatch = /\[data-theme='dark'\] \{([\s\S]*?)\n\}/.exec(css);

describe('token layer integrity', () => {
  it('finds the light and dark token blocks in tokens.css', () => {
    expect(rootMatch).not.toBeNull();
    expect(darkMatch).not.toBeNull();
  });

  it('tokens.ts mirrors every :root token exactly (no drift either direction)', () => {
    const root = variableNames(rootMatch?.[1] ?? '');
    const ts = new Set<string>(TOKEN_NAMES);
    const missingInTs = [...root].filter((name) => !ts.has(name));
    const missingInCss = [...ts].filter((name) => !root.has(name));
    expect(missingInTs, 'defined in CSS but missing from TOKEN_NAMES').toEqual([]);
    expect(missingInCss, 'declared in TOKEN_NAMES but missing from tokens.css').toEqual([]);
  });

  it('dark theme only overrides tokens that exist in light (theme-invariant tokens stay put)', () => {
    const root = variableNames(rootMatch?.[1] ?? '');
    const dark = variableNames(darkMatch?.[1] ?? '');
    const unknown = [...dark].filter((name) => !root.has(name));
    expect(unknown, 'dark-only tokens are not allowed').toEqual([]);
  });

  it('guide language colors are theme-invariant (closed language — 08 §9)', () => {
    const dark = variableNames(darkMatch?.[1] ?? '');
    for (const guide of ['guide-geometry', 'guide-spacing', 'guide-snap', 'guide-ruler']) {
      expect(dark.has(guide), `${guide} must not be overridden in dark`).toBe(false);
    }
  });
});

describe('token helpers', () => {
  it('cssVar produces a var() reference', () => {
    expect(cssVar('accent')).toBe('var(--accent)');
    expect(cssVar('guide-spacing')).toBe('var(--guide-spacing)');
  });

  it('applyTheme sets and clears the data-theme attribute', () => {
    const root = document.documentElement;
    applyTheme('dark', root);
    expect(root.getAttribute('data-theme')).toBe('dark');
    applyTheme('light', root);
    expect(root.getAttribute('data-theme')).toBe('light');
    applyTheme('system', root);
    expect(root.hasAttribute('data-theme')).toBe(false);
  });
});
