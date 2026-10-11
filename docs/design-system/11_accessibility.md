# 11 — Accessibility (part of the DS, not a retrofit)

> Requirements: Design System Brief §Accessibility + `rules/ui.md` UI10 + `rules/canvas.md`
> CAN-A11Y (Mirror-DOM plan). An advanced visual editor must be fully usable without depending
> on color, and without a mouse.

## 1. Commitments

1. WCAG AA contrast for all text and meaning-bearing non-text (both themes).
2. Complete keyboard operation of the editor (canvas commands included — CAN-A11Y-2).
3. Visible focus everywhere; focus order follows visual order (interactions I2 scopes).
4. State never communicated by color alone (icons, shapes, dash patterns, text accompany).
5. Screen-reader access to canvas content via the Mirror-DOM architecture (canvas §24); UI
   labels match visible labels exactly `[UI3]` (`03` §6).
6. Reduced-motion fully honored (`10` §2.5).

## 2. Contrast audit of the reference tokens (findings + recommendations)

| Pair | Approx. ratio | AA verdict | Recommendation |
|---|---|---|---|
| `text/primary` on `bg/panel` | ≈ 15:1 | ✓ | — |
| `text/secondary` (rgba .5 ≈ #808080) on white | ≈ 3.9:1 | ✗ for 11px text | **[PROPOSED] bump text use of `text/secondary` to rgba(0,0,0,.6) ≈ 5.5:1 light / rgba(255,255,255,.7) ≈ 5.5:1 dark**; icons may keep .5 (non-text AA needs 3:1 ✓) |
| `text/tertiary` (rgba .3 ≈ #b3b3b3) on white | ≈ 2.2:1 | ✗ | Allowed ONLY for non-informational captions; informational text must use the bumped secondary — **[owner approval requested as one DS revision]** |
| `accent` #0d99ff as text on white | ≈ 3.2:1 | ✗ for small text | Accent reserved for fills/borders/selection; accent TEXT only ≥14px semibold, or with icon — use `accent/lo` (#0b82dc ≈ 4.6:1) for small accent-colored text |
| `danger` #f24822 as text on white | ≈ 3.7:1 | ✗ small text | Error text pairs with icon + weight 500; for dense required reading use darker `#c93a17` variant **[PROPOSED token `danger/text`]** |
| `success` #14ae5c on white | ≈ 3.0:1 | ✗ tiny text | Same rule: status with icon, or darker text variant `[PROPOSED]` |
| White on `tip/surface` #1e1e1e | ≈ 17:1 | ✓ | — |
| White on `accent` (primary button) | ≈ 3.3:1 | ✗ strict | **[PROPOSED] primary button label weight 500 + gradient: verify; if kept, acceptable as large/bold (AA-large 3:1 ✓ at 14px bold — our 11px is not large.)** → flagged as an owner decision (brand identity vs strict AA); mitigation options: darken gradient lo, or add subtle text shadow — decide in DS revision |
| Guide colors on `bg/canvas` light | blue 2.8:1 · magenta 3.2:1 · orange 2.1:1 · red 3.5:1 | non-text | Guides are strokes ≥1px with casing/contrast laws (`08` §1 white halo); keys: never color-alone — dash patterns + labels differentiate guides (CAN-GUIDE-1) ✓ |

**These findings are the DS doing its job** — they are the only places the reference does not yet
meet AA for very small text. One consolidated owner review can approve the proposed token bumps
(as a documented DS revision) — until then, the reference values stand `[DS]` and the
recommendations are binding for NEW surfaces where no legacy constraint exists.

## 3. Focus & keyboard

- Focus ring: 2px `accent` at 1px offset (`02` §5); controls with adjacent surfaces use the
  double ring (`05` §1). `:focus-visible` only — never removed, never restyled per feature.
- Focus scopes (interactions I2): canvas · panel · modal · menu · palette — Escape returns to
  the invoking scope; focus is trapped in modals/dialogs; returning focus restores the invoker.
- Full canvas operation: tool shortcuts, nudge, resize via keyboard, layer navigation, history,
  pan/zoom (`rules/interactions.md` I-shortcut registry; CAN-A11Y-2).
- Focus order: left panel → canvas → right panel (declared per view); panels are labelled
  landmarks (`role="complementary"` + labels).

## 4. Hit targets & input

- Minimum interactive target **24px** (`02` §10); visual size may be smaller (density law).
- Canvas handle hit areas: 16px (`08` §2); rotate arcs r≈14; splitters 8px bands.
- Touch/pen (where supported): targets never below 24px; hover-only affordances
  (lock/eye reveal) have always-visible equivalents on touch surfaces `[PROPOSED]`.
- Drag operations always have keyboard equivalents (nudge, numeric entry — I7).

## 5. Screen readers

- UI: semantic roles on all DS components (dialogs `role=dialog` + labelled title; menus
  `role=menu` with `menuitem`s; tabs `role=tablist`; toasts `aria-live=polite` (coalesced —
  CAN-A11Y-2); progress `role=progressbar`).
- Icons: `aria-hidden` with text, `aria-label` without (`04` §7); labels match visible text
  `[UI3]`.
- Canvas: Mirror-DOM per `rules/canvas.md` §24 — the scene graph mirrored as semantic DOM with
  per-layer accessible summaries, bidirectional selection/focus sync, positioned via affine
  transforms (public Figma pattern). Canvas features may not ship without their Mirror-DOM
  projection (CAN-A11Y-1).
- Announcements: selection changes, tool changes, mode changes, transaction commits (intent
  labels — `gip-history` IntentLabel), coalesced (arrow nudges = one announcement per burst).

## 6. Non-color feedback (the color-blind test)

- Every state pair (default/selected, locked/unlocked, error/ok) differs by ≥2 channels:
  color + shape/icon OR dash pattern OR position (e.g. locked = lock icon + dashed; RVS
  activated = solid + count pill; checkboxes = glyph, not just fill).
- Guides are already multiply-encoded (color + dash pattern + labels) — this is why the closed
  language works for accessibility.

## 7. Errors, disabled, zoom

- Errors: icon + message + recovery action; message in plain language; never code-only; error
  announced via live region; invalid fields flagged on blur/submit, not while typing.
- Disabled: opacity .4 + `aria-disabled` + cursor per policy; disabled controls remain
  focusable for discoverability `[PROPOSED]` (so SR users can find features) — never
  color-only.
- Zoom: layouts verified at 80%–200% browser zoom; text remains ≥10px effective; no clipping
  (`12` §7).
- `prefers-contrast`/forced-colors: Gipulo honors forced-colors by degrading to system colors
  for chrome `[PROPOSED — verify in implementation]`.
