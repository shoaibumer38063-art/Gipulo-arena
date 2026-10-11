# 10 — Motion & Interaction Design

> Requirements: Design System Brief §Motion (small interface motion only; must feel fast —
> never make the editor feel slow) + `rules/interactions.md` I8 (binding timings). Values `[DS]`
> from the reference: hover **80ms** · toggle **120ms** · menu **160ms** · `ease/standard`
> `cubic-bezier(.2,0,0,1)` · reduced motion ⇒ none.

## 1. Token set (nothing else exists — `02` §8)

| Token | Value | Owns |
|---|---|---|
| `motion/hover` | 80ms | hover/press color & background changes, row action fades, tooltip fade |
| `motion/toggle` | 120ms | toggles, checkbox checks, snap-guide release fade, small state morphs |
| `motion/menu` | 160ms | menus, popovers, dialogs, panel collapse, toast enter |
| `ease/standard` | `cubic-bezier(.2,0,0,1)` | everything (single easing — no easing zoo) |
| `motion/none` | 0ms | geometry, dragging, cursor response, reduced-motion |

## 2. The five motion laws

1. **Feedback, not decoration.** Motion exists to confirm an action or preserve context. If an
   animation cannot name what it communicates, it is deleted.
2. **Nothing exceeds 160ms.** Anything slower reads as lag in an editor. (Long-running work is
   a loading STATE (`06` §13), not an animation.)
3. **Motion never moves the work.** Selection geometry, drags, guides, and canvas content are
   DIRECT — zero animation (the pointer→pixel loop owns them).
4. **One easing.** `ease/standard` for all transitions. Springs `[PROPOSED: not used in v1]`.
5. **Reduced motion is a full stop.** `prefers-reduced-motion: reduce` ⇒ every transition
   `none` `[DS]`; skeletons become static; preset zooms jump instantly.

## 3. Pattern table (complete for v1 surfaces)

| Pattern | Motion |
|---|---|
| Hover (rows, buttons, controls) | background/color 80ms |
| Button press / release | pressed state instant; release 80ms |
| Toggle / checkbox / radio | 120ms (knob translate / check draw) |
| Segmented selection | active-pill transition `[PROPOSED]`: none v1 — instant (avoids misalignment; state change is 80ms color) |
| Menu open | 160ms: opacity 0→1 + translateY(-4px)→0 |
| Menu close | 96ms `[PROPOSED]` opacity only (fast exit) |
| Submenu open | 160ms after 150ms intent delay (canvas §27) |
| Popover open | 160ms opacity + 4px shift toward anchor |
| Tooltip | appears after 400ms delay `[PROPOSED]`, fade 80ms; instant re-show within same target |
| Toast in/out | in 160ms (fade + 8px rise), out 120ms fade; auto 3–4s `[PROPOSED]` |
| Dialog | 160ms fade; content itself does not scale `[PROPOSED]` (no bouncy modals) |
| Panel collapse/expand | 160ms height/opacity `[PROPOSED: height on cold surface only]` |
| Layer action fade-in (lock/eye) | 80ms opacity, that row only `[DS]` |
| Drag start | none (direct); dragged row gets `.7` opacity + shadow instantly `[DS]` |
| Drop / insert indicators | none (state change is the feedback) |
| Snap guide lifecycle | approach 40% dotted → snapped 100% dashed (instant) → release fade 120ms `[DS]` |
| Zoom: wheel/pinch | none — direct, cursor-anchored |
| Zoom: presets (fit, 100%, selection) | 120ms ease `[PROPOSED]`; none under reduced motion |
| Skeleton shimmer | 1.2s loop `[PROPOSED]`; static under reduced motion |
| Scroll | native behavior; smooth only for programmatic jumps (zoom-to-selection reveal) |

## 4. When NOT to animate (binding list)

- Any geometry under manipulation: move/resize/rotate, path editing, guide dragging.
- Cursor changes and hit feedback.
- Snapping transitions (beyond the spec'd release fade) — snapping must be instant.
- Any transition on a HOT component beyond color/opacity (no height, no shadow, no blur —
  `01` §6, `07` §7).
- Anything while a pointer gesture is active (pointerup commits; the UI never "settles" after).

## 5. Performance rules for motion

- Animate only `transform`, `opacity`, `background-color`, `color`, `border-color` on hot paths.
- Never animate `width/height/top/left` on hot components (cold panels only, `[PROPOSED]`
  height on collapse is the single allowed exception).
- Never animate `filter`/`box-shadow`/`backdrop-filter` on hot components at all.
- Motion must not cause layout shift; animated elements are composited or cheap-painted.

## 6. Division of ownership (architecture reminder)

Timings are TOKENS: the shell uses them directly; the Rust core receives them via theme push
(`engine-core.md` §2) so canvas chrome (menu-like surfaces inside canvas? none — canvas has no
menus; but pill fades, snap release, selection chrome) uses the SAME numbers. One motion
language across both renderers — no core-side "feel" divergence.
