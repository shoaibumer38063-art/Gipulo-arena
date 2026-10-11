# Rendering Engine — Architecture Contract

> Status: DESIGN v1 (2026-10-11), shaped by ADR-001 (ACCEPTED — Option B: renderer lives inside
> the Rust/Wasm core via wgpu). Binding rules: `rendering.md` (R1–R7), CAN-RENDER, CAN-OVL,
> CAN-PERF. Hub: `canvas.md`.

## 1. Purpose & scope

Turn (document + geometry + camera) into pixels: the retained scene, the graphics abstraction,
backend implementations, invalidation/caching, the screen-space overlay layer, and the export
renderer. Consumes truth; never writes it (R1).

## 2. Pipeline (stages are module boundaries)

```
document+geometry ─► SCENE (retained nodes: paint ops, transforms, bounds, cache keys)
                 ─► CULL/LOD (viewport+margin; zoom-keyed display tolerance)
                 ─► TESSELLATE/PREP (paths→GPU-ready; text runs; images; effects graph)
                 ─► DRAW (batched by material/texture; document layer)
                 ─► OVERLAY DRAW (screen-space editor chrome, separate batch)
                 ─► PRESENT (DPR-aware; half-pixel alignment for 1px chrome)
```

## 3. Owns

- **Graphics abstraction interface** (R1, CAN-RENDER-1): resource creation (buffers/textures/
  pipelines), explicit draw calls (Figma's public lesson: explicit draw arguments prevent
  stale-binding bugs), shader representation with backend translation strategy (their public
  GLSL→WGSL processor is the precedent; Gipulo's shader source-of-truth language ADR'd at impl).
- **Backends (decided — ADR-001 Option B):** wgpu inside the Rust core — browser WebGPU
  pass-through when available; `webgl` feature + Naga WGSL→GLSL translation for WebGL2 fallback;
  backend negotiation via wgpu adapter/backend selection (Figma's dynamic-fallback posture).
  Canvas2D surface allowed ONLY for debug/bootstrap (pre-decision Option D rejection stands).
- **Scene & invalidation** (R2): dirty-region tracking from transaction effects + camera changes;
  caches (tessellation, textures, text runs, effect renders) keyed deterministically on geometry
  versions (vector-engine §4), bounded with eviction policy, loss-invisible.
- **Overlay layer** (CAN-OVL): renders overlay *descriptors* submitted by tools/guides engine
  (typed + tokenized — no raw draw calls from features, CAN-OVL-2); screen-space sizing per
  CAN-SS; batched separately; never exported/serialized/selectable.
- **Effects pipeline** (CAN-FX-1): per-layer effect graph (shadows/blurs/blend isolation per
  CAN-Z-2); failure isolation (fallback paint + error badge, renderer never corrupted);
  effect regions feed visualBounds-based caching only.
- **Media resources** (R5): async image decode → texture lifecycle (tile/LOD for large images);
  video textures allocated on visible play, released off-screen; missing-asset state rendering.
- **Export renderer:** separate consumer of the same scene truth (R1); export = render with
  fixed tolerances, no chrome, no LOD coarsening; parity with on-canvas within documented
  tolerance (parity tests, canvas §25).

## 4. Must never

- Write document/geometry/selection/camera state (R1); read pixels back for logic except
  user-facing pickers (CAN-ARCH-3).
- Full-scene redraw for single-node edits (dirty-region law R2) or re-tessellate on camera-only
  changes (reuse cached scene at new transforms, R2).
- Run heavy work in the pointer→pixel loop (async/worker + cancellation, R3/CAN-RENDER-4).
- Produce nondeterministic pixels (R4: same doc+camera+DPR ⇒ same pixels).
- Let chrome visuals scale with zoom (CAN-SS) or document content snap to pixels automatically
  (CAN-NUM-3).

## 5. Frame contract

- One rAF-driven frame loop; input events mark state, the loop draws once per frame max
  (coalescing multiple invalidations); overlay pass after document pass; DPR/resize changes
  re-derive device-space params before next present (CAN-CAM-5).
- Budget wiring: frame instrumentation points (cull count, draw calls, tessellation ms, cache
  hit rate) feed the budget suite (canvas §26, performance P3) — instrumentation is part of the
  architecture, not an afterthought.

## 6. Dependencies

- Consumes: document model, geometry engine (paths/bounds/flatten/versions), camera+coordinates,
  layout results, text-engine run geometry, Design System tokens (chrome colors/sizes).
- Consumed by: app shell (canvas element host), export, code-preview parity tooling (compares
  rendered design vs runtime preview — vision Solution 2), perf fixtures.

## 7. Invariants

- Backend-swap invariance: canvas logic and tests pass identically on WebGPU and WebGL2 backends
  (within documented parity tiers); tests run against the abstraction + both backends in CI
  (when CI exists).
- Cache-loss safety: dropping any cache changes performance only (R2).
- Effect isolation: killed/failing effect ⇒ degraded layer, healthy renderer (CAN-FX-1).

## 8. Open questions (ADR-gated)

1. Shader source-of-truth: **WGSL** (constrained by wgpu; Naga translates for the WebGL2
   fallback) — per-shader strategy decided at feature impl.
2. WebGL2 fallback parity tiers (which effects degrade how) → ADR at impl.
3. Tiling strategy for very large documents (tile size, cache residency) → ADR at impl.
4. Text rasterization path (SDF/GPU atlas vs CPU raster) → ADR with text engine.
5. Color management (sRGB display pipeline, wide-gamut policy) → ADR with file-format (media
   color handling, CAN-MEDIA-1).

## 9. Test focus

Golden-image snapshots per shape/effect/blend class on both backends · invalidation correctness
(edit one node ⇒ minimal dirty region; measured) · culling/LOD correctness (nothing visible
  missing; LOD never changes hit/export) · determinism (repeat-render pixel identity) · effect
  failure isolation (fault injection) · media lifecycle (off-screen video paused/released) ·
  export parity · budget suite on the five reference documents (canvas §26).
