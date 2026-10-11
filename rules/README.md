# Gipulo Rules System

Rules are the engineering operating system for AI agents and future human developers. There is NOT
one giant rules.md — different systems get different rule files. Planned set (from the vision
document; more categories may be added later — that is expected):

```
rules/global.md            rules/code-quality.md      rules/canvas.md
rules/vector-geometry.md   rules/rendering.md         rules/ui.md
rules/layout.md            rules/icons.md             rules/interactions.md
rules/state-management.md  rules/performance.md       rules/feature-addition.md
rules/bug-fixing.md        rules/refactoring.md       rules/feature-removal.md
rules/testing.md           rules/documentation.md
```

**Status as of 2026-10-11 (v1): ALL 17 rule files AUTHORED and ACTIVE.**

| File | Scope |
|---|---|
| `global.md` | Top-level laws: fresh project, truth hierarchy, session start, reuse, honesty, scope, determinism |
| `code-quality.md` | The eleven prohibitions + what minimal must NOT mean + reuse protocol + naming/types/constants |
| `canvas.md` | **The main law of the canvas** — 28 sections, rule IDs CAN-*, research-grounded, micro-detail registry, change protocol |
| `vector-geometry.md` | Canonical path model, one operations library, tolerances, non-destructive discipline |
| `rendering.md` | Renderer boundaries, graphics abstraction, invalidation/caching, frame discipline |
| `ui.md` | Design System law: tokens-only, no browser defaults, complete states, binding metrics, reserved colors |
| `layout.md` | Layout engine contracts, constraints, **Responsive Vector Segments product law (L3)** |
| `icons.md` | One icon language: 16px grid, stroke 1.25, states via color, no external packs |
| `interactions.md` | Shortcuts registry, focus scopes, menus/tooltips/toasts behavior, DnD, panels, fields |
| `state-management.md` | Four state domains (document/camera/UI/transient), single ownership, mutation discipline |
| `performance.md` | The "no lag" law, budgets, reference-document measurement, design-level prevention |
| `feature-addition.md` | The mandatory 10(+1)-step flow with per-step Gipulo specifics |
| `bug-fixing.md` | The mandatory 10-step flow, root-cause discipline, banned symptom patches, P0 data integrity |
| `refactoring.md` | When allowed/required/forbidden, preconditions, behavior-preservation verification |
| `feature-removal.md` | Decision gate, the full removal sweep checklist, `.gip` data migration gating |
| `testing.md` | Coverage mandates, test pyramid, mandatory-test triggers, quality rules |
| `documentation.md` | Doc structure, the ten-question Update Rule, templates, ADRs, writing standards, memory maintenance |

Values tagged **[PROPOSED]** inside the rules are binding defaults until an ADR confirms/changes
them (see `canvas.md` §27 and the ADR process in `documentation.md` D7).

## Quality bar (non-negotiable)

- Rules must read like internal engineering standards of a serious software company.
- NO generic instructions ("write clean code", "keep the canvas performant"). Every rule must
  explain exactly what the concept means INSIDE Gipulo.
- For each important area, a rule defines: the rule · why it exists · source of truth · affected
  systems · what AI may do · what AI must NEVER do · invariants · required tests after change ·
  neighboring systems that can regress.

## Special law: canvas

`rules/canvas.md` will be the **main law for anything touching the canvas**. Any agent working on
the canvas MUST read the relevant canvas rules BEFORE touching implementation. Full authoring
requirements: `docs/memory/06_canvas_rules_requirements.md`.
