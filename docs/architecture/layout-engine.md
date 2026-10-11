# Layout Engine — Architecture Contract (Auto Layout · Constraints · RVS)

> Status: DESIGN v1 (2026-10-11), shaped by ADR-001 (ACCEPTED: Rust/Wasm core + TS shell). Binding rules: `layout.md` (L1–L5),
> CAN-FRAME, CAN-GRID-1, CAN-TR-3. Hub: `canvas.md`.

## 1. Purpose & scope

Computes ALL automatic positioning from document data: Auto Layout (direction, gap, padding,
alignment, sizing modes, wrap as supported), classic constraints on frame resize, and the
stretch-region semantics of **Responsive Vector Segments (RVS)** — the signature feature
(vision Solution 1; product law L3). Layout results are derived state: cached, invalidated,
never hand-editable positions for laid-out children (L1).

## 2. Owns

- **Sizing model per axis:** `fixed | hug | fill` (explicit document data, L2); resolution order
  deterministic and documented (hug from children's laid-out bounds → fill from parent's resolved
  inner size → fixed as stored; cycles impossible because fill requires a resolved ancestor —
  validation surfaces a documented error state instead of infinite recursion).
- **Auto Layout algorithm:** single pass per subtree in document child order: resolve child sizes
  (recursively), distribute along main axis (gap, padding, alignment, negative gaps allowed per
  L2 with overlap order = child order), cross-axis alignment, wrap rows if/when supported
  (scope ADR-gated). Recompute-from-inputs only — never incremental position patching (L5
  drift law).
- **Constraints system:** per-child axis constraints (left/right/top/bottom/center/scale) applied
  on frame resize for non-Auto-Layout children; rotated children resolve in parent space
  deterministically (L4); Auto Layout wins on laid-out axes — never double-applied (L4).
- **RVS stretch regions (L3 contract):** consumes per-segment activation flags (path-segment
  document data) via the geometry engine; when an RVS-enabled vector's bounds change (frame
  resize, fill/hug in Auto Layout, direct resize), computes which straight segments absorb the
  delta and produces updated canonical path geometry through the geometry engine's editing ops
  (no private stretch math — L3-3). Curves/cutouts/inactive segments are geometrically preserved
  (L3-1 — the feature's entire reason to exist).
- **Layout results publication:** child positions/sizes written through transactions when
  committed (e.g. frame resize gesture end), or held as derived preview during gestures
  (transient state, SM1.4); consumers read results via geometry/document interfaces (L1).

## 3. Must never

- Let rendering, tools, or panels re-implement layout math to "predict" positions (L1).
- Treat visual grids as layout (CAN-GRID-1) or groups as layout containers (L2).
- Store layout results as authoritative document data (derived-state law, document-tree §3).
- Produce order-dependent or float-drifting results: same document ⇒ same layout on every
  platform/run (L5 determinism; iteration = child order).
- Stretch non-activated vector segments under any code path (L3-1 is absolute).

## 4. Public interface (conceptual)

- `layoutSubtree(frameId, opts)` → resolved positions/sizes (+ invalidation info).
- `onFrameResize(frameId, newRect)` → constraint/Auto-Layout reflow plan (consumed by transform
  system handoff, CAN-TR-3).
- `rvsStretch(pathNodeId, targetBounds)` → updated path geometry via geometry-engine ops.
- `validateLayout(nodeId)` → sizing-mode errors (fill-cycle, hug-of-empty) as documented states.
- Subscriptions: layout-version per subtree for scene/geometry cache invalidation (R2 pattern).

## 5. Dependencies

- Consumes: document model (frame/layout/constraint/RVS-flag data), geometry engine (child
  bounds, path ops for RVS), transforms (resize intents trigger reflow).
- Consumed by: scene/rendering (positions), selection/snapping (laid-out bounds), panels (Auto
  Layout controls read/write document data, results displayed), codegen (Auto Layout ⇒ flexbox
  mapping is the flagship code-equivalent, vision Problem 2), export.

## 6. Invariants

- Determinism & stability: 1000× resize cycles return original geometry within ε (L5).
- Fractional/negative legality honored (CAN-NUM-1): gaps/padding/sizes may be fractional or
  negative where the product spec allows; no silent clamping.
- Subtree-scoped invalidation: editing one child reflows its subtree, not the document (L5 perf).
- RVS preservation invariant: for any resize, non-activated segment geometry is bit-preserved
  (test law; this is the feature's contract with the user).

## 7. Open questions (ADR-gated)

1. **RVS redistribution semantics** (L3-4, must be ADR'd BEFORE implementation): multiple
   activated segments (proportional vs equal distribution), min/max extents, behavior under
   hug/fill, interaction with corner-radius adjacency, RVS on strokes/outlines.
2. RVS visual affordance colors: vision doc says red-highlight/green-activated; the closed guide
   language (CAN-GUIDE-1) reserves red for rulers — the RVS editing affordances need DS-defined
   treatment (L3-2 flags this explicitly) → DS decision + ADR.
3. Wrap support scope (CSS-flex-wrap-like rows?) and grid-layout ambition (vision mentions
   rows/columns/Auto Layout grid) → feature-scope ADRs.
4. Constraint rotation-edge cases matrix → documented with constraints feature.
5. Code-equivalence mapping table (Auto Layout→flex, constraints→?, RVS→stretch strategy) →
   owned with `code-generation.md` §mappings (design-to-code duty, L3-6).

## 8. Test focus

Sizing-mode matrix (fixed/hug/fill × both axes × nesting) · gap/padding incl. negatives &
fractional · alignment matrix · constraint matrix incl. rotated children · frame-resize reflow
determinism (1000×) · fill-cycle validation states · **RVS suite (once implemented): activated-
only stretch property tests, curve preservation bit-checks, multi-segment redistribution per
ADR, undo of RVS edits, RVS inside Auto Layout** · subtree-invalidation scope measurements
(1k-child reflow within budget, L5/perf P2).
