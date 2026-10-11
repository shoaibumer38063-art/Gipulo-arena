# Task 010 — Geometry phase (`gip-geometry` + path data model)

**Date:** 2026-10-11 · **Status:** COMPLETE (native + wasm checks green) · **Governs:** `docs/architecture/vector-engine.md` §10 · `ADR-006` (PROPOSED — binding implementation default) · `rules/vector-geometry.md` (VG1–VG7)

## Goal

Build the single geometry engine (VG2): canonical path model, cubic math, bounds, hit geometry,
flattening, and the first editing operations — test-first against VG7 golden fixtures. No editor
features, no UI, no caches or spatial index yet (ADR-gated).

## Decisions made in the phase

1. **ADR-006 authored before code** (vector-engine §8.1 was an open question blocking every
   operation's input type). Cubic-only storage; vertex = anchor + absolute in/out handles;
   explicit closed flag and fill rule; straight segment = data definition (controls equal their
   endpoints, exact).
2. **Placement:** path DATA types in `gip-document::path` (plain values); OPERATIONS in
   `gip-geometry`. Forced by the layer-1 law: document cannot depend on geometry, geometry depends
   on document. No dependency edge was added or reversed.
3. **No new dependencies.** Shared test PRNG policy continues (dependencies.md §5 rule 5).
4. **Tolerances are caller parameters** (VG3). The crate invents none; the only comparison ε is
   the single `gip-coords` module.

## What was built

| Module | Contents |
|---|---|
| `gip-document::path` | `Path`, `Subpath`, `Vertex`, `FillRule`, `PointType`, `HandleMode`, `Point = [f64;2]` |
| `cubic.rs` | eval · de Casteljau split · exact tight extrema · flattening (chord-line deviation ≤ tol) · derivative · second derivative · quadratic→cubic · straight test · `nearest_within` (branch-and-bound + Newton polish) |
| `segments.rs` | implied segment enumeration · `segment_count` · `segment_cubic` · tight bounds · flatten subpath |
| `hit.rs` | point-in-fill (both rules; open-as-closed) · `distance_to_path` with a shared running bound |
| `ops.rs` | `transform` (exact, rejects non-finite) · `reverse_subpath` · `set_closed` · `insert_vertex` (shape-exact; straight stays straight) · `delete_vertex` |
| `lib.rs` | `BoundsKind` (6, CAN-GEOM-2) · `BoundsRect` · re-exports |
| `gip-coords::vec2` | + `dot`, `cross`, `length_sq`, `distance_sq`, `lerp`, `min/max_components` |

## Bugs and wrong laws caught (all fixed; none shipped)

1. **Nearest point wrong by 3.3 units for a point on the curve.** The first refinement searched
   only the bracket around the best grid sample; on fast curves the best sample can sit on another
   branch. Fixed by exhaustive branch-and-bound.
2. **Branch-and-bound hang (>60 s).** Depth-first left-first ordering never tightened the bound,
   so pruning did nothing. Fixed by visiting the nearer child first. Verified: suite < 2 s.
3. **Newton polish silently did nothing.** Acceptance was by distance comparison, which is below
   rounding noise near the root. Fixed by accepting steps that shrink the offset condition with a
   positive-curvature check. Also removed a final distance guard that had the same rounding flaw.
4. **Flatten bound stated against the wrong object.** The proof bounds deviation from the chord's
   *line*; the test measured distance to the finite *segment* (which can exceed it by the tangential
   overshoot). Corrected the documented guarantee and the test.
5. **Wrong test references:** a 400k-sample grid cannot resolve positions finer than its spacing
   (replaced by the perpendicularity optimality condition + "never worse than grid"); an
   endpoint-minimum case was asserted as interior (now only interior minima are asserted).

**Lesson (extends task 009):** three of the five bugs in this phase were in *test expectations or
stated guarantees*, and the two code bugs were both found by a property check that had a
trustworthy oracle. When an oracle is weak (sampling), assert the optimality condition instead.

## Verification (2026-10-11)

- `cargo test --workspace`: **88 tests pass, 0 failed** — gip-geometry 29 unit + 8 golden + 2
  property; gip-coords 38; others.
- `cargo clippy --all-targets -- -D warnings`: clean (one `partial_cmp` lint fixed on the way).
- `cargo fmt --all -- --check`: clean · `check-boundaries.py`: PASS · wasm target check: clean.

**Measured:** `distance_to_path` on a 1000-segment closed path ≈ **10.4 ms/query** (release,
200-query average). Within the canvas §27 p95 hover budget but not comfortable → spatial index is
the next performance item.

## Deferred (named owners)

Spatial index (vector-engine §8.2, ADR-gated) · booleans (§8.3, algorithm ADR) · outline stroke /
offset · stroke & effects bounds (need stroke/effect data) · text geometry (CAN-TEXT-1) ·
selection/transform/visual bounds composition · RVS stretch (layout-engine; ADR-005 colors first) ·
vector networks (VG6).

## Next steps

1. **`gip-document` schema + serializer** (serde = registered exception; golden `.gip` fixtures
   built by the serializer; ADR-003). Path data is ready for it.
2. Spatial index ADR (R-tree vs grid vs hybrid) — triggered by the 10 ms hover measurement.
3. Playwright E2E when the shell imports the wasm bindings (roadmap step 6).
4. Owner review queue unchanged (ADR-002/003/004 items + ADR-006 smooth-point mirroring).
