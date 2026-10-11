# Task 012 — History: transactions and undo/redo (`gip-history`, roadmap step 12)

**Date:** 2026-10-11 · **Status:** COMPLETE (write-door gap A6 CLOSED in the same task — see the final section) · **Governs:** `docs/architecture/ADR-004-history-storage.md` (amendments A1–A7) · `undo-redo.md` §2–§8 · CAN-HIST · SM3

## Goal

Build the single mutation path for the document: an op log with recorded before/after values,
atomic transactions, exact undo and redo, a byte budget, and dirty tracking. Test-first against the
undo-redo §8 test focus.

## What was built

| File | Contents |
|---|---|
| `gip-document/src/node.rs` | `FieldKey` (11 scalar fields, closed), `FieldValue`, `Node::field` / `Node::set_field` (type-checked), `Node::approx_bytes` |
| `gip-history/src/op.rs` | `EditOp` (SetField · InsertNode · RemoveNode · MoveNode · ReorderChildren · SetName), `Parent`, `snapshot`, `apply(Forward/Backward)`, `op_bytes`, `affected_ids` |
| `gip-history/src/tx.rs` | `Tx`: `alloc_id`, `set_field`, `insert`, `remove` (subtree snapshot), `move_node`, `reorder`, `set_name`; every write is applied and recorded immediately |
| `gip-history/src/history.rs` | `History`: `transact`, `undo`, `redo`, label accessors, byte budget, `mark_saved`/`is_dirty`, `TxError`, `Changes` |
| `gip-history/tests/history.rs` | 15 law tests |

## Laws enforced (and where they are tested)

- One gesture = one entry (1000 drag events → one undo) — `a_thousand_drag_events_are_one_entry_and_one_undo`
- Rejected mutator rolls back every write, no entry — `rejected_mutator_rolls_back_every_write_and_records_nothing`
- Invariant violation (negative width) rolls back atomically — `invariant_violation_rolls_back_atomically`
- No-op transaction records nothing — `no_op_transaction_records_no_entry`
- Undo, then redo, reproduce the exact document and affected set — `undo_then_redo_restores_exact_document_and_affected_set`
- Subtree delete is atomic and its undo restores children in place — `remove_subtree_is_atomic_and_undo_restores_children_in_place`
- Move between parents and reorder undo exactly — two tests
- Ids are never reused, even after undo — `ids_are_never_reused_even_after_undo`
- Edit after undo truncates the redo tail — `new_edit_after_undo_truncates_redo_tail`
- Labels follow the stacks — `undo_and_redo_labels_follow_the_stacks`
- Rename is undoable — `rename_is_undoable`
- Dirty tracks position relative to the save point — `dirty_tracks_position_relative_to_save_point`
- Budget drops the oldest entries; the boundary is documented, not silent — `budget_drops_oldest_entries_and_keeps_the_newest`
- Nested helpers fold into one entry — `nested_helpers_fold_into_one_entry`

## Bugs caught during the phase

1. **Test expectation wrong (budget):** I asserted the value reset to 0 after eviction. The correct invariant is `49 − steps` (the value just before the oldest kept step). The code was right; the test was rewritten to state the boundary.
2. **Fixture built a throwaway history** first: removed.
3. **Weak assertion** (`!can_undo || …`) replaced with an exact label check.
4. **Label assertion after the last undo** expected a label that no longer exists: corrected to `None`.
5. **Missing field docs** on `EditOp` variants (workspace `missing_docs`): added per field.

## Environment note

The sandbox was restarted during this task; the Rust toolchain was missing from `~/.cache`. Reinstalled
with `rustup` at `1.99.0` (clippy, rustfmt) and added the `wasm32-unknown-unknown` target. Pinned
versions are unchanged.

## Measured

128 workspace tests green (up from 114 at task 011; 15 of them are the history law tests). Boundary check now includes the write-door rule (probe-verified). `cargo clippy --all-targets -D warnings` clean.
`cargo fmt --check` clean. `check-boundaries.py` PASS. `gip-session` wasm check clean.

## A6 write-door enforcement — CLOSED

The owner chose "move the write logic into gip-document" (question answered 2026-10-11).

- `gip-document::edit` now holds `EditOp`, `Parent`, `Dir`, `Subtree`, `apply`, `snapshot`,
  `op_bytes`, `affected_ids`.
- `GipFile` and `Document` fields are `pub(crate)`. Read accessors: `GipFile::{version, meta,
  document, assets}`, `Document::{pages, nodes}`. `GipFile::allocate_id` is the only id source.
- `GipFile::new` creates one empty page (`Page 1`) so every document has an edit target.
- The gip-document serialization tests moved in-crate (`src/serial_tests.rs`) because they must
  build deliberately broken documents. The golden fixture stays in `tests/golden/`.
- `scripts/check-boundaries.py` gained a write-door rule: only `gip-document` and `gip-history` may
  name `gip_document::edit`. A probe file was verified to fail the check, then removed.
- Residual caveat (stated in ADR-004 A6): the rule is per crate. A capability token is the stricter
  option if you want it later.

## Deferred (explicit)

Text-session coalescing (500 ms window) · gesture sessions (pointer begin/update/commit) ·
subscriptions for affected-set publication (the set is returned now; push subscribers come with the scene) ·
`SetShared` op (no shared collections yet) · hash-based dirty check (SHA-256 decision) · history-tree (not v1).
