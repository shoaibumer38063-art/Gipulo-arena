# 06 — Components: Surfaces (menus · popovers · tooltips · dialogs · panels · rows · lists)

> Values `[DS]` from the HTML reference unless tagged. These are the repeated surfaces of the
> editor — layer rows and property rows are HOT components (`01` §6): cheap paint only.

## 1. Menus & context menus `[DS]`

- Surface: width **224px**, bg `menu/bg`, border 1px `menu/border`, radius **13px**, padding 8px,
  shadow `shadow/menu` (dark) / `shadow/elevation` (light).
- Items: height **24px**, padding `0 8px 0 24px` (icon/check gutter 24px), radius 5,
  `type/body`; shortcut right-aligned, same size, opacity .5 (e.g. ⌘C, ⌘⇧V, ⇧R, ⌘]).
- Item states: hover = bg `accent/default` + white text (menus keep the accent hover language);
  disabled opacity .4; destructive `danger` text, hover bg `danger` + white; checked = `check`
  icon at left 8px; submenu = `chev` rotated -90° at right, opens on hover/keyboard (150ms
  intent delay [PROPOSED], canvas §27).
- Separator: 1px `menu/sep`, full-bleed (margin `4px -8px`).
- Behavior: keyboard-first navigation (arrows, type-ahead [PROPOSED], Enter, Escape, Backspace
  closes submenu), viewport collision flips/shifts (never clipped), rollover scrolling at
  edges, search-in-menu when >12 items [PROPOSED].
- Context menus = same system: canvas right-click (node/selection/empty contexts), layer rows,
  property rows, tabs — feature docs define ITEMS, never styling.

## 2. Dropdowns & selects `[PROPOSED]` (no native `<select>` exists in Gipulo — `01` §3)

- Trigger: DS field geometry (`05` §3) with `chev` 16px right; selected value left.
- Menu: DS menu surface (`06` §1) with selected item check; search field inside when >12 items;
  keyboard: arrows + type-ahead + Enter; Escape restores focus to trigger.
- Specialized variants follow the same chrome: font-family dropdown (preview label in
  `type/body`), blend-mode (with live preview swatch), variable modes [future].

## 3. Popovers `[PROPOSED]`

- Surface: bg `menu/bg`, radius 13, border 1px `menu/border`, `shadow/elevation`, padding per
  content (8px default).
- Positioning: anchored, collision-aware (flip/shift), never covers its anchor, 8px gap.
- Dismissal: outside click, Escape; focus returns to invoker (`11` §3). No arrow pointers.
- Used by: color picker, effects editors, blend pickers, export settings, font pickers.

## 4. Tooltips `[DS]`

- Surface: bg `tip/surface` (#1e1e1e, theme-invariant), white `type/body`, radius 5, padding
  `5px 8px`, gap 8px; shortcut part opacity .5 ("Rectangle  R").
- Behavior: 400ms hover delay [PROPOSED, canvas §27] (0ms for already-visible tool zone),
  fade 80ms, follows collision rules; never shown for touch-only flows.
- Content: noun phrase; when a control has a visible label, tooltip adds the DIRECTION/context
  ("vertical padding") and must match the aria-label exactly `[UI3]` (`03` §6).

## 5. Toasts `[DS]`

- Surface: bg `tip/surface`, white, radius 8, padding `8px 12px`, gap 8px, `shadow/elevation`;
  icon + message (e.g. ✓ Published). Action link optional (`accent`-tinted text on dark).
- Lifecycle: enter/exit via `10` §4; auto-dismiss 3–4s [PROPOSED] (interactions), pause on
  hover; max 3 stacked, oldest pushed out `[PROPOSED]`; screen-reader announcement via live
  region (`11` §6); errors persist until dismissed [PROPOSED].

## 6. Tabs `[DS]`

- Height 40px, gap 16px, padding `0 16px`, border-bottom 1px `border/default`.
- Inactive `text/secondary` weight 500; active `text/primary` weight 500; active indicator:
  none beyond color in reference — [PROPOSED]: 2px `accent` underline on the active tab sits
  on the container border (consistent with focus underline pattern).
- Hover: `text/primary`. Disabled: .4. Keyboard: arrows cycle tabs (ARIA tablist).

## 7. Panels & sections `[DS]`

- Panel surface bg `bg/panel`; tabs at top (§6); section header: padding `12px 16px 4px`,
  weight 600 (`type/section`).
- Property section: padding `8px 16px 12px`, border-top 1px `border/default`; section heading
  (h6) padding `4px 0 8px`.
- Collapsible sections: `chev` rotated (90° expanded) in the header, whole header row is the
  hit area; collapse motion per `10` §4.
- Panel resize: 1px visual splitter, 8px hit band `[PROPOSED]`, hover/drag shows `accent`;
  constraints in `12` §2.
- Panel scroll: DS scrollbars (§12), only the content scrolls (headers/actions pinned).

## 8. Property rows `[DS]` + states

- Row height 28px; label left (`type/body`, `text/secondary` for label column in pair layouts),
  value control right (DS field/segmented/dropdown as appropriate).
- Row states: default · hover (row bg `bg/hover` in list-like panels) · focus (control ring) ·
  editing · invalid (`danger` border + message) · mixed ("Mixed" placeholder) · disabled (.4) ·
  inherited/overridden indicators (`05` §3) · selected (multi-select operations) `[PROPOSED
  for row-level selection]`.
- Adaptivity law (`05` brief): the properties panel shows DIFFERENT rows per selection type
  (nothing/shape/frame/group/text/vector/component/instance/image/multi) — feature docs define
  contents; the DS defines row anatomy + states. Never show a dead row for the wrong type.

## 9. Layer rows (the hottest component) `[DS]`

- Height **28px**; structure: [chevron 16px, opacity .6] [type icon 16px `text/secondary`]
  [name `type/body`, ellipsis] [lock + eye actions].
- Padding-left = `8px + depth × 16px`; **beyond level 8: 8px per level**; vertical indent guide
  lines 1px `border/default` per level.

| State | Treatment |
|---|---|
| Hover | bg `bg/hover`; lock+eye fade in (0→1, 80ms) on that row ONLY |
| Selected | bg `bg/selected` |
| Descendant of selected parent | tint `rgba(13,153,255,.08)`; hover `.18` |
| Hidden | row opacity .45 (eye stays visible) |
| Locked | lock icon stays visible without hover |
| Component/Instance | icon + name `component` purple |
| Error (missing font etc.) | name + icon `danger` |
| Renaming | name → 120px input, `outline 1px accent`, radius 2, padding `0 4px`, bg `bg/panel` |
| Dragging | opacity .7 + `shadow/elevation` + bg `bg/panel` |
| Drop-inside frame/group | `inset 0 0 0 1.5px accent` ring + bg `bg/selected` |
| Insert-between | **2px `accent` line (radius 1) + 6px circle marker** (1.5px accent border, panel bg) |
| Multi-selected | all selected rows `bg/selected`; ancestor tint suppressed for selected set |
| Warning | small `warn` badge at right [PROPOSED] (`danger` styling reserved for errors) |
- Banned: per-row blur/shadows/gradients, hover effects affecting neighbor rows, layout shift on
  action buttons (they fade into reserved space).

## 10. Lists & trees

- Tree = layer row system at any depth (§9) with disclosure `chev`s and indent guides.
- Generic lists (fonts, presets, assets): row height per density (`02` §10), hover `bg/hover`,
  selected `bg/selected`, keyboard navigation (arrows + type-ahead), virtualization mandatory
  above ~200 rows (performance law `01` §6) — with NO visual difference when virtualized.

## 11. Chips & badges `[DS]`

- Chip/pill: height 20px, padding `0 8px`, radius 10px, bg `bg/input`; active: bg `bg/selected`
  + `accent` text; error: bg `#fde8e3` + `danger` text (light) / themed `[PROPOSED]` dark pair.
- Count badge `[PROPOSED]`: min 16px height, radius 8, `bg/input` + `text/secondary`, tabular.
- Badges never carry meaning by color alone (icon or text accompanies — `11` §7).

## 12. Scrollbars `[DS]`

- 10px wide/tall; thumb `border/strong`, radius 5, 3px transparent border,
  `background-clip: content-box`; `scrollbar-width: thin`; overlay behavior (no layout shift
  when appearing) `[PROPOSED]`; track transparent; hover darkens thumb `[PROPOSED]`.

## 13. Empty, error, loading states `[DS scope]` + `[PROPOSED]` details

- Empty state: 24px `[PROPOSED]` icon at `text/tertiary`, one-line statement `type/body`,
  optional ONE primary action button (§1 gradient law applies per view).
- Error state: `danger` icon + plain-language message + a recovery action where one exists
  (`11` §7); never raw error codes as copy.
- Loading: for <100ms operations show nothing (avoid flash); otherwise spinner (12px, in-place)
  or skeleton rows (28px rhythm, `bg/hover` shimmer at 80ms `[PROPOSED]`); layout never shifts.

## 14. Future-gated surfaces (specified at requirement level only — feature phases design them)

- **Comments/presence** `[UI3 research]`: comment pins on canvas (geometry owned by the future
  feature doc; chrome follows guide/handle laws), avatar stack (24px circles, `border/default`
  ring) in toolbar; cursor labels for multiplayer — deferred with collaboration.
- **Version history**: timeline panel; row anatomy = layer-row class; restore dialog = §6/§7
  dialogs.
- **Command palette** `[PROPOSED, researched]`: 480px wide top-centered menu-class surface,
  DS search field, result rows (icon + label + shortcut), keyboard-first; used for all
  searchable actions.
- **Code Workspace surfaces** (Code + Preview): DS tabs, panels, scrollbars, code typography
  (`03` §7), error surfaces = §13; designed with that feature phase per
  `docs/architecture/code-generation.md` §5.
