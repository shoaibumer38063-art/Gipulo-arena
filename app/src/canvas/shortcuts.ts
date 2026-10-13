/**
 * Keyboard shortcut registry. One table maps keys to editor actions; the canvas host
 * dispatches through it, so no key handling is scattered through components.
 */

/** Editor actions a shortcut can trigger. */
export type ShortcutAction =
  | 'tool.select'
  | 'tool.rectangle'
  | 'edit.delete'
  | 'edit.undo'
  | 'edit.redo'
  | 'edit.cancel'
  | 'edit.flipH'
  | 'edit.flipV';

/** One binding. `key` is compared case-insensitively. `mod` = Ctrl or ⌘. */
export interface ShortcutBinding {
  readonly action: ShortcutAction;
  readonly key: string;
  readonly mod?: boolean;
  readonly shift?: boolean;
}

export const SHORTCUTS: readonly ShortcutBinding[] = [
  { action: 'tool.select', key: 'v' },
  { action: 'tool.rectangle', key: 'r' },
  { action: 'edit.delete', key: 'Delete' },
  { action: 'edit.delete', key: 'Backspace' },
  { action: 'edit.undo', key: 'z', mod: true },
  { action: 'edit.redo', key: 'z', mod: true, shift: true },
  { action: 'edit.cancel', key: 'Escape' },
  { action: 'edit.flipH', key: 'h', shift: true },
  { action: 'edit.flipV', key: 'v', shift: true },
];

/** The keyboard fields the matcher reads (a subset of KeyboardEvent). */
export interface KeyInput {
  readonly key: string;
  readonly ctrlKey: boolean;
  readonly metaKey: boolean;
  readonly shiftKey: boolean;
}

/** Returns the action bound to this key press, if any. Modifiers must match exactly. */
export function matchShortcut(
  input: KeyInput,
  bindings: readonly ShortcutBinding[] = SHORTCUTS,
): ShortcutAction | undefined {
  const mod = input.ctrlKey || input.metaKey;
  const key = input.key.toLowerCase();
  const hit = bindings.find(
    (b) =>
      b.key.toLowerCase() === key &&
      (b.mod ?? false) === mod &&
      (b.shift ?? false) === input.shiftKey,
  );
  return hit?.action;
}
