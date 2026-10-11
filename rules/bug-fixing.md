# BUG FIXING RULES — Root causes only, regressions never

> Status: ACTIVE (v1, 2026-10-11). Source: vision doc (`04_rules_system.md`). The agent must NOT
> immediately start changing code when a bug is reported.

## The flow (mandatory order)

**1. Reproduce the bug.** Exact steps, exact environment (zoom level, theme, document content,
selection state, modifiers). Cannot reproduce → gather more info from the user before touching
anything; an unreproduced "fix" is a guess. Reproduction becomes a test fixture (step 9).

**2. Identify the exact EXPECTED behavior.** From the sources of truth in order: user documents →
memory distillations → rules (canvas §27 micro-details, DS specs in `07`) → feature docs →
observable Figma behavior for parity features [OBS]. Write the expected behavior down; "wrong"
without a defined "right" is not diagnosable. If docs are silent/ambiguous → that is itself a
finding: ask the user or record an open question (global rule G10) — do not invent product
behavior to justify a patch.

**3. Inspect related bug history.** Search `docs/bugs/` for the area/symptom. Prior art tells you:
root causes that recur, fixes that held, files that are fragile, "must not change" notes, and
existing regression tests that should have caught this (if one existed and passed — the bug is
elsewhere than it looks).

**4. Identify the REAL root cause.** Trace through the owning subsystems (canvas rules §2 map):
is it geometry, coordinates, state machine, layout, rendering, serialization, or UI state?
Evidence-based: the cause must explain ALL symptoms, including the weird ones. Correlation
("changing X makes it disappear") is not causation.

**5. Avoid symptom-only patches.** Banned: clamping/rounding to hide numerical causes · special-
casing one shape/tool to dodge a general defect · adding z-index/opacity/re-render bandaids over
state bugs · "fixing" the display of a value whose stored value is corrupt · duplicating logic to
route around the broken owner. If the true fix is too large for this task: fix nothing silently —
record the bug properly, propose the plan, get direction.

**6. Make the smallest safe fix.** In the owning subsystem, through its public interface,
consistent with all binding rules. Touching a [PROPOSED]/invariant value (canvas §27, guide
meanings, epsilons) → ADR path, not a fix path. Cross-subsystem fixes follow dependency direction
(CAN-ARCH-2).

**7. Test the original problem.** The step-1 reproduction now passes — as an automated regression
test wherever practical (testing rules T3), naming the bug ID.

**8. Test related behavior.** The neighborhood of the root cause: the canvas §25 matrix rows for
the touched area + the systems the cause touched (a coordinate bug ⇒ round-trips, hit-testing,
snapping, overlays…). Performance-sensitive paths ⇒ budget check (performance rules P3).

**9. Add regression coverage where practical.** Reproduction fixture committed; golden-document
update if serialization/parity was involved; the coverage is permanent — the law: **a difficult
bug fixed once must never silently return six months later** (canvas rules CAN-TEST-1).

**10. Update the bug documentation.** `docs/bugs/BUG-NNN-*.md` with the full mandatory template:
Bug · Symptoms · Affected Feature · Root Cause · Fix · Files Changed · Tests Added · Regression
Risks · Verification · Related Bugs. Plus the Documentation Update Rule sweep (feature docs'
known-bugs sections, registry if status changed, architecture docs if a fragility was learned,
memory CHANGELOG if memory changed) and the task summary.

## Severity & honesty

- Data-corruption bugs (document mutation wrong/lost, serialization lossy, undo corrupting state)
  outrank everything: stop feature work, fix first. Any document-integrity defect is P0.
- Visual-only defects still get root-cause treatment — "just cosmetic" patches accumulate into
  the inconsistent-canvas disaster the rules exist to prevent.
- Report honestly (global rule G5): what was fixed, what was verified, what risk remains. A fix
  you could not verify is labeled unverified.
