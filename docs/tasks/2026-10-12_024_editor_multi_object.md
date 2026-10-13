# Task 024 — Editor: rotate and resize several objects together (DONE locally)

Follow-up to task 022. Multi-object rotate and resize, which were deferred there.

## Decisions (taken by default, since the owner asked to proceed one item at a time)

- **Group box** = axis-aligned bounds of the members' world boxes. Handles and rotate zones sit on it.
- **Rotate** works for any members. All members turn by the same angle about the group center, and
  their centers move around that pivot. Shift snaps to 15°.
- **Resize** needs every member to be axis-aligned (rotation 0, unflipped). Otherwise the scale would
  shear members, and the group shows no handles. Rotated groups stay rotatable.
- **Members stay in the same relative positions** inside the box. Each member's size and center scale
  with the box. Shift and Alt follow the Slice 1 rules.
- One history entry per gesture (`Commit::Transform` with one item per member). Escape restores all.

## What changed

- `gip-interaction/src/group.rs` (new): `group_box`, `group_frame`, `axis_aligned`, `rotate_group`,
  `resize_group`.
- `machine.rs`: `SelectionFrame` and `selection_frame` (one object → itself; several → group box),
  `Control::GroupResize` / `GroupRotate`, `State::GroupResizing` / `GroupRotating`,
  `Preview::TransformingMany`. `effective_transform` moved to the machine and is shared.
- `gip-session`: handles and the size readout use the selection frame. Several objects now show a
  size readout (the group box). The rotation readout still appears for one object only.
- No protocol change (still 5). No new wasm methods.

## Tests

- Pure (`group.rs`): group box bounds; rotate keeps pivot distances and turns each member by the same
  angle; resize scales positions and sizes; `axis_aligned` rules.
- Session (`group_tests`): one shared box and eight handles; group resize is one entry with undo;
  group rotate by 90° swaps the box size about the same center; rotated members hide the group
  handles; Escape during a group rotate restores every member.
- E2E: two selected rectangles (Shift-click) show a 260 × 140 box, resize to 300 × 160, and undo.

## Verification

- Rust: 211 tests pass; clippy `-D warnings` and fmt are clean.
- App: typecheck clean; Vitest 61 pass; Playwright 27 pass.

## Deferred (recorded)

- Multi-object resize of rotated or flipped members (needs shear-aware math or per-member frames).
- Marquee selection (dragging on empty canvas) is still not built.
