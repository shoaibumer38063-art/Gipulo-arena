# ⚠️ GIPULO — MASTER AGENT MEMORY (READ THIS FIRST, EVERY SESSION)

> **This file is the entry point of the agent's permanent memory for the Gipulo project.**
> Any AI agent working in this workspace MUST read this file completely before doing ANY work,
> then follow the reading order below. This file and the files it points to ARE the project memory.

---

## 0. FRESH-PROJECT DECLARATION (CRITICAL — READ FIRST)

- **This is a COMPLETELY NEW project.** It was freshly started on **2026-10-11**.
- **Any memory, files, decisions, code, or context from ANY previous project named "gipulo"
  (from older sessions or older workspaces) is INVALID and MUST be ignored.**
- Do NOT mix old gipulo memories with this project. Do NOT assume anything from previous
  gipulo work carries over. The ONLY valid foundation of this project is the four canonical
  source documents listed in Section 2 — nothing else.
- If you (the agent) recall anything about a "gipulo" project from before 2026-10-11,
  treat that recollection as belonging to a DIFFERENT, DEAD project. This project's truth
  lives ONLY in the files in this workspace.

## 1. WHO THE USER IS & WHAT THIS PROJECT IS

- The user is a **UI/UX designer, not a coder**. The project will be built heavily with AI agents.
- The product: **Gipulo** (file extension **`.gip`**) — a **1-to-1, production-level alternative
  to Figma**. NOT a simple design tool with a few Figma-like features. A huge, serious,
  production-level application with a proper **custom engine** for vector geometry, canvas
  interaction, selection, transforms, paths, layout, rendering, and all systems a professional
  design application requires.
- The user believes AI agents can do the work of a large team (UI/UX designers, product designers,
  senior frontend engineers, graphics engineers, C++/WebAssembly engineers, rendering engineers,
  infrastructure engineers, QA) — **but only if the project is properly structured, documented,
  tested, and controlled.**
- The goal is NOT to blindly tell AI to "build a Figma clone." The goal is a foundation
  (documentation, rules, memory, testing, architecture) strong enough that multiple AI agents can
  work on a very large codebase **without forgetting context, duplicating logic, breaking old
  features, or creating unnecessary code.**

## 2. CANONICAL SOURCE DOCUMENTS (VERBATIM, NEVER EDIT, ULTIMATE TRUTH)

Stored at `gipulo/docs/source/` (byte-identical copies of the user's originals in `uploads/`):

| # | File | What it is |
|---|------|------------|
| 1 | `gipulo_project_vision_refined_v2.md` | Vision, 5 problems + 5 solutions, AI-first development foundation, documentation system, rules system, final principles |
| 2 | `Gipulo — Complete Design System Brief.md` | Complete requirements for the Design System (macro → micro), every component/state/canvas-UI/guide/cursor requirement |
| 3 | `gipulo_canvas_rules_prompt_refined.md` | Requirements for the future `canvas-rules.md` — the "main law" of the canvas; every canvas subsystem that must be ruled |
| 4 | `Gipulo — Design System.html` | WORKING VISUAL DESIGN SYSTEM REFERENCE with concrete tokens, colors, sizes, states, guide specs, motion values. A living spec, not decoration. |

**Every line of these documents is highly important (user's explicit words).**
If a distilled memory file ever conflicts with a source document, **the source document wins.**

## 3. DISTILLED MEMORY (FAST, STRUCTURED RECALL)

Stored at `gipulo/docs/memory/` — comprehensive topic-by-topic distillations of ALL details
from the four sources. Reading order and index: `gipulo/docs/memory/00_INDEX.md`.

| File | Contents |
|------|----------|
| `01_project_identity_and_vision.md` | Identity, goals, problem statement (why Gipulo exists), the "match Figma then exceed" goal |
| `02_problems_and_solutions.md` | All 5 Problems + 5 Solutions in full detail (Responsive Vector Segments, Native Design-to-Code + Code Workspace, 3D + Live Link, Native Scroll Interaction, affordable core) |
| `03_documentation_system.md` | The 9-part production documentation system + Documentation Update Rule (update docs at end of EVERY meaningful task) |
| `04_rules_system.md` | Rules system plan, canvas rules importance, code quality, feature-addition 10 steps, bug-fixing 10 steps, removal checklist, final principles |
| `05_design_system_brief.md` | Full distillation of the Design System Brief (scope, states, guides, cursors, panels, motion, tokens, accessibility, performance) |
| `06_canvas_rules_requirements.md` | Full distillation of the canvas-rules requirements (coordinate systems, screen-space vs canvas-space, geometry sync, selection, snapping, rendering, etc.) |
| `07_design_tokens_and_visual_specs.md` | ALL concrete values extracted from the Design System HTML: every color token (light+dark), typography, sizes, radii, states, guide specs, motion timings, zoom-visibility rules |
| `08_foundation_roadmap.md` | Current project status + foundation build order derived from the documents |
| `CHANGELOG.md` | Memory change log — append an entry whenever memory files change |

## 4. THE TEN COMMANDMENTS OF GIPULO (ALWAYS TRUE)

1. **Foundation before features.** Perfect agent setup, memory, rules, and documentation come
   BEFORE aggressively adding features. Without it, agents forget decisions, duplicate logic,
   break features, create clutter — "for a project as large as Gipulo, that would become a disaster."
2. **Documentation must be updated at the end of EVERY meaningful task** (only the affected docs;
   see the 10-question checklist in `03_documentation_system.md`).
3. **Canvas is the most dangerous system.** Any agent touching the canvas must FIRST read the
   canvas rules before touching implementation. Never solve one canvas problem by creating
   another canvas problem elsewhere.
4. **Single sources of truth.** One authoritative coordinate-conversion system; canonical geometry
   operations; rendered pixels are NEVER the only source of geometry truth; camera state and
   document state stay separate; UI state and document state stay separate.
5. **Screen-space vs canvas-space discipline.** Interactive UI (handles, outlines, guides,
   thresholds) stays constant in SCREEN PIXELS while positioned on canvas geometry. Zoom must not
   shrink/grow hit areas.
6. **Extremely detailed documentation + extremely clean minimal code — at the same time.**
   Minimal means: no duplication, no unnecessary logic/files/dependencies/abstractions, no dead
   code, no clutter, no conflicting systems. NOT minimal line count at the cost of readability.
   Complexity must come from product capability, never from a messy codebase.
7. **Reuse before create.** Before writing new code, check whether the logic already exists.
   Reuse or extend it. Never create a second implementation of the same idea.
8. **No generic rules, no generic UI.** Rules must define exactly what "clean" means inside
   Gipulo. UI must never fall back to browser/React/OS defaults (selection color, dropdowns,
   focus rings, scrollbars…). Everything uses Design System tokens — no magic values like
   `border-radius: 7px` unless from the Design System or with a documented feature-specific reason.
9. **Familiar workflow, different identity.** Editor layout/interaction stays familiar to Figma
   users (layers left, canvas center, properties right); visual identity is Gipulo's own.
10. **Performance is a feature requirement.** A feature is not working if it lags. No agent may
    fix one feature by creating a performance problem elsewhere. Budgets must be measurable,
    tested on representative heavy documents.

## 5. KEY FACTS AT A GLANCE (details in memory files)

**The 5 Problems → Solutions** (full detail: `02_problems_and_solutions.md`):
1. Complex shapes break in responsive layouts → **Responsive Vector Segments**: only user-selected
   straight segments stretch (highlighted red → activated green); curves/cutouts stay preserved.
   NOT Flutter CustomClipper; a visual, easier 9-slice-style selective stretch for editable vectors.
2. Design-to-code is an afterthought in other tools → **Native Design-to-Code**: planned into the
   editor architecture from the beginning; every design feature has a code representation; output =
   real project structure (folders/components/tokens/assets/routes), plus a **Code Workspace**
   (canvas left, code/runtime right) with **Code** and **Preview** sub-sections where Preview runs
   the ACTUAL generated code (not a re-render of the canvas).
3. 3D matters in modern UI but Figma is 2D → **Simple native 3D + external app connection**:
   import from Blender/Spline/Cinema 4D; only UI-relevant controls (move/rotate/scale/camera/
   display/interaction); **Live Link / Source Link** so edits in the source app update Gipulo while
   Gipulo-side settings stay intact. Gipulo must NOT try to replace professional 3D software.
4. Scroll-driven design is rising → **Native Scroll Interaction System**: connect scroll progress
   to position/scale/rotation/opacity/blur/mask/path-progress/3D/depth/clipping/reveal; define
   start/end/trigger/range/easing/pinning/direction/timeline — without hand-writing JavaScript.
5. Advanced features get paywalled → keep the **core professional design experience accessible**;
   expensive services (AI inference, cloud rendering, storage, enterprise collab, hosted builds,
   premium team features) may be paid. Sustainable business model, but no artificial restriction
   of basic professional capabilities.

**Core identity tokens from the Design System HTML** (full tables: `07_design_tokens_and_visual_specs.md`):
- Accent `#0d99ff` (hi `#2aa7ff`, lo `#0b82dc`) · Danger `#f24822` · Success `#14ae5c` ·
  Warning `#ffb800` · Component purple `#9747ff`
- Guide colors with FIXED meanings: **blue `#0d99ff` = geometry/selection/paths** ·
  **magenta `#ec2fd0` = spacing/distance/padding** · **orange `#ff8a00` = snapping/measurement** ·
  **red `#f24822` = ruler guides (never errors on canvas)**
- UI font: Inter, base `450 11px/16px` · Mono: JetBrains Mono · Title `13/20 · 600` ·
  Caption `10/16 · 500`
- Light: canvas `#f5f5f5`, panel `#fff`, input `#f5f5f5`, toolbar `#2c2c2c` ·
  Dark: canvas `#1e1e1e`, panel `#2c2c2c`, input `#383838`
- Controls: buttons 24px · fields 28px · layer rows 28px · menus 224px wide, radius 13px,
  items 24px · radius medium 5px
- Motion: hover 80ms · toggle 120ms · menu 160ms · ease `cubic-bezier(.2,0,0,1)` ·
  reduced-motion: none
- Canvas overlays: guide lines 1px at EVERY zoom · handles 7–8px with 16px hit areas ·
  labels hide below 25% zoom, handles below 12% · snap lines dashed `4 3` · density levels
  20/24/28px · min hit target 24px

## 6. PROJECT STATUS (as of 2026-10-11)

- **Phase: FOUNDATION → ENGINE.** Engine subsystems built: coords (009), geometry (010), document serializer (011). No editor UI yet.
- Completed so far:
  1. Ingestion of the four canonical documents into this memory system (task summary 001);
     docs skeleton (`docs/features|architecture|bugs|prompts|tasks`), empty feature registry,
     prompt archive, first task summary.
  2. ✅ **Rules system v1 — ALL 17 rule files authored and ACTIVE** in `gipulo/rules/`
     (task summary 002), including `rules/canvas.md` — the research-grounded "main law of the
     canvas" with binding micro-detail registry and mandatory change protocol. Values tagged
     [PROPOSED] are binding defaults until ADRs confirm them (`rules/documentation.md` D7).
  3. ✅ **Architecture contracts v1 — ALL 14 docs authored** (DESIGN phase) in
     `gipulo/docs/architecture/`, hub = `canvas.md`, placement law = `engine-core.md`,
     registry = `dependencies.md` (task summaries 003 + 004). PLUS: **ADR-001 ACCEPTED
     (2026-10-11, owner decision — OPTION B): Rust/Wasm engine core + thin TypeScript/React
     shell; renderer = wgpu inside the core (WebGPU primary, WebGL2 via Naga fallback).**
     Boundary Duplication Law: no editor logic in the TS shell; shell has NO document write API.
  4. ✅ **Repo scaffold COMPLETE (2026-10-11)** — `gipulo/core` cargo workspace (9 crates
     compiling; dependency law enforced by `scripts/check-boundaries.py`; native tests green),
     `gipulo/app` shell (React 19 + Vite + strict TS; typecheck/tests/build green), wasm
     bindings pipeline (`scripts/build-wasm.sh` → `bindings/pkg`, Node-validated module
     exporting `protocol_version`), CI gates (core · wasm · shell), fixtures spec for the five
     canvas §26 reference documents (task summary 005). **Zero editor features — foundation
     tooling only.**
  5. ✅ **Design System FORMALIZED (2026-10-11)** — all 14 docs in `gipulo/docs/design-system/`
     (the single visual+interaction source of truth: foundations, tokens, typography, icons,
     controls, surfaces, component architecture, canvas UI with the **CLOSED guide-color
     language**, cursors, motion, accessibility incl. a WCAG-AA contrast audit, responsive,
     research notes — task summary 006). Roadmap steps 3 + 4 COMPLETE.
     TWO owner-review items queued: (a) AA contrast token bumps (`11` §2); (b) RVS affordance
     color recommendation (magenta eligible / green activated) → ADR-005 before RVS ships.
  6. ✅ **Design System IN CODE (2026-10-11)** — `gipulo/app/src/design-system/`: drift-tested
     token layer (`tokens.css` ↔ `tokens.ts`; a test fails on any mismatch in either direction),
     self-hosted Inter + JetBrains Mono (offline-safe), 17 registry entries (15 component modules + token layer + global scrollbar) with
     state-matrix tests (**47/47 passing**), gallery story page with an illustrative editor
     frame + light/dark/system switcher. `tsc --noEmit` PASS · `vitest run` PASS · `vite build`
     PASS (task summary 007). The DS is now reusable code, not just docs — **all future UI MUST
     use `app/src/design-system/`** (composition law) and tokens only (UI1).
  7. ✅ **ADR BUNDLE 002/003/004 AUTHORED (2026-10-11)** — decision records now exist (all
     **PROPOSED; binding as implementation defaults until the owner reviews** — task summary
     008): ADR-002 transform/matrix/numeric conventions (decomposed canonical fields; matrices
     derived in CSS/SVG/Canvas order; degrees (−180,180]; flip flags; one constants module) ·
     ADR-003 `.gip` v0 (single-file JSON; u64 salt+counter IDs with merge-remap law; bit-exact
     f64; sidecar content-addressed assets; forward-only migrations) · ADR-004 history
     (structured op logs; undo replays recorded values; truncate redo tail; in-memory v1).
     All affected contracts' open-question sections closed. Owner-review queue (non-blocking):
     stroke scaling under resize · rotation display range · public `.gip` spec · undo cap;
     plus carried DS items (AA contrast, RVS colors → ADR-005 before RVS).
  8. ✅ **COORDINATES PHASE (2026-10-11, task 009)** — `core/crates/gip-coords/` realized:
     `Mat2` (column-major, CSS/SVG order) · `Trs` canonical fields + decomposition ·
     `Camera` (pointer-anchored zoom 2%–4096%) · viewport/screen/device conversions ·
     `normalize_deg` · single `eps` module · `Space` enum. 38 crate tests; workspace 48 Rust
     tests green; clippy/fmt/boundaries/wasm check green. Rust 1.99.0 reinstall needed each
     session (`~/.cache` not snapshotted).
  9. ✅ **GEOMETRY PHASE (2026-10-11, task 010)** — ADR-006 (PROPOSED): cubic-only paths,
     vertex = anchor + absolute handles, explicit closed/fill rule; path DATA in
     `gip-document::path`, OPERATIONS in `gip-geometry` (cubic math, exact bounds, flatten,
     nearest/hit, transform/insert/delete/reverse). 88 workspace tests green. Hover baseline
     ≈10 ms on 1000 segments → spatial index is the next performance item.
  10. ✅ **DOCUMENT SERIALIZER (2026-10-11, task 011)** — `gip-document`: node/page/document schema,
     `.gip` v1 container (`to_json`/`from_json`, validate-then-write), 16-hex IDs (ADR-003 A1 —
     JSON numbers are lossy above 2^53), 17 validation rules, bit-exact f64, golden fixture generated
     by the serializer (`GIP_UPDATE_GOLDEN=1`). `gip-geometry::bridge` = `trs_of`/`transform_of`.
     114 workspace tests green. Deferred: documentHash/SHA-256, NFC, asset store, transaction API.
  11. ✅ **HISTORY / TRANSACTIONS (2026-10-11, task 012)** — `gip-history`: op log (ADR-004),
     atomic `transact` with rollback, exact undo/redo, redo truncation, byte budget, dirty tracking.
     128 workspace tests green. **Write door CLOSED (A6):** `gip-document::edit` holds the ops and
     `apply`; document fields are crate-private; boundary check enforces that only gip-history uses it.
  12. ✅ **SEGMENT INDEX (2026-10-11, task 013)** — ADR-007 (PROPOSED): `SegmentIndex` static BVH per
     path; equals brute force; 1.13 ms vs 142 ms per query on a dense 1000-segment fixture (fixture-specific).
     131 workspace tests green.
  13. ✅ **P0 RECOVERY POLICY (2026-10-11, task 014)** — ADR-008 (PROPOSED): trap recovery from committed
     validated whole-file snapshots (shell stores, core serializes); autosave 2 s debounce / 30 s max; 3
     generations; local-only diagnostics; fault injection release-excluded. Owner review: cadence, retention,
     clean-close, copy. Docs-only.
- Next engine steps:
  → Playwright E2E (roadmap step 6) → ADR-005 (RVS colors) + P0 panic/trap ADR before their
  subsystems — see `gipulo/docs/memory/08_foundation_roadmap.md`.
- **Do not start building editor FEATURES until the user says so.** (DS formalization + DS code
  are foundation work, not editor features.)

## 7. MANDATORY WORKFLOWS (from the source documents)

- **Adding a feature (10 steps):** read feature index → inspect related features → inspect
  architecture docs → inspect rules → search codebase for reusable logic → identify affected
  systems → plan smallest correct implementation → implement → test → verify neighboring
  behavior → update documentation.
- **Fixing a bug (10 steps):** reproduce → identify exact expected behavior → inspect bug history
  → find real root cause → avoid symptom-only patches → smallest safe fix → test original problem
  → test related behavior → add regression coverage → update bug documentation.
- **Removing a feature:** also remove dead imports/styles/tests/docs, unused dependencies,
  obsolete flags, compatibility code, old state/listeners/unreachable code. Never leave half a
  feature scattered around.
- **Every bug record uses this template:** Bug / Symptoms / Affected Feature / Root Cause / Fix /
  Files Changed / Tests Added / Regression Risks / Verification / Related Bugs.
- **After every important task:** save a structured Task Summary (problem, what was inspected,
  approach chosen, why, rejected alternatives, files changed, behavior changed, verification,
  remaining risks, next steps) in `gipulo/docs/tasks/`, and archive important user prompts in
  `gipulo/docs/prompts/`.

## 8. FILE MAP

```
/home/user/
├── GIPULO_AGENT_MEMORY.md          ← THIS FILE (master beacon)
├── AGENTS.md                       ← short pointer to this file
├── uploads/                        ← user's original attachments (do not edit)
└── gipulo/                         ← PROJECT ROOT (fresh project, started 2026-10-11)
    ├── AGENTS.md                   ← operating contract for agents inside the project
    ├── docs/
    │   ├── source/                 ← 4 verbatim canonical documents (NEVER EDIT)
    │   ├── memory/                 ← distilled permanent memory (00_INDEX.md first)
    │   ├── features/               ← feature docs + FEATURE_REGISTRY.md (currently empty)
    │   ├── architecture/           ← architecture docs + ADRs (not yet written)
    │   ├── bugs/                   ← bug/fix/regression records (template in README)
    │   ├── prompts/                ← user prompt archive (first entry: 2026-10-11)
    │   └── tasks/                  ← agent task/decision summaries
    └── rules/                      ← AI rules system (planned files listed in README)
```

---
*Memory established 2026-10-11. Update the CHANGELOG (`gipulo/docs/memory/CHANGELOG.md`)
whenever any memory file changes. This document must stay accurate at all times.*
