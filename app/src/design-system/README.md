# Design System — implementation module (shell side)

Agent-facing conventions for `app/src/design-system/`. The authored system lives in
`docs/design-system/` (the law); this module is its **implementation form**. If code and docs
disagree, the docs win and the code is fixed in the same task.

## Laws that bind this folder

1. **Tokens only** — every color/size/radius/motion comes from `tokens.css`/`tokens.ts`
   (`rules/ui.md` UI1). A raw value in a component is a bug.
2. **Complete states** — a component ships its full state matrix from `docs/design-system/05–06`
   or it is unfinished (`07` §4 state-completeness gate).
3. **Compose, never recreate** — features use these components with their variants; local
   restyling is forbidden (`07` §1).
4. **Hot vs cold tiers** (`07` §7) — layer rows / property rows stay cheap-painted
   (flat fills, 1px strokes, color/opacity transitions only). Menus/popovers/dialogs may use
   elevation.
5. **No browser defaults** — resets live in `globals.css`; never re-enable a native control
   (`01` §3 checklist).

## Layout

```
tokens.css        the numeric source of truth (light/dark, fonts, all tokens)
globals.css       reset + selection/focus/scrollbar/reduced-motion laws
tokens.ts         typed access (TOKEN_NAMES ↔ tokens.css, tested for drift)
fonts/            self-hosted Inter + JetBrains Mono (variable, latin) — offline-safe
components/       one component per file + co-located .css (class prefix `gip-`)
gallery/          story page rendering every component in its states (dev preview surface)
```

## Test rule

The state matrix is the test matrix: new states require a test (or an explicit "visual-only"
note in the component file). `tokens.test.ts` guards token drift; component tests cover
role/aria/keyboard/behavior — not pixel styles.

## Adding a component

Follow `docs/design-system/07_component_architecture.md` §6 (spec first → registry → code →
tests → documentation sweep). Register it in the registry table in the same task.
