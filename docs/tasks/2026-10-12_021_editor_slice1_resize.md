# Task 021 — Editor Slice 1: resize with handles (DONE locally)

- **Plan:** `2026-10-12_021_editor_slice1_resize_plan.md` (decisions D1–D3 answered: clamp, 12 px clickable square around the 8 px visual, protocol bump).
- **Status:** implemented and committed locally. Not pushed in this turn.

## What shipped

- **Eight handles** on the single selected, unrotated, unflipped, visible, unlocked object: corners and
  edge midpoints (`Handle`, `handle_points`). Drawn at `--handle-size` (8 px, new token) with the
  fill and accent colors from tokens.
- **Corner and edge semantics** (CAN-TR-3): a corner moves two edges, an edge moves one, the
  opposite edge stays fixed (`resize_box`).
- **Shift** keeps the start aspect ratio on corners. **Alt** resizes from the center.
- **Clamp at zero** (D1): a drag past the opposite edge stops at zero size, never mirrors.
- **Start-value math** (CAN-TR-2): each move computes the box from the start box and the total
  delta, so repeated resizing does not drift.
- **One history entry per resize** (`Commit::Resize`, writes X, Y, W, H). Escape or pointercancel
  restores the start box exactly and writes nothing.
- **Handle activation** is a 12 px square (`HANDLE_SLOP_PX` = 6 px half-size), checked before the
  body hit. A press on a handle does not change the selection.
- **Size readout** (`size-readout`): the selected object's W × H in world units.
- **Boundary**: protocol 3 → 4. New methods: `handles()`, `selection_bounds()`, and `pointer_move`
  now takes `shift` and `alt`.

## Acceptance coverage (plan AC-29..40)

| AC | Status | Evidence |
|---|---|---|
| AC-29 | covered | `handles()` returns 8 points; drawn with tokens (`paintHandles`) |
| AC-30 | covered | corner, edge, and NW tests (`session.rs` `resize_tests`) |
| AC-31 | covered | Shift test: aspect exact; E2E Shift test (120×60 → 180×90) |
| AC-32 | covered | Alt test: center stays fixed |
| AC-33 | covered | clamp test: width 0, left edge fixed |
| AC-34 | covered | one-entry and undo test; E2E undo |
| AC-35 | covered | Escape test: draw list bitwise equal; no entry. E2E Escape. |
| AC-36 | covered | 1,000 back-and-forth resizes, no drift |
| AC-37 | covered | handle press takes precedence over the body; test |
| AC-38 | covered, partly | screen-square activation at 400 % tested; 5 % not tested |
| AC-39 | covered | `canvas-perf.spec.ts`: resize on `rects-1000`, p95 16.8 ms headless |
| AC-40 | covered, with note | lint still green; the handle drawing subtracts half the size from a point to center the square. That is visual layout, not a coordinate conversion. |

## Verification

- Rust: 185 tests pass; clippy `-D warnings`, fmt, and the boundary check pass; the wasm32 check passes.
- App: typecheck, Vitest 60 pass, production build passes, Playwright 17 pass
  (including 3 new resize E2E tests and 1 new resize pacing test).

## Deferred (recorded, not hidden)

- Cursor hints on handles (CAN-CUR-2): not built. Handles work; the cursor does not change.
- Rotated and flipped objects: no handles (rotate and flip are later slices).
- Resize of several objects at once: needs multi-bounds.
- Snapping during resize (CAN-SNAP-6): later slice.
- Undo does not reselect, same as Slice 0.
