# Task 020a — Editor view boundary, canvas host, pan and zoom

- **Date:** 2026-10-11 · **Plan:** `2026-10-11_020_editor_slice0_plan.md` · **Roadmap:** step 19 (020a)
- **Decisions:** ADR-012 (D1 Canvas2D bootstrap, D2 `gip-interaction` planned, D3 typed boundary).
- **Status:** DONE for 020a. Slice 0b (rectangles, selection, move, cancel, undo) is next.

## Delivered
- `gip-session`: `Session` (viewport rect, camera), `ViewSnapshot`, validated `set_viewport` / `zoom_at` / `pan_by`, snapshot with zoom percent and origin. Browser binding: `Engine` + `ViewState` (transport only). Protocol bumped to **1**.
- Shell: `engine/client.ts` (typed handle, plain view copies), `canvas/CanvasHost.tsx` (raw input forwarding, non-passive wheel, ResizeObserver viewport sync), `canvas/paint.ts` (pure painters), `App.tsx` (canvas above the gallery).
- Token added: `--canvas-host-h` (tokens.css + tokens.ts, drift test green).
- `docs/architecture/canvas-implementation.md` (implementation map; the hub `canvas.md` is unchanged) and ADR-012.

## Acceptance criteria covered (plan §4)
- AC-1 round trip: covered by gip-coords tests (existing) and anchor test.
- AC-2 anchor invariance: `pointer_zoom_keeps_the_anchor_point_fixed`.
- AC-3 pan/zoom never edit the document: no document in session; E2E confirms no edit path.
- AC-4 clamp: `zoom_is_clamped_to_the_camera_range` (2 %–4096 %, per CAN-CAM-3 as implemented).
- AC-20 no conversion formulas in shell: `paint.test.ts` boundary lint.
- AC-21 mode: **partially** — the transitional pan gesture variable is a recorded deviation until Slice 0b.
- AC-22 protocol bump: done (0 → 1; client mirror and tests updated).
- AC-26 tokens only: CSS uses tokens; new token added.

Not covered yet (Slice 0b/0c): AC-5..19 (documents), AC-23..25 (boundary/lint for gip-interaction, performance), AC-27..28 (keyboard, zoom-visible handles).

## Verification
- `cargo test --workspace`: 146 passed, 0 failed. `cargo clippy --all-targets -D warnings`, `cargo fmt --check`, wasm check, boundary check: pass.
- `npm run typecheck`: pass · `npm test` (Vitest): 55 passed · `npm run build`: pass.
- `npm run test:e2e` (Playwright, Chromium): 5 passed (start state, middle-drag pan, ctrl-wheel zoom, handshake, engine-load failure).

## Open
- Owner review of ADR-012 (non-blocking).
- Push to GitHub pending a token.
