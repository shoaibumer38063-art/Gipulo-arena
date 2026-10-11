# Transform System — Architecture Contract

> Status: DESIGN v1 (2026-10-11), shaped by ADR-001 (ACCEPTED: Rust/Wasm core + TS shell). Binding rules: CAN-TR (all), CAN-NUM,
> CAN-COORD-3/5. Hub: `canvas.md`.

## 1. Purpose & scope

One system for all spatial mutation and resolution of nodes: move, resize, rotate, flip, and the
composition/resolution of nested transforms. Tools express intents; this system computes exact
document-field changes; the geometry engine and coordinate module consume transform data.

## 2. Owns

- **Canonical transform representation** per node: position (x, y in parent space) · size (w, h) ·
  rotation (single canonical angle convention — ADR-002) · flip state. Whether flips are stored
  as flags or negative scale is ADR-002's call (CAN-TR-5 [PROPOSED: retain flip flags]);
  decomposition always works from the canonical stored form, never from raw matrix round-trips
  (CAN-TR-6).
- **Gesture-transform pattern** (CAN-TR-2, binding): at gesture start capture
  `{nodeId → canonical transform}` snapshot in transient state; every pointer move computes
  `target = start + f(pointer, modifiers)`; commit writes targets once. No frame-to-frame
  accumulation anywhere.
- **Operation semantics:** move (parent-space deltas; Shift = axis lock) · resize (per-handle
  edge/corner semantics; Shift = proportional from gesture-start aspect; ⌥/Alt = around center;
  frame resize ⇒ layout engine owns child reflow — transform system must NOT hand-move laid-out
  children, CAN-TR-3) · rotate (pivot = object center default; rotate-handle zone per DS;
  Shift = 15° increments with exact snap at gesture end, CAN-TR-4) · flip (about center,
  CAN-TR-5) · multi-selection (transform in selection common space; correct per-child
  decomposition of position AND orientation under group rotation, CAN-TR-1).
- **Pivot/anchor resolution** for all ops (corner handles, center, opposite-edge anchoring) —
  one pivot calculator, no per-tool variants.
- **Commit-time cleanup:** snap-to-intent (exact 15°/90°/axis values after constrained gestures),
  zero/negative-size policies (minimum size or inversion handling — documented behavior,
  ADR-002 bundle), all applied at gesture END only (CAN-TR-6).

## 3. Must never

- Round or quantize stored values mid-gesture (CAN-TR-6, CAN-NUM-2).
- Bake parent transforms into children (nesting resolved at read time via coordinate module,
  CAN-TR-7).
- Duplicate coordinate math (all space resolution through the canonical module, CAN-COORD-2).
- Write outside transactions (one gesture = one history entry, CAN-HIST-1 via `undo-redo.md`).
- Corrupt under repetition: 1000× transform cycles return within ε of original (CAN-TR-6 test law).

## 4. Public interface (conceptual)

- `beginGesture(selection, op, params)` → gesture handle (snapshot in transient state).
- `updateGesture(handle, pointerWorld, modifiers)` → preview transforms (transient; consumed by
  overlays/scene preview — no document writes).
- `commitGesture(handle)` → single transaction with final canonical values (cleanup applied).
- `cancelGesture(handle)` → exact restore, zero writes (CAN-INPUT-4).
- Pure helpers (exported for tools/panels/tests): `applyDelta`, `resizeRect(rect, handle, delta,
  {proportional, fromCenter})`, `rotateAbout(rect, angle, pivot)`, `flip(rect, axis)`,
  `decompose/compose` (canonical ⇄ matrix, via coordinate module conventions).
- Numeric entry paths (property panel X/Y/W/H/rotation fields) reuse the SAME pure helpers —
  panel edits and canvas gestures produce identical results for identical inputs.

## 5. Dependencies

- Consumes: document model (transform fields, via transactions), coordinate module (matrix
  conventions, space conversions), geometry engine (bounds for pivots/snapping attach points),
  layout engine (frame-resize reflow handoff), input-system (gesture lifecycle owner).
- Consumed by: tools (move/resize/rotate), panels (numeric fields), snapping (during-gesture
  target application), history (commit), a11y (keyboard transform commands route here).

## 6. Invariants

- Gesture determinism: same start + same pointer sequence + same modifiers ⇒ same committed
  values (no float-order variance).
- Preview/commit equivalence: committed result == final preview within display ε.
- Modifier live-application: modifiers toggled mid-gesture take effect on the next move event
  (CAN-INPUT-3) because computation is start-relative, not incremental.

## 7. Open questions → resolved by ADR-002 (PROPOSED 2026-10-11)

1. ✅ **DECIDED:** angle = degrees, f64, normalized to (−180, 180] at every write; one convention
   in storage, UI, tests, files (trig converts to radians only inside `gip-coords`).
2. ✅ **DECIDED (confirms CAN-TR-5 [PROPOSED]):** flip = boolean flags; stored sizes are never
   negative.
3. ✅ **DECIDED:** matrices are column-major 6-tuple (CSS/SVG/Canvas2D order), derived only;
   canonical fields are the input to every decomposition (CAN-TR-6).
4. ✅ **DECIDED:** crossing zero during resize toggles the flip flag **during preview**; committed
   `w/h ≥ 0`; no hidden minimums; 0-size nodes are legal but inert (selectable via outline).
5. ⏳ **OWNER REVIEW (recommended default in ADR-002):** plain resize never scales strokes/
   effects/radii; an explicit Scale tool scales geometry + a documented style list — DS-visible,
   written into DS `05`/`06` when those features land.

## 8. Test focus

Ops × modifiers matrix (move/resize/rotate/flip × Shift/⌥/⌘ × single/multi/rotated/nested/
flipped parents) · gesture-start-delta equivalence (cancel restores exactly; replay determinism) ·
1000× stability (ε law) · frame-resize⇄layout handoff (no double-application) · panel-numeric ⇄
canvas-gesture parity · commit-time cleanup exactness (15°, axes) · mixed multi-selection
decomposition correctness.
