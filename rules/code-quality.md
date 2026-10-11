# CODE QUALITY RULES — What "clean and minimal" concretely means inside Gipulo

> Status: ACTIVE (v1, 2026-10-11). Source: vision document (`docs/memory/04_rules_system.md`).
> "Minimal" is NOT minimal line count. These rules define the exact standard.

## CQ1 — The eleven prohibitions (violations are bugs, not style opinions)

1. **No duplicated logic** — one concept, one implementation, one owner module. Two functions
   computing bounding boxes = defect. Two similar helpers in different folders = defect.
2. **No unnecessary abstractions** — an abstraction must have ≥2 real consumers or a documented
   architectural mandate (e.g. renderer backend abstraction is mandated by canvas rules §21).
   Speculative "might need someday" layers are forbidden.
3. **No unnecessary files** — a file exists for a coherent responsibility, not per tiny helper.
   No `utils.js`/`misc.js`/`helpers.js` catch-alls; name files by domain
   (`coordinate-system`, `bounds`, `snap-engine`).
4. **No unnecessary dependencies** — every new dependency requires: what problem it solves, why
   existing code/standard platform can't, bundle/maintenance cost, and an ADR. Default answer: no.
5. **No dead code** — deleting a feature deletes ALL of it (`rules/feature-removal.md`).
   Commented-out code is dead code. Unreachable branches are dead code.
6. **No temporary hacks left permanently** — a hack either never lands, or lands with a bug
   record ID and removal plan referenced in the code comment.
7. **No duplicated geometry systems** — all geometry via the geometry engine (canvas rules §5–6).
8. **No duplicate state** — one owner per piece of state (`rules/state-management.md`); derived
   values are computed or cached-with-invalidation, never stored twice.
9. **No random architecture** — structure follows the subsystem map (canvas rules §2) and
   architecture docs; new top-level structure requires an ADR.
10. **No unnecessary wrappers** — a wrapper that only forwards calls is noise; wrap only to add a
    real boundary (interface swap, isolation, error translation).
11. **No clutter** — no leftover logs/debuggers/scratch files/test-only branches shipped; no
    orphan docs; no stale TODOs without IDs.

## CQ2 — What minimal must NOT mean (equally binding)

- NOT forcing everything into one file; modules keep coherent responsibilities.
- NOT unreadable one-line logic; clarity beats brevity. Multi-line explicit code is correct code.
- NOT avoiding useful abstractions where they are mandated or have real consumers.
- NOT removing comments that explain important behavior — comments explain WHY (decisions,
  invariants, gotchas, spec references like `CAN-COORD-2`), never WHAT the syntax already says.
- NOT combining unrelated systems just to reduce line count.

## CQ3 — Reuse protocol (before writing any new code)

1. Search the codebase + FEATURE_REGISTRY + architecture docs for the concept.
2. Found → reuse as-is, or extend with backward-compatible changes (run the owner's tests).
3. Not found → place the new implementation in the subsystem that OWNS that responsibility
   (per canvas rules §2 / architecture docs) — not in the calling feature's folder.
4. If two existing implementations are discovered: stop the feature work, unify them first
   (refactoring rules), then proceed.

## CQ4 — Naming & language discipline

- Use the domain vocabulary exactly: `screen/viewport/world/parent/local/path/text/device` spaces;
  `geometricBounds/strokeBounds/visualBounds/selectionBounds/transformBounds/hitBounds`;
  `frame/group/component/instance`; tool/mode/state names from canvas rules §7.
- No abbreviations that aren't project vocabulary (`mgr`, `ctx2`, `doIt`, `temp`, `data1`).
- Booleans read as assertions: `isVisible`, `hasOverride`, `canSnap`. Mode enums, not boolean
  constellations (canvas rules CAN-MODE-2).

## CQ5 — Type & error discipline

- Strict typing everywhere once a stack exists (no implicit any-equivalents); document/model
  types mirror the `.gip` schema exactly — the schema is the contract.
- Errors: expected failures (missing font, failed decode, shader error) produce documented
  degraded states + user-visible feedback (DS error styles); unexpected failures never corrupt
  document state — mutations are transactional (canvas rules §22).
- Never swallow errors silently; never `catch {}` without a recorded reason.

## CQ6 — Numbers & constants

- No magic numbers in logic: named constants from ONE constants/tokens module (epsilons,
  thresholds, zoom limits, DS values). Values that appear in `rules/canvas.md` §27 or
  `docs/memory/07` must reference the same single source in code.
- Displayed precision ≠ internal precision (canvas rules CAN-NUM-2) — enforced by separating
  format functions from stored values.

## CQ7 — Function & module shape

- One function = one operation at one level of abstraction; gesture math, geometry math, and
  rendering never mix inside one function body.
- Pure where possible: geometry/coordinate/layout functions are pure (input → output) and
  unit-testable without a DOM/GPU; side effects live at the edges (input handlers, renderer).
- Public surface minimal: subsystems export their interface; internals are not importable
  (enforced by module boundaries/lint once tooling exists).

## CQ8 — Test-adjacency

- New logic ships with tests per `rules/testing.md`; a PR that grows a subsystem without growing
  its test surface is incomplete. Fixed bug ⇒ regression test (bug-fixing rules).

## CQ9 — Review questions (agent self-review before declaring done)

- Did I duplicate anything? (search evidence, not memory)
- Did I add state that already exists elsewhere?
- Did I use tokens/constants or invent values?
- Did I touch a subsystem boundary? Is the dependency direction legal (CAN-ARCH-2)?
- Are tests covering the changed behavior AND the neighbors listed in the rules?
- Did I update every doc the Documentation Update Rule requires?
