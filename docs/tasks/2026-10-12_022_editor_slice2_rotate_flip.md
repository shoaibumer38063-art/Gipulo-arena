# Task 022 — Editor Slice 2: rotate, flip, resize of rotated objects, cursor hints (DONE locally)

- **Plan:** `2026-10-12_022_editor_slice2_rotate_flip_plan.md`. Decisions: D1 keep flip flags (ADR),
  D2 rotation normalized to −180…180°, D3 cursor hints included.
- **Status:** implemented and committed locally. Not pushed (no token provided; "push later").

## What shipped

- **Rotate** by a rotate zone just outside each corner (`ROTATE_RADIUS_PX` = 20 px outer, 6 px inner).
  Pivot is the object center. Shift snaps the change to 15° steps, measured from the start angle.
  Result normalized to −180…180° (`rotate_transform`).
- **Flip** horizontal / vertical about the object center with **Shift+H** and **Shift+V**. The flags are
  kept on the node (`flip_h`, `flip_v`), and the box stays centered (`flip_transform`). One history
  entry per flip.
- **Resize of rotated objects** in the object's local frame. The opposite edge stays fixed in world
  space (`resize_transform`). Handles follow the rotation (`handle_points`).
- **Cursor hints** (CAN-CUR-1/2): `Interaction::cursor()` reports the action a press would start.
  Rotate zone → `grab`. Corner or edge → resize cursor, quantized to the object's axes
  (`resize_cursor`). The canvas applies it as an inline style.
- **Activation priority** (CAN-HOVER-2): handle > edge > rotate zone > body (`control_at`, checked in
  that order before the body hit).
- **Commit**: `Commit::Transform { items }` replaces `Commit::Resize`. It writes X, Y, W, H,
  Rotation, FlipH, FlipV in one transaction. Escape restores the exact start transform and writes
  nothing.
- **Readouts**: a `rotation-readout` shows the selected object's angle, and `size-readout` now
  reports the box around the center. That is identical to before when unflipped and unrotated, and
  correct when flipped.
- **Boundary**: protocol 4 → 5. New wasm methods: `flip(axis)`, `cursor()`, `selection_rotation()`.
  The shell mirror (`PROTOCOL_VERSION`, `client.ts`) is updated in the same change.

## Bug found and fixed during verification

A flip wrote the new `x` but not the flip flags. The stored node then had the wrong box. The commit
now writes `FlipH` / `FlipV` as well, and the `flip_twice` test caught it. Also: `selection_bounds`
read `x` as the left edge, but for a flipped node `x` is the right edge. Fixed by reading the box
around the center.

## Acceptance coverage (plan AC-41..50)

| AC | Status | Evidence |
|---|---|---|
| AC-41 | covered | `rotate_zone_rotates_about_the_center_by_the_angular_change`; E2E "dragging outside a corner rotates…" |
| AC-42 | covered | `shift_snaps…` (session, 22° → 15°); pure test `shift_snaps_the_change_to_fifteen_degrees`. Tolerance is 1e-6° in the session test and 1e-9° in the pure test, not 1e-9 rad. |
| AC-43 | covered | `flip_twice_restores_the_rects_exactly_and_flip_once_is_one_entry` (exact, no tolerance needed). Pure `flip_twice_restores_the_transform_exactly`. |
| AC-44 | covered, partly | pure `resize_of_a_rotated_object_keeps_the_opposite_edge_fixed_in_world`. Shift and Alt on rotated objects are not tested separately. |
| AC-45 | covered | rotate: `rotation_is_one_entry_and_undo_restores_zero`; Escape: `escape_during_rotate_restores_the_exact_transform`; E2E Escape |
| AC-46 | covered | `a_thousand_back_and_forth_rotations_do_not_drift` (1,000 × ±7°, |rotation| < 1e-6) |
| AC-47 | covered, partly | rotate zone is only tested outside the box, so body and zone do not overlap. The corner-over-zone order is by code order and is not tested alone. |
| AC-48 | covered | `cursor_hints_follow_the_control_under_the_pointer`; `cursor_follows_the_rotated_axes`; E2E cursor test |
| AC-49 | covered | `protocol_version_is_five…`; `client.ts` mirror = 5; the engine smoke test checks protocol 5 |
| AC-50 | covered | `canvas-perf.spec.ts` rotate on `rects-1000`: p95 16.8 ms, headless (same as resize). |

## Verification

- Rust: 199 tests pass (`cargo test --workspace`). Clippy `-D warnings` is clean. `cargo fmt --check`
  is clean. Boundary check passes.
- wasm: `scripts/build-wasm.sh` builds. New methods are in `bindings/pkg` (generated, not committed).
- App: typecheck clean. Vitest 61 pass. Production build passes.
- Playwright: 25 pass (17 from Slice 1 and earlier, 7 new rotate/flip/cursor E2E, 1 new rotate pacing).
  The engine smoke test now expects protocol 5.
- Environment note: after the sandbox reset, the Playwright Chromium needed shared libraries that were
  not installed. They were extracted from Debian packages into `~/.local/pwlibs` and loaded through
  `LD_LIBRARY_PATH`. This is an environment fix, not a code change.

## Deferred (recorded, not hidden)

- **Resize of flipped objects**: handles are hidden while an object is flipped. Flip itself works.
- Rotating or resizing several objects at once (needs multi-bounds). Next, see task 024.
- Snapping and guides (CAN-SNAP): later slice.
- Undo does not reselect, as in Slice 0.
- Eslint is not configured for the app (no flat config and no lint script), so there is no lint gate for the TS side.
