# AGENTS.md — Gipulo Project Operating Contract

> Every AI agent (and future human developer) working inside `/home/user/gipulo/` must follow
> this contract. Read `/home/user/GIPULO_AGENT_MEMORY.md` first — it is the master memory file.

## 0. Fresh project

This project started fresh on **2026-10-11**. Any older "gipulo" project memory from previous
sessions is **invalid** and must not influence decisions here. The project's truth consists ONLY of:

1. The four canonical source documents in `docs/source/` (verbatim, never edit them).
2. The distilled memory in `docs/memory/` (kept in sync with the sources).
3. Everything created afterwards under this folder, following the workflows below.

## 1. Required reading order (every session, before any work)

1. `/home/user/GIPULO_AGENT_MEMORY.md` — master memory + ten commandments + workflows.
2. `docs/memory/00_INDEX.md` — map of the distilled memory.
3. The memory file(s) for the topic you are about to touch (see index).
4. If the task touches the **canvas** in any way: `docs/memory/06_canvas_rules_requirements.md`
   and (once written) `rules/canvas.md`. Canvas work without reading the canvas rules is FORBIDDEN.
5. If the task touches **UI/visuals**: `docs/memory/05_design_system_brief.md` and
   `docs/memory/07_design_tokens_and_visual_specs.md`.
6. The relevant verbatim source document in `docs/source/` whenever a distilled file is
   ambiguous or insufficient. **Sources always win over distillations.**

## 2. Non-negotiable engineering laws (condensed; full text in memory files)

- **Foundation before features** — documentation, rules, memory, and testing systems come first.
- **Update documentation at the end of every meaningful task** (only affected docs; use the
  10-question checklist in `docs/memory/03_documentation_system.md`).
- **Single sources of truth** — one coordinate-conversion system; canonical geometry operations;
  rendered pixels are never the only geometry truth; camera state ≠ document state;
  UI state ≠ document state; displayed precision ≠ internal precision.
- **Screen-space invariance** — handles, outlines, guides, thresholds stay constant in screen
  pixels; their positions stay attached to canvas geometry.
- **Canvas overlays are editor UI, never document geometry** — they don't export, aren't
  selectable, don't affect layout/bounds/saved files.
- **Reuse before create** — search for existing logic first; never build a second implementation
  of the same idea; no duplicate geometry/state/systems.
- **Clean minimal code** — no duplication, unnecessary files/dependencies/abstractions, dead code,
  temporary hacks left permanently, or clutter. Minimal ≠ unreadable one-liners or forced
  single-file everything.
- **Design System tokens only** — no magic values (e.g. `border-radius: 7px`, `#3478ff`) unless
  they come from the Design System or have a documented feature-specific reason. No browser/OS
  default UI behavior (selection colors, native `<select>`, default focus rings, scrollbars…).
- **Familiar workflow, different identity** — Figma-familiar layout and interaction; Gipulo's own
  visual identity.
- **Performance is part of correctness** — never fix one feature by creating lag elsewhere;
  measure against budgets on representative heavy documents.
- **Determinism** — the same saved `.gip` document must reconstruct the same geometry and visuals.
- **History hygiene** — one drag/resize/rotate = ONE undo operation; hover/preview states never
  enter document history.
- **Rules quality** — project rules must be specific engineering standards, never generic advice
  like "write clean code."

## 3. Mandatory workflows

- **Feature addition (10 steps)**, **bug fixing (10 steps)**, **removal checklist**, **bug record
  template**, **task summary template**: all defined in `/home/user/GIPULO_AGENT_MEMORY.md` §7 and
  `docs/memory/03_documentation_system.md` + `docs/memory/04_rules_system.md`. Follow them exactly.
- **Feature registry** (`docs/features/FEATURE_REGISTRY.md`) must be updated whenever a feature is
  added, removed, changes status, or changes major behavior.
- **Prompt archive** (`docs/prompts/`) — important user prompts get archived with date, task ID,
  feature, subsystem, related bug, keywords, related docs.
- **Task summaries** (`docs/tasks/`) — after every important task, save a structured summary
  (problem, inspection, approach + why, rejected alternatives, files changed, behavior changed,
  verification, remaining risks, next steps). Store reasoning summaries — NOT private raw
  chain-of-thought.
- **ADRs** (`docs/architecture/`) — every important architecture decision gets a permanent record
  (decision, why, alternatives, consequences, still-active status).

## 4. Folder map

```
gipulo/
├── AGENTS.md                 ← this contract
├── docs/
│   ├── source/               ← 4 canonical verbatim documents — READ-ONLY
│   ├── memory/               ← distilled permanent memory — keep in sync, log changes
│   ├── features/             ← per-feature docs + FEATURE_REGISTRY.md
│   ├── architecture/         ← architecture docs + ADRs
│   ├── bugs/                 ← bug/fix/regression records
│   ├── prompts/              ← user prompt archive
│   └── tasks/                ← agent task/decision summaries
└── rules/                    ← AI rules system (global, canvas, vector-geometry, ui, …)
```

## 5. Memory maintenance

- When project reality changes, update the affected memory file(s) AND append an entry to
  `docs/memory/CHANGELOG.md`.
- Never edit `docs/source/` files. If a source document is superseded by a newer user document,
  add the new document to `docs/source/` and note the supersession in the memory index + changelog.
- If distilled memory and a source document ever conflict, fix the distilled memory.

## GitHub sync (end of every task)

After each completed task, run `scripts/sync-github.sh "<short task message>"` from the repo root.
It commits everything and pushes to `origin/main`. Credentials come only from the `GITHUB_TOKEN`
environment variable and are never written to any file. Exit code 2 means the push was skipped
(no token): the commit is still made locally, and the owner must be told.
