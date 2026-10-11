# 02 — Token Layer (the numeric source of truth)

> Every value here is `[DS]` (extracted from the Design System HTML) unless tagged otherwise.
> Code form: CSS custom properties in the shell + theme push into the Rust core for canvas
> chrome (`docs/architecture/engine-core.md` §2). **No consumer may inline a raw value** —
> `rules/ui.md` UI1. Token names follow the reference's semantic naming style (`07` §14).

## 1. Color — semantic groups

### 1.1 Surface tokens

| Token | Light `[DS]` | Dark `[DS]` | Used for |
|---|---|---|---|
| `bg/canvas` | `#f5f5f5` | `#1e1e1e` | editor canvas backdrop |
| `bg/panel` | `#fff` | `#2c2c2c` | panels, raised surfaces, dragged rows |
| `bg/input` | `#f5f5f5` | `#383838` | fields, segmented backgrounds, ruler band |
| `bg/hover` | `rgba(0,0,0,.06)` | `rgba(255,255,255,.1)` | generic hover |
| `bg/selected` | `#e5f4ff` | `#0a3a5e` | selected row |
| `frame/surface` | `#fff` | `#2c2c2c` | design frame fill |
| `toolbar/bg` | `#2c2c2c` (theme-invariant) | same | top toolbar ribbon |
| `tip/surface` | `#1e1e1e` (theme-invariant) | same | tooltips + toasts |
| `menu/bg` | `#fff` | `#1e1e1e` | menus, popovers |
| `menu/text` | `rgba(0,0,0,.9)` | `#fff` | menu text |
| `menu/sep` | `rgba(0,0,0,.1)` | `rgba(255,255,255,.14)` | menu separators |
| `menu/border` | `rgba(0,0,0,.08)` | `rgba(255,255,255,.12)` | menu edge |

### 1.2 Text tokens

| Token | Light | Dark | Used for |
|---|---|---|---|
| `text/primary` | `rgba(0,0,0,.9)` | `rgba(255,255,255,.9)` | primary text |
| `text/secondary` | `rgba(0,0,0,.5)` | `rgba(255,255,255,.55)` | secondary text, icons, code |
| `text/tertiary` | `rgba(0,0,0,.3)` | `rgba(255,255,255,.3)` | captions, hints |

### 1.3 Border tokens

| Token | Light | Dark | Used for |
|---|---|---|---|
| `border/default` | `rgba(0,0,0,.1)` | `rgba(255,255,255,.1)` | separators, standard borders |
| `border/strong` | `rgba(0,0,0,.22)` | `rgba(255,255,255,.25)` | hover/focus edges, slider rest, thumbs, scrollbar thumb |
| `border/focus` | `--accent` | same | focus edges (`05`) |

### 1.4 Identity + status tokens (theme-invariant unless noted)

| Token | Value | Meaning (fixed) |
|---|---|---|
| `accent/default` | `#0d99ff` | primary actions, selection, focus, active tool |
| `accent/hi` / `accent/lo` | `#2aa7ff` / `#0b82dc` | gradient endpoints (primary buttons); pressed = lo |
| `success` | `#14ae5c` | success status (also RVS-activated recommendation — `08` §8) |
| `warning` | `#ffb800` | warning status |
| `danger` | `#f24822` | destructive/error (never a canvas guide except ruler red below) |
| `component` | `#9747ff` | components/instances (purple) |
| `selection/fill` | `rgba(13,153,255,.3)` | text/range selection fill |
| `guide/geometry` | `#0d99ff` | blue — geometry overlays (`08` §2) |
| `guide/spacing` | `#ec2fd0` | magenta — spacing/layout relationships (`08` §4) |
| `guide/snap` | `#ff8a00` | orange — snapping + measurement (`08` §5) |
| `guide/ruler` | `#f24822` | red — ruler guides & layout grids ONLY on canvas (`08` §6) |
| `brand/gradient` | `conic-gradient(from 200deg, #0d99ff, #9747ff, #ec2fd0, #0d99ff)` | logo mark only |

## 2. Typography tokens

Fonts: `font/ui` = **Inter** variable (400–600) · `font/mono` = **JetBrains Mono** (400/500).

| Token | Spec | Used for |
|---|---|---|
| `type/title` | `600 13px/20px` | panel titles, dialog titles |
| `type/body` | `450 11px/16px` | default UI text, property label + value |
| `type/section` | `600 11px` | section headings (h6) |
| `type/caption` | `500 10px/16px`, `ls .02em`, `text/tertiary` | captions, frame titles |
| `type/code` | `500 10px/14px` mono | code, hex values, numeric pills when mono-styled |
| `type/pill` | `500 10px` | canvas pills/labels (Inter) |

Rules: all UI text antialiased; numeric values always tabular (`03` §4). Full usage law:
`03_typography.md`.

## 3. Spacing scale (4px base grid `[DS]`)

| Token | Value | Typical use |
|---|---|---|
| `space/1` | 2px | toolbar gaps, segmented gaps, micro insets |
| `space/2` | 4px | icon-text gaps, tight paddings |
| `space/3` | 6px | toolbar separators margins (0 6px), small stacks |
| `space/4` | 8px | button padding, menu padding, tooltip padding |
| `space/5` | 12px | panel section paddings, frame inner padding |
| `space/6` | 16px | panel horizontal padding, tabs padding, row indent step |
| `space/7` | 24px | canvas page margin (single-column fallback) |
| `space/8` | 32px | large empty-state padding `[PROPOSED]` |

Composition law: spacing values are selected from this scale — never arbitrary px.

## 4. Radii `[DS]`

| Token | Value | Applied to |
|---|---|---|
| `radius/xs` | 2px | renaming input |
| `radius/s` | 3px | chips/segments, checkboxes, canvas pills, swatches, logos' inner elements |
| `radius/m` | 5px | controls: buttons, fields, toggles' track uses 8, menu items, tooltips |
| `radius/logo` | 6px | brand mark |
| `radius/l` | 13px | menus |
| `radius/toast` | 8px | toasts, toggles (track) |
| `radius/pill` | 10px (chips) · full for capsules | chips/pills in panels |

## 5. Borders & strokes `[DS]`

| Token | Value | Applied to |
|---|---|---|
| `stroke/hair` | `.5px` | inset rings (buttons, focused fields), swatch rings |
| `stroke/default` | `1px` | all chrome borders, canvas guide strokes (any zoom), separators |
| `stroke/strong` | `2px` | focus rings, selected ruler guide, insertion lines |
| `stroke/canvas-path` | `1.5px` | vector path stroke in edit mode |
| `stroke/handle-line` | `.75px` | Bézier handle lines |
| `stroke/halo` | `3px` white | contrast casing under selection outline (`08` §7) |

## 6. Elevation `[DS]`

| Token | Value | Applied to |
|---|---|---|
| `shadow/elevation` | `0 0 .5px rgba(0,0,0,.3), 0 10px 16px rgba(0,0,0,.12), 0 2px 5px rgba(0,0,0,.15)` | floating elements, dragged rows, toasts |
| `shadow/menu` (dark theme) | `0 0 .5px #000, 0 10px 16px rgba(0,0,0,.4), 0 2px 5px rgba(0,0,0,.5)` | menus in dark |
| `shadow/button` | `0 1px 1px rgba(0,0,0,.15)` | primary/secondary buttons |
| `shadow/knob` | `0 1px 3px rgba(0,0,0,.3)` + ring `.5px rgba(0,0,0,.2)` | slider knob |
| `shadow/ring-active` | `0 0 0 .5px border/strong + 0 1px 2px rgba(0,0,0,.12)` | active segment |
| Rule | 3-layer max (hairline + deep + tight); never on hot list rows (see `01` §6) |

## 7. Opacity laws `[DS]`

| Value | Meaning |
|---|---|
| `.4` | disabled (all variants) |
| `.45` | hidden layer row |
| `.6` | hover guide stroke / parent descendant outline / inactive lock dash |
| `.5` | shortcut text, drag ghost base, vector pen preview |
| `.35` | inactive selection (no hit areas) |
| `.3` | tertiary text (tokenized separately) |
| `.08/.12/.14/.18/.25/.35` | canvas fills/overlays — see `08` (contextual, not ad hoc) |

## 8. Motion tokens `[DS]`

`motion/hover` 80ms · `motion/toggle` 120ms · `motion/menu` 160ms · `motion/snap-release`
120ms · `ease/standard` `cubic-bezier(.2,0,0,1)` · reduced-motion ⇒ all transitions none.
Full pattern table: `10_motion.md`.

## 9. Z-layers `[PROPOSED]` (authored; the reference implies ordering, not numbers)

| Layer | z | Notes |
|---|---|---|
| document content | 0 | inside core renderer |
| canvas overlays (guides, handles) | 100 | core overlay pass |
| canvas pills/labels | 200 | above overlays |
| floating toolbars (contextual) | 300 | shell |
| panels (docked) | 400 | shell |
| popovers/dropdowns | 500 | shell |
| menus/context menus | 600 | shell |
| dialogs/modals | 700 | shell |
| toasts | 800 | shell |
| tooltips | 900 | always topmost chrome |

Law: layers never overlap-contextually (a tooltip inside a dialog is still 900); no feature
invents a z-index outside this scale (canvas content is NEVER in this stack — it renders in the
core pass).

## 10. Density, control metrics, icon sizes `[DS]`

| Class | Values |
|---|---|
| Density levels (row/field heights) | ultra-compact 20px · compact 24px · comfortable 28px (defaults: fields & rows 28px, menus 24px, layer rows 28px) |
| Minimum hit target | **24px** — visual size and hit area are SEPARATE (a 20px visual may still have a 24px hit area) |
| Control heights | buttons 24 · icon buttons 24×24 · fields 28 · toggle 28×16 · checkbox/radio 16×16 · menu items 24 · tabs 40 · toolbar 40 · logo 24×24 |
| Panel widths | layers 240 · properties 240 · [PROPOSED] resizable 200–480 (`12` §2) |
| Icon sizes | 12 (checkbox check, small badges) · 14 (dense rows) · **16 (default)** · 20 (toolbar emphasis `[PROPOSED]`) · 24 (empty states `[PROPOSED]`) |
| Base grid | 4px |

## 11. Tokenization in code (next phase — the plan)

1. `app/src/design-system/tokens.css` — CSS custom properties, exact names above, light + dark
   (`[data-theme]` + `prefers-color-scheme`), plus the reset law (custom selection, focus rings,
   scrollbars — `01` §3 checklist).
2. `app/src/design-system/tokens.ts` — typed token access for TS code (no stringly-typed CSS
   vars).
3. Theme push into the core: the shell sends the active token set to `gip-session`
   (`set_theme_tokens`) so canvas chrome (guides, handles, pills) draws with the same values —
   single source, two renderers (`engine-core.md` §2).
4. Future theming (beyond light/dark) must be possible without touching components — that is
   the test of this layer.
