# ADR-007 — Path-Level Segment Index (bounding-volume hierarchy)

```text
Decision:   Hit candidates for a path come from a STATIC BOUNDING-VOLUME HIERARCHY (BVH) over that
            path's segments, built from each segment's tight bounds (cubic.tight_bounds). Build is
            O(n log n) median split along the longest centroid axis. The index belongs to one path
            and is rebuilt only when THAT path changes (per committed transaction that touches it).
            It is never rebuilt per frame, and it is never shared across paths in v1.
            Brute force (hit::distance_to_path) remains the reference oracle in tests.
Date:       2026-10-11 (authored)
Status:     PROPOSED — binding implementation default (D7 pattern), awaiting owner review.
Refines:    vector-engine.md §8.2 (spatial index — ADR-gated), §9 (index vs brute force test),
            canvas.md §5 item 5 (structure choice). Interacts with: ADR-004 #12 (affected-set
            invalidation), ADR-006 (segments are implied by vertices).
```

## Context

- Measured (task 010, release): `distance_to_path` costs **10.4 ms per query at 1000 segments**.
  Hover runs on every pointer move, so this is over budget for large paths (frame budget 16 ms,
  p95 ≤ 32 ms, canvas §26).
- `vector-engine.md` §8.2 requires an index, but leaves the structure to this ADR. It also says
  the index updates **incrementally with transactions**, never per frame.
- Options considered: uniform grid (simple, but degrades on uneven density and needs a cell-size
  tuning constant); R-tree (the tldraw precedent, but more code for a static per-path set); BVH
  (static, no tuning constant, handles uneven density, small code).

## The decision (sub-decisions)

| # | Sub-decision | Choice | Rationale |
|---|---|---|---|
| 1 | Structure | Static BVH, median split on the longer centroid axis, leaves of ≤ 8 segments | No tuning constant; robust to uneven density; ~100 lines |
| 2 | Bounds | Tight bounds per segment (from `Cubic::tight_bounds`) | Tight boxes prune far better than control-hull boxes; the curve lies inside them, so pruning is safe |
| 3 | Scope | One index per path | Matches the edit unit (one path is edited at a time) and keeps rebuild cost local |
| 4 | Update | Rebuild the affected path's index at commit (ADR-004 #12 publishes the affected ids); never per frame | Cost is O(n log n) on the edited path only; measured below |
| 5 | Query | Branch-and-bound nearest: visit the nearer child first; prune a box when its distance² ≥ best² | Same pruning law as `Cubic::nearest_within`, so results are provably identical to brute force |
| 6 | Oracle | Brute force (`hit::distance_to_path`) stays as the reference; tests compare the two on random paths | vector-engine §9 requires index-vs-brute-force correctness |
| 7 | Tie-breaking | Distance must match brute force exactly (within eps); the reported segment may differ only on exact ties | The traversal order differs, so exact-tie segments can differ; the distance cannot |
| 8 | Ownership | Lives in `gip-geometry` (layer 2). No cache or index state in `gip-document` | Layering and CAN-SER purity: the document holds no derived data |

## Rejected alternatives

- **Uniform grid:** needs a cell size tied to zoom or document scale. That is a tuning constant the
  rules forbid as a hidden default, and it degrades on clustered segments.
- **Dynamic R-tree:** built for many inserts and removes. A path's segment set changes only as a
  whole per commit, so the dynamic machinery buys nothing here. Revisit it for the document-level
  index (all nodes), which is a separate ADR.
- **Document-level index now:** not needed for hover on one path, and it would need the
  transaction-to-index update path. Deferred to the scene phase.

## Consequences

1. `gip-geometry::index::SegmentIndex` with `build(&Path)` and `nearest(Vec2) -> Option<Nearest>`.
2. Hover uses the index's candidate set once the editor reads it (vector-engine §10).
3. Tests (vector-engine §9): index equals brute force on random paths (distance within 1e-9;
   `None` agreement), empty path, anchor-only path, open and closed paths.
4. Budgets: query time measured and recorded in the task summary; build time measured too.

## Measured (task 013, release mode, 1000-segment wavy open path, 1000 queries)

- Build: 0.27 ms. Index equals brute force exactly (max distance difference 0).
- Brute force: 142 ms per query on this fixture. Index: 1.13 ms per query.
- CAVEAT: the task-010 baseline (10.4 ms) came from a different fixture. Figures are per fixture,
  not a single multiplier. Dense, densely overlapping segments (3 units apart, sinusoidal) still
  visit many leaves; 1.1 ms is above a comfortable hover budget. Next optimization, if measured
  need persists: tighter leaf bounds or candidate pruning by the cursor's pixel radius (ADR amendment).

## Owner review items (non-blocking)

1. Confirm "rebuild per committed transaction that touches the path" as the update rule (vs.
   incremental segment updates). Recommended: rebuild, because measured rebuild cost is small.
2. Document-level index scope (all nodes, scene phase) — separate ADR when the scene lands.
