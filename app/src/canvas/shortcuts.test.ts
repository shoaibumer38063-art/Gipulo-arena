import { describe, expect, it } from 'vitest';

import { SHORTCUTS, matchShortcut } from './shortcuts';

const press = (key: string, extra: Partial<{ ctrlKey: boolean; metaKey: boolean; shiftKey: boolean }> = {}) => ({
  key,
  ctrlKey: false,
  metaKey: false,
  shiftKey: false,
  ...extra,
});

describe('shortcut registry', () => {
  it('maps the Slice 0 keys to their actions', () => {
    expect(matchShortcut(press('r'))).toBe('tool.rectangle');
    expect(matchShortcut(press('R'))).toBe('tool.rectangle');
    expect(matchShortcut(press('v'))).toBe('tool.select');
    expect(matchShortcut(press('Delete'))).toBe('edit.delete');
    expect(matchShortcut(press('Backspace'))).toBe('edit.delete');
    expect(matchShortcut(press('Escape'))).toBe('edit.cancel');
  });

  it('requires the modifier to match exactly', () => {
    expect(matchShortcut(press('z', { ctrlKey: true }))).toBe('edit.undo');
    expect(matchShortcut(press('z', { metaKey: true }))).toBe('edit.undo');
    expect(matchShortcut(press('z', { ctrlKey: true, shiftKey: true }))).toBe('edit.redo');
    expect(matchShortcut(press('z'))).toBeUndefined();
    expect(matchShortcut(press('r', { ctrlKey: true }))).toBeUndefined();
  });

  it('maps Shift+H and Shift+V to flips, not to the tools', () => {
    expect(matchShortcut(press('H', { shiftKey: true }))).toBe('edit.flipH');
    expect(matchShortcut(press('V', { shiftKey: true }))).toBe('edit.flipV');
    expect(matchShortcut(press('v'))).toBe('tool.select');
    expect(matchShortcut(press('h'))).toBeUndefined();
  });

  it('has no two bindings for the same key and modifiers', () => {
    const keys = SHORTCUTS.map((b) => `${b.key.toLowerCase()}|${b.mod ?? false}|${b.shift ?? false}`);
    expect(new Set(keys).size).toBe(keys.length);
  });
});
