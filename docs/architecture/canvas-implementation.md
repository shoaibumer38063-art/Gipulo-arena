# Canvas — Implementation Map (Slice 0a–0b)

> Companion to `canvas.md` (the hub). Status: ACTIVE for Slice 0a (2026-10-11, task 020). Binding rules: `rules/canvas.md` (read §0,
> §3–§5, §7, §8, §11, §13, §22, §25–§28 before changing this map). This file maps the rules to code.
> Update it in every canvas task.

## 1. Where each responsibility lives

| Responsibility | Owner (code) | Rule |
|---|---|---|
| Coordinate conversions (screen ↔ viewport ↔ world) | `core/crates/gip-coords` (`viewport.rs`, `camera.rs`) | CAN-COORD-2 |
| Camera (pan, zoom, clamp 2 %–4096 %) | `gip-coords::Camera` | CAN-CAM-1/2/3 |
| Viewport rect and view state for the shell | `core/crates/gip-session/src/session.rs` (`Session`, `ViewSnapshot`) | CAN-CAM-5 |
| Boundary transport (typed calls, plain structs) | `core/crates/gip-session/src/wasm.rs` (transport only) | engine-core §3, §10 |
| Shell engine facade | `app/src/engine/client.ts` (only module that touches bindings) | engine-core §10 |
| Canvas host (DOM element, raw input forwarding) | `app/src/canvas/CanvasHost.tsx` | CAN-INPUT-1 |
| Canvas painting (pure functions) | `app/src/canvas/paint.ts` | CAN-SS-3, CAN-COORD-2 |
| Interaction state machine (gestures, tool, selection, hover) | `core/crates/gip-interaction/src/machine.rs` (ADR-012 D2) | CAN-MODE-1/2/4, CAN-INPUT-2 |
| Document edits, history | `gip-session` commits each gesture as one `History::transact` (`session.rs`); `gip-history`, `gip-document` | CAN-HIST-1..4 |
| Rectangle transform and hit-testing | `gip-geometry::trs_of` + `Mat2::from_trs` / `world_to_local` (`machine.rs::hit_test`) | CAN-GEOM-1..6 |
| Selection and hover | `machine.rs` (`Interaction::selection`, `hover`); outline drawn by `paint.ts::paintRects` | CAN-SEL-1, 2, 5, 6 |
| Keyboard shortcuts | `app/src/canvas/shortcuts.ts` (registry + pure matcher) | rules I1 |

## 2. Implemented behavior (Slice 0a)

- Middle-button drag pans; the content follows the pointer by the same pixels (`pan_by`).
- Plain wheel and two-finger scroll pan.
- Ctrl/⌘ + wheel and trackpad pinch zoom around the pointer (`zoom_at`, factor `exp(-deltaY × s)`
  with `s` = `WHEEL_ZOOM_SENSITIVITY`).
- Origin crosshair at the core-reported world origin, 1 px, half-pixel aligned (CAN-SS-3).
- Readout shows the core-computed zoom percent and origin.

## 2b. Implemented behavior (Slice 0b)

- Tools: `R` (Rectangle) and `V` (Select), via the registry and the toolbar. The tool is sticky: it stays
  Rectangle after a create.
- Rectangle create: drag past 3 px (`DRAG_THRESHOLD_PX`) with the Rectangle tool; a normalized,
  positive-area rect commits as one history entry and becomes the selection.
- Select: click selects the topmost visible, unlocked object under the pointer (true rotated shape,
  no tolerance); click on empty canvas clears; Shift-click toggles membership.
- Move: press on a selected object, drag past 3 px; all selected objects move together; one
  `Commit::Move` = one history entry; no entry if the net delta is zero.
- Cancel: Escape or `pointercancel` during a gesture restores the preview; nothing was written, so
  nothing is recorded (CAN-MODE-4). Escape while idle clears the selection.
- Delete / Backspace: removes the selection as one entry. Undo and redo are whole-gesture steps.
  Undo and redo do not change the selection, except that deleted ids are pruned.
- Middle-button pan now runs through the machine (`Effect::Pan`). The Slice 0a transitional
  pan variable in `CanvasHost` is gone.
- Draw list: `Engine.rects()` returns a flat `Float64Array` (9 floats per rect: 4 corners, flag
  bitmask). Flags: 1 selected, 2 hovered, 4 creation preview.

## 3. Known gaps (recorded, not hidden)

- Marquee (drag-select on empty canvas): out of scope for Slice 0; a drag from empty canvas does nothing.
- Zero-width or zero-height rectangles cannot be created (the positive-area rule), but a file may
  contain one. Such an object cannot be hit, because AC-11's minimum hit tolerance is not built yet.
- Selection handles and resize: out of scope (1 px outline only).
- Undo does not reselect the restored object.

- Space-hold temporary hand tool (CAN-CAM-4): not yet built.
- Keyboard zoom shortcuts: wait for the shortcut registry (rules/interactions I1).
- Zoom-to-fit and presets: later.
- Performance budgets (CAN-BUDGET) are measured once there are reference documents (Slice 0c).

## 4. Tests

| Test | Where |
|---|---|
| Anchor invariance, clamps, pan, rejection of invalid input, client→viewport conversion | `gip-session` unit tests (session.rs) |
| Origin crosshair geometry, readout, no-conversion lint | `app/src/canvas/paint.test.ts` |
| Start state, middle-drag pan, ctrl-wheel zoom | `app/e2e/canvas-view.spec.ts` (Playwright, production build) |
| Handshake and engine-load failure | `app/e2e/engine-smoke.spec.ts`, `engine-error.spec.ts` |
| Hit test, create, move, cancel, undo/redo, delete, Shift-select, rotated hit, pan has no entry | `gip-session` unit tests (`interaction_tests` in session.rs) |
| Create, Escape cancel, move = one undo step, cancelled move, delete, pan has no undo | `app/e2e/canvas-edit.spec.ts` |
| Shortcut mapping and modifiers; readout text; no-conversion lint (extended to shortcuts.ts) | `app/src/canvas/shortcuts.test.ts`, `paint.test.ts` |
