# 12 — Responsive Editor UI

> Requirements: Design System Brief §Responsive Application UI — the editor must NOT compress
> everything until controls become unusable. Reference fallback `[DS]`: **≤900px → single
> column, side panels hidden, canvas padding 24px**. Extended breakpoints `[PROPOSED]`.
> Research note `[UI3]`: Figma's UI3 kept panels DOCKED (not floating) because floating slowed
> people down, made panels resizable, and added **Minimize UI** for small/split-screen work —
> adopted below.

## 1. Breakpoints (editor window width)

| Band | Width | Behavior |
|---|---|---|
| Wide | ≥1440 | panels 240 + canvas breathing room; floating controls full labels |
| Standard | 1200–1439 | default target: layers 240 · canvas · properties 240 |
| Laptop | 1000–1199 | panels stay (they are already compact); toolbar file name truncates; floating toolbar compacts to icons |
| Small / split-screen | 900–999 | **Minimize UI available** (both panels collapsible via ⌘\⌘ style shortcuts); one panel may dock at a time `[PROPOSED]` |
| Narrow | ≤900 `[DS]` | **single column: side panels hidden, canvas padding 24px**; panel access via overlay (slide-in panels) `[PROPOSED]` |
| Minimal | ≤640 `[PROPOSED]` | canvas-first read/view mode; editing surfaces via overlays; export/deep controls deferred to dialogs |

## 2. Panel behavior

- Panels: docked by default (never floating-by-default), resizable splitter 1px visual / 8px
  hit (`06` §7), range **200–480px** `[PROPOSED]`, snap to 240 default on double-click
  splitter.
- Collapse: panel header collapse control + keyboard; collapsing hides the panel and leaves a
  restore affordance on the edge (a 16px chip with the panel icon `[PROPOSED]`).
- **Minimize UI** `[UI3]`: one action collapses BOTH panels for distraction-free work; floating
  corner chips restore either panel; Escape exits Minimize UI `[PROPOSED]`.
- Minimum widths are protected: layers ≥200, properties ≥200, canvas ≥320 even in worst case;
  below that, panels overlay the canvas instead of squeezing it.

## 3. Label → icon folding (when names disappear)

- Toolbar: ALWAYS icon-only + tooltips (`04`, `06` §4) — file name is the only text, truncates
  with ellipsis, then hides below 900px.
- Floating contextual toolbar: full labels ≥1200; icons + tooltips below `[PROPOSED]`.
- Properties rows: labels NEVER fold to icons (they are the meaning; typography is already
  11px) — instead the *panel* narrows to 200 min and rows keep one-line values.
- Menu items: never fold; menus are 224px fixed (`06` §1).

## 4. Floating & contextual controls

- Contextual toolbars anchor to the selection, clamp inside the viewport (8px margins), and
  flip below when space is short — never clipped, never cover the selection's own handles.
- Canvas page controls (zoom control, rulers corner) clamp to corners with safe-area insets
  (`env(safe-area-inset-*)` `[DS]`).
- Bottom-right zoom/status cluster compacts below 1000px (zoom % only) `[PROPOSED]`.

## 5. Dialogs & overlays at small sizes

- Dialog max-width = `min(480px, 90vw)` `[PROPOSED]`; menus/popovers collision-flip
  (`06` §1/§3); tooltips prefer above, flip below.
- Overlay panels (narrow mode): slide-in from edge, 240px, scrim `rgba(0,0,0,.4)` `[PROPOSED]`,
  focus trapped while open.

## 6. Scrolling rules

- Panels: vertical scroll only (virtualized where long — `06` §10); no horizontal scrolling in
  panels, ever.
- Canvas: infinite pan/zoom — scrolling is the canvas, not a container.
- Code workspace: horizontal scroll IS acceptable (code lines) — the only surface where it is
  the designed behavior; split view stacks vertically below 1200px `[PROPOSED]`
  (canvas on top, Code/Preview below).

## 7. Scale, zoom, and text size

- Browser zoom 80%–200%: layouts hold (grid/no fixed pixel traps), no clipped labels, no
  overlapping chrome (`11` §7).
- OS text scaling: typography tokens are px-based by design (editor-density product) — at high
  browser zoom the UI scales uniformly; we do not fight it with viewport tricks.
- The layout never depends on hover to be usable at small sizes (`11` §4).

## 8. Platform & input affordances

- Windows/Linux/macOS: shortcuts display as ⌘ on macOS else Ctrl (`rules/interactions.md` I1
  platform mapping); menus adapt labels, never positions.
- Safe-area insets respected on tablet-class devices `[DS]`.
- Split-screen monitoring (common in remote/hybrid work) is the design target for the 900–999
  band `[UI3 rationale]`.
