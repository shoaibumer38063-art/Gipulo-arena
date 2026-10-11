# REFACTORING RULES — Behavior-preserving change discipline

> Status: ACTIVE (v1, 2026-10-11). Refactoring = changing structure without changing behavior.
> Anything that changes behavior is a feature change or bug fix and follows those rules instead.

## RF1 — When refactoring is allowed/required

- Required: unification duty (code-quality CQ3.4) — discovering two implementations of one
  concept obliges unification before building on either. Also: boundary violations (canvas rules
  CAN-ARCH-2), duplicate state (SM2), dead code found during adjacent work (removal rules).
- Allowed: as a clearly separated step within a task when the current structure blocks the
  smallest correct implementation (feature-addition step 7).
- Forbidden: drive-by refactors mixed into feature/fix changes (unreviewable diffs, blurred
  regression causes); speculative restructuring without a measured problem (performance rules
  P3: profile first); rewriting subsystems because an agent "would have designed it differently"
  — existing structure is protected by its architecture docs/ADRs (documentation rules D7).

## RF2 — Preconditions (all mandatory)

1. The affected area's tests exist and are GREEN before starting (write missing characterization
   tests first — refactoring untested critical code without them is reckless).
2. Read the area's architecture doc + binding rules; identify the invariants that must survive
   (e.g. CAN-COORD round-trip invariant, CAN-GEOM parity, determinism CAN-SER-1).
3. Scope is declared: files/subsystems touched, behavior that must NOT change, rollback plan.
4. Architectural-shape changes (moving responsibilities between subsystems, changing public
   interfaces other subsystems consume, altering data model) require an ADR — a refactor may
   implement an ADR, never replace one.

## RF3 — Execution discipline

- Behavior-preserving, verified at every step: identical test surface green after; public API
  changes updated in ALL consumers in the same change (no compatibility shims left behind —
  removal rules apply to the old shape immediately).
- Small, coherent steps; each step leaves the tree working (no half-migrated states parked
  overnight/in commits).
- Tests are migrated, never weakened: if a test blocks a legitimate structural change, adapt the
  test to assert the same contract at the new boundary — deleting coverage or loosening tolerances
  to get green is a violation (testing rules T4).
- Numerical/geometry refactors additionally verify: determinism preserved, epsilons unchanged
  (CAN-NUM-5), golden round-trips byte/semantic-identical, performance budgets not regressed
  (performance rules P3 areas).

## RF4 — Completion

- Run the Documentation Update Rule: architecture docs, feature docs' implementation notes,
  registry code-location pointers, rules (if a rule's detail moved), ADR status updates.
- Task summary records: why the refactor was needed, what structure changed, evidence behavior
  didn't (test results, parity checks, budget numbers), and any follow-up debt discovered
  (recorded in docs — not silently absorbed).
