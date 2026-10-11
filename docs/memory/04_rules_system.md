# 04 — Rules System, Design-System-First, and Final Principles

> Distilled from `docs/source/gipulo_project_vision_refined_v2.md` (AI rules system sections,
> "After the Foundation — Build the Design System", code quality, workflows, final principle).

## Build order: foundation → Design System → features

- After the multi-layer documentation/memory/testing/rule foundation is properly created, the
  **first major visible system is the Gipulo Design System**.
- The design system must exist BEFORE hundreds of UI features are implemented, so different AI
  agents don't create inconsistent buttons, panels, spacing, radii, icons, states, tooltips,
  menus, inputs, and dialogs.

## Familiar workflow, different identity

- The UI must remain **highly familiar to Figma users** — many designers spent years learning
  Figma. If Gipulo completely changes toolbar locations, layer panel behavior, properties panel
  structure, selection behavior, navigation, or editing patterns, users must learn a whole new
  tool → unnecessary friction.
- Familiar layout to preserve: **layer panel on the left · canvas in the center · properties
  panel on the right · familiar toolbar placement · familiar selection interaction · familiar
  frame/group/component concepts where appropriate.**
- BUT the **visual identity is Gipulo's own**: accent colors, details, styling, polish, branding,
  and certain interaction improvements can differ.
- Motto: **"Familiar workflow, different identity."**

## AI Rules System structure

- There must NOT be only one giant `rules.md`. Different systems require different rules.
  Planned files (in `rules/`; more categories may be added later — that is expected):

```text
/rules/global.md
/rules/code-quality.md
/rules/canvas.md
/rules/vector-geometry.md
/rules/rendering.md
/rules/ui.md
/rules/layout.md
/rules/icons.md
/rules/interactions.md
/rules/state-management.md
/rules/performance.md
/rules/feature-addition.md
/rules/bug-fixing.md
/rules/refactoring.md
/rules/feature-removal.md
/rules/testing.md
/rules/documentation.md
```

## Canvas rules are especially important

- The canvas is **one of the most dangerous areas to change** — a small bad change can affect
  many systems. Canvas rules must be extremely strict and cover at least:
  world coordinates · screen coordinates · local coordinates · parent coordinates · transform
  matrices · zoom · pan · hit-testing · selection bounds · object bounds · vector bounds ·
  pointer tolerance · screen-pixel-invariant interactions · resize handles · rotate handles ·
  snapping · guides · overlays · input priority · nested transforms · viewport rendering ·
  invalidation · rendering order.
- **When an AI agent works on the canvas, it must FIRST read the relevant canvas rules before
  touching the implementation.**
- Full requirements for the future `rules/canvas.md`: see `06_canvas_rules_requirements.md`.

## Code Quality Rules

- The codebase must stay **as minimal and clean as realistically possible**.
- "Minimal code" must NOT mean: forcing everything into one file · unreadable one-line logic ·
  avoiding useful abstractions · removing comments that explain important behavior · combining
  unrelated systems only to reduce line count.
- The real goals — the codebase must have:
  no duplicated logic · no unnecessary abstractions · no unnecessary files · no unnecessary
  dependencies · no dead code · no temporary hacks left permanently · no duplicated geometry
  systems · no duplicate state · no random architecture · no unnecessary wrappers · no clutter.
- **Before creating new code, check whether the required logic already exists.** If it exists:
  reuse or extend it. Never create another implementation of the same idea elsewhere.

## Feature Addition Rules (mandatory 10-step order)

Before adding a feature, the agent must:
1. read the feature index (FEATURE_REGISTRY);
2. inspect related existing features;
3. inspect relevant architecture documents;
4. inspect relevant rules;
5. search the codebase for reusable logic;
6. identify affected systems;
7. plan the smallest correct implementation;
8. implement it;
9. test it;
10. verify neighboring behavior;
11. update documentation.
(steps 8–11 complete the sequence; the source lists this as the canonical flow)

## Bug Fixing Rules (mandatory order)

The agent must NOT immediately start changing code. First:
1. reproduce the bug;
2. identify the exact expected behavior;
3. inspect related bug history;
4. identify the real root cause;
5. avoid symptom-only patches;
6. make the smallest safe fix;
7. test the original problem;
8. test related behavior;
9. add regression coverage where practical;
10. update the bug documentation.

## Removal Rules

When a feature or old implementation is removed, also check for and remove:
dead imports · dead styles · dead tests · dead documentation · unused dependencies · obsolete
feature flags · old compatibility code · old state · old event listeners · unreachable code.
**Removing a feature must not leave half of that feature scattered around the project.**

## Rules quality standard (IMPORTANT)

- The rule system must be treated like the **internal engineering standards of a serious software
  company**.
- Rules must NOT contain generic AI instructions like "Write clean code." They must explain
  **exactly what "clean" means inside Gipulo**, e.g.: how canvas coordinates must be handled ·
  where geometry calculations live · how UI state is separated from document state · when to
  create dependencies · how to add features safely · what documentation must change · how to
  verify performance · how to avoid duplicate logic.
- The rules become a **real engineering operating system** for both AI agents and future human
  developers.

## Final Principle (both at the same time)

1. **Extremely detailed documentation** — a new AI agent, senior engineer, designer, or future
   human team can enter the project and understand what is happening without rebuilding context
   from scratch.
2. **Extremely clean and minimal code** — minimal in duplication, unnecessary logic, unnecessary
   files, unnecessary dependencies, unnecessary abstractions, dead code, clutter, and conflicting
   systems. The project may become extremely large — expected — but complexity must come from the
   **capability of the product**, not a messy codebase.
- Ultimate goal: Gipulo keeps growing in advanced features **without becoming impossible for AI
  agents or humans to understand and maintain.**
