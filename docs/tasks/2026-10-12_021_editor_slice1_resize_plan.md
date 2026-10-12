# Task 021 — Editor Slice 1: resize with handles (PLAN)

- **Date:** 2026-10-12 · **Roadmap:** step 20 (after Slice 0) · **Status:** PLAN, awaiting owner review before code
- **Predecessor:** Slice 0 complete (020a–020c), on GitHub at `8d961bd`.
- **Rules read:** `rules/canvas.md` CAN-SEL-6, CAN-TR-1..3, CAN-TR-6, CAN-GEOM-5, CAN-CAM-5,
  CAN-HIST-2, CAN-MODE-1/2/4, CAN-INPUT-2, performance §26 (pointer→visual ≤ 2 frames p95 during resize).

## 1. Scope — "a rectangle you can resize"

In scope:
- **8 resize handles** on the selected single rectangle (corners and edge midpoints), drawn at 8 px
  visual size (CAN-SEL-6). Handles are drawn by the shell from core-computed positions (no geometry math in the shell).
- **Corner and edge semantics** (CAN-TR-3): a corner handle moves two edges, an edge handle moves one.
- **Shift** = proportional (aspect locked from gesture start). **Alt/⌥** = resize from center.
- **Cursor hints** for handles (CAN-CUR-2) on unrotated objects only in this slice.
- **Gesture math from gesture-start values** (CAN-TR-2): the new box is `f(start box, pointer delta)`,
  never accumulated per event. Repeated 1,000× resize stays stable (CAN-TR-6).
- **One history entry per resize gesture**; Escape or pointercancel restores the exact start box (CAN-MODE-4).
- **Minimum size:** a resize cannot make width or height negative. A drag past the opposite edge
  clamps at zero width or height. [PROPOSED: clamp, do not flip]. Zero-size objects remain addressable (CAN-GEOM-5, already built in 020c).
- Undo / redo of resize; the readout shows the object count and selection as before.
- Keyboard: none new (Escape already cancels).

Out of scope (deferred, with reason):
- **Rotate and flip** — separate slice (CAN-TR-4, CAN-TR-5).
- **Resize of multiple selected objects** — needs multi-bounds; Slice 1 handles a single object only.
- **Snapping during resize** (CAN-SNAP-6) — snapping is its own slice; the snap budget is measured then.
- **Rotated-object resize** — the rotated-handle cursor and math need the rotated box; deferred to rotate slice.
- **Text, vector edit, layers, properties, groups** — other slices.

## 2. Architecture placement

- `gip-interaction`: new `Resize` gesture state (pressed on a handle, dragging), handle hit testing
  (handle activation zones take precedence over body hits), and `Commit::Resize { node, from, to }`.
  Resize math lives in `gip-coords`/`gip-geometry` helpers, not in the shell.
- `gip-session`: applies `Commit::Resize` as one `History::transact` writing X, Y, W, H; exposes handle positions
  in the draw list (new flag bits or a handle record list). Protocol bump 3 → 4 (boundary shape changes).
- Shell: draws handles from the core list; forwards pointer and modifier keys (Shift, Alt) only.
  No new coordinate arithmetic in `app/src/canvas` (CAN-COORD-2 lint stays green).
- Tokens only for handle fill, stroke, and size (AC-26 pattern from Slice 0).

## 3. Acceptance criteria (proposed, numbered AC-29..)

- **AC-29** Selecting one rectangle shows 8 handles at its corners and edge midpoints, 8 px visual, token-colored.
- **AC-30** Dragging a corner handle resizes two edges; dragging an edge handle resizes one edge; the opposite edge stays fixed.
- **AC-31** Shift during a corner drag keeps the start aspect ratio exactly (within 1e-9).
- **AC-32** Alt during a drag keeps the center fixed.
- **AC-33** Dragging past the opposite edge clamps at zero size; no negative width or height is ever stored.
- **AC-34** A resize gesture is one history entry; undo restores the exact start box; redo restores the resized box.
- **AC-35** Escape or pointercancel during resize restores the exact start box and adds no history entry (bitwise equal draw list).
- **AC-36** 1,000 back-and-forth resize gestures produce no drift (start-value math, CAN-TR-6).
- **AC-37** Handle activation zones take precedence over body hits; a handle is not "missed" when the pointer is 4 px outside its visual.
- **AC-38** Resize at 5 % and 3200 % zoom: handle visual stays 8 px on screen; world-space resize matches the pointer delta divided by zoom.
- **AC-39** Pointer→visual ≤ 2 frames p95 during resize on the 1,000-rectangle fixture (headless measurement, as in 020c).
- **AC-40** No coordinate arithmetic in the shell (existing lint extended to the handle drawing code).

## 4. Decisions needed from you (recommended option first)

- **D1 Clamp vs flip when dragged past the opposite edge:** A) clamp at zero (recommended: simple, no mirrored
  geometry); B) flip (Figma-like, but needs flip-state semantics in the transform). 
- **D2 Handle hit slop:** A) 8 px visual with a 12 px activation zone (recommended, matches CAN-HOVER-2 spirit); B) exact 8 px.
- **D3 Protocol:** A) bump to 4 (recommended, shape changes); B) keep 3 by reusing existing draw-list fields (not recommended).

## 5. Verification plan

- Rust: unit tests for corner/edge math, Shift and Alt constraints, clamping, bitwise cancel, 1,000× stability, handle hit precedence.
- App: typecheck, Vitest (lint extension), production build.
- Playwright: handles appear on select; corner drag resizes; Escape cancels; Ctrl+Z restores; frame pacing on `rects-1000` (one selected object resized).
- Gates: clippy `-D warnings`, fmt, wasm check, boundary check.

## 6. Estimated size

About 1 new gesture state, ~600 lines of core plus tests, ~250 lines of shell and E2E. One task
summary (`021`) at completion.

## 7. Status

PLAN ONLY. No code written. Start only after owner go-ahead (and the D1–D3 choices).
