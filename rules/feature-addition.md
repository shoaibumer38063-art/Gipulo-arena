# FEATURE ADDITION RULES — The mandatory 10-step flow

> Status: ACTIVE (v1, 2026-10-11). Source: vision doc (`04_rules_system.md`). This is the only
> legal path for adding anything to Gipulo. Skipping or reordering steps is a violation.

## The flow

**1. Read the feature index.** `docs/features/FEATURE_REGISTRY.md` — does this (or part of it)
already exist? Is it planned/experimental/deprecated? If a similar capability exists, the task
becomes reuse/extension (code-quality CQ3), not a new feature.

**2. Inspect related existing features.** Read their feature docs AND their code surface:
behavior overlaps, shared components, shared document data. List them in the task summary.

**3. Inspect relevant architecture documents.** Which core systems will this touch (canvas rules
§2 map)? Read `docs/architecture/*` for each. If the feature needs an architectural decision that
doesn't exist yet → write the ADR FIRST (documentation rules D7), get user input for
product-affecting choices (global rule G10).

**4. Inspect relevant rules.** Always: `global.md`, `code-quality.md`. Canvas in any way →
`canvas.md` (its §28 protocol binds). UI → `ui.md` + `icons.md` + `interactions.md`. Geometry →
`vector-geometry.md`. Positioning/sizing → `layout.md`. Rendering → `rendering.md`. State →
`state-management.md`. [PROPOSED] values in the way of the design → ADR before hardening.

**5. Search the codebase for reusable logic.** Concept-by-concept search (bounds? conversion?
debounce? a similar control?). Findings → reuse. Discovering DUPLICATE existing implementations →
stop, unify first (refactoring rules RF1).

**6. Identify affected systems.** From the subsystem map + architecture docs: who consumes what
this feature touches; which neighbors can regress (each binding rule names its regression
neighbors). Write the list down — it becomes the test plan (step 9) and doc update plan (step 11).

**7. Plan the smallest correct implementation.** Smallest change that fully satisfies the
requirement WITHOUT violating any rule: reuse DS components (ui rules UI4), place logic in owning
subsystems (CQ3), no speculative extras (global rule G6). Blocked by a needed refactor → the
refactor is a separate declared step (RF1). New dependency → ADR (CQ1.4). Plan goes in the task
summary.

**8. Implement.** Follow the plan and all binding rules. Tokens/constants only; transactions for
document mutations; state in its owning domain; no native UI behavior; icons from the set;
shortcuts in the registry. Design-to-code duty (vision Problem 2): while implementing any design
feature, record its intended code representation in the feature doc ("code equivalent" section) —
the design system and code system are developed together.

**9. Test.** Per testing rules: behavior matrix of the feature + every neighbor listed in step 6
+ area matrices that bind (canvas §25 for canvas work). Performance-touching → budget suite on
reference documents. New golden fixtures where the feature adds a pathological case.

**10. Verify neighboring behavior.** Actually exercise (or test-run) the neighbors from step 6 —
not just the feature itself. Selection still right? Zoom unaffected? Undo intact? Exports match?
Any surprise → fix within this task or file a bug record; never ship knowingly-broken neighbors.

**11. Update documentation.** The Documentation Update Rule, fully: feature doc created/updated ·
FEATURE_REGISTRY entry/status · architecture docs + ADRs if touched · rules if a detail changed ·
DS docs/reference if visual patterns or tokens were added · task summary saved · prompt archived
if the user's request carried intent worth preserving · memory CHANGELOG if memory changed.
**The task is complete ONLY when this step is complete.**

## Feature-doc minimum (created in step 11, drafted from step 6)

What/why · user & UI behavior · interaction states · keyboard/pointer behavior · selection &
canvas behavior · dependencies & related systems · implementation notes · binding rule IDs ·
**intended code representation (design-to-code duty)** · edge cases · known bugs/limitations ·
test cases · regression risks.

## Forbidden patterns

- Building first, "documenting later." · Adding a parallel mini-system because the real one was
  hard to find. · Shipping default-state-only UI. · Introducing new guide colors/cursor behaviors/
  magic numbers as feature-local decisions. · Expanding scope mid-task without recording it
  (G6). · Declaring done without step 9–11 evidence in the task summary.
