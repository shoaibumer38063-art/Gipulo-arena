# ADR-006 — Canonical Path Model & Geometry Data Placement

```text
Decision:   1. ONE stored path representation: subpaths of vertices. Each vertex = anchor +
               absolute in/out handle positions + point type + handle mode. Segments are IMPLIED
               between consecutive vertices (plus one closing segment when `closed`). Every
               segment is a CUBIC Bézier. Quadratics convert exactly to cubics at import;
               arcs/other primitives convert at their boundary (no arc primitive stored in v1).
            2. Closed/open is an explicit flag, never inferred from coincident endpoints.
               Fill rule (NonZero | EvenOdd) is explicit per path.
            3. Straight segment (data definition): both controls coincide with their endpoints
               (out-handle of the start vertex == its anchor AND in-handle of the end vertex ==
               its anchor). Exact data classification — no tolerance in the definition.
            4. Data placement: the path DATA types live in `gip-document` (plain, logic-free);
               the OPERATIONS live in `gip-geometry` (VG2). Layer-1 dependency law forbids
               document depending on geometry; geometry already depends on document.
Date:       2026-10-11 (authored)
Status:     PROPOSED — binding as the implementation default (D7 pattern). Owner review is
            non-blocking; ADR-gated open questions in vector-engine.md §8.1 are closed by this
            ADR as far as cubic-only normalization goes.
Refines:    vector-geometry.md VG1, VG4, VG5 · vector-engine.md §2, §8.1 · document-tree.md
            (vector-path node kind) · ADR-002 (coordinate frame, Mat2) · ADR-003 (serialization
            of path data — shape fixed here, field order decided at schema time).
```

## Context

- VG1 marks cubic normalization as `[REC — ADR to confirm at implementation design]`. vector-engine
  §8.1 asked for it to be settled in the ADR-002 bundle; ADR-002 did not cover paths, so this ADR
  closes it. The geometry phase cannot start without it: every operation's input type depends on it.
- The layer-1 law (`dependencies.md` §2) makes `gip-document` depend on nothing internal. Geometry
  depends on document. A path type owned by geometry would force document to hold geometry data it
  cannot name — a placement contradiction. Data-in-document / operations-in-geometry resolves it
  without reversing any edge.
- Requirements already fixed by rules: explicit closed flag (VG1), per-point type and handle mode
  (VG1), explicit fill rule consumed identically by every consumer (VG5), non-destructive
  parametric shapes (VG4), no network structure before an ADR (VG6).

## Precedent research (tagged)

| Source | Fact [tag] | Consequence |
|---|---|---|
| W3C SVG 2 path grammar | Path data supports M/L/C/Q/S/T/A/Z; quadratics and arcs are first-class [PUBLIC] | Import must convert Q and A; storage stays uniform |
| Figma vector paths | Figma distinguishes simple vector paths from vector networks (branching structures) [UNVERIFIED — recalled, not re-fetched this session; verify before citing externally]. Internal storage is not public → any claim about it is INFERENCE | Keep VG6 gate: no network model now; do not design against undocumented internals |
| Illustrator / Inkscape anchor models | Anchor + in/out handles + smooth/corner point types are the common editing model [PUBLIC UI convention] | Adopt vertex-with-handles, not segment-with-controls |
| Cubic-only path engines (e.g. CSS/Canvas2D path APIs) | Canvas2D exposes quadratic and cubic segments; cubic is the universal common denominator for stroking, offsetting and boolean libraries [PUBLIC API surface] | Cubic-only storage minimizes operation variants |

## Decisions

| # | Sub-decision | Chosen | Rejected | Rationale |
|---|---|---|---|---|
| 1 | Segment kind | Cubic only, stored; Q converts exactly (`c1 = p0 + ⅔(q − p0)`, `c2 = p2 + ⅔(q − p2)`) | Store quads/arcs natively | One operation per primitive; exact conversion loses no shape |
| 2 | Storage shape | `Vertex { anchor, handle_in, handle_out, point_type, handle_mode }` in ordered `Subpath { vertices, closed }` | Segment list with control points | Editing (move point/handle, insert, delete) is natural in vertex form; straight/curve status is data on vertices |
| 3 | Handle positions | Absolute world-of-path coordinates (f64) | Relative-to-anchor offsets | Matches SVG/Canvas semantics; no hidden addition on every read; anchor moves are one field write with explicit handle moves |
| 4 | Closed flag | Explicit `closed: bool` | Infer from coincident endpoints | VG1 law; coincident endpoints are a legal open-path state |
| 5 | Fill rule | Explicit per path (`NonZero` default · `EvenOdd`) | Per-subpath | Single consumer contract (VG5); multi-subpath compound behaviour is path-wide |
| 6 | Straight segment | Data definition: both controls equal their endpoints | Geometric test with ε (collinear within tolerance) | Exact, deterministic, serialization-stable; geometric straightness is a measurement concern, not editing data |
| 7 | Point type / handle mode | `Corner | Smooth` × `Mirrored | Asymmetric | Disconnected` stored as data | Derive from handle positions | Designer intent must survive edits that would otherwise re-derive it (VG1) |
| 8 | Data home | `gip-document::path` (plain structs, no logic) | `gip-geometry` owns type | Layer-1 edge law (see Context) |
| 9 | Point representation in data | `[f64; 2]` in document; geometry converts to `Vec2` at its boundary | Depend on `gip-coords` from document | Keeps document dependency-free (layer-1) |
| 10 | Arc / text / boolean results | Not stored in v1; booleans = non-destructive node groups, text geometry comes from text engine | Store arc primitives | VG4 + VG6 gates; text per CAN-TEXT-1 |

## Consequences

1. `gip-document` gains `path` module: data types only (`FillRule`, `PointType`, `HandleMode`,
   `Vertex`, `Subpath`, `Path`) with constructors for corner vertices. No logic beyond construction.
2. `gip-geometry` implements segment math (cubic eval/split/extrema/flatten/nearest), path bounds,
   hit tests, and the VG2 editing operations as pure functions over `Path` (returning new values).
3. A segment's index is `(subpath, index)`; segment count = `vertices − 1` (open) or `vertices`
   (closed, closing segment last→first). Subpaths with fewer than 2 vertices have zero segments.
4. RVS (layout-engine L3) consumes straight-segment status from this data definition. RVS colors
   still need ADR-005 before the feature ships; this ADR does not decide them.
5. ADR-003 serialization of paths follows the vertex shape above; float rules apply unchanged.

## Owner review items (non-blocking, recommended defaults)

1. **Smooth-point handle mirroring on drag** — recommended: Smooth = Mirrored by default; the
   interaction rule belongs to the vector editing feature doc, not this data model.

## Open questions (deferred, tracked)

1. Vector networks (VG6) — future-gated, needs its own ADR before any code.
2. Arc primitive storage — revisit only if a measured fidelity/size problem appears.
3. Boolean algorithm choice — vector-engine §8.3, ADR at implementation (CQ1.4).
