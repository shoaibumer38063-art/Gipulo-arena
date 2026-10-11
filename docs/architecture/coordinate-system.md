# Coordinate System & Camera — Architecture Contract

> Status: DESIGN v1 (2026-10-11), shaped by ADR-001 (ACCEPTED: Rust/Wasm core + TS shell). Binding rules: CAN-COORD (all),
> CAN-CAM (all), CAN-NUM-3/4. Hub: `canvas.md`.

## 1. Purpose & scope

ONE authoritative module for all coordinate spaces and conversions, plus the camera store
(pan/zoom). Every pointer position, overlay anchor, ruler tick, snap candidate, and export region
passes through this module. A second conversion implementation anywhere is a critical violation
(CAN-COORD-2).

## 2. The spaces (CAN-COORD-1, canonical names — use exactly these)

`screen` (client px) · `viewport` (px relative to canvas element) · `world` (infinite page space) ·
`parent` (relative to parent's transform) · `local` (object's own untransformed space) ·
`path` (pre-transform path coordinates) · `text` (shaping/run space, text engine internal) ·
`device` (physical px = viewport × DPR; renderer-internal only).

## 3. Owns

- Conversion API (canonical surface; names mirror the public tldraw pattern):
  `screenToViewport / viewportToScreen · screenToWorld / worldToScreen · worldToLocal /
  localToWorld · worldToParent / parentToWorld · viewportToDevice` (device only inside renderer).
- Matrix utilities: compose/decompose 2D affine (translate, rotate, scale, flip), accumulate
  ancestor chains (root→leaf), invert; rotation-aware full-matrix paths (CAN-COORD-5).
- Camera store: `{x, y, zoom}` per viewport + commands: `zoomAround(screenPoint, factor)`,
  `zoomToFit(bounds)`, `zoomToSelection()`, `panBy(dx, dy)`, `setZoom(z)` with clamps
  (CAN-CAM-3 [PROPOSED] 2%–4096%), preset stops for UI.
- Viewport registry: canvas element rect, DPR, resize observation → all screen-space derivations
  recompute on change (CAN-CAM-5).

## 4. Conversion order (CAN-COORD-3, binding)

`device = viewport × DPR` (renderer only) → `screen→viewport` (element offset) →
`viewport→world` (camera: translate + scale) → `world→parent→local` (accumulated ancestor
transforms) → `local/path` (object data). Composite conversions call the chain; no partial math.

## 5. Must never

- Store baked screen positions (overlays convert per frame, CAN-COORD-4).
- Let camera changes write document data or vice versa (CAN-CAM-1, CAN-ARCH-5).
- Round world values during conversion (float64 throughout; display rounding is a UI-layer
  format function only, CAN-NUM-2).
- Use axis-separated shortcuts on rotated/transformed content except documented+tested fast paths
  (CAN-COORD-5).

## 6. Invariants

- Round-trip: `A→B(B→A(p)) ≈ p` within 1e-9 world units for all space pairs, at zoom 0.05–32,
  any rotation/flip/nesting (CAN-COORD invariant).
- Pointer-zoom invariance: world point under cursor stays under cursor within 0.5 screen px
  through wheel/pinch/keyboard-zoom gestures (CAN-CAM-2).
- Rulers, guides, snap lines, overlays all agree with this module — they have no private math
  (CAN-RULER-1).

## 7. Dependencies

- Consumes: viewport DOM measurements (only the module itself touches DOM for this), camera
  commands from input-system, node transforms (read-only) from document model.
- Consumed by: literally everything spatial — input/hit-testing, selection, snapping, overlays,
  rendering, layout (parent-space resolution), export, a11y mirror positioning.

## 8. Open questions → resolved by ADRs

1. ✅ **DECIDED — ADR-002 (PROPOSED 2026-10-11):** matrix convention = column-major 6-tuple
   (`Mat2 {a,b,c,d,tx,ty}`, CSS/SVG/Canvas2D order); transform storage = decomposed canonical
   fields (x·y·w·h·rotation·flipH·flipV). One constants module here for all epsilons.
2. ✅ **DECIDED — ADR-003 (PROPOSED 2026-10-11):** per-view camera persistence is app-level
   workspace state, NOT `.gip` document data, in v0.
3. Momentum/inertia panning: v1 = off (CAN-CAM-4 [PROPOSED]); revisit with measurement.
4. Device-space precision policy under fractional DPR — implementation detail; constants live in
   the module (CAN-NUM-5), tested at DPR 1/1.5/2/3.

## 9. Test focus

Property-based round-trips (all pairs × zoom × rotation × flip × nesting × fractional/negative
coordinates × DPR 1/1.5/2/3); pointer-zoom invariance across gesture sequences; clamp behavior;
viewport resize/DPR change recompute; camera-never-dirties-document; conversion performance
(budget: conversions are hot — micro-benchmark in budget suite, canvas §26).

## 10. Implementation status (2026-10-11 — coordinates phase realized, task 009)

**Realized in `gip-coords`** (against ADR-002, PROPOSED — binding as implementation default):

- `Mat2` — the ONE 2D affine type, column-major 6-tuple (CSS/SVG/Canvas2D order), composition,
  inverse (singular ⇒ `None`, zero-size nodes are legal), local↔world helpers.
- `Trs` — canonical decomposed fields with `T·R·S` composition, matrix decomposition (returns
  `None` for singular or skewed matrices), and the canonical **math-frame** normalization
  (flips folded to `flip_v`; the *stored* form keeps user-authored flags — module docs explain).
- `Camera` — world⇄viewport math, `pan_by_*`, `zoom_around_viewport` (invariance preserved even
  at the clamps), 2%–4096% (`MIN_ZOOM`/`MAX_ZOOM`).
- `ViewportRect` + conversions — `screen↔viewport`, `viewport↔device`, composite
  `screen↔world`; the shell measures DOM, the core computes (CAN-COORD-2/3).
- `angle::normalize_deg` (canonical (−180, 180], bit-stable for snapped values, `−0.0` collapsed)
  and `eps` — the single epsilon module (CAN-NUM-5).

**Verified (native `cargo test`, 38 tests in this crate):** all-pairs round-trips within
`EPS_ROUNDTRIP` across the full zoom range, fractional/negative coordinates and DPR 1/1.5/2/3 ·
pointer-zoom invariance ≤ 0.5 px through gesture sequences incl. clamp boundaries ·
screen-constant pan law · decompose/recompose matrix identity + canonical field identity ·
1000× apply→revert cycle stability within `EPS_MATRIX` · nested-chain (depth ≤ 4) round-trips ·
angle table + property (range, idempotence, rotation equivalence ≤ 1e6°).

**Still deferred (deliberate, with owners named):** `zoomToFit`/`zoomToSelection` (need geometry
bounds — geometry phase) · viewport registry DOM observation (shell measures; core receives
numbers — boundary/renderer phase) · momentum panning (off in v1, CAN-CAM-4) · conversion
performance micro-benchmarks (budget suite, canvas §26). **`camera-never-dirties-document` is
structural:** `gip-coords` has no document types at all, so no code path can touch one.
