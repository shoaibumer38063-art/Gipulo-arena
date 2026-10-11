# VECTOR GEOMETRY RULES — Canonical geometry law

> Status: ACTIVE (v1, 2026-10-11). Sources: `docs/memory/06_canvas_rules_requirements.md`
> (Vector geometry section), canvas rules §5–6. Applies to the geometry engine and every consumer.

## VG1 — One canonical path model

- The document stores vector paths in ONE canonical representation owned by the geometry engine:
  subpaths of segments; curves normalized to **cubic Béziers** (quadratics converted on import;
  arcs/other primitives converted at boundaries) [REC — ADR to confirm at implementation design].
- Every path carries explicit data: closed/open flag (never inferred from coincident endpoints),
  winding/fill rule, per-point type (corner/smooth), handle mirroring mode (mirrored/asymmetric/
  disconnected), stroke alignment where relevant.
- Parametric shapes (rect/ellipse/star/polygon…) store PARAMETERS; their geometry is produced
  through the same engine interfaces (`getGeometry()`), never bypassed.

## VG2 — One canonical operations library

All of these exist exactly once, in the geometry engine, and are used by every feature:
- measurement: length, nearest-point-on-path (with parameter t), bbox (tight + axis-aligned),
  point-in-fill, distance-to-stroke;
- editing: insert point (at t), delete point, move point/handle, split, join, close, reverse,
  change point type, curve/straighten segment, outline stroke, offset path;
- booleans: union/subtract/intersect/exclude (non-destructive operand model per canvas rules
  CAN-BOOL-1); flatten (explicit command only);
- transforms: apply/rebase transforms on path data through the transform system only.
**NEVER:** a feature re-implementing Bézier approximation, flattening, or bbox privately — even
"slightly differently for its needs." Need different tolerance? Pass a tolerance parameter.

## VG3 — Tolerance & precision policy

- Fixed, named tolerances in ONE constants module (canvas rules CAN-NUM-5): display ε, comparison
  ε, path-coincidence ε; flattening tolerance for RENDERING scales with zoom (LOD, canvas rules
  CAN-RENDER-3) while HIT-TEST and EXPORT use fixed tolerances — display coarsening must never
  change what is clickable or exported.
- Internal values are float64; operations must not round intermediates. Committed results may be
  cleaned (e.g. exact snap values, integer degrees after constrained rotate) ONLY at gesture end
  (canvas rules CAN-TR-6).
- Repeated-op stability: `op⁻¹(op(x)) == x` within ε for transforms; boolean/flatten round-trips
  tested on golden paths; no error accumulation across 1000 repeated ops (test requirement).

## VG4 — Non-destructive discipline

- Booleans keep operands; masks keep content; parametric shapes keep parameters; text stays text.
  Destructive conversion (flatten, outline stroke, rasterize, vectorize text) happens ONLY via
  explicit user commands, each a single history entry, each documented in its feature doc.
- Generated geometry (booleans result, parametric output, offsets) is a CACHE: deterministic from
  sources, invalidated on source change, never hand-edited in place — editing converts to plain
  vectors first (explicit, one-way, per canvas rules CAN-SHAPE-2/CAN-PARAM-1).

## VG5 — Consistency across consumers

- Rendering, hit-testing, snapping, selection bounds, vector-edit UI, export, and code generation
  all consume the SAME geometry results (canvas rules CAN-GEOM-1). A visual outline that doesn't
  match the clickable region is a P1 defect.
- Open paths: hit region = stroked path (with documented tolerance); fill of open paths follows
  the fill rule as if closed per standard behavior — documented in the paths feature doc.
- Compound paths/multiple subpaths: fill rule (nonzero/evenodd) is explicit document data; all
  consumers respect it identically.

## VG6 — Vector networks [future-gated]

- Branching vector networks (Figma-style) are a PLANNED capability (vision doc lists them "if
  supported"). No partial/private network code may land before an ADR defines the data model;
  until then, paths are the only vector structure and features must not assume network readiness.

## VG7 — Testing requirements

- Golden path fixtures: straight, cubic, mixed open/closed, self-intersecting, coincident points,
  tiny (<1e-3), huge (1e5), negative-coordinate, many-subpath, degenerate (zero-length segment).
- Unit tests per VG2 operation on those fixtures; property tests for round-trip stability (VG3);
- parity tests: rendered pixels vs geometry bounds (canvas rules CAN-GEOM-6); hit-test matrix.
- Any vector bug fix ⇒ fixture added to the golden set (regression law, canvas rules CAN-TEST-1).
