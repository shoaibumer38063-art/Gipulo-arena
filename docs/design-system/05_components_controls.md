# 05 — Components: Controls (buttons · fields · selectors · sliders · color)

> All values `[DS]` from the HTML reference unless tagged. Component states are MANDATORY —
> a component without its full matrix is unfinished (`rules/ui.md` UI3, `07` §4). Every control
> here is a `07`-registered component; features compose them, never restyle them.

## 1. Buttons

Geometry `[DS]`: height **24px** · radius **5px** (`radius/m`) · min-width 64px · padding `0 10px`
· weight 500 · gap 4px (icon+label) · transition background 80ms, box-shadow 80ms.

| Variant | Rest | Hover | Pressed | Disabled |
|---|---|---|---|---|
| **Primary** (the ONE per view) | white text on `linear-gradient(180deg, accent/hi, accent/lo)` + `inset 0 0 0 .5px rgba(255,255,255,.35)` ring + `shadow/button` | gradient `#4db8ff → accent/default` | solid `accent/lo` + `inset 0 1px 2px rgba(0,0,0,.3)` | opacity .4 |
| **Secondary** | bg `bg/panel`, border 1px `border/strong` | bg `bg/hover` | bg `border/default` + `inset 0 1px 1px rgba(0,0,0,.12)` | opacity .4 |
| **Ghost** | transparent | bg `bg/hover` | bg `border/default` | opacity .4 |
| **Icon button** | 24×24, padding 0, no min-width | bg `bg/hover` | bg `border/default` | opacity .4 |
| **Destructive** `[PROPOSED]` | secondary geometry, `danger` text/icon | bg `danger`, white text | bg darker danger `[PROPOSED]` | opacity .4 |

Additional mandatory states:
- **Focus (keyboard)** `[DS]`: double ring `0 0 0 2px bg/panel, 0 0 0 4px accent/default`
  (for controls on canvas-adjacent chrome use `:focus-visible` equivalents; never removed).
- **Loading** `[PROPOSED]`: width locked, label at opacity .6 replaced by `spinner` (12px) +
  label; interaction blocked; no layout shift.
- **Selected/toggle buttons** (tool buttons in toolbar): active = solid `accent/default` bg,
  white content `[DS]` — that is the ONLY filled accent button state in chrome besides primary.

Laws: text is `type/body`; gradient reserved (`01` §4); icon-only buttons require aria-label
(`04` §7); buttons never contain more than one line — truncate label with tooltip fallback.

## 2. Segmented control `[DS]`

- Container: bg `bg/input`, radius 5, padding 2, gap 2.
- Segment: height 22, padding `0 10px`, radius 3, `type/body`, color `text/secondary`.
- Active segment: bg `bg/panel`, color `text/primary`, ring `0 0 0 .5px border/strong` +
  `0 1px 2px rgba(0,0,0,.12)`.
- Hover (inactive): `text/primary`, bg transparent (container tint carries it).
- Disabled: opacity .4 (whole control). Keyboard: Left/Right moves selection; focus ring per §1.
- Use for: 2–5 mutually exclusive views (Design/Prototype tabs use full Tab style — `06` §6).

## 3. Fields (text + numeric) `[DS]`

- Geometry: height **28px**, bg `bg/input`, border 1px transparent, radius 5.
- Label prefix (`W`, `X`, `Y`, `R`…): 26px wide, centered, `text/secondary`, weight 500,
  `cursor: ew-resize` (scrubbable — step 1, ⌥ = 0.1, per interactions I7).
- Text: `type/body`, tabular numerics; selects-all on focus; commit on Enter/blur; Escape
  restores (interactions I6).
- Placeholder ("Mixed") uses `text/primary` — intentionally readable `[DS]`.

| State | Treatment |
|---|---|
| Hover | border → `border/strong` |
| Focus | border `accent/default` + `inset 0 0 0 .5px accent/default` |
| Invalid | border `danger` + error surfaced on submit/blur (`11` §7) |
| Disabled | opacity .4, no hover/focus |
| Mixed values | placeholder "Mixed" (readable); typing replaces all |
| Inherited value `[PROPOSED]` | "Inherited" affordance from style/component sources: value shown normally + indicator dot; hover reveals source tooltip |
| Overridden value `[PROPOSED]` | accent dot/ring on the field label prefix; hover shows "Overridden" tooltip |
| Editing (renaming contexts) | `outline 1px accent` radius 2 padding `0 4px` bg `bg/panel` (layer rename, `[DS]` from layer spec) |
- Prefix hover: the prefix itself is the scrub handle — cursor `ew-resize` over prefix only.
- No native spinners, no native validation bubbles (`01` §3).

## 4. Search field `[PROPOSED]`

Field base (§3) +: `search` icon 16px left inset 8px; text padding-left 24px; clear (`cross`
small) appears while non-empty (icon button 16×16); placeholder `Search…` at `text/tertiary`;
⌘F/Escape behaviors defined by the owning surface (panels: focus/Escape clears).

## 5. Toggle `[DS]`

- 28×16px, radius 8. Off: bg `border/strong`, knob 12px white at 2px inset, `0 1px 1px` shadow.
- On: bg `accent/default`, knob `translateX(12px)`. Motion 120ms `ease/standard` (`10` §3).
- Hover: track border-color emphasis `[PROPOSED]`; Focus: double ring (§1); Disabled: .4.
- A toggle applies immediately (no confirm); it never masquerades as a Checkbox.

## 6. Checkbox & radio `[DS]`

- Checkbox: 16×16, border 1px `border/strong`, radius 3. Checked: bg+border `accent/default`,
  `check` icon 12px stroke-width 1.75 white. Mixed: accent minus glyph.
- Radio: 16×16, border 1px `border/strong`, circle. Checked: `border: 5px solid accent/default`
  (donut). Group layout: 8px row gap, 4px control-label gap.
- Hover: border `accent/default`; Focus: double ring; Disabled: .4 (both control and label).

## 7. Slider `[DS]`

- Track: 2px (radius 1) — filled `accent/default`, rest `border/strong`.
- Knob: 12px white circle, ring `0 0 0 .5px rgba(0,0,0,.2)` + `shadow/knob`.
- States: hover knob scale none (color-only `[PROPOSED]`); active/focus: knob ring at full
  `accent`; disabled: .4.
- Behavior: click-to-jump, drag continuous commit-as-one-transaction (interactions I7);
  value shown in adjacent numeric field when the surface provides one.

## 8. Color control `[DS]` + `[PROPOSED]` picker

- Swatch (rest state): 16×16, radius 3, ring `0 0 0 1px border/default`; palette chips: 40px
  tall, radius 5 `[DS]`.
- Swatch states: hover (ring `border/strong` + slight raise), selected (ring 2px `accent`),
  opacity < 100% shows checkerboard behind fill `[PROPOSED]`.
- Picker popover `[PROPOSED]` (authored outline; full design in the color feature phase):
  saturation/value field, hue + alpha sliders (DS slider §7), RGBA/HEX fields (DS fields §3),
  eyedropper trigger (uses canvas eyedropper cursor, `09`), swatch palettes grid, token
  bindings (variables) section, "no color" row.
- The picker is a DS popover (`06` §3) — never a native color input.

## 9. Do-not list (all controls)

- No control may hardcode sizes/colors outside `02` tokens.
- No variant may be invented per-feature; missing variant = DS update first (`07` §6).
- No control may animate layout (width/height/top/left) — transform/opacity/color only.
- No control may shrink below the 24px hit target even when visually smaller (`02` §10).
- No zoom-dependent bitmap assets in controls; icons are vector (`04`).
