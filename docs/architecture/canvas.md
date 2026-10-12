# Canvas — Implementation Map

> Status: ACTIVE for Slice 0a (2026-10-11, task 020). Binding rules: `rules/canvas.md` (read §0,
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
| Interaction state machine | **not yet** (Slice 0b: `gip-interaction`, ADR-012 D2). Transitional: `CanvasHost` pan gesture | CAN-MODE-1/2 |
| Document edits, history | `gip-history`, `gip-document` (Slice 0b onward) | CAN-HIST-1..4 |
| Geometry, hit-testing | `gip-geometry` (Slice 0b onward) | CAN-GEOM-1..6 |
| Selection | not yet (Slice 0b) | CAN-SEL-1..7 |

## 2. Implemented behavior (Slice 0a)

- Middle-button drag pans; the content follows the pointer by the same pixels (`pan_by`).
- Plain wheel and two-finger scroll pan.
- Ctrl/⌘ + wheel and trackpad pinch zoom around the pointer (`zoom_at`, factor `exp(-deltaY × s)`
  with `s` = `WHEEL_ZOOM_SENSITIVITY`).
- Origin crosshair at the core-reported world origin, 1 px, half-pixel aligned (CAN-SS-3).
- Readout shows the core-computed zoom percent and origin.

## 3. Known gaps (recorded, not hidden)

- Space-hold temporary hand tool (CAN-CAM-4): Slice 0b.
- Pan has no cancel (Escape) yet: pan does not change the document, so the rule's effect is
  limited to the view; Slice 0b routes it through the state machine.
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
