# 07 — Design Tokens & Concrete Visual Specs (extracted from the Design System HTML)

> Source: `docs/source/Gipulo — Design System.html` — a WORKING visual reference containing the
> concrete design language. Every value below was extracted from it. When the brief (`05`) states
> a requirement, this file states the **actual numbers/colors/behaviors**. The HTML itself is the
> rendered ground truth — open it in a browser to see all states live (it has light/dark toggle).

## 1. Color tokens — LIGHT theme (`:root`)

| Token | Value | Meaning |
|---|---|---|
| `--bg-canvas` | `#f5f5f5` | editor canvas background |
| `--bg-panel` | `#fff` | panels / raised surfaces |
| `--bg-input` | `#f5f5f5` | input & segmented-control background |
| `--bg-hover` | `rgba(0,0,0,.06)` | generic hover background |
| `--bg-sel` | `#e5f4ff` | selected row background |
| `--bd` | `rgba(0,0,0,.1)` | standard border / separators |
| `--bd2` | `rgba(0,0,0,.22)` | stronger border (hover/focus edges, thumbs) |
| `--t1` | `rgba(0,0,0,.9)` | primary text |
| `--t2` | `rgba(0,0,0,.5)` | secondary text / icons / code |
| `--t3` | `rgba(0,0,0,.3)` | tertiary text / captions |
| `--accent` | `#0d99ff` | accent — primary actions, selection, focus |
| `--accent-hi` | `#2aa7ff` | accent gradient top |
| `--accent-lo` | `#0b82dc` | accent gradient bottom / pressed |
| `--danger` | `#f24822` | destructive / error |
| `--ok` | `#14ae5c` | success |
| `--warn` | `#ffb800` | warning |
| `--g-geo` | `#0d99ff` | **blue = geometry guides** (selection, paths, bounds) |
| `--g-space` | `#ec2fd0` | **magenta = spacing guides** (distance, padding, distribution) |
| `--g-snap` | `#ff8a00` | **orange = snapping & measurement guides** |
| `--g-ruler` | `#f24822` | **red = ruler guides & layout grids — NEVER used for errors on canvas** |
| `--comp` | `#9747ff` | **purple = components/instances** |
| `--sel` | `rgba(13,153,255,.3)` | text-selection / range-selection fill (accent 30%) |
| `--menu` | `#fff` | menu surface |
| `--menu-t` | `rgba(0,0,0,.9)` | menu text |
| `--menu-sep` | `rgba(0,0,0,.1)` | menu separator |
| `--menu-bd` | `rgba(0,0,0,.08)` | menu border |
| `--tip` | `#1e1e1e` | tooltip & toast background |
| `--tb` | `#2c2c2c` | top toolbar background |
| `--frame` | `#fff` | design frame surface |
| `--shadow` | `0 0 .5px rgba(0,0,0,.3), 0 10px 16px rgba(0,0,0,.12), 0 2px 5px rgba(0,0,0,.15)` | elevation (3-layer: hairline + deep + tight) |
| `--ease` | `cubic-bezier(.2,0,0,1)` | standard easing |
| `--fast` | `80ms` | fast transition (hover) |

## 2. Color tokens — DARK theme (`[data-theme=dark]` / `prefers-color-scheme: dark`)

| Token | Dark value |
|---|---|
| `--bg-canvas` | `#1e1e1e` |
| `--bg-panel` | `#2c2c2c` |
| `--bg-input` | `#383838` |
| `--bg-hover` | `rgba(255,255,255,.1)` |
| `--bg-sel` | `#0a3a5e` |
| `--bd` | `rgba(255,255,255,.1)` |
| `--bd2` | `rgba(255,255,255,.25)` |
| `--t1` | `rgba(255,255,255,.9)` |
| `--t2` | `rgba(255,255,255,.55)` |
| `--t3` | `rgba(255,255,255,.3)` |
| `--frame` | `#2c2c2c` |
| `--menu` | `#1e1e1e` |
| `--menu-t` | `#fff` |
| `--menu-sep` | `rgba(255,255,255,.14)` |
| `--menu-bd` | `rgba(255,255,255,.12)` |
| `--menu-sh` | `0 0 .5px #000, 0 10px 16px rgba(0,0,0,.4), 0 2px 5px rgba(0,0,0,.5)` |

Unchanged in dark: accent (+hi/lo), danger, ok, warn, all four guide colors, `--comp`, `--sel`,
`--tip`, `--tb`. → **Guide/accent identity colors are theme-invariant.**

## 3. Typography

- Fonts: **Inter** (variable 400–600) for UI · **JetBrains Mono** (400/500) for code/numeric.
- Base body: `450 11px/16px Inter` with `-webkit-font-smoothing: antialiased`.
- Scale (from the Typography frame):
  - **Title:** `600 13px/20px`
  - **Body / property label & value:** `450 11px/16px`
  - **Caption:** `500 10px/16px`, letter-spacing `.02em`, color `--t3`
  - **Code/mono:** `500 10px/14px JetBrains Mono`, color `--t2`; numeric values use
    `font-variant-numeric: tabular-nums` (e.g. `X 128.00 · Y 64.00`)
  - Section headings (h6): `600 11px`
- Truncation: layer names = 1 line + `…` (ellipsis), nowrap.
- Guide/pill SVG text: `500 10px Inter`.

## 4. Application shell & layout metrics

- App grid: **40px top toolbar**; columns **240px layers panel / flexible canvas / 240px
  properties panel**.
- Top toolbar: bg `--tb` (#2c2c2c), white content, padding `0 8px`, gap 2px.
  - Logo: **24×24px, radius 6px, `conic-gradient(from 200deg, #0d99ff, #9747ff, #ec2fd0, #0d99ff)`**
    (blue→purple→magenta brand gradient).
  - Tool buttons: **32×32px, radius 5px**; hover `rgba(255,255,255,.12)`; active tool = solid
    `--accent` background; transition 80ms.
  - Toolbar separator: 1×16px `rgba(255,255,255,.2)`, margin `0 6px`.
  - File name: opacity .9 weight 500; "Gipulo /" prefix opacity .5 weight 450.
  - Share button: height 24px, padding `0 10px`, radius 5px, gradient `--accent-hi→--accent-lo`,
    `inset 0 0 0 .5px rgba(255,255,255,.35)` ring, weight 500.
- Panel tabs (Layers/Assets, Design/Prototype): height 40px, gap 16px, padding `0 16px`,
  border-bottom 1px `--bd`; inactive `--t2`, active `--t1`, weight 500.
- Panel section header: padding `12px 16px 4px`, weight 600. Properties section: padding
  `8px 16px 12px`, border-top 1px `--bd`; section h6 padding `4px 0 8px`.
- Responsive fallback (≤900px): single column, side panels hidden, canvas padding 24px.
- Safe-area insets respected (`env(safe-area-inset-*)`).

## 5. Global interaction defaults

- `::selection` → background `--sel`, color inherit (custom text selection — no browser blue).
- `:focus-visible` → **outline 2px `--accent`, offset 1px**.
- Buttons: `cursor: default` (cursors are part of the custom cursor system, never accidental).
- Scrollbars: **10px** wide/tall; thumb `--bd2`, radius 5px, 3px transparent border,
  `background-clip: content-box`; `scrollbar-width: thin`.
- Reduced motion (`prefers-reduced-motion: reduce`): **all transitions none**.

## 6. Icon system

- Grid: **16×16 viewBox**; stroke `currentColor`; **stroke-width 1.25**; round caps & joins;
  fill none (exceptions: `move` cursor-arrow icon and `comp` icon are solid fills).
- Symbol set present in reference: `move`, `frame`, `rect`, `pen`, `text`, `hand`, `eye`
  (visibility), `lock`, `chev` (chevron, rotated 90° = expanded), `check`, `plus`, `comp`
  (component: four solid diamonds), `inst` (instance: hollow diamond).
- Layer-row icons colored `--t2`; component/instance rows colored `--comp`; error rows `--danger`.

## 7. Buttons (height 24px · radius 5px · min-width 64px · padding 0 10px · weight 500 · gap 4px)

Variants × states matrix (all defined):
- **Primary:** white text on `linear-gradient(180deg, --accent-hi, --accent-lo)` + inset white
  .5px ring + `0 1px 1px rgba(0,0,0,.15)`. Hover: gradient `#4db8ff→--accent`. Pressed: solid
  `--accent-lo` + `inset 0 1px 2px rgba(0,0,0,.3)`.
  **Rule: gradient is reserved for the ONE primary action in a view.**
- **Secondary:** bg `--bg-panel`, border `--bd2`. Hover bg `--bg-hover`. Pressed bg `--bd` +
  inset `0 1px 1px rgba(0,0,0,.12)`.
- **Ghost:** transparent. Hover `--bg-hover`. Pressed `--bd`.
- **Icon button:** 24×24px, no min-width, padding 0.
- **Focus:** double ring `0 0 0 2px --bg-panel, 0 0 0 4px --accent`.
- **Disabled:** opacity .4 (all variants).
- Transitions: background 80ms, box-shadow 80ms.

## 8. Property fields & controls

- **Field:** height **28px**, bg `--bg-input`, border 1px transparent, radius 5px.
  Hover: border `--bd2`. Focus: border `--accent` + `inset 0 0 0 .5px --accent`. Invalid: border
  `--danger`. Disabled: opacity .4. Label prefix: 26px wide, centered, `--t2`, weight 500,
  `cursor: ew-resize` (scrubbable). Input text tabular-nums; **placeholder color = `--t1`**
  (intentional; "Mixed" placeholder is readable).
- **Segmented control:** bg `--bg-input`, radius 5px, padding 2px, gap 2px; segment height 22px,
  padding `0 10px`, radius 3px, color `--t2`; active segment: bg `--bg-panel`, color `--t1`,
  ring `0 0 0 .5px --bd2` + `0 1px 2px rgba(0,0,0,.12)`.
- **Toggle:** **28×16px**, radius 8; off bg `--bd2`; knob 12px white circle at 2px inset with
  `0 1px 1px` shadow; on: bg `--accent`, knob `translateX(12px)`, **120ms** `--ease`.
- **Checkbox:** 16×16, border 1px `--bd2`, radius 3; checked: bg+border `--accent`; check icon
  12px, stroke-width 1.75, white.
- **Radio:** 16×16, border 1px `--bd2`; checked = `border: 5px solid --accent`.
- **Slider:** track 2px (radius 1) — filled portion `--accent`, rest `--bd2`; knob 12px white,
  ring `0 0 0 .5px rgba(0,0,0,.2)` + `0 1px 3px rgba(0,0,0,.3)`.

## 9. Layer rows (height 28px · indent 16px/level)

- Structure: [chevron 16px, opacity .6] [type icon 16px `--t2`] [name flex, ellipsis] [actions].
  Padding-left = `8px + depth × 16px`.
- **Deep nesting:** indent 16px/level for levels 0–8; **beyond level 8: 8px per level**; vertical
  indent guide lines 1px `--bd` per level. Names truncate with `…`.
- States:
  - hover: bg `--bg-hover`; lock+eye actions fade in (opacity 0→1, 80ms) **on that row only**;
  - selected: bg `--bg-sel`;
  - selected parent's descendants: tinted `rgba(13,153,255,.08)`; hovered descendant
    `rgba(13,153,255,.18)`;
  - hidden: row opacity .45 (eye icon stays visible);
  - locked: lock icon stays visible without hover;
  - component/instance: icon+name `--comp` purple;
  - error (e.g. missing font): name+icon `--danger`;
  - renaming: name becomes 120px input, `outline 1px --accent`, radius 2px, padding `0 4px`,
    bg `--bg-panel`;
  - dragging: opacity .7 + `--shadow` + bg `--bg-panel`;
  - drop-inside frame: `inset 0 0 0 1.5px --accent` ring + bg `--bg-sel`;
  - insert-between indicator: **2px `--accent` line (radius 1) + 6px circle marker** (1.5px accent
    border, panel bg) at the insertion point.

## 10. Menus · tooltips · toasts · chips

- **Menu:** width **224px**, bg `--menu`, border 1px `--menu-bd`, **radius 13px**, padding 8px,
  shadow `--menu-sh`. Items: height **24px**, padding `0 8px 0 24px`, radius 5px; hover = bg
  `--accent` + white text; shortcut right-aligned opacity .5 (⌘C, ⌘⇧V, ⇧R, ⌘], ⌘⇧G, ⌫, ›);
  checked item = `✓` at left 8px; disabled opacity .4; destructive = `--danger` text, hover bg
  `--danger` + white; separator 1px `--menu-sep` full-bleed (margin `4px -8px`).
- **Tooltip:** bg `--tip` #1e1e1e, white, radius 5px, padding `5px 8px`, gap 8px; shortcut part
  opacity .5 (e.g. "Rectangle R").
- **Toast:** bg `--tip`, white, radius 8px, padding `8px 12px`, gap 8px, `--shadow`; icon+message
  (e.g. ✓ Published).
- **Chip/pill:** height 20px, padding `0 8px`, radius 10px, bg `--bg-input`; active: bg `--bg-sel`
  + `--accent` text; error: bg `#fde8e3` + `--danger` text.
- **Color swatch:** 16×16 radius 3 with `0 0 0 1px --bd` ring; palette chips 40px tall radius 5.

## 11. Canvas frame chrome (editor viewport objects)

- Frame title row: 20px tall, gap 4px, `--t2`; selected frame → title turns `--accent`.
- Frame body: bg `--frame`, padding 16px, column gap 12px; hover/selected → `outline 1px --accent`.
- Selection corner handles: **8×8px white with 1px `--accent` border**, centered on corners.
- Size pill below frame: bg `--accent`, white text, padding `1px 6px`, radius 3px, 10px font,
  tabular-nums, offset 6px below.

## 12. Canvas overlay system — universal zoom laws

- **All guide strokes stay 1px on screen at EVERY zoom.** Handles & labels stay constant screen
  size (positions follow geometry).
- **Labels hide below 25% zoom; handles hide below 12% zoom.**
- Pixel-crisp rule: 1px lines drawn at `x.5` offsets (never blurry 2px-at-40%-opacity lines).
- Contrast casing: on strongly colored/dark content, selected objects get a **white halo**
  (3px white stroke under the blue outline); handles stay white-filled. Dark canvas uses the same
  blue guides + white handles.
- Size pills flip above the object near viewport edges.

### 12.1 Blue geometry guides (`--g-geo` #0d99ff)
Spec: 1px stroke · handles **7px white squares with 1px stroke** · pill radius 3, text 10/16 ·
modes: object / frame-group / vector edit / text edit / text-on-path · **hover 60% opacity ·
selected 100% · parent context 40% dashed**.
States (from reference table):
- Hover: 1px outline, 100%, no handles.
- Selected: box + **8 handles** (corners+edges) + size pill (`60 × 36`).
- Multi-select: children outlines 60% + group box 100% with handles.
- Resizing: original ghost dashed `3 2` at 50% + live box + live size pill.
- Rotating: box follows angle + angle pill (`20°`); **angle snap tick marks every 15°**.
- Corner radius: 4 inner circle handles r3 + live pill (`R 10`).
- Locked: dashed `2 2`, 60% + lock badge (9×7px).
- Inactive: everything 35%, no hit areas.
- Text editing: box + **1px caret** + `--sel` selection fill.
- Component: purple `--comp` box/handles + purple name tab pill.
- Frame hover: outline + blue name label above; frame selected: name tab + box + pill; frame name
  hover: 15% accent chip behind name; frame renaming: white input box 1px stroke + caret; group
  selected: box only, NO name; child hover inside group: child outline 60%; deep-select (⌘):
  parent dashed `3 2` at 40%, child solid with handles; clipped child: outline continues past
  clip at 50% dashed `3 2`; mask: dashed `4 2` contour; component set: purple dashed `4 2` +
  purple name tab.
- Dragging object: ghost at origin dashed `3 2` + moving copy at 50% + delta pill (`Δ 6, 8`).
- Precision: **small object (<24px): only 4 corner handles** · **tiny (<8px): outline + center
  dot r1.5** · handles 7px visual with **16px hit areas** (shown as 16px dashed `2 2` 12% zones) ·
  rotate zone = arc outside corner (r≈14, dashed `2 2`, ↻ glyph).

### 12.2 Vector edit mode (blue)
- Enter mode: anchors as hollow 7px squares (white fill, blue stroke), no bounding box.
- Point hover: 9px square with light-blue fill `#b3dfff`.
- Point selected: solid blue fill; Bézier handles = **r3 circles** white/blue + 0.75px handle lines.
- Marquee: dashed `3 2` + 8% fill; touched points become selected.
- Points box: bounding box + 8 handles around several selected points.
- Segment hover: segment thickens to 2px (wait: reference shows 3px preview) + add-point ghost
  (circle r4 white + plus glyph).
- Pen preview: rubber-band dashed `3 2` at 50% + preview anchor at 60%.
- Close path: **ring r7 around first point**.
- Bend tool: pull handles out of a segment (creates two r3 handle circles).
- Mirroring: handles same length, independently draggable.
- Region hover (vector networks): closed region filled 12%.
- Lasso: dashed loop `3 2`, 6% fill; enclosed points selected.
- Path curve: 1.5px stroke for the path itself, 0.75px for handle lines.

### 12.3 Text edit & text-on-path (blue)
- Auto-width: box hugs text, 8 handles. Fixed-width: wraps, side handles emphasized. Auto-height:
  width handles only. Hover: 1px outline. Range select: `--sel` fill + 1px box. IME composition:
  **dashed underline `2 1` at 1.5px**. Overflow (fixed height): `…` pill tab (min 18px).
- Text on path (proposal): start/end square handles on the path; offset drag with **% pill**
  (`12%`); flip = small side toggle chip on path (`⇅`).

### 12.4 Magenta spacing guides (`--g-space` #ec2fd0)
Spec: 1px line · **region fill 14%** · end ticks 6px · pill radius 3 min-width 18px ·
smart selection: **ring r4.5 at each object center** (hollow = available, **solid fill = marked**) ·
**gap handle 4×16px radius 2** · requires 2+ layers with equal spacing (list or grid); "Tidy up"
creates the relationship.
States: gap (fill+line+pill) · padding (4 sides, pill per side) · equal distribution (bracket line
over equal gaps + pills) · margin-to-parent (4 lines to parent edges + pills) · label hover (ring
r6, 3px stroke, 40%) · editing value (white input box 1px magenta stroke + caret) · fixed (hollow
pill: panel bg + magenta stroke/text) · auto (dashed `3 2` + "Auto" pill) · preview (50% + dashed,
no label hit).
Smart-selection states: available (rings + pink line 60%) · hover (gap handles appear) · marked
(solid ring) · drag gap (ALL gaps change together + live pills, handle stretches 6px wide) ·
reorder (ghost 60% + **2px insertion line**) · resize marked (neighbors keep equal gap) ·
duplicate (mark → duplicate; ghost 50% dashed `3 2`) · delete marked (gap closes; ghost dashed
50%) · 2D grid (row AND column gap handles) · tidy up (messy → dashed target positions) ·
not available (unequal gaps → no pink shown, plain blue outlines).

### 12.5 Orange snapping & measurement (`--g-snap` #ff8a00)
Spec: **snap = 1px dashed `4 3` + × markers at snap points, active within 4px while dragging** ·
**measure = 1px solid + 6px end ticks + pixel pill (10/16 text, r3, min 18px)** · measure on
**hold ⌥** · nested measure **⌘⌥** · pinned measure **⇧M** (hollow pill, stays until Esc) ·
**measures bounds, not strokes** · multi-selection uses the group bounding box · values 0–2
decimals.
- Single-object drag snapping: edge (line spans both objects + × at both edges) · center (line
  through both centers) · equal gap (both gaps get measure lines + pills) · size match (value pill) ·
  parent edge (line on frame edge) · pixel grid (**cell highlight 35% fill + crosshair ×**) · text
  baseline (dashed line under baselines + ×) · ruler guide (**the ruler guide stays RED; the snap
  line is orange**).
- Multiple-object drag snapping: group box shown dashed `3 2` at 60% (blue); snap lines attach to
  group edge / group center / group gap-to-neighbor / child edge (line to a child, not the group).
- Measure (⌥): beside (selected blue → hovered orange outline; solid line + ticks + pill) ·
  above/below (vertical) · diagonal (two axes, x pill + y pill) · inside hovered (4 distances
  outward) · hovered inside (4 distances inward) · overlapping (edge pairs) · decimal (pill grows,
  max 2dp) · **tight gap (<16px): label lifts above with vertical leader** · **zero distance:
  dot r2.5 + lifted "0" pill** · group→object (from group bounds) · vector anchors (anchor→anchor,
  dx/dy pills) · nudge (⌥⇧↑: live value + step arrow).
- Lifecycle: approach = **40% dotted `2 3`** → snapped = **100% dashed `4 3`** → released =
  **fade out 120ms** · snap disabled (⌘ held) = guide at 20% + "⌘ held" chip · **zoom <25%:
  lines only, no labels** · label collision: stagger pills above/below · fractional values: pixel
  snap off, 2dp (e.g. `28.37`).

### 12.6 Red ruler guides & grids (`--g-ruler` #f24822)
Spec: 1px · **hit area 9px** · **never used for errors on canvas**.
States: default (1px full length) · hover (9px band at 15% + resize cursor ↔) · selected (**2px** +
position pill e.g. `128`) · dragging from ruler (ruler band `--bg-input` 8px + live pill `x 128`) ·
locked (dashed `4 2` at 80% + lock badge) · hidden (toggle **⌘;** — no hit area).
Layout grids: columns (e.g. 4 × 20px columns, 8px gutter, **12% fill**) · square grid (8px cells,
**25% opacity**).

## 13. Density & motion

- **Density levels (field/row heights): ultra-compact 20px · compact 24px · comfortable 28px.**
  Visual size and hit area are separate: **minimum hit target 24px**. Base grid **4px**.
- **Motion: hover 80ms · toggle 120ms · menu 160ms · ease `cubic-bezier(.2,0,0,1)` ·
  reduced-motion: none.** (Snap-guide release fade: 120ms.)

## 14. Token naming convention (from "Variables used" inspector)

Semantic slash-names used in the reference: `bg/canvas`, `bg/panel`, `bg/input`, `bg/selected`,
`accent/default`, `text/primary`, `text/secondary`, `danger`, `success`, `guide/geometry`,
`guide/spacing`, `guide/snap`, `font/ui` (Inter), `font/mono` (JetBrains Mono), `size/body`
(11/16), `control/height` (24px buttons / 28px fields), `radius/m` (5px), `radius/l` (13px menus),
`motion/hover` (80ms), `ease/standard` (.2,0,0,1), `menu/bg`, `menu/item` (24px), `field/bg`,
`border/focus` (--accent), `row/height` (28px), `row/indent` (16px), `density/compact` (24px),
`grid` (4px). → Use this naming style when formalizing the token layer.

## 15. Usage laws demonstrated by the reference

- Neutral UI everywhere; **accent reserved for primary actions, selection, and focus**.
- **Gradient reserved for the ONE primary action in a view.**
- Guide colors have FIXED meanings (blue geometry / magenta spacing / orange snap+measure /
  red rulers); red is never an error color on canvas; purple = components; error text = `--danger`.
- Menus/tooltips/toasts are dark-surfaced (`--tip`/themed `--menu`) regardless of theme.
- Everything composable from tokens — no magic numbers (this file IS the numeric source of truth).
