# Vector / Geometry Engine — Architecture Contract

> Status: DESIGN v1 (2026-10-11), shaped by ADR-001 (ACCEPTED: Rust/Wasm core + TS shell). Binding rules: `vector-geometry.md`
> (VG1–VG7), CAN-GEOM, CAN-VEC. Hub: `canvas.md`.

## 1. Purpose & scope

The single owner of geometric truth computation: canonical path model, the one operations
library, all bounds kinds, hit-geometry, snapping geometry, flattening/tessellation input,
spatial indexing. Rendering, selection, hit-testing, snapping, layout, export, and codegen
consume it; none re-implements geometry (VG2, CAN-GEOM-1).

## 2. Owns

- **Canonical path model** (VG1): subpaths of segments; curves normalized to cubic Béziers
  [REC, ADR-confirm]; explicit closed/open flag, winding/fill rule, point types (corner/smooth),
  handle modes (mirrored/asymmetric/disconnected), stroke alignment data consumers.
- **Parametric shape adapters**: rect/rounded-rect/ellipse/line/arrow/polygon/star produce path
  geometry through ONE interface (`getGeometry()`, `getBounds(kind)`, `hitTest()`) — parameters
  stay truth in the document; generated geometry is cache (CAN-SHAPE-1, CAN-PARAM-1, VG4).
- **Operations library** (exactly once each, VG2): length · nearest-point(t) · tight &
  axis-aligned bbox · point-in-fill · distance-to-stroke · insert/delete/move point · split ·
  join · close · reverse · point-type change · outline-stroke · offset · union/subtract/
  intersect/exclude (non-destructive) · flatten (explicit only).
- **Bounds computation** for every kind in the CAN-GEOM-2 vocabulary (geometric/stroke/visual/
  selection/transform/hit), in world/parent/oriented/axis-aligned forms.
- **Spatial index** over world bounds for culling, hit candidates, snap candidates, marquee
  (structure choice ADR-gated; tldraw's public R-tree adoption is the precedent datapoint).
  Index updates incrementally with transactions; never rebuilt per frame.
- **Caches**: flattening results (LOD-keyed for display, fixed-tolerance for hit/export),
  boolean results, offsets — deterministic keys, invalidation bound to source versions (R2).

## 3. Must never

- Round or mutate stored document geometry (precision law CAN-NUM, VG3); caches may coarsen for
  DISPLAY only (zoom-scaled LOD tolerance) — hit/export use fixed tolerances (VG3).
- Depend on DOM, GPU, camera, or UI state — the engine is pure (input: document data + query
  params; output: geometry results). Purity is what keeps it headless-testable via native
  `cargo test` in the Rust core (ADR-001 ACCEPTED — Option B; engine-core.md; CQ7).
- Accept feature-private tolerance values or private Bézier approximations (VG2 NEVER clause).
- Store results as document truth (everything here is derived/cache; document model owns data).

## 4. Public interface (conceptual)

- Queries: `boundsOf(nodeId, kind, space)` · `pathOf(nodeId)` (canonical, transform-applied on
  request) · `hitQuery(worldPoint, opts{tolerancePx→world, kinds})` → ranked candidates ·
  `candidatesInRect(worldRect)` · `nearestOnPath(nodeId, worldPoint)` · `flatten(nodeId,
  tolerance)` · `snapCandidates(worldRect, kinds)`.
- Operations (called by tools via transactions): the VG2 editing ops — each takes canonical
  inputs, returns canonical outputs + affected-region info for invalidation.
- Versioning: every node's geometry carries a version stamp from the document transaction log;
  caches key on it (invalidation correctness, R2).

## 5. Consumers & their bounds contracts (who must ask for what)

selection/transform → `selectionBounds`/`transformBounds` · hit-testing → precise `hitQuery`
(paths, not boxes; bbox-empty-space law CAN-GEOM-4) · snapping → edges/centers/anchors/paths via
`snapCandidates` from canonical bounds · rendering → LOD flatten + visualBounds for culling ·
export → fixed-tolerance flatten + visualBounds · layout → child geometricBounds · measurement
guides → bounds per CAN-GUIDE-2 ("bounds, not strokes") · codegen → canonical path data.

## 6. Invariants

- Determinism: same input ⇒ bit-identical output across runs/platforms (no iterator-order or
  FMA-variance leaks in stored results; CAN-SER-1).
- Stability: `op⁻¹(op(x)) == x` within ε (1e-6 world) for transforms; 1000× repeated ops do not
  accumulate error beyond ε (VG3).
- Parity: rendered outline vs `geometricBounds` agree within tolerance on golden docs
  (CAN-GEOM-6); hit region == visible region for fills/strokes (VG5).
- ε policy from ONE constants module (CAN-NUM-5) — engine never invents tolerances.

## 7. Dependencies

- Consumes: document model (read-only node/path/param data), transform data (via `transforms.md`
  contracts), constants.
- Consumed by: selection, input/hit-testing, snapping, rendering/scene, layout (bounds reads),
  export, codegen, a11y (descriptions of geometry), guides/measurement.

## 8. Open questions (ADR-gated)

1. ✅ CLOSED by `ADR-006` (PROPOSED): cubic-only storage; quadratics convert exactly at import;
   arcs convert at their boundary; no arc primitive stored in v1.
2. Spatial index structure (R-tree / uniform grid / hybrid) + update strategy → ADR at impl.
3. Boolean algorithm choice (clipper-class library vs custom) — dependency rule CQ1.4 applies;
   benchmark against budgets first (P3) → ADR at impl.
4. Text geometry boundary: text engine computes run geometry; this engine consumes it as paths
   for bounds/hit — interface finalized when text feature starts (CAN-TEXT-1).
5. Vector networks (VG6) — future-gated; no partial code before ADR.

## 9. Test focus

VG7 golden path fixtures (all pathologies) × every VG2 op · bounds-kind matrix (shape classes ×
rotated/nested/flipped/effects/masks/strokes) · hit-test matrix (inside/outside/on-stroke/cutout/
tiny/zero-size/low-zoom) · round-trip & accumulation stability (1000×) · determinism across runs ·
spatial-index correctness vs brute force · LOD-vs-fixed tolerance separation (display coarsening
must never change hit/export results) · perf: 1k-point path ops within budgets (canvas §26).

## 10. Implementation status (2026-10-11 — geometry phase realized, task 010; segment index added, task 013)

**Segment index (ADR-007, PROPOSED):** `gip-geometry::index::SegmentIndex` — static BVH per path,
built from tight segment bounds, nearest query equal to brute force. Measured: 1.13 ms per query
vs 142 ms brute force on a 1000-segment dense fixture (see ADR-007 caveat). Rebuilt per committed
transaction that touches the path. Document-level index is deferred to the scene phase.

## 10a. Implementation status detail (geometry phase, task 010)

**Realized in `gip-geometry` (against ADR-006, PROPOSED):**

- **Data (in `gip-document::path`, ADR-006 #8):** `Path` / `Subpath` / `Vertex` / `FillRule` /
  `PointType` / `HandleMode` — plain values, no logic.
- **Cubic math (`cubic.rs`):** evaluation, de Casteljau split, exact tight extrema, deterministic
  flattening (chord-line deviation ≤ tol, proven — see module docs), derivative, second
  derivative, quadratic→cubic conversion, straight-segment data test, nearest point via
  branch-and-bound with `nearest_within(p, bound)` for shared bounds across segments.
- **Path views (`segments.rs`):** implied segment enumeration (`n−1` open / `n` closed; closing
  segment last→first), tight bounds (anchors + curve extrema; handles excluded), flattening.
- **Hit geometry (`hit.rs`):** point-in-fill for both fill rules with open-as-closed (VG5);
  `distance_to_path` with a shared running bound across segments.
- **Operations (`ops.rs`, VG2 subset):** transform (exact for affine maps; non-finite rejected),
  reverse subpath, set closed, insert vertex (shape-exact; straight stays exactly straight),
  delete vertex (neighbours keep their handles; empty subpath removed).
- **Bounds vocabulary:** `BoundsKind` (6 kinds, CAN-GEOM-2) + `BoundsRect` (union, contains).

**Verified:** 29 unit tests · 7 golden VG7 fixture tests (all ten pathologies × measurement /
flatten / hit / ops / 1000-cycle stability / determinism / reverse-involution / shared-bound search
vs brute force) · 2 randomized property suites · clippy `-D warnings` · fmt · boundary check.

**Measured baseline (release, 2026-10-11):** `distance_to_path` on a 1000-segment closed path =
**≈10.4 ms/query** (200-query average). Within the canvas §27 p95 hover budget (≤ 32 ms) but
not comfortable; the spatial index (§8.2) is the planned fix and is the next performance item
before large documents. Hover should use the index's candidate set once it exists.

**Deferred (named owners):** spatial index (§8.2 — ADR-gated) · booleans (§8.3 — algorithm ADR) ·
outline stroke / offset (needs stroke data) · stroke and effects bounds kinds (need stroke/effect
data) · text geometry (CAN-TEXT-1, text phase) · selection/transform/visual bounds composition
(transform & selection phases) · RVS stretch (layout-engine; ADR-005 colors first) · vector
networks (VG6, future-gated).
