# GLOBAL RULES — Apply to every task, every session, every subsystem

> Status: ACTIVE (v1, 2026-10-11). These are the top-level laws of the Gipulo project.
> All other rule files are specializations of these; if a specialized rule seems to conflict,
> re-read this file — the stricter, more specific rule wins only when it does not violate these.

## G0 — Fresh project law

- Gipulo was freshly started on **2026-10-11**. Any memory of older projects named "gipulo" is
  invalid and must never influence decisions.
- Truth hierarchy: (1) the user's canonical documents in `docs/source/` → (2) distilled memory in
  `docs/memory/` → (3) `rules/` → (4) architecture docs/ADRs → (5) code comments. If levels
  disagree, the higher level wins and the lower level gets fixed in the same task.

## G1 — Required session start

1. Read `/home/user/GIPULO_AGENT_MEMORY.md`.
2. Read `docs/memory/00_INDEX.md` and the memory files relevant to the task.
3. Canvas task → read `rules/canvas.md` BEFORE touching implementation (non-negotiable).
4. UI task → read `rules/ui.md` + `docs/memory/07_design_tokens_and_visual_specs.md`.
5. Check `docs/features/FEATURE_REGISTRY.md` for what already exists.

## G2 — Foundation before features

- The documentation/rule/memory/testing foundation precedes feature work. No editor feature is
  built "quick and dirty" on the assumption that structure comes later — the structure is the
  product's survival mechanism at Gipulo's scale.

## G3 — One implementation per idea (reuse before create)

- Before writing new code: search for existing logic. If it exists — reuse or extend it.
- A second implementation of an existing concept (coordinate conversion, bounds, snapping,
  debounce, icon, button variant, color value) is a critical violation regardless of how small.

## G4 — Documentation update rule (task completion)

- A task is NOT complete until the 10-question documentation checklist has been run
  (`docs/memory/03_documentation_system.md`): feature changed? behavior changed? architecture
  changed? bug fixed? regression test added? rule changed? file structure changed? registry
  updated? task summary saved? prompt archived?
- Plus: memory `CHANGELOG.md` entry when memory files change.

## G5 — Honest communication

- Never claim work is done, tested, or working when it is not. Report exactly: what was done,
  what was verified how, what remains risky.
- Distinguish facts from inference in research and docs ([PUBLIC]/[OBS]/[REC]/[PROPOSED]/[INF]
  tags as in `rules/canvas.md`). Never present assumptions as confirmed facts.
- Surface blockers and scope problems immediately instead of silently shrinking or inflating scope.

## G6 — No scope creep, no silent patches

- Implement what was asked, following the workflows (`rules/feature-addition.md` etc.).
- Extra ideas discovered mid-task → write them into docs (feature registry "planned", task summary
  "next steps") — do NOT build them unrequested.
- Temporary hacks are forbidden from persisting: either the real fix, or a documented bug record
  with a tracked plan. No commented-out code, no `// TODO fix later` without an ID.

## G7 — Determinism & document fidelity

- The same `.gip` document always reconstructs the same geometry and visual result (details:
  `rules/canvas.md` §23). Nondeterminism in document paths (time, random, iteration order,
  locale) is a bug.

## G8 — Performance is correctness

- A feature that lags is not working. No change may trade another subsystem's performance for
  its own convenience. Budgets: `rules/performance.md` + `rules/canvas.md` §26.

## G9 — Design System adherence

- All UI follows `rules/ui.md` and the Design System tokens. No browser/OS/React default visuals
  or behaviors. No magic numbers. Familiar workflow, different identity.

## G10 — Ask, don't guess (product decisions)

- The user is the product owner and a UI/UX designer. When a decision changes product behavior,
  visual language, or architecture direction and the documents don't answer it — ask the user
  (or record an open question in the task summary + relevant doc) instead of guessing silently.
- Purely internal engineering choices follow the rules/ADR process without bothering the user.

## G11 — Workspace hygiene

- Files persist under `/home/user/gipulo/`. Never leave scratch files in the project tree; use a
  temp location and clean up. Generated directories (node_modules, build, dist, caches) are
  expected to be gitignored/excluded once the repo exists — keep the saved workspace clean and small.
