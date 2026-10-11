# Task 013 — Segment index (`gip-geometry`, roadmap step 13)

**Date:** 2026-10-11 · **Status:** COMPLETE (tests, clippy, fmt, wasm check, boundary check green) · **Governs:** `docs/architecture/ADR-007-segment-index.md` (PROPOSED) · `vector-engine.md` §8.2, §9, §10

## Goal

Replace the brute-force hover query with a path-level spatial index, with results provably equal
to the brute-force oracle. Record measured build and query costs.

## What was built

- `gip-geometry/src/index.rs`: `SegmentIndex::{build, nearest, len, is_empty}`. Static BVH over
  per-segment tight bounds. Median split on the longer centroid axis; leaves hold at most 8
  segments; branch-and-bound nearest visits the nearer child first.
- Re-exported as `gip_geometry::SegmentIndex`.
- Tests (`index::tests`): empty and anchor-only paths; equality with brute force on a deterministic
  sweep over three shapes (including open paths and multiple subpaths); the reported segment owns the distance.

## Measured (release, 1000-segment dense open path, 1000 queries)

| Measure | Value |
|---|---|
| Build | 0.27 ms |
| Brute force per query | 142 ms |
| Index per query | 1.13 ms |
| Max distance difference, index vs brute | 0 |

**Caveat (recorded in ADR-007):** the task-010 baseline (10.4 ms) used a different fixture. These
numbers are per fixture, not a single multiplier. Index query time is still above a comfortable
hover budget on this dense fixture; leaf-bound tightening or cursor-radius pruning is the next step
if measured need persists.

## Process notes

- The first sweep test took 94 s in debug mode because the brute-force oracle ran on 200 segments
  for every query. Fixed by reducing the sweep (fewer shape sizes and sampled queries); the test
  now runs in about 5 s in debug. Coverage shape unchanged.
- A temporary perf test was used for release measurements and deleted afterwards (not left in the tree).

## Totals

131 workspace tests green (up from 128: 3 index tests). fmt, clippy `-D warnings`, wasm check,
and boundary check all clean.

## Deferred

Document-level index across nodes (scene phase, separate ADR) · incremental segment updates (rebuild
chosen, see ADR-007 owner review #1) · cursor-radius candidate pruning (only if measured need).
