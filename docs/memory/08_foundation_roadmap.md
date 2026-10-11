# 08 — Foundation Roadmap & Project Status

> Synthesis of the four source documents. Items marked **[user-mandated]** come directly from the
> documents; items marked **[derived]** are planning inferences consistent with them (the user
> must confirm before big new directions are started).

## Current status

- **Phase: FOUNDATION.** Established 2026-10-11.
- **No editor features exist yet.** The repo scaffold (Rust/Wasm engine-core skeleton, TS/React
  shell skeleton, CI gates, wasm bindings pipeline) exists as of 2026-10-11 — foundation tooling
  only, by design (task summary 005).
- Completed: fresh-project memory system (this folder), verbatim source archive, docs/rules
  skeleton, empty feature registry, first prompt-archive entry, first task summary.
- ✅ **Roadmap step 1 COMPLETE (2026-10-11):** all 17 rule files authored (v1, ACTIVE) — see
  `rules/README.md` status table and task summary `docs/tasks/2026-10-11_002_rules-system.md`.
  The canvas main law (`rules/canvas.md`) is research-grounded with [PUBLIC]/[OBS]/[DS]/[REC]/
  [PROPOSED]/[INF] source tags; [PROPOSED] values are binding defaults until ADRs confirm them.
- ✅ **Roadmap step 2 COMPLETE (2026-10-11):** all 14 architecture contracts authored
  (DESIGN v1, hub = `docs/architecture/canvas.md`, placement law = `engine-core.md`, registry =
  `dependencies.md`) AND **ADR-001 ACCEPTED — Option B: Rust/Wasm engine core + thin TS/React
  shell; renderer = wgpu inside the core (WebGPU primary, WebGL2 fallback).** See task summaries
  003 and 004.
- ✅ **Repo scaffold COMPLETE (2026-10-11):** `core/` cargo workspace (9 crates compiling;
  dependency law enforced by `scripts/check-boundaries.py`; native tests green) · `app/` shell
  (typecheck/tests/build green) · wasm pipeline verified (`bindings/pkg` — 132 KB validated
  module exporting the protocol handshake) · CI gates (core · wasm · shell) · fixtures spec
  (canvas §26 five reference documents). **Zero editor features** (by design). Task summary 005.
- ✅ **Design System FORMALIZED (2026-10-11):** all 14 documents in `docs/design-system/` (the
  single visual+interaction source of truth — every component with ALL states; the guide-color
  language closed; research mandate executed). **Roadmap steps 3 + 4 COMPLETE** (task summary
  006). TWO owner-review items queued: (a) AA contrast token bumps (`11` §2); (b) RVS affordance
  color recommendation → ADR-005 (before RVS ships).
- ✅ **Design System IN CODE (2026-10-11):** `app/src/design-system/` — drift-tested token layer
  (`tokens.css` ↔ `tokens.ts`), self-hosted Inter + JetBrains Mono, 17 registry entries (15 component modules + token layer + global scrollbar) with
  state-matrix tests (**47/47 passing**), gallery story page incl. an illustrative editor frame
  (dev server preview). Typecheck + build green; "state matrix = test matrix" is now enforced in
  CI (shell job). Task summary 007.
- ✅ **ADR BUNDLE 002/003/004 AUTHORED (2026-10-11, PROPOSED — binding as implementation
  defaults):** transform/matrix/numeric conventions (decomposed canonical fields; column-major
  6-tuple matrices; degrees (−180,180]; flip flags; zero-crossing law; no skew; one constants
  module) · `.gip` v0 (single-file JSON; salt+counter u64 IDs with remap law; shortest round-trip
  f64; sidecar content-addressed assets; forward-only migrations) · history (structured op logs;
  replay-recorded-values undo; truncate redo tail; byte-budget cap; in-memory v1). All affected
  contracts' open questions closed; task summary 008. Remaining queued: ADR-005 (RVS colors),
  P0-class panic/trap recovery ADR before risky subsystems. **Next: first engine subsystems
  (coordinates → geometry → document schema/serializer) against these ADRs.**

## Build order mandated by the documents [user-mandated]

1. **AI development foundation first** — agent setup, project memory system, rule system,
   documentation system, testing system, architecture. ("Before we start aggressively adding
   features…") Without it: forgotten decisions, duplicated logic, broken features, clutter =
   "disaster" at Gipulo's scale.
2. **Then the Gipulo Design System** — the first major visible system, before hundreds of UI
   features, so agents never create inconsistent buttons/panels/spacing/radii/icons/states/
   tooltips/menus/inputs/dialogs. Requirements: `05`; concrete values: `07`; living reference:
   the Design System HTML.
3. **Then the editor**, with canvas rules as the main law of the most dangerous subsystem
   (requirements: `06`), reaching Figma-comfortable parity first, then the differentiators
   (Responsive Vector Segments, native design-to-code + Code Workspace, 3D Live Link, native
   scroll interactions, accessible pricing) — `02`.

## Concrete next foundation steps [derived — await user confirmation on order]

1. ✅ **DONE (2026-10-11): `rules/` authored** — all 17 files ACTIVE, incl. `rules/canvas.md`
   (research-grounded per `06`: Figma WebGPU/Wasm + Mirror DOM public architecture, tldraw
   camera/state-machine docs, Penpot renderer; facts vs inference tagged; no generic advice).
2. ✅ **DONE (2026-10-11): core architecture docs** — 14 contracts in `docs/architecture/`
   (`engine-core.md` boundary law; `dependencies.md` registry) + **ADR-001** (stack) ACCEPTED;
   ADR-002/003/004 registered as the next decision bundles.
3. ✅ **DONE (2026-10-11): repo scaffold** — `core/` Rust/Wasm workspace (9 crates, boundary law
   enforced in CI), `app/` shell green (typecheck/tests/build), wasm bindings pipeline verified,
   CI gates (core · wasm · shell), fixtures spec. Task summary `005`. No editor features (by design).
4. ✅ **DONE (2026-10-11): Design System formalized** — 14 docs in `docs/design-system/`
   (single visual+interaction source of truth: foundations, tokens, typography, icons, controls,
   surfaces, component architecture, canvas UI, cursors, motion, accessibility, responsive,
   research notes). Task summary `006`.
5. ✅ **DONE (2026-10-11): guide-color language formally closed** — `docs/design-system/
   08_canvas_ui.md` §9 (blue geometry · magenta spacing · orange snap+measure · red rulers ·
   purple components) + `rules/canvas.md` CAN-GUIDE-1 pointer; new guide colors = ADR-level.
6. **Set up the testing system** with the coverage targets from `03` §8 and `06` (testing rules,
   performance budgets on representative heavy documents). — partially seeded: shell Vitest
   suites + token-drift guard already run in CI (task 007); Rust test/golden-fixture phase lands
   with the document/geometry subsystems.
7. ✅ **DONE (2026-10-11): decision bundles authored — ADR-002 · ADR-003 · ADR-004** (PROPOSED;
   binding as implementation defaults pending owner review — task summary 008). Still queued:
   ADR-005 (RVS affordance colors — required before that feature) · P0-class panic/trap recovery
   ADR before risky subsystems ship · spatial-index ADR with vector-engine impl · WebGL2 parity
   ADR with renderer impl · text-shaping ADR when the text feature starts.
8. ✅ **DONE (2026-10-11): DS code implementation** — token module + 15 components + state tests
   + gallery story page in `app/src/design-system/` (typecheck/tests/build green). Task summary
   `007`. Remaining registry entries (picker, popover, toast, cursors, canvas overlays, …) ship
   with their first real consumer.
9. ✅ **DONE (2026-10-11): coordinates phase — `gip-coords`** (task 009): Mat2/Trs/Camera/
   viewport conversions/angles, 38 crate tests.
10. ✅ **DONE (2026-10-11): geometry phase — `gip-geometry` + ADR-006 path model** (task 010):
   cubic math, bounds, flatten, hit geometry, VG2 op subset; 88 workspace tests green; hover
   baseline ≈10 ms on 1000 segments. **Next engine steps:** spatial index ADR (measured need) →
   document schema/serializer (`gip-document`, serde per registered exception, golden fixtures) →
   Playwright E2E when the shell imports the bindings.
11. ✅ **DONE (2026-10-11): document serializer — `gip-document`** (task 011): node/page/document
   schema, hex-string IDs (ADR-003 A1), transform bridge `gip-geometry::trs_of/transform_of` with
   exhaustive-destructuring drift guard, `.gip` v1 container, 17 structural/value validation rules,
   bit-exact f64 round trip, golden fixture `tests/golden/sample.gip` (generated by the serializer,
   `GIP_UPDATE_GOLDEN=1`). **Next:** `documentHash` + NFC once their dependency decisions are
   registered; transaction API (SM3); spatial index ADR (measured need from task 010).
12. ✅ **DONE (2026-10-11): history and transactions — `gip-history`** (task 012): op log per
   ADR-004, atomic `transact`, exact undo/redo, redo truncation, byte budget, dirty tracking.
   **Write door CLOSED (A6):** op vocabulary in `gip-document::edit`; boundary check enforces it.
   **Next:** spatial index ADR (measured need: 10 ms hover at 1000 segments) → Playwright E2E.
13. ✅ **DONE (2026-10-11): segment index — `gip-geometry::SegmentIndex`** (ADR-007 PROPOSED; task 013): static BVH per path, equal to brute force. Measured 1.13 ms vs 142 ms per query on a dense 1000-segment fixture (fixture-specific). **Next:** tighten the hover budget if measured need persists; Playwright E2E (roadmap step 6); ADR-005 (RVS colours); P0 panic/trap ADR.
14. ✅ **DONE (2026-10-11): P0 recovery & integrity policy — ADR-008** (PROPOSED; task 014): committed
   whole-file snapshots, shell-owned storage, 3 generations, 2 s/30 s autosave, local-only diagnostics,
   release-excluded fault injection. Owner review: cadence, retention, clean-close, copy. Unblocks risky subsystems
   (their own tests required). **Remaining queued foundation items:** ADR-005 (RVS affordance colours — needs
   owner/designer input before the RVS feature) · Playwright E2E (starts when the shell imports the bindings) ·
   WebGL2 parity ADR (renderer phase) · text-shaping ADR (text phase) · `documentHash` + NFC (dependency decisions).
15. ✅ **DONE (2026-10-11): ADR-005 RVS affordance language** — ACCEPTED under owner delegation (task 015). Magenta eligible, green activated, red reserved for rulers.
16. ✅ **DONE (2026-10-11): documentHash (SHA-256) + NFC — ADR-009 ACCEPTED** under owner delegation (task 017). `gip-document` writes and verifies `documentHash`; rejects non-NFC strings; deps registered (`sha2`, `unicode-normalization`).
17. ✅ **DONE (2026-10-11): WebGL2 parity ADR-010 and text-shaping ADR-011** — both ACCEPTED under owner delegation (task 018). No dependencies added; they are registered when their features start.
18. ✅ **DONE (2026-10-11): shell ↔ engine wiring + Playwright E2E** (task 019): status line proves the handshake in a real browser; CI builds bindings before the shell and runs E2E.
19. 🟡 **IN PROGRESS: editor Slice 0 — plan written (task 020), awaiting owner decisions D1–D4** before code. Split: 020a boundary + draw list + canvas host; 020b rectangle create/select/move/cancel/undo; 020c performance fixture + keyboard + E2E.
20. Only after the above: begin editor implementation (explicit user go-ahead required). Decided later with their features: text rasterization path, text-on-path.

## Standing task-completion definition (every task, always)

A task is complete ONLY when: implementation done + tested + neighboring behavior verified +
Documentation Update Rule checklist run (10 questions, `03`) + FEATURE_REGISTRY updated if
statuses changed + task summary saved (`docs/tasks/`) + important prompts archived
(`docs/prompts/`) + memory CHANGELOG appended if memory changed.
