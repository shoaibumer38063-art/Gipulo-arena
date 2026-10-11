# 04 — Icon System

> Requirements: Design System Brief §Icon System (one complete language — agents must never mix
> packs per feature) + `rules/icons.md` IC1–IC6 (binding). Spec values `[DS]` from the HTML
> reference; the expanded inventory is `[PROPOSED]` (authored to cover the whole editor).

## 1. The spec `[DS]`

- Grid: **16×16 viewBox**; single line language; `stroke: currentColor`; **stroke-width 1.25**;
  round caps and round joins; `fill: none`.
- Solid-fill exceptions (exactly two): `move` (cursor-arrow) and `comp` (four diamonds) `[DS]`.
- Optical sizing: icons are drawn on the 16 grid but visually compensated (a 16px icon may have
  1–2px internal padding for equal perceived weight); centering is optical, not just
  mathematical (`01` §7).
- Corner treatment: rounded (matches stroke caps); no sharp corners anywhere.

## 2. Sizes per surface `[DS]` + `[PROPOSED]` extensions

| Size | Surface |
|---|---|
| 16 (default) | toolbar buttons, panel rows, layer rows, menu items, property icons, chevrons |
| 12 | checkbox/radio check glyph, tiny status badges |
| 14 | dense (ultra-compact) rows `[PROPOSED]` |
| 20 | empty-state illustrations, toolbar emphasis contexts `[PROPOSED]` |
| 24 | onboarding/empty surfaces `[PROPOSED]` |

Derivation law: sizes are DERIVED from the 16 master (stroke scales proportionally; a 12px icon
is not a different glyph). Never ship two hand-drawn variants of one icon (`rules/icons.md` IC4).

## 3. Color & states (via tokens, never glyph swaps)

| State | Treatment |
|---|---|
| Default (panel/toolbar rows) | `text/secondary` `[DS]` |
| Hover | `text/primary` (with the row's hover bg) |
| Active/selected tool | white (on accent toolbar button) `[DS]` |
| Disabled | inherits parent opacity .4 (`02` §7) |
| Component/instance rows | `component` purple `[DS]` |
| Error state rows | `danger` `[DS]` |
| Destructive menu items | `danger` text — icon inherits |
| On tinted/colored bg | white or context foreground — never a third tint |

Law: states change COLOR only. A glyph never swaps to indicate a state (`rules/icons.md` IC5);
the two exceptions are semantic pairs below.

## 4. Semantic pairs (allowed variants — not states)

`eye` / `eye-off` (visibility) · `lock` / `lock-open` · `chev` (rotates 90° for expanded —
one glyph, rotation not redraw `[DS]`) · `check`/`cross` (status) · `link`/`unlink`.
These are distinct MEANINGS, not visual states.

## 5. Inventory — reference set `[DS]`

`move` · `frame` · `rect` · `pen` · `text` · `hand` · `eye` · `lock` · `chev` · `check` ·
`plus` · `comp` (four solid diamonds) · `inst` (hollow diamond).

## 6. Inventory — required editor set `[PROPOSED]` (ships WITH features, not in advance)

| Surface | Icons |
|---|---|
| Tools | `move` `frame` `rect` `ellipse` `line` `polygon` `star` `pen` `pencil` `text` `hand` `comment` |
| Node types | `frame` `group` `section` `rect` `ellipse` `polygon` `star` `line` `vector` `text` `image` `video` `boolean-union` `boolean-subtract` `boolean-intersect` `boolean-exclude` `comp` `inst` `mask` |
| Panels/actions | `chev` `plus` `minus` `search` `filter` `sort` `eye` `eye-off` `lock` `lock-open` `more` `settings` `collapse-left` `collapse-right` `panel-left` `panel-right` `history` `grid-list` `adjust` |
| Properties | `link` `unlink` `flip-h` `flip-v` `align-left` `align-hcenter` `align-right` `align-top` `align-vcenter` `align-bottom` `distribute-h` `distribute-v` `constr-left` `constr-right` `constr-top` `constr-bottom` `constr-center` `constr-scale` `layout-h` `layout-v` `layout-wrap` `layout-grid` `radius` `effects` `blur` `shadow` `blend` `opacity` `fill` `stroke` `swap` `font` `align-text-left` `align-text-center` `align-text-right` `list-bullet` `list-number` |
| Status | `check` `cross` `warn` `info` `spinner` `forbidden` |
| Canvas chrome | `rotate` `bend` `lasso` `close-path` `ruler` `grid-square` `grid-columns` `snap` `measure` `eyedropper` `zoom-in` `zoom-out` `zoom-fit` `zoom-selection` |

Layer-type mapping for the layer panel (rows use node-type icons; components purple — `06` §9).

## 7. Discipline laws (binding)

1. **One sprite set, by ID** — no copy-pasted inline SVGs, no external icon packs without an ADR
   + full restyling to this spec (`rules/icons.md` IC3).
2. Icons are `aria-hidden` when accompanied by visible text; icon-only controls MUST carry an
   `aria-label` (`rules/icons.md` IC6, `11` §5).
3. Adding an icon = adding it to this inventory + the sprite in the same task; unused icons are
   removed (no speculative sprite bloat).
4. No per-feature "temporary" icons. If it ships, it follows §1.
5. Optical pass before merge: 16px, 12px, and on dark + light backgrounds.
