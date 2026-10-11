# Memory CHANGELOG (append-only)

Every change to any file in `docs/memory/` (or to the master beacon `/home/user/GIPULO_AGENT_MEMORY.md`)
must be logged here with date, what changed, and why.

---

## 2026-10-11 — Geometry phase: path model (ADR-006) + `gip-geometry` realized (task 010)

- **What:** ADR-006 (PROPOSED): cubic-only path model; vertex = anchor + absolute handles; explicit
  closed flag and fill rule; straight-segment data definition. Path DATA in `gip-document::path`
  (layer-1 law); OPERATIONS in `gip-geometry`. Implemented: cubic math, exact bounds, flattening
  (proven chord-line bound), branch-and-bound nearest point, point-in-fill (both rules), distance
  to path with shared bound, transform/reverse/close/insert/delete ops.
- **Tests:** workspace 88 green (geometry 29 unit + 8 VG7 golden + 2 property). clippy/fmt/boundaries/wasm green.
- **Measured:** distance_to_path ≈10.4 ms/query on 1000 segments (release) — spatial index is next.
- **Corrections:** vector-engine §8.1 closed (cubic-only); §10 realized status added.
- **Lesson:** 5 bugs found this phase; 3 were wrong test laws/guarantees. Stated bounds must be
  checked against the object they bound (line vs segment).

## 2026-10-11 — Coordinates phase: `gip-coords` realized (task 009)

- **What:** `core/crates/gip-coords/` implemented against ADR-002: `Mat2` (column-major, CSS/SVG
  order), `Trs` canonical fields + decomposition + canonical math frame, `Camera` (pointer-anchored
  zoom, 2%–4096%), viewport/screen/device conversions, `normalize_deg`, single `eps` module,
  `Space` enum (8 spaces). Coordinate-system contract §10 added (realized status).
- **Tests:** 38 crate tests (25 unit + 12 property suites over a deterministic splitmix64 PRNG +
  1 doctest). Workspace: 48 Rust tests green. clippy `-D warnings`, fmt, boundary check, wasm check — all green.
- **Dependency law:** dev-dependencies deliberately NOT added; recorded in `dependencies.md` §5.5.
- **Toolchain:** Rust 1.99.0 reinstalled into `~/.cache` (not snapshotted).
- **Why:** first engine subsystem; every later canvas feature depends on these conversions.
- **Lesson:** three of four property failures were mis-stated laws, not code bugs (see task 009).

## 2026-10-11 — ADR bundle 002/003/004 authored (transforms · `.gip` v0 · history; task 008)

- **What:** Three decision records in `docs/architecture/`, closing ~15 ADR-gated open questions
  that blocked the first engine subsystems:
  - **ADR-002 (transform conventions):** canonical transforms = decomposed f64 fields
    (x·y·w·h·rotation·flipH·flipV; sizes ≥ 0), never stored matrices; matrices derived in the
    browser-native column-major 6-tuple order (CSS/SVG/Canvas2D — verified against W3C CSS
    Transforms L1 + MDN); `T·R·S` with rotation origin at the node's top-left; degrees normalized
    to (−180, 180] everywhere; flip = flags (confirms CAN-TR-5); zero-crossing flips during
    preview (preview/commit equivalence by construction); no skew v1; no external math crate —
    hand-rolled `Mat2` in `gip-coords`; ONE constants module (EPS_ROUNDTRIP/EPS_ANGLE/EPS_MATRIX).
  - **ADR-003 (`.gip` v0):** single-file UTF-8 JSON, schema-declared order, explicit fields,
    BTreeMap collections, byte-identical output; f64 as shortest round-trippable decimals
    (bit-exact); IDs = u64 (`salt(24)`+`counter(40)`) with a mandatory remap law for cross-lineage
    merges; sidecar content-addressed asset store + manifest (assets immutable v0); forward-only
    migrations; unknown fields preserved at bounded `ext` points; `meta.documentHash`; NO
    compression/NO camera/NO history in v0; TS mirror guarded by a cross-language fixture test.
  - **ADR-004 (history):** entries = structured op logs of field-level changes (closed op
    vocabulary); undo replays recorded values (mutators never re-run — byte-identity provable);
    commands + snapshots explicitly rejected; truncate redo tail; byte-budget cap (64 MiB
    [PROPOSED]); 500 ms text coalescing; in-memory v1; collab-non-precluding shape, no speculative
    CRDT code.
- **Why:** Roadmap step 7 — the decision bundles registered at scaffold time. Task summary
  `docs/tasks/2026-10-11_008_adr_bundle_002_003_004.md`.
- **Cascade:** `docs/architecture/README.md` ADR index (002/003/004 PROPOSED + ADR-005/P0 queued);
  `canvas.md` §5 tracker rewritten; open-question sections closed in `transforms.md` §7 ·
  `coordinate-system.md` §8 · `file-format.md` §8 · `undo-redo.md` §7 · `document-tree.md` §8.1.
  **Dependency-registry change (deliberate, ADR-003):** `dependencies.md` §2/§3 +
  `scripts/check-boundaries.py` register `serde`/`serde_json` for `gip-document` as the ONE
  layer-1 exception (version pinned at first use); boundary check re-run PASS.
  Owner-review items queued (non-blocking): stroke/effects scaling under resize · rotation display
  range · public `.gip` spec · undo memory cap/no-history-on-close (+ the two carried DS items).
- **Note:** All three are **PROPOSED — binding as implementation defaults** until the owner
  reviews. No code changed; scaffold + DS code remain green. Next: coordinates phase
  (`gip-coords` Mat2 + conversions) → geometry → document schema/serializer.

## 2026-10-11 — Design System implemented in code (tokens + 17 registry entries + gallery; task 007)

- **What:** `app/src/design-system/` — the DS docs made real and testable: `tokens.css` (numeric
  source of truth, all doc `02` values, light/dark) mirrored 1:1 by `tokens.ts` with a
  **drift test that fails both directions** (this is now the enforced boundary between docs and
  code); self-hosted Inter variable + JetBrains Mono variable woff2 (offline-safe, no CDN);
  `globals.css` resets (custom scrollbars, selection, caret, reduced-motion kill switch);
  17 registry entries — 15 component modules (Button · Segmented · Field (scrub previews→single commit) · SearchField
  · Toggle · Checkbox/Radio · Slider · Chip · Menu (keyboard-complete, submenu intent) · Tooltip
  (400ms/instant focus) · Tabs · Panel/Section · PropertyRow · LayerRow (indent law) · Icon
  sprite (reference set + node types)) each with its CSS and **state-matrix tests** — 47 tests
  across 6 suites, all passing; gallery story page (`gallery/Gallery.tsx`) rendering every
  component's states + an illustrative editor frame with a light/dark/system switcher; shell
  wired to render the gallery (`App.tsx`, `main.tsx`).
- **Why:** Roadmap step 8 — the vision's "agents should never guess what this looks like" only
  holds when the DS exists as tested code. Task summary
  `docs/tasks/2026-10-11_007_design_system_implementation.md`.
- **Bugs found by the new tests and fixed:** Field's Escape-then-blur committed a discarded draft
  (fixed with an explicit cancel flag — the test now guards it); pointer capture made progressive
  for jsdom/older browsers; strict-TS fallout (`exactOptionalPropertyTypes`, `@types/node`,
  Vitest URL semantics).
- **Cascade:** `07_component_architecture.md` registry now carries per-entry code status (17 ✅;
  remaining entries marked with their first-consumer gate); DS README "what exists in code";
  tasks README + roadmap status updated. Verified: `tsc --noEmit` PASS · `vitest run` 47/47 PASS
  · `vite build` PASS (252 kB JS / 77 kB gzip).
- **Note:** No editor features built. Gallery is foundation tooling. The DS AA-contrast owner
  review (`11` §2) and ADR-005 (RVS colors) remain open as before.

## 2026-10-11 — Design System FORMALIZED (roadmap steps 3 + 4 complete)

- **What:** All 14 Design System documents authored in `gipulo/docs/design-system/` — the single
  visual + interaction source of truth: README hub (eight DS laws, authority order, change
  protocol) · `01_foundations` · `02_tokens` (full light/dark color set, typography, spacing,
  radii, borders, elevation, opacity, motion, z-layers, density, control metrics) ·
  `03_typography` · `04_icons` · `05_components_controls` · `06_components_surfaces` · 
  `07_component_architecture` (composition law, 26-entry registry, state-completeness gate) ·
  `08_canvas_ui` (universal zoom laws, blue/magenta/orange/red guide systems with every state,
  **§9 THE CLOSED COLOR LANGUAGE**) · `09_cursors` · `10_motion` · `11_accessibility`
  (incl. a real WCAG-AA contrast audit of the reference tokens with proposed token bumps) ·
  `12_responsive` · `13_research_notes` (the brief's research mandate executed with sources:
  Figma UI3 ×2, Fast Company, Cursor Design Mode analyses).
- **Why:** Roadmap step 3 (DS before editor features) + step 4 (guide-color language closed).
  Task summary: `docs/tasks/2026-10-11_006_design_system_formalization.md`.
- **Cascade:** `rules/ui.md` header now points to the DS set (implementation form of the rule);
  `rules/canvas.md` visual-values pointer updated; `03_documentation_system.md` gained §6b
  (canonical DS location). TWO items await ONE owner review: (a) AA contrast token bumps
  (`11` §2), (b) RVS affordance color recommendation (magenta eligible / green activated vs the
  vision doc's red/green) → small ADR-005 before RVS ships.
- **Note:** No DS code yet — next phase is tokens module + component library in
  `app/src/design-system/` with tests. No editor features exist.

## 2026-10-11 — Repository scaffold (foundation tooling; ADR-001 Option B realized)

- **What:** Full repo scaffold per ADR-001 + owner direction. `core/` cargo workspace with the
  nine decided crates (compiling, documented skeletons; `unsafe_code` forbidden; native tests
  green) · dependency law made enforceable via `scripts/check-boundaries.py` (CI-blocking;
  verified passing on the repo and failing on a violated copy) · `app/` shell (React 19 + Vite 8
  + strict TS 7 + Vitest; typecheck/tests/build green) · wasm pipeline
  `scripts/build-wasm.sh` → `bindings/pkg` (validated module, 132 KB, exports
  `protocol_version`) · CI workflows (core · wasm · shell) from day one · fixtures spec for the
  five canvas §26 reference documents. New `docs/architecture/dependencies.md` (registry:
  internal edges, externals w/ resolved versions + CQ1.4 justification, toolchain pins,
  npm-over-pnpm decision). ADR-001 addendum + `engine-core.md` §10 updated to realized paths.
  Task summary: `docs/tasks/2026-10-11_005_repo-scaffold.md`.
- **Why:** Foundation-first law: the architecture's boundaries must be PHYSICAL before editor
  features start (shell has no document write API; purity edges lint-enforced; bindings never
  hand-edited). Tests-from-day-one was explicitly requested and immediately caught two real
  scaffold bugs (clippy lint-priority config; wasm-pack out-dir resolution — both fixed + recorded).
- **Note:** Still ZERO editor features. Open next: ADR-002/003/004 bundles; P0-class panic/trap
  recovery ADR before risky subsystems. CI yaml executes for the first time on the first push.

## 2026-10-11 — ADR-001 ACCEPTED (Option B: Rust/Wasm engine core) + engine-core boundary law

- **What:** Product owner DECIDED the gating technology-stack question: **Option B — Wasm engine
  core from day one** (Rust core → WebAssembly owning document/geometry/layout/state/rendering;
  thin TypeScript + React shell; renderer = wgpu inside the core, WebGPU primary + WebGL2 via
  Naga fallback). ADR-001 rewritten to ACCEPTED with sub-decisions, accepted costs + mitigations,
  and updated open questions (pre-decision Option A analysis kept as permanent record). NEW
  contract: `docs/architecture/engine-core.md` — the placement law (core/shell ownership table,
  wasm-bindgen boundary protocol, event→frame loop, worker/OffscreenCanvas target + COOP/COEP
  note, wasm32 memory discipline, trap/panic policy flagged P0-class ADR, testing pyramid).
  Cascade edits: `rules/rendering.md` R1+R7 (backend decided = wgpu; Wasm core is now THE
  architecture, no longer "future"), all architecture headers, hub table + open questions,
  rendering/vector-engine/file-format/input-system/selection-system/undo-redo placement notes,
  `architecture/README.md` (13 docs + ADR index). Task summary:
  `docs/tasks/2026-10-11_004_adr001_accepted_option_b.md`.
- **Why:** Owner direction = maximum performance headroom from day one (Figma-shaped). Verified
  tooling facts recorded in the ADR: wgpu gives one API for native + browser WebGPU + WebGL2
  (Naga translation) [PUBLIC: gfx-rs/wgpu]; wgpu-in-worker with OffscreenCanvas is a proven
  pattern [PUBLIC: matthewjberger/webgpu-worker]. Boundary Duplication Law added: no editor logic
  may exist in both halves; shell has NO document write API (makes SM3 physical).
- **Note:** No code exists yet — next is the repo scaffold (cargo workspace + shell + CI), then
  ADR-002/003 bundles. Rust-vs-C++ recorded as ACTIVE sub-decision (Rust chosen; superseding ADR
  required to change).

## 2026-10-11 — Architecture contracts v1 + ADR-001 (roadmap step 2 substantially complete)

- **What:** All 12 core architecture contract documents authored in `gipulo/docs/architecture/`
  (DESIGN v1, stack-agnostic): `canvas.md` (HUB: subsystem map, dependency law, frame lifecycle)
  · `document-tree.md` · `coordinate-system.md` · `vector-engine.md` · `rendering.md` ·
  `selection-system.md` · `transforms.md` · `layout-engine.md` (incl. RVS stretch contract) ·
  `input-system.md` · `undo-redo.md` · `file-format.md` · `code-generation.md` (the two founding
  design-to-code laws + mapping registry + Code Workspace/Preview contract).
  Plus **`ADR-001-technology-stack.md` (PROPOSED — awaiting owner decision; recommends Option A:
  TypeScript monorepo, React UI shell, pure-TS engine core, WebGPU-primary + WebGL2-fallback
  renderer behind a graphics abstraction, measurement-gated Wasm hot-path escalation)**.
  `docs/architecture/README.md` updated with status + ADR index (ADR-002/003/004 candidates
  registered). Task summary: `docs/tasks/2026-10-11_003_architecture_contracts.md`.
- **Why:** Vision doc mandates architecture documentation (foundational-systems awareness);
  roadmap step 2. Research this round: WebGPU Baseline Jan 2026 (utsubo/vr.org), tldraw public
  DOM-rendering bottleneck + R-tree/canvas migrations (issue #8307, SDK 4.4), Excalidraw stack —
  feeding ADR-001's evidence base alongside prior Figma/Penpot findings.
- **Note:** NO code exists; ADR-001 gates all implementation. `08_foundation_roadmap.md` and the
  master beacon §6 updated to match.

## 2026-10-11 — Rules system v1 authored (foundation roadmap step 1 COMPLETE)

- **What:** All 17 planned rule files created and ACTIVE in `gipulo/rules/` (1,860 lines total):
  global · code-quality · **canvas (649 lines, the "main law", research-grounded with source
  classification [PUBLIC]/[OBS]/[DS]/[REC]/[PROPOSED]/[INF], binding micro-detail registry §27,
  mandatory change protocol §28)** · vector-geometry · rendering · ui · layout (incl. Responsive
  Vector Segments product law L3) · icons · interactions · state-management · performance ·
  feature-addition · bug-fixing · refactoring · feature-removal · testing · documentation.
  `rules/README.md` updated with the status table. Task summary: `docs/tasks/2026-10-11_002_rules-system.md`.
- **Why:** Vision document mandates the rules system before any feature work; canvas-rules prompt
  (`docs/memory/06`) required deep research (Figma WebGPU/Wasm + Mirror DOM posts, tldraw docs,
  Penpot) and strict fact-vs-inference separation — both done and recorded in `canvas.md` §1.
- **Note:** Values tagged [PROPOSED] in the rules are binding defaults until confirmed/changed by
  ADRs (process: `rules/documentation.md` D7). No memory distillation files changed content;
  `08_foundation_roadmap.md` status and the master beacon §6 updated to reflect step 1 complete.

## 2026-10-11 — Task 016: hover index tuning

- Leaf size 8 → 2 in `SegmentIndex`: 1.08 ms → 0.60 ms per query (dense 1000-seg fixture), results identical. 131 workspace tests green; clippy, fmt, wasm, boundary checks clean.

## 2026-10-11 — Task 020: editor Slice 0 plan (no code)

- **What:** plan with 28 testable acceptance criteria, the canvas-rule checklist, deliberate deferrals (handles, snapping, text), and four blocking decisions (D1 bootstrap painter, D2 interaction state machine home, D3 boundary protocol shape, D4 multi-select and Delete in Slice 0).
- **Verified state:** the core has document, history, coordinates, and geometry; the scene and renderer are vocabulary-only; the boundary is version-only. Slice 0 must add the boundary, a draw list, the state machine, and a canvas host.
- **Honest scope note:** the plan lists the canvas rules read and the ones still to read before the build.

## 2026-10-11 — Task 019: shell ↔ engine wiring + browser E2E

- **What:** the shell loads the Wasm core (`loadEngine()` in `app/src/engine/client.ts`), runs the protocol handshake, and shows a status line (`Engine ready · protocol 0`). No editor features.
- **Tests:** Playwright tests against the production build: handshake success (`engine-smoke.spec.ts`) and engine-load failure shows the error state (`engine-error.spec.ts`): both pass. Vitest 47/47, typecheck, production build all green.
- **CI:** the shell job now builds the bindings first (the shell imports them) and runs the browser E2E.
- **Dependency:** `@playwright/test` (dev only) registered in `dependencies.md`.

## 2026-10-11 — Task 018: WebGL2 parity tiers (ADR-010) and text shaping (ADR-011)

- **ADR-010 ACCEPTED:** WebGPU = T1 reference; WebGL2 = T2 parity for core canvas features; T3 = declared degraded fallbacks with a visible badge. Shared renderer path is render-only (WebGL2 has no compute and no storage buffers).
- **ADR-011 ACCEPTED:** shaping = harfrust; font parsing = skrifa/read-fonts. rustybuzz (archived 2026-07-26) and ttf-parser (unmaintained) rejected. No dependency added yet; registered when the text feature starts.
- **Open, explicitly:** text rasterization path (SDF/atlas vs CPU) and text-on-path.

## 2026-10-11 — Task 017: documentHash (SHA-256) and NFC (ADR-009)

- **What:** `gip-document` now writes `meta.documentHash` = SHA-256 of the canonical document section on save, verifies it on load (mismatch = invalid), and rejects any non-NFC string (keys and values) on save and load. New module `integrity.rs`; deps `sha2` 0.11 and `unicode-normalization` 0.1 registered in ADR-009 and the dependency registry; boundary allow-list updated.
- **Golden:** `sample.gip` regenerated; the only diff is the `documentHash` line.
- **Tests:** 138 workspace tests green (7 new in `tests/integrity.rs`); clippy, fmt, wasm, boundary checks clean.
- **Closes:** ADR-003 A4; the documentHash + NFC items in the roadmap.

## 2026-10-11 — Task 015: RVS affordance language (ADR-005)

- **What:** ADR-005 ACCEPTED under the owner's delegation: eligible segments = magenta (guide/spacing) + dot; activated = success green + count pill; context = blue dashed; red stays rulers-only. Closes the 08 §8 pending item.
- **Why:** RVS is the signature feature and was blocked on this decision. Recommendation already in the DS docs; no new colour tokens.

## 2026-10-11 — Task 014: recovery & integrity policy (ADR-008, P0-class)

- **What:** ADR-008 (PROPOSED): trap recovery from committed, validated whole-file snapshots kept by the shell; core serializes/validates, shell stores; 3 generations; autosave 2 s debounce / 30 s max while dirty; local-only diagnostics via panic hook; fault-injection feature excluded from release; restore copy with honest loss statement.
- **Owner review:** cadence, retention, clean-close behaviour when dirty, recovery copy.
- **Why:** engine-core §8 and ADR-001 require this before any risky subsystem ships. Docs-only; no code.

## 2026-10-11 — Task 013: segment index (`gip-geometry`, roadmap step 13)

- **What:** `SegmentIndex` (static BVH over tight segment bounds; median split; leaves ≤ 8; branch-and-bound nearest). ADR-007 (PROPOSED).
- **Measured (release, 1000-segment dense fixture):** build 0.27 ms; index 1.13 ms/query vs brute 142 ms/query; max diff vs brute 0. Fixture-specific — the task-010 baseline (10.4 ms) was another fixture. Still above a comfortable hover budget on dense input; next step if needed: leaf-bound tightening or cursor-radius pruning.
- **Tests:** 131 workspace tests green (3 new). fmt, clippy, wasm check, boundary check clean.

## 2026-10-11 — Task 012: history and transactions (`gip-history`, roadmap step 12)

- **What:** `EditOp` op log (ADR-004), `Tx` collector (apply + record, rollback on failure),
  `History` (transact, undo, redo, label accessors, byte budget, dirty tracking). `FieldKey`/
  `FieldValue` accessors on `Node` in `gip-document`.
- **Decisions:** ADR-004 amendments A1–A7 (ids outside history; positional dirty tracking; MoveNode
  in, SetShared deferred; rollback = undo mechanism; budget keeps newest).
- **A6 CLOSED:** the op vocabulary and `apply` moved to `gip-document::edit`; `GipFile`/`Document` fields are crate-private with read accessors; `scripts/check-boundaries.py` enforces that only `gip-history` uses the write door (probe-verified). Residual per-crate caveat in ADR-004 A6.
- **Environment:** toolchain reinstalled after sandbox restart (rust 1.99.0 + wasm32 target).
- **Tests:** 128 workspace tests green; clippy `-D warnings`, fmt, boundary check, wasm check clean.

## 2026-10-11 — Task 011: document serializer (`gip-document`, roadmap step 11)

- **What:** `gip-document` now holds the `.gip` v1 schema: `NodeId`/`AssetId` (salt<<40|counter,
  16-hex wire form), `Transform` (ADR-002 fields, no ext), `Node` (kebab-case kinds, ext), `Page`,
  `Document` (BTreeMap nodes, validate), `GipFile`/`Meta`/`AssetRecord`, `to_json`/`from_json`
  (validate-then-write, forward-only migration table), `GipError`. `gip-geometry::bridge` adds
  `trs_of`/`transform_of` with exhaustive destructuring.
  Golden fixture `tests/golden/sample.gip` is generated by the serializer (`GIP_UPDATE_GOLDEN=1`).
- **Dependency:** `serde` + `serde_json` (`float_roundtrip`) added to `gip-document` — the registered
  layer-1 exception. Transitive: `serde_core`, `serde_derive`, `itoa`, `memchr`, `zmij`, `proc-macro2`,
  `quote`, `syn`. Boundary check PASS.
- **Decisions:** ADR-003 amendments A1–A6 (hex IDs, ext scope, asset id space, documentHash null
  until dependency registered, validate-then-write, migrations). Deferred: documentHash/SHA-256,
  NFC, asset store, transaction API.
- **Tests:** 114 workspace tests green (gip-document 29 + 1 smoke). clippy `-D warnings`, fmt, wasm check clean.
- **Next:** transaction API (SM3) and `documentHash`/NFC once dependency decisions are made.

## 2026-10-11 — Memory system established (fresh project start)

- **What:** Created the complete Gipulo agent-memory system from the four canonical user documents:
  - `/home/user/GIPULO_AGENT_MEMORY.md` (master beacon, fresh-project declaration, ten commandments,
    workflows, file map);
  - `/home/user/AGENTS.md` (workspace pointer);
  - `gipulo/AGENTS.md` (project operating contract);
  - `docs/source/` — byte-identical verbatim copies of all four originals (verified with `diff`);
  - `docs/memory/00…08` — full distilled memory (identity/vision, problems/solutions, documentation
    system, rules system, design system brief, canvas rules requirements, concrete design tokens &
    visual specs extracted from the Design System HTML, foundation roadmap);
  - skeleton: `docs/features/` (+ empty FEATURE_REGISTRY), `docs/architecture/`, `docs/bugs/`
    (+ record template), `docs/prompts/` (+ first archived prompt), `docs/tasks/` (+ first task
    summary), `rules/` (+ planned rule-file list).
- **Why:** User instruction (2026-10-11): this is a COMPLETELY NEW project; all details of the four
  documents are highly important and must be permanently remembered by the agent. Any older
  "gipulo" project memory is declared invalid.
- **Status:** Project phase = FOUNDATION. No application code exists.
