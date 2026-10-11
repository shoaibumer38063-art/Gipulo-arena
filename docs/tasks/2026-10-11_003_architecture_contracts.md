# Task Summary 003 — Architecture Contracts + ADR-001 (Technology Stack)

```text
Task: Author the 12 core architecture contract documents (foundation roadmap step 2) and prepare
      the gating technology-stack ADR as a researched options analysis for the product owner.
Date: 2026-10-11
```

**Problem being solved:** The vision mandates dedicated architecture documentation so agents
always know which systems are foundational and how they interlock — and the roadmap's next phase
(any code) is gated by the technology-stack decision, which per global rule G10 belongs to the
product owner. No architecture docs existed; no stack decision existed.

**What was inspected:** All binding rules (`rules/canvas.md` CAN-* sections per subsystem,
rendering/vector-geometry/state-management/layout/testing/documentation rules), memory files
`01–08` (esp. `02` Solution 2 for the codegen contract and `06` for canvas subsystem
requirements), plus fresh web research: WebGPU Baseline status Jan 2026 (Chrome/Edge 113+,
Safari 26+, Firefox 141+/145+, ~82% global; WebGL2 still needed as fallback), tldraw's public
DOM/React rendering bottleneck (issue #8307) and SDK 4.4 migrations (SVG→Canvas2D indicators 25×,
R-tree spatial index), Excalidraw's React+TS+Canvas2D stack, on top of the previous turn's Figma
(C++/Wasm, WebGL→WebGPU, Mirror DOM) and Penpot (SVG→Rust/Wasm/Skia rebuild) findings.

**Approach selected:** (1) Write all 12 architecture docs as stack-agnostic DESIGN contracts
(purpose · owns · must-never · conceptual interface · dependencies · invariants · ADR-gated open
questions · test focus), with `canvas.md` as the hub carrying the subsystem map, the binding
dependency direction, and the frame lifecycle. (2) Prepare ADR-001 as a PROPOSED options analysis
(A: TS monorepo + React shell + pure-TS engine + WebGPU-primary/WebGL2-fallback behind graphics
abstraction + measured Wasm hot-path escalation — RECOMMENDED; B: Wasm-core day one — rejected
for v1; C: DOM/SVG rendering — rejected per public bottleneck evidence; D: Canvas2D primary —
rejected per effects ceiling) with a plain-language decision guide for the designer-founder.
(3) Register the ADR pipeline: ADR-002 (transform/matrix conventions bundle), ADR-003 (.gip
format bundle), ADR-004 (history storage) as candidates unblocked by ADR-001.

**Why this approach:** Contracts-before-code matches the foundation-first law; writing them
stack-agnostic keeps momentum while correctly leaving the ONE product-level decision (stack) to
the owner; every precedent claim is tagged/evidenced per the canvas-rules research law
(fact vs inference); the WebGPU-Baseline-2026 finding materially updates what's viable vs the
vision doc's writing time and is recorded with sources inside the ADR.

**Alternatives rejected:** Deciding the stack unilaterally (violates G10 — product-level);
writing architecture docs after the stack ADR (wasted the turn — 95% of contract content is
stack-independent, and the rules already mandated the boundaries); one mega-architecture file
(vision explicitly wants per-system docs); adopting tldraw/Excalidraw/Figma stacks wholesale
(their contexts differ — Gipulo needs Figma-class effects+scale with agent-build velocity).

**Files changed:** Created `docs/architecture/ADR-001-technology-stack.md` + 12 contracts
(`canvas.md`, `document-tree.md`, `coordinate-system.md`, `vector-engine.md`, `rendering.md`,
`selection-system.md`, `transforms.md`, `layout-engine.md`, `input-system.md`, `undo-redo.md`,
`file-format.md`, `code-generation.md`). Updated: `docs/architecture/README.md` (status + ADR
index), `docs/memory/CHANGELOG.md`, `docs/memory/08_foundation_roadmap.md` (step 2 status),
`/home/user/GIPULO_AGENT_MEMORY.md` §6. No code (ADR-001 gates it).

**Behavior changed:** None (design phase). The project now has a complete structural blueprint:
every subsystem's ownership, boundaries, interfaces, invariants, open questions, and test focus —
all traceable to rule IDs.

**How result was verified:** Hub-table completeness check (all 21 CAN-ARCH-1 subsystems mapped to
a doc or an explicit fold-in); dependency-direction consistency across docs (no cycles; matches
CAN-ARCH-2); every open question ADR-gated and indexed in `architecture/README.md`; ADR-001
claims tagged [PUBLIC]/[OBS]/[REC]/[PROPOSED] with source list; cross-references resolve.

**Remaining risks:** (1) Contracts may need refinement once ADR-001 lands and implementation
reveals friction — that is expected and handled by the doc-update rule. (2) Text-engine,
overlay-engine, and a11y-mirror full docs deferred to their feature phases (folded-in contracts
exist) — acceptable per hub table, must not be forgotten when those phases start. (3) ADR-002/003
bundles carry several [PROPOSED] values that harden into law once accepted — until then the
proposed defaults bind (canvas §27 convention).

**What to do next if the problem returns (structural ambiguity between subsystems):** the hub
(`docs/architecture/canvas.md`) §2 dependency law + §4 single-source table is the referee; if
still ambiguous → ADR, not improvisation.

**Next foundation steps:** (1) OWNER DECISION on ADR-001 (asked via decision prompt); (2) then
ADR-002/003/004 bundles; (3) roadmap step 3: Design System formalization (`docs/design-system/`:
token layer per `07` §14 naming, component inventory with state matrices, cursor + icon
registries) — stack-agnostic, can proceed in parallel if the owner prefers; (4) step 5: testing
fixtures/tooling skeleton once stack is chosen.
