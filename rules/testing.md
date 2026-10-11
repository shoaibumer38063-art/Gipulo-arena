# TESTING RULES — What must be tested, how, and when

> Status: ACTIVE (v1, 2026-10-11). Sources: vision doc §8 testing/regression documentation (`03`),
> canvas rules §25–26. Testing is part of the foundation, not a later phase.

## T1 — Coverage mandates (systems that MUST have tests as they are built)

Vector geometry · path editing · coordinate conversion · hit-testing · selection · resize ·
rotate · snapping · zoom/pan · canvas input state machine · Auto Layout/constraints · responsive
vector behavior (RVS) · undo/redo · file saving/loading (`.gip` round-trips) · code generation ·
generated-code preview · performance budgets. The per-area matrix for canvas lives in canvas
rules §25 and binds as the minimum.

## T2 — Test pyramid for Gipulo

1. **Unit (majority):** pure functions of the geometry/coordinate/layout/serialization engines —
   no DOM/GPU needed (code-quality CQ7 keeps them pure precisely so they are testable here).
2. **Integration:** state-machine flows (press→drag→snap→commit→undo), selection/mode transitions,
   transaction/history behavior, layout reflow chains.
3. **Golden documents:** committed fixtures (the heavy reference set from canvas rules §26 +
   pathological geometry set from vector-geometry rules VG7) used for: serialization round-trips
   (byte/semantic identity), render-vs-geometry parity, export parity, determinism across runs.
4. **Interaction/UI:** component state matrices (every DS state snapshot-verified in both themes),
   keyboard/focus flows, DnD, panel behaviors (interactions rules I8).
5. **Performance:** budget suite on reference documents (performance rules P3–P5).

## T3 — When tests are mandatory

- New feature: tests for its behavior matrix + affected neighbors identified in its feature doc
  ("regression risks" section is the test plan seed).
- Bug fix: a regression test reproducing the ORIGINAL symptom is mandatory wherever practical —
  the fix is not complete without it. Law: **a difficult bug fixed once must never silently
  return** (canvas rules CAN-TEST-1). The bug record links the test; the test names the bug ID.
- Refactor: identical test surface green before and after; refactors don't weaken tests
  (refactoring rules RF3).
- Canvas change: §25 matrix rows for touched + neighboring areas.

## T4 — Test quality rules

- Tests assert behavior/contracts, not implementation details — they must survive renderer
  backend swaps and internal refactors (canvas rules CAN-TEST-2).
- Deterministic tests only: no time-of-day, random, or iteration-order dependence; async tests
  use controlled clocks/awaits, never sleep-and-hope.
- Numerical asserts use the project epsilons (CAN-NUM-5 constants), never ad-hoc tolerances.
- Fixture hygiene: fixtures are small, named by the pathology they cover, and committed; a bug's
  reproduction becomes a permanent fixture.
- No test may mutate shared fixtures; no test may depend on another test's execution.

## T5 — Test documentation

- Every important feature doc lists its test cases and regression risks (`03` §1); bug records
  list "Tests Added"; architecture docs note which suites guard their invariants.
- Test inventory stays discoverable: suite names mirror subsystem names (canvas rules §2) so an
  agent can find "what guards the coordinate system" in one step.

## T6 — Execution discipline (once tooling exists)

- All tests run before declaring any task complete (part of the completion definition,
  `08_foundation_roadmap.md`); failing unrelated tests are reported, never silently skipped.
- CI wiring, coverage targets, and runner choices are ADR-gated tooling decisions; until then,
  tests run locally and results are recorded in task summaries.
