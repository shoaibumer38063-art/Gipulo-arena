# 03 — The Production Documentation System (9 parts + Update Rule)

> Distilled from `docs/source/gipulo_project_vision_refined_v2.md` (documentation foundation
> sections). This system is part of Gipulo's engineering architecture — not optional paperwork.

## Why it exists

- Documentation must be like a **real production company documenting a large product**: features ·
  behavior · implementation · architecture · APIs · edge cases · limitations · testing · known
  issues · product decisions · onboarding information.
- For human teams: a new developer/designer joining mid-production must understand the system
  without asking the original team about every small detail.
- For Gipulo this matters even MORE because the project is built heavily with AI agents:
  **every new AI session must be able to understand the relevant part of the project without
  guessing.**

---

## 1. Feature Documentation

- Every major feature gets its own Markdown documentation, e.g.:
  `/docs/features/vector-editing/`, `/docs/features/auto-layout/`, `/docs/features/components/`,
  `/docs/features/groups/`, `/docs/features/frames/`, `/docs/features/text/`,
  `/docs/features/responsive-vectors/`.
- Each feature document should explain (as applicable): what the feature is · why it exists ·
  user behavior · UI behavior · interaction states · keyboard behavior · mouse/pointer behavior ·
  selection behavior · expected canvas behavior · dependencies · related systems · implementation
  notes · important rules · edge cases · known bugs · limitations · test cases · regression risks.
- Not every feature needs identical headings, but documentation must be detailed enough that
  **another agent can understand the feature without reconstructing it from the code.**

## 2. Feature Index / Feature Registry

- A separate central `.md` file = **master index of ALL Gipulo features** → implemented at
  `docs/features/FEATURE_REGISTRY.md`.
- Covers: completed · partially completed · planned · experimental · deprecated · removed features.
- For each feature: pointer to related documentation AND major code location.
- MUST be updated whenever: a feature is added · a feature is removed · a feature changes status ·
  a major feature behavior changes.
- Purpose: an agent understands **what already exists before creating something new**.

## 3. Debugging and Fix Documentation (project's debugging memory)

- Separate documentation system specifically for **bugs, fixes, and regressions** → `docs/bugs/`.
- Scenario that motivates it: many hours spent fixing a difficult canvas bug (transform
  coordinates, zoom, hit-testing, vector point selection, bounding boxes…); six months later a
  similar bug appears. The agent must NOT start from zero — it should search previous debugging
  history and learn: symptoms · cause · what was attempted · what actually fixed it · which files
  were involved · what must not be changed · which regression test was added.
- **Mandatory bug record template:**

```text
Bug:
Symptoms:
Affected Feature:
Root Cause:
Fix:
Files Changed:
Tests Added:
Regression Risks:
Verification:
Related Bugs:
```

## 4. User Prompt Archive

- Separate archive for important user prompts → `docs/prompts/` (`.txt`, `.md`, or another
  searchable plain-text format).
- Why: one prompt may contain multiple details not fully represented by the final implementation
  (e.g. two bugs + one visual correction + one feature request + one behavior rule + one Figma
  comparison). If the prompt disappears from history, the original intent is lost.
- Index each archived prompt with: date · task ID · feature · subsystem · related bug · keywords ·
  related documentation.
- The archive does NOT replace proper documentation — it is the **historical reference showing
  how and why the user guided the AI toward a certain result.**

## 5. Agent Task Summary / Reasoning Summary

- The user originally wanted a documentation system for "AI thinking" — refined to: **do NOT store
  private raw chain-of-thought.** Instead, after an important task, save a structured
  **Task Summary / Decision Summary** → `docs/tasks/`.
- Must explain: what problem it was solving · what it inspected · what approach it selected · why
  that approach · what alternatives were rejected · what files were changed · what behavior
  changed · how it verified the result · what risks remain · what should be done next if the
  problem returns.
- Gives future agents useful reasoning context without hidden internal thought processes.

## 6. Architecture Documentation

- Dedicated docs for core architecture, SEPARATE from feature documentation → `docs/architecture/`:
  `canvas.md` (hub), `engine-core.md` (core/shell boundary law), `document-tree.md`,
  `coordinate-system.md`, `vector-engine.md`, `rendering.md`, `selection-system.md`,
  `transforms.md`, `layout-engine.md`, `input-system.md`, `undo-redo.md`, `file-format.md`,
  `code-generation.md`, `dependencies.md` (registry).
- Why critical: many features depend on the same core systems. Example — changing coordinate
  conversion may affect: selection · resize · rotate · vector editing · snapping · rulers ·
  guides · comments · plugins · exports.
- **AI should always know which systems are foundational.**

## 6b. Design System Documentation (formalized 2026-10-11)

- Dedicated docs for the visual + interaction foundation, SEPARATE from features and
  architecture → `docs/design-system/`: `01_foundations` · `02_tokens` · `03_typography` ·
  `04_icons` · `05_components_controls` · `06_components_surfaces` ·
  `07_component_architecture` · `08_canvas_ui` (incl. the CLOSED guide-color language) ·
  `09_cursors` · `10_motion` · `11_accessibility` · `12_responsive` · `13_research_notes`.
- Division of authority: `rules/ui.md` + `rules/icons.md` + `rules/canvas.md` state the LAW;
  `docs/design-system/` states the SYSTEM (structure, states, values); the source HTML
  (`docs/source/Gipulo — Design System.html`) remains rendered ground truth; `07` remains the
  distilled value record.
- Guide-color meanings are closed (`08` §9) — a new canvas overlay color is an ADR-level change
  that updates the DS doc + `rules/canvas.md` in the same task.

## 7. Architecture Decision Records (ADRs)

- For important architecture decisions, short permanent records (stored in `docs/architecture/`):
  what decision was made · why · what alternatives were considered · what consequences it creates ·
  whether the decision is still active.
- Prevents a future agent from replacing an important system **just because it didn't understand
  why it existed.**

## 8. Testing and Regression Documentation

- Every important feature and difficult bug fix should have tests wherever practical.
- Systems to maintain tests for: vector geometry · path editing · coordinate conversion ·
  hit-testing · selection · resize · rotate · snapping · zoom · canvas input · Auto Layout ·
  responsive behavior · undo/redo · file saving/loading · code generation · generated preview ·
  performance.
- When a bug is fixed, **add a regression test whenever possible**.
- Purpose: **"The same bug should not be allowed to silently return later."**

## 9. Performance Documentation

- Performance is extremely important in a design editor: **a feature is not truly working if it
  technically works but creates lag.**
- Define performance expectations for: canvas FPS · pan · zoom · pointer response · selection ·
  large documents · vector editing · drag operations · memory use · file loading · file saving ·
  generated-code preview.
- **AI agents are not allowed to solve one feature by creating a major performance problem
  somewhere else.**

---

## THE Documentation Update Rule (one of the most important rules)

**Documentation must be updated at the end of EVERY meaningful task.**
This does NOT mean rewriting every document — update only the documents affected by the task.
After completing a task, the agent must check ALL of these questions:

1. Did a feature change?
2. Did behavior change?
3. Did architecture change?
4. Was a bug fixed?
5. Was a new regression test added?
6. Did a rule change?
7. Did a file structure change?
8. Did the feature index need updating?
9. Should the task summary be recorded?
10. Should the prompt be archived?

This documentation update is **part of the normal completion process for every task** — a task
is not "done" until this checklist has been run.
