# 00 — Memory Index (Gipulo Distilled Memory)

> Purpose of this folder: give any agent **fast, structured recall of every detail** in the four
> canonical source documents (`docs/source/`), without re-reading 135 KB of prose each session.
> The source documents remain the ultimate truth; these files mirror them faithfully and are kept
> in sync. **If a conflict is ever found: source wins, fix the distillation, log it in CHANGELOG.**

## Files

| File | Distills which source | Use it when… |
|------|----------------------|--------------|
| `01_project_identity_and_vision.md` | Vision doc (parts 1–2) | You need the project's identity, the user's situation, why Gipulo exists, the problem statement, the overall goal |
| `02_problems_and_solutions.md` | Vision doc (Problems/Solutions 1–5) | Working on Responsive Vector Segments, Design-to-Code/Code Workspace, 3D/Live Link, Scroll Interactions, pricing/accessibility-of-features — or explaining the product's differentiators |
| `03_documentation_system.md` | Vision doc (documentation foundation) | Creating/updating any documentation; finishing any task (Documentation Update Rule); writing bug records, task summaries, prompt archives, ADRs, test/perf docs |
| `04_rules_system.md` | Vision doc (rules + design-system-first + final principles) | Writing or applying rules; adding a feature (10 steps); fixing a bug (10 steps); removing a feature; judging code quality |
| `05_design_system_brief.md` | Design System Brief doc | Building ANY UI: components, states, panels, toolbars, menus, motion, icons, typography, spacing, cursors, canvas UI, guides, accessibility, theming |
| `06_canvas_rules_requirements.md` | Canvas Rules Prompt doc | Touching the canvas in ANY way; authoring `rules/canvas.md`; designing canvas subsystems (coordinates, geometry, selection, snapping, rendering, input, text, masks, effects, history) |
| `07_design_tokens_and_visual_specs.md` | Design System HTML | You need a CONCRETE value: color, size, radius, spacing, state appearance, guide spec, motion timing, zoom-visibility rule |
| `08_foundation_roadmap.md` | Synthesis of all docs | Planning what to build next; checking project status/phase |
| `CHANGELOG.md` | — | Log every memory change here (append-only) |

## How memory works in this project

1. **Session start:** read `/home/user/GIPULO_AGENT_MEMORY.md` → this index → relevant topic file(s).
2. **During work:** consult `07` for concrete values; consult `06` before any canvas change;
   consult `05` before any UI change; consult `03`/`04` before adding/fixing/removing anything.
3. **Session end / task end:** run the Documentation Update Rule checklist (`03`, §Update Rule);
   write task summary (`docs/tasks/`); archive important prompts (`docs/prompts/`); update
   `FEATURE_REGISTRY.md` if feature status changed; append to `CHANGELOG.md` if memory changed.

## Fresh-project reminder

This memory system was created on **2026-10-11** for a **completely new project**. Any older
"gipulo" memories from previous sessions/projects are invalid and must never be mixed in.
