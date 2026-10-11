# 01 — Project Identity & Vision

> Distilled from `docs/source/gipulo_project_vision_refined_v2.md` (sections: intro, problem
> statement, "other opportunities", goal). All facts here are the user's own words/intent.

## Identity

- Product name: **Gipulo**. Document/file extension: **`.gip`**.
- Ambition: a **1-to-1, production-level alternative to Figma**.
  - Explicitly NOT "a simple design tool with a few Figma-like features."
  - A **huge, serious, production-level application** with a **proper custom engine** for:
    vector geometry, canvas interaction, selection, transforms, paths, layout, rendering,
    and all other systems required in a professional design application.

## The user & the working model

- The user is a **UI/UX designer, not a coder**.
- The user knows Figma is extremely complex — normally built/maintained by a large team:
  UI/UX designers, product designers, senior frontend engineers, graphics engineers,
  C++/WebAssembly engineers, rendering engineers, infrastructure engineers, QA engineers,
  and many other specialists.
- The user's conviction: **modern AI agents can now do a very large amount of that team's work —
  but ONLY if the entire project is properly structured, documented, tested, and controlled.**
- Therefore: the goal is NOT to blindly tell AI to "build a Figma clone." The goal is to create
  such a strong **foundation, documentation system, rule system, memory system, testing system,
  and architecture** that multiple AI agents can work on a very large codebase without:
  - constantly forgetting context;
  - duplicating logic;
  - breaking old features;
  - creating unnecessary code.

## Problem statement — why the user wants to create Gipulo

- Modern product design increasingly targets younger audiences, especially **Gen Z**.
- Users are **tired of the same traditional layouts**, cards, sections, rectangles, and
  predictable UI structures everywhere. Modern products use much more expressive interfaces.
- Designers are now expected to create things that feel:
  **more unique · more custom-crafted · more experimental · more visually impressive ·
  more dynamic · more interactive · less template-like · more "out of the box."**
- This forces complex layouts that **break normal geometric rules and layout patterns**, e.g.:
  asymmetrical cards · irregular sections · non-rectangular containers · complex cutouts ·
  inverted curves · concave corners · custom masks · overlapping layouts · broken-grid
  compositions · curved sections · irregular buttons · custom responsive shapes · 3D objects ·
  motion-based layouts · scroll-based interactions.
- Creating these as **static graphics** is possible in Figma today.
- The real problem: making them **properly responsive**. Designers build websites, applications,
  dashboards, mobile apps, and digital products that must work across many screen sizes —
  and that is where complex visual ideas start breaking.

## Overall goal vs Figma (the "match, then exceed" principle)

- The goal is NOT simply "copy Figma exactly."
- The goal, in order:
  1. **First reach the level where a serious Figma user can comfortably work inside Gipulo.**
  2. **Then solve the important problems that still make advanced design unnecessarily difficult.**
- The five concrete problems/solutions are captured in `02_problems_and_solutions.md`.
- The user stresses there are **many other missing opportunities** beyond those five; the list
  will grow. Gipulo should keep improving beyond the initial five.

## Build order mandate

- **Before aggressively adding features**, the first goal is the **perfect agent setup, project
  memory system, rule system, and documentation system** (details: `03`, `04`).
- If hundreds of features are built without that foundation, AI agents will eventually:
  forget previous decisions · recreate existing functions · add duplicate logic · create
  unnecessary files · add unnecessary dependencies · break old features · change architecture
  without understanding it · create inconsistent UI · create conflicting canvas logic · create
  temporary patches instead of real fixes · forget why something was implemented a certain way.
  → "For a project as large as Gipulo, that would become a disaster."
- **The documentation system must be treated as part of the engineering architecture.**
- After the foundation, the **first major visible system is the Gipulo Design System**
  (details: `04_rules_system.md` §Design-System-first and `05_design_system_brief.md`).

## Final principle (two simultaneous requirements)

1. **Extremely detailed documentation** — detailed enough that a new AI agent, a new senior
   engineer, a new designer, or a future human team can enter the project and understand what is
   happening **without rebuilding the entire context from scratch**.
2. **Extremely clean and minimal code** — minimal in duplication, unnecessary logic, unnecessary
   files, unnecessary dependencies, unnecessary abstractions, dead code, clutter, conflicting
   systems. NOT minimal only by line count.
- The project **may become extremely large — that is expected**. But the complexity must come
  from the **capability of the product**, not from a messy codebase.
- End goal: advanced features can keep growing **without the project becoming impossible for AI
  agents or human developers to understand and maintain.**
