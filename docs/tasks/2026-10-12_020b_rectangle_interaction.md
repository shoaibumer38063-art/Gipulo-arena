# Task 020b — Rectangle create, select, move, cancel, undo (Slice 0b)

**Status:** done locally, not pushed (no `GITHUB_TOKEN`).
**Plan:** `2026-10-11_020_editor_slice0_plan.md` (AC-5..19, AC-23 for this slice).
**Decisions:** ADR-012 D1–D4 (D2 = `gip-interaction` crate, D4 = Shift-click and Delete in Slice 0).
**Implementation map:** `docs/architecture/canvas-implementation.md`.

## What shipped

- **`gip-interaction`** (new, layer 2; deps: document, coords, geometry). Pure state machine: gestures
  (Pressed, Moving, Creating, Panning), tool, selection, hover. It never writes the document. It returns
  `Effect`s (`Pan`, `Commit`).
- **`gip-session`**: owns `History` and the machine. Each committed gesture is exactly one
  `History::transact`. Adds pointer methods, `cancel`, `escape`, `set_tool`, `delete_selection`,
  `undo`/`redo`, `rects()` (draw list in viewport pixels), `save_json`.
- **Wasm boundary**: new typed methods; `rect_stride()`; draw list as a flat `Float64Array`
  (9 floats/rect). **Protocol bumped 1 → 2** because the boundary shape changed (AC-22 said 1 for 0a).
- **Boundary checker** registers `gip-interaction` with the allowed internal edges. Dependency doc updated.
- **Shell**: `R`/`V` tools (keyboard registry `shortcuts.ts` + toolbar), rectangle drawing, 1 px
  selection outline, hover outline, Delete/Backspace, Ctrl/⌘+Z, Ctrl/⌘+Shift+Z, Escape.
  Pan now runs through the machine; the Slice 0a transitional pan variable is gone.

## Behavior decisions (recorded)

- The Rectangle tool stays active after a create (sticky).
- Undo and redo do not change the selection (except pruning deleted ids). Selection is UI state.
- A press on a selected object drags the whole selection. Press on empty canvas clears the selection.
- No marquee (out of scope).

## Acceptance coverage

| AC | Status | Evidence |
|---|---|---|
| AC-5 | covered | create test; negative-direction drag test |
| AC-6 | covered | below-threshold and zero-area create tests |
| AC-7 | covered | create undo/redo; redo keeps the same node id |
| AC-8 | covered | rotated-rectangle hit test (outside corner misses) |
| AC-9 | covered | topmost-object-wins test |
| AC-10 | partial | inside hit has no tolerance; no stroked objects exist yet, so the stroke tolerance is untested |
| AC-11 | **not done** | zero-size minimum tolerance not built (recorded gap) |
| AC-12 | covered | Shift-click toggle test; Escape clears |
| AC-13 | covered | exact move at 200 % zoom |
| AC-14 | covered | 1000 back-and-forth moves, no drift |
| AC-15 | covered | cancelled move: draw list bitwise equal; no entry |
| AC-16 | covered | below-threshold select drag does not move |
| AC-17 | covered | preview not in history until release |
| AC-18 | covered | create and move undo/redo; `can_undo`/`can_redo` exposed |
| AC-19 | covered | save → load → save stable (validated) |
| AC-21 | covered | machine is the only gesture owner; no shell mode flags |
| AC-23 | covered | boundary check passes; `gip-interaction` row and ADR-012 |
| AC-26 | covered | outline and hover colors and opacity read from tokens |
| AC-27 | covered | outline is 1 px in viewport pixels at every zoom |
| AC-28 | covered (here) | `R`, Escape, Delete tested in E2E; 020c adds the registry round trip |

AC-20 (no-conversion lint) is extended to `shortcuts.ts`. The rule still passes.

## Verification

- Rust: `cargo test --workspace` passes (all crates; 169 tests in total; the 23 new interaction tests are in `gip-session`);
  clippy `-D warnings`, fmt, wasm32 check, `check-boundaries.py` pass.
- Build: `scripts/build-wasm.sh` (run with `PATH=$HOME/.local/bin:$PATH`; wasm-pack lives there).
- App: typecheck, Vitest 59 pass, `vite build` pass, Playwright 11 pass (3 canvas-view, 6 canvas-edit, 2 engine).

## Open items for 020c

- AC-11 zero-size minimum tolerance (and AC-10 stroke tolerance once strokes exist).
- AC-24 performance fixture (1,000 rectangles) and frame-time measurement.
- Keyboard registry round-trip E2E (AC-28 full), and any wider shortcut set.
- Marquee and selection handles are out of scope for Slice 0.
