# UI RULES — Design System law for all Gipulo interface work

> Status: ACTIVE (v1, 2026-10-11; DS docs formalized 2026-10-11). Sources:
> `docs/memory/05_design_system_brief.md` (requirements), `docs/memory/07_design_tokens_and_visual_specs.md`
> (distilled values), **`docs/design-system/` (the formalized system — implementation form of
> this rule: tokens, components, states, canvas UI, cursors, motion, a11y, responsive)**, Design
> System HTML (`docs/source/Gipulo — Design System.html` — rendered ground truth).
> Motto: **Familiar workflow, different identity.** The DS is the single visual+interaction source
> of truth; an agent creating UI should almost never have to guess "what should this look like?"

## UI1 — Tokens only, zero magic values

- Every color/size/radius/spacing/duration/easing/z-index comes from the Design System token layer
  (naming convention per `07` §14: `bg/canvas`, `accent/default`, `radius/m`, `motion/hover`, …).
- Writing raw values (`border-radius: 7px`, `padding: 9px`, `color: #3478ff`) is a violation
  UNLESS the value exists in `07`/the DS HTML or an ADR records a feature-specific reason.
- Colors are NEVER hardcoded in components — theming (light/dark, future themes) works only
  through tokens. Guide/accent identity colors are theme-invariant (`07` §2).

## UI2 — No default browser/OS/framework behavior (zero exceptions)

Banned unless visually replaced by DS components: native `<select>` · `alert/confirm/prompt` ·
default text-selection blue (use `--sel`) · default focus rings (use DS: `:focus-visible` 2px
accent, offset 1px; button focus = double ring) · default scrollbars (DS: 10px, thumb `--bd2`,
radius 5px, 3px transparent border) · default context menus on app chrome · default drag ghosts ·
default `cursor` values chosen ad hoc (cursor system, UI8) · browser validation bubbles.

## UI3 — Complete states, always

- No component ships with only its default state. The full state matrix per component type is
  defined in `05` §Complete state system and demonstrated in the DS HTML:
  buttons (default/hover/pressed/focus/disabled × primary/secondary/ghost/icon), fields
  (default/hover/focus/invalid/disabled/mixed), layer rows (13 states), toggles/checkboxes/radios/
  segmented (on/off/hover/disabled), menus (hover/checked/disabled/destructive/submenu), etc.
- New component ⇒ add its states to the DS (reference + docs) in the same task.
- Interaction values are binding: hover 80ms · toggle 120ms · menu 160ms · ease
  `cubic-bezier(.2,0,0,1)` · reduced-motion ⇒ all transitions none.

## UI4 — Compose, never recreate

- Features compose existing DS components (buttons, fields, menus, tooltips, rows, chips…).
  Creating a slightly-different variant of an existing component is a CQ1 duplication violation.
- A genuinely new visual pattern ⇒ extend the DS first (component + tokens + states + docs), then
  use it. One-off floating UIs for tools are forbidden; contextual controls follow the DS.

## UI5 — Binding metrics (from `07`; single source — do not re-derive)

- Shell: 40px top toolbar · 240px side panels · tabs 40px · tools 32px radius 5px.
- Controls: buttons 24px (icon 24×24) · fields 28px (label prefix 26px) · layer rows 28px ·
  menu width 224px, radius 13px, items 24px · chips 20px · toggles 28×16 · checkbox/radio 16px.
- Radii: m=5px controls · l=13px menus · 3px pills/segments · 2px rename outline.
- Typography: Inter `450 11/16` base · Title `600 13/20` · Caption `500 10/16 +0.02em` ·
  JetBrains Mono `500 10/14` · tabular-nums for all numeric values.
- Density: 20/24/28px (ultra-compact/compact/comfortable); **visual size ≠ hit area: minimum hit
  target 24px**; base grid 4px.
- Spacing examples: panel section padding `8px 16px 12px` · section header `12px 16px 4px` ·
  layer row indent 16px/level (8px beyond level 8, guide lines 1px `--bd`).

## UI6 — Reserved-meaning colors (closed language)

- Accent `#0d99ff`: primary actions, selection, focus ONLY ("neutral UI; accent reserved").
- Gradient (accent-hi→accent-lo): the ONE primary action per view — nowhere else.
- Purple `#9747ff`: components/instances. Red `#f24822`: danger/errors in UI chrome — and on
  canvas ONLY ruler guides (never errors; canvas rules CAN-GUIDE-1). Magenta `#ec2fd0`: spacing
  guides. Orange `#ff8a00`: snap/measure guides. Success `#14ae5c`, warning `#ffb800`.
- Adding/recoloring a meaning = DS change + ADR, not a feature decision.

## UI7 — Panels & editor layout (familiar workflow)

- Layers left · canvas center · properties right · top toolbar; panel tabs (Layers/Assets,
  Design/Prototype); properties adapt per selection type (11 selection states defined in `05`).
- Responsive editor: reference breakpoint ≤900px ⇒ single column, panels hidden, first 4 tools
  only (`07` §4); further breakpoints get defined in the DS before implementation — the editor
  must never compress controls below usability (min hit 24px).
- Layer panel: full behavior spec in `07` §9 (hover actions, insertion indicator 2px line + 6px
  circle, drop-inside ring 1.5px accent, descendant tint 8%/18%, renaming 120px input).

## UI8 — Cursors & icons

- Cursors: one cursor engine (canvas rules CAN-CUR); UI-side: default `cursor: default` on
  buttons (never accidental pointer/text cursors); ew-resize on scrubbable field labels; custom
  cursor asset set is [PROPOSED] — until designed, use the documented standard-cursor mapping and
  record gaps in the DS docs.
- Icons: ONE icon language (`rules/icons.md`): 16px grid · stroke 1.25 · round caps/joins ·
  fill-none default (solid exceptions documented) · states via color tokens, never new glyphs ·
  no mixing external icon packs.

## UI9 — Menus, tooltips, toasts, dialogs

- All follow `07` §10 exactly (menu 224px/13px radius/24px items, hover = accent bg + white text,
  destructive = danger, shortcuts right-aligned 50% opacity; tooltip `--tip` bg 5px radius
  `5px 8px` padding; toast 8px radius; same visual language across dropdowns/context menus/
  command palette/property menus).
- Keyboard navigation is mandatory: arrow traversal, Enter activation, Escape close, typeahead
  where search exists, focus return to the invoker, viewport-collision flipping (documented per
  component).

## UI10 — Accessibility & performance of UI

- Contrast: text ≥ WCAG AA against its surface in BOTH themes; state feedback never color-only
  (add icon/dash/text — e.g. hidden rows also dim to 45%, locked rows show the lock icon).
- Hit targets ≥24px even when visuals are smaller (separate hit areas — density rule).
- Repeated components (layer rows, property rows, list items) stay cheap: NO blur, NO multi-shadow,
  NO complex filters per-row; elevation/blur reserved for floating layers (menus/dialogs).
  Virtualize long lists (layer panel with thousands of rows) — never render all DOM rows.
- Focus order logical (Tab through panels/toolbars per DS); canvas a11y per canvas rules §24.

## UI11 — UI change protocol

Before: read this file + `07` + the DS HTML section being extended. Check for an existing
component first (UI4).
After: update DS docs/reference if patterns/tokens changed; verify both themes + reduced-motion +
density variants; run the Documentation Update Rule; screenshot/snapshot states where test
tooling exists.
