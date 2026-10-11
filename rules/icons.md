# ICON RULES — One icon language, no exceptions

> Status: ACTIVE (v1, 2026-10-11). Sources: DS brief (`05` §Icon System), DS HTML reference
> (`07` §6). Icons are part of the Design System — not per-feature decoration.

## IC1 — The single icon language (binding specs from the DS reference)

- Grid/viewBox: **16×16** base. Stroke: `currentColor`, **stroke-width 1.25**, round caps,
  round joins, `fill: none` by default.
- Solid-fill exceptions are deliberate and listed: `move` (cursor arrow), `comp` (component
  diamonds). Any new solid icon requires DS sign-off (it must still read at 16px next to stroked
  neighbors).
- Optical alignment within the 16px box: ~1.5–2.5px padding to live area; strokes land on
  half-pixel boundaries when axis-aligned; no accidental visual-weight outliers (an icon that
  looks heavier/lighter than its row neighbors is a defect).

## IC2 — One set, one source

- All icons live in ONE icon set (sprite/symbol system like the DS reference `<symbol id="i-*">`),
  imported by ID. Copy-pasting SVG paths into components, or pulling icons from random external
  packs per feature, is a CQ1 duplication violation.
- Adding an icon = extending the set (same grid/stroke/language) + documenting it in the DS icon
  registry (name, meaning, usage areas). If a suitable icon exists — reuse it; near-duplicates
  ("plus" vs "add" vs "new") are banned.
- External icon libraries: forbidden without an ADR (dependency rule CQ1.4) AND restyling to the
  Gipulo language (stroke 1.25/16px grid) — adopting a pack "as-is" is never acceptable.

## IC3 — Size variants by area (all derived from the 16px master)

- Toolbar: 16px in 32px buttons [DS]. Panels/menus/rows: 16px. Small property-inline icons:
  12–14px [DS example: checkbox check renders 12px at stroke-width 1.75 — smaller renders may
  bump stroke weight for legibility; documented per size]. Canvas-control icons/status: 16px
  screen-constant (canvas rules CAN-SS-1). Larger display sizes (empty states, onboarding) get
  dedicated optical variants — never naive upscaling of the 16px path if it degrades.

## IC4 — States via color, never via glyph swaps

- Icon states use text/icon color tokens only: default `--t2` · active/selected `--accent` (or
  per DS) · component context `--comp` · error `--danger` · disabled = parent's 40% opacity ·
  hover = row/button hover color. Creating separate "active icon" glyph variants is banned
  (exception: filled-vs-outlined toggle where the DS explicitly defines both, e.g. visibility).
- Meaning pairs already fixed by DS: eye = visibility (hidden rows dim + keep icon), lock =
  locked, chev = expand/collapse (rotated 90° = expanded), check = on/checked/done.

## IC5 — Accessibility & behavior

- Interactive icons are buttons with accessible labels (`aria-label` — the DS reference does this
  for toolbar tools); decorative icons are `aria-hidden`. Icon-only controls expose name+shortcut
  in tooltips (interactions rules I3).
- Hit areas: icon buttons follow control sizes (24px icon buttons, 32px tools) — the glyph is
  never the hit target (min hit 24px, ui rules UI10).

## IC6 — Testing/checks

- Icon set lint (when tooling exists): all symbols on 16px viewBox, stroke-width 1.25 (documented
  exceptions), no inline duplicates in components, every `i-*` ID used or pruned (dead-icon sweep
  per removal rules). Visual regression snapshots for icon rows in both themes.
