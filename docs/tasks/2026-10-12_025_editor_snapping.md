# Task 025 — Editor: snapping and guides (DONE locally)

Follow-up to task 024. Snapping (CAN-SNAP) for move, resize, and create, with snap guides.
The rules come from `docs/architecture/input-system.md` §snapping: 4 px activation, 6 px release
(hysteresis, marked PROPOSED there and adopted here), ⌘ / Ctrl turns snapping off for the gesture.

## Decisions (taken by default, since the owner asked to proceed one item at a time)

- **Targets:** the start, center, and end lines of every other visible, unlocked object on the page
  (world AABBs). No grid, no custom guides yet.
- **Move** snaps the selection's group box on both axes.
- **Resize** snaps the grabbed moving edge (the right or left, top or bottom, following flips) for
  axis-aligned frames. Rotated single objects do not snap during resize. Their edges are not
  axis-aligned, so snapping would be ambiguous.
- **Create** snaps the dragged corner on both axes.
- **Guides** are computed by the core and drawn by the shell in `--guide-snap`. A guide spans the
  moving box and the matching targets.
- **Protocol 5 → 6:** `pointer_move` takes `snap_off` (⌘ / Ctrl). New `guides()` returns the segments.

## What changed

- `gip-interaction/src/snap.rs` (new): `snap_value` (activation and hold), `lines`, `guide_for`,
  `candidate_boxes`, `transform_box`, and the constants.
- `machine.rs`: `Modifiers.snap_off`, `SnapHold` and `guides` in the machine, `snap_axis`,
  `snapped_control_world`, `snapped_create_world`, and `update_move` now snaps. Reset on press,
  release, and cancel.
- `gip-session`: `pointer_move_snap`, `guides()` in viewport pixels.
- Shell: `pointerMove(…, snapOff)`, `guides()`, `paintGuides`, and `data-guides` on the canvas (count
  of guides, for tests).

## Tests

- Pure (`snap.rs`): activation, no activation, hold released only past 6 px, lines order.
- Session (`snap_tests`): move snaps within 4 px; Ctrl-off leaves the raw position; hold keeps a snap
  to 6 px and releases past it; a guide shows during the gesture and clears on release; a group or
  single resize snaps the grabbed edge; create snaps the dragged corner.
- E2E (`snapping (Slice 4)`): a snapped move shows one guide and clears on release; Ctrl keeps the
  guide count at 0.

## Verification

- Rust: 221 tests pass; clippy `-D warnings` and fmt clean.
- App: typecheck clean; Vitest 61 pass; Playwright 29 pass.
- Pacing with snapping on (headless, `rects-1000`): pan 16.7, drag 16.7, resize 16.7, rotate 16.8 ms p95.

## Deferred (recorded)

- Snapping to the grid, to the page, and to custom guides.
- Snapping for rotated single objects during resize (needs guides in the object's own frame).
- Snapping during rotate (not in the spec list).
- The edge-to-center snap on the opposite side during Alt resize is not separate; only the moving
  edge snaps.
