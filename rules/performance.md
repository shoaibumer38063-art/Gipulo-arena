# PERFORMANCE RULES — Budgets, measurement, and the "no lag" law

> Status: ACTIVE (v1, 2026-10-11). Sources: vision doc §9 performance documentation (`03`),
> canvas rules §26 (canvas budgets bind here too), DS brief (performance affects UI design).

## P1 — The law

- **A feature is not truly working if it technically works but creates lag.** Performance is a
  functional requirement, verified like any behavior.
- No change may solve its own goal by creating a performance problem elsewhere (global rule G8).
  "It's fast on my empty test file" is not evidence — see P3.

## P2 — Budget surface (product-level; canvas-specific numbers in canvas rules §26)

Define and maintain expectations for: canvas FPS (pan/zoom/drag) · pointer-response latency ·
hover feedback · selection operations on large multi-selections · large-document open/save ·
vector editing on complex paths · layout reflow · file loading/saving · generated-code preview
build+run · memory footprint · effect-heavy rendering · media decoding/playback.
Budget values are [PROPOSED] until the calibration ADR (canvas rules §26) — until then the canvas
§26 table is the binding acceptance target and new areas get budgets in their feature docs.

## P3 — Measurement discipline

- **Reference documents only**: performance claims require runs on the committed heavy fixtures
  (10k mixed nodes · 1k-point vector · 500 frames + images · deep nesting 10×500 · 100
  effect-heavy layers). Empty-canvas or 10-node demos are inadmissible.
- Measure before/after for any change touching: render loop, geometry engine, layout engine,
  hit-testing/snapping paths, serialization, list UIs (layer panel), or memory caches. Record
  numbers in the task summary.
- Profile first, optimize second: no speculative optimization without a measurement showing the
  hot spot; no optimization that violates architecture boundaries (canvas rules §2, rendering
  rules R1) — a fast wrong-layer fix is a rejected fix.

## P4 — Design-level performance (prevention beats cure)

- UI: no blur/multi-shadow/complex filters on frequently repeated components (layer rows,
  property rows); virtualize long lists; transitions only on cheap properties (transform/opacity/
  background-color) per DS motion budget (hover 80ms etc. — motion must never make the editor
  feel slow).
- Canvas: culling + LOD + dirty-region invalidation are the default posture (rendering rules
  R2–R3); overlays batch separately; screen-space chrome is cheap by construction (1px lines,
  solid fills).
- Data: caches bounded and invalidation-correct; no per-frame allocation storms; heavy compute
  off the interaction path with cancellation (rendering rules R3).
- Media: nothing expensive runs for invisible content (video off-screen paused, images decoded
  on demand, previews suspended when hidden).

## P5 — Regression protection

- Budget suite runs on every change touching P3 areas; regressions block completion (the task is
  not done — documentation update rule includes performance verification).
- Each fixed performance bug gets: measurement evidence in the bug record, a benchmark added to
  the suite where practical, and root cause documented (not just the patch).
- Known-acceptable costs (e.g. first-open tessellation spike) are documented with numbers in the
  relevant architecture/feature doc — undocumented spikes are bugs.

## P6 — Honest trade-offs

- If a genuinely required feature cannot meet budget: measure alternatives, write an ADR
  (decision, numbers, consequences), and get user visibility for product-affecting trade-offs
  (global rule G10). Silent degradation, fake numbers, and "optimize later" TODOs are banned.
