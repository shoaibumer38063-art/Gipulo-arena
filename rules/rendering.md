# RENDERING RULES — Renderer boundaries, invalidation, and performance law

> Status: ACTIVE (v1, 2026-10-11). Sources: canvas rules §21 (+ §1 public research),
> `docs/memory/06_canvas_rules_requirements.md` (Rendering Architecture section).

## R1 — Architecture boundary

- Pipeline: document model → scene representation (retained) → renderer backend. The renderer
  consumes geometry + paint from the geometry engine/document; it NEVER writes back into document
  or geometry state, and canvas logic NEVER depends on backend-specific behavior.
- A **graphics abstraction layer** isolates the backend (public Figma pattern: one C++ interface,
  WebGL+WebGPU implementations; Penpot: Rust/Wasm/Skia). Gipulo's backend is DECIDED by
  ADR-001 (ACCEPTED — Option B): a Rust/Wasm engine core rendering through **wgpu** (browser
  WebGPU pass-through; WebGL2 via wgpu's `webgl` feature + Naga translation). The abstraction
  is wgpu itself; these rules bind regardless of backend, and a backend swap must remain
  possible without redesigning canvas logic.
- The **export renderer** is a separate consumer of the same geometry/paint truth: export output
  must match on-canvas rendering within documented tolerance (parity tests, canvas rules §25) —
  exporters never re-implement painting rules privately.

## R2 — Invalidation & caching

- Dirty-region invalidation: a mutation marks affected regions/nodes; only those re-tessellate/
  redraw. Full-page re-render on a single-node edit is a performance defect.
- Caches (tessellation, GPU resources, text runs, image textures, effect renders) are: keyed
  deterministically, invalidated with their sources, bounded (eviction policy documented), and
  never a source of truth — cache loss must be invisible except as a performance blip.
- Camera changes (pan/zoom) reuse cached scene data at new transforms where possible; they never
  trigger document-level recomputation.

## R3 — Frame discipline

- The pointer→pixel loop is synchronous and lean: input → state machine → transform/geometry
  delta → invalidate → draw. Heavy work (booleans, big flattens, decoding, effect builds, code
  preview) is async/worker-side with cancellation when the gesture changes (canvas rules
  CAN-RENDER-4).
- Per-frame allocations minimized: pools/reuse for transient geometry buffers; batching by
  material/texture; overlays batch separately from document content; draw calls kept proportional
  to visible complexity, not document size (culling + LOD per canvas rules CAN-RENDER-3).
- Culling: only viewport(+margin) content draws; LOD coarsens DISPLAY flattening at low zoom —
  canonical geometry precision is untouched.

## R4 — Correctness invariants

- Deterministic output: same document + same camera + same DPR ⇒ same pixels (no time-, random-,
  or iteration-order-dependent rendering).
- DPR handling: device-space scaling only inside the renderer (canvas rules CAN-NUM-4); 1px
  editor chrome uses half-pixel alignment for crispness [DS].
- Effects isolation: a failing shader/effect degrades its layer (fallback paint + error badge),
  never crashes or corrupts the renderer (canvas rules CAN-FX-1). GPU resource exhaustion paths
  are handled (release caches, retry, inform user) — not silent blank canvas.
- Blend/composite order = document order within stacking contexts (canvas rules §20); group
  opacity flattens the group result.

## R5 — Media & resources

- Images: async decode, progressive placeholders, tiled/LOD for large images, color handling per
  file-format ADR; missing assets render the documented missing-asset state.
- Video: GPU texture lifecycle managed explicitly (allocate on visible play, release off-screen);
  off-screen/deselected video pauses; no expensive media work runs for invisible content
  (canvas rules CAN-MEDIA-2).

## R6 — Performance verification

- Every renderer-touching change runs the budget suite on the reference heavy documents
  (canvas rules §26: 10k nodes, 1k-point vector, 500 frames + images, deep nesting, heavy
  effects). Empty-canvas measurements prove nothing.
- Regressions block merge; honest budget conflicts require an ADR with measured alternatives
  (canvas rules CAN-BUDGET-2).

## R7 — Future-proofing notes (binding on design, not on schedule)

- WebGPU-class features (compute-assisted tessellation, effect pipelines) may be adopted later;
  the abstraction layer (R1) exists so adoption is an implementation swap, not a rewrite.
- The Wasm engine core (Figma-style, from R7's original note) is no longer a future option — it
  IS the architecture (ADR-001 ACCEPTED, Option B): Rust crates under `core/` + TS/React shell;
  boundary law: `docs/architecture/engine-core.md`. The remaining future-proofing concern is
  unchanged: no code may weld canvas logic to a specific backend's quirks (wgpu is the
  isolation layer).
