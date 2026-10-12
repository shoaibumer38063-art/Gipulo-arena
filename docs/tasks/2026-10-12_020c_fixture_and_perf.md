# Task 020c — Performance fixture, zero-size hit tolerance, keyboard round trip (Slice 0c)

**Status:** done locally. Not pushed in this turn (no `GITHUB_TOKEN` was set; see the chat).
**Plan:** `2026-10-11_020_editor_slice0_plan.md` (AC-24, AC-26..28; AC-11 carried over from 0b).

## What shipped

- **AC-11 zero-size hit tolerance (CAN-GEOM-5).** A zero-width or zero-height object is hit within
  `HIT_TOLERANCE_PX` = 8 screen px of its degenerate outline, at any zoom. The value is [PROPOSED].
  Test: `zero_width_object_is_addressable_within_the_screen_tolerance` (zooms 5 %, 100 %, 3200 %).
- **Committed fixture `fixtures/rects-1000.gip`** (AC-24): 1,000 rectangles, generated deterministically
  by `gip-session/src/fixture.rs`. A test fails if the committed file drifts from the generator.
  Open → save is byte-stable. Also copied to `app/public/fixtures/`.
- **Boundary additions** (protocol 2 → 3): `open_json`, `save_json` on `Engine`. The shell opens the
  fixture with `?fixture=rects-1000`.
- **Frame coalescing in `CanvasHost`.** The first measurement failed the budget: pan p95 was 33.3 ms
  because every pointer event re-rendered React and repainted everything. The shell now reads and
  repaints once per animation frame. The core still applies each event immediately. The budget was
  not loosened. After the change, pan p95 is 16.7 ms and drag p95 is 16.8 ms.
- **Keyboard (AC-28):** the registry in `shortcuts.ts` drives R, V, Delete, Backspace, Ctrl/⌘+Z,
  Ctrl/⌘+Shift+Z, and Escape. Unit tests cover the mapping. E2E covers the round trip.

## Acceptance coverage

| AC | Status | Evidence |
|---|---|---|
| AC-11 | covered | zero-width hit test at three zooms (`session.rs`) |
| AC-24 | covered, partly | fixture + p95 frame interval for pan and drag (`canvas-perf.spec.ts`, headless). Pointer→visual latency in frames is not measured; the browser run is headless software rendering. |
| AC-25 | N/A | no snapping in Slice 0 |
| AC-26 | covered | outline and hover colors and opacity come from tokens (020b) |
| AC-27 | covered | 1 px outline (020b) |
| AC-28 | covered | registry unit tests + `canvas-edit.spec.ts` E2E |

## Verification

- Rust: 172 tests pass. Clippy `-D warnings`, fmt, wasm32 check, `check-boundaries.py` all pass.
- App: typecheck, Vitest 59 pass, `vite build` pass, Playwright 13 pass.
- Measured (headless Chromium, this sandbox): `rects-1000` pan p95 16.7 ms over 66 frames;
  drag of a selected rectangle p95 16.8 ms over 67 frames.

## Open items

- Fixture performance is measured only on the 1,000-object fixture. The 10k fixture is a follow-up
  (plan, AC-24).
- Pointer→visual latency is not measured in frames.
- Headless numbers may differ on real hardware or GPU-backed browsers.
