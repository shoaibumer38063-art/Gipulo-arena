# Task 023 — Editor: resize flipped objects (DONE locally)

Follow-up to Slice 2 (task 022). The deferral "handles are hidden while an object is flipped" is lifted.

## What changed

- **Handles stay visible** on flipped objects. `resize_target` no longer excludes flipped nodes.
- **Drag direction is mirrored** for a flipped axis (`resize_transform`). The grabbed edge follows the
  pointer, and the opposite edge stays fixed in world space. The anchor is computed in unit space,
  so the flip mapping does not change it.
- **Flip flags are kept** through a resize.
- No protocol change (still 5). No new wasm methods.

## Tests

- Pure: `resize_of_a_flipped_object_moves_the_handle_that_was_grabbed`,
  `resize_of_a_flipped_and_rotated_object_keeps_the_anchor_fixed`.
- Session: `a_flipped_object_resizes_from_the_handle_under_the_pointer` (drag left edge of a flipped
  box left by 30 → 130 wide, one undo entry).
- E2E: `a flipped rectangle resizes from the handle under the pointer`.

## Verification

- Rust: 202 tests pass; clippy `-D warnings` and fmt clean.
- App: typecheck clean; Vitest 61 pass; Playwright 26 pass (including rotate pacing on `rects-1000`,
  p95 16.7 ms).
