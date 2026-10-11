# LAYOUT RULES — Auto Layout, constraints, and Responsive Vector Segments law

> Status: ACTIVE (v1, 2026-10-11). Sources: vision doc Problems/Solutions 1–2 (`02`), canvas
> rules (CAN-FRAME-1, CAN-GRID-1, CAN-TR-3), DS brief. Layout is a document-level engine — not a
> rendering side-effect and not per-feature math.

## L1 — One layout engine

- All automatic positioning (Auto Layout rows/columns/grids [as supported], constraints on frame
  resize, hug/fill/fixed sizing) is computed by the layout engine from DOCUMENT data. Layout
  results are derived state: recomputed on input change, cached with invalidation, never stored
  as hand-editable positions for laid-out children.
- Rendering, selection, snapping, hit-testing, and export consume layout RESULTS through the
  standard geometry interfaces — no consumer re-implements layout math to "predict" positions.
- Visual grids are NOT layout (canvas rules CAN-GRID-1). Only the layout engine positions content.

## L2 — Contracts

- **Frame** = the layout container: own coordinate space, clipping, constraints, optional Auto
  Layout; resizing a frame reflows children per constraints/Auto Layout — the transform system
  must not hand-move laid-out children (canvas rules CAN-TR-3).
- **Group** = never a layout container: bounds derive from children; no reflow semantics.
- Sizing modes per axis: fixed / hug-contents / fill-container (DS segmented control:
  Fill · Hug · Fixed). Mode is explicit document data; mixed-mode resolution order is documented
  in the layout architecture doc and deterministic.
- Negative gaps/padding: supported where the product spec says so (vision doc lists "negative
  spacing where supported"); behavior at extremes (overlap order = child order) is deterministic
  and tested.

## L3 — Responsive Vector Segments (signature feature — product law)

From the vision doc (Solution 1); these behaviors are product requirements, not suggestions:
- **RVS-1** Responsiveness affects ONLY straight vector segments the designer explicitly
  activates. Curves, cutouts, inverted corners, decorative geometry stay geometrically preserved
  — stretching them is the exact bug this feature exists to kill.
- **RVS-2** Interaction contract: toggle on a selected vector → eligible straight segments
  highlighted (red/inactive affordance) → user clicks segments to activate (turn green) → only
  activated segments absorb dimensional change on resize/layout reflow. (Final colors must map to
  DS tokens via an ADR — red/green here are the vision doc's descriptive colors; the guide-color
  language CAN-GUIDE-1 stays closed, so RVS affordances get their own DS-defined treatment.)
- **RVS-3** Data model: activation flags are path-segment document data, serialized, undoable,
  instance-overridable per the component rules; layout/engine consumers read them through the
  geometry engine — no private stretch math in the layout engine's callers.
- **RVS-4** Semantics: activated segments behave like constrained stretch regions (9-slice-style
  selective resizing on editable vectors). Redistribution across multiple activated segments,
  min/max extents, and behavior under Auto Layout hug/fill are design decisions requiring feature
  doc + ADR BEFORE implementation; until then the feature stays `planned` in the registry.
- **RVS-5** Communication rule (user-mandated): never describe RVS as Flutter `CustomClipper`;
  it is "selective stretch regions / constrained vector resizing / 9-slice-style behavior applied
  directly to editable vector segments," made visual — no manual slice calculation by the user.
- **RVS-6** Code representation: per the native design-to-code mandate (vision Problem 2), RVS
  must have a planned code equivalent (e.g. generated CSS/asset strategy for stretch regions) —
  defined in the code-generation architecture doc before the feature ships.

## L4 — Constraints

- Classic constraints (left/right/top/bottom/center/scale per axis) apply on frame resize for
  non-Auto-Layout children; constraint math lives in the layout engine with canonical bounds
  (CAN-GEOM-2); rotated children resolve constraints in parent space deterministically (documented
  behavior + tests).
- Constraints + Auto Layout interaction is explicitly defined (Auto Layout wins on laid-out axes)
  — no silent double-application.

## L5 — Determinism, stability, testing

- Layout is deterministic: same document ⇒ same layout everywhere (editor, preview, export,
  code generation). Iteration order = document child order; no float-accumulation drift across
  reflows (recompute from inputs, don't incrementally patch positions).
- Layout must be correct at fractional/negative values (CAN-NUM-1) and stable under repeated
  resize cycles (resize 1000× returns to original geometry within ε).
- Required tests: hug/fill/fixed matrix per axis · wrap behaviors (as supported) · padding/gap
  incl. negatives · constraint matrix · frame resize reflow · group bounds recompute · RVS
  stretch-region cases (once implemented) · layout ⇄ selection/snapping/export parity.
- Performance: reflow of a 1k-child Auto Layout subtree stays within budget (performance rules +
  canvas budgets); invalidation is subtree-scoped, never whole-document.
