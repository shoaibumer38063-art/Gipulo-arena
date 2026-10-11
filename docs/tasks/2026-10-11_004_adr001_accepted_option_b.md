# Task Summary 004 — ADR-001 ACCEPTED (Option B) + Engine-Core Boundary Contract + Cascade

```text
Task: Record the product owner's technology-stack decision (Option B — Wasm engine core from day
      one), author the resulting boundary law (engine-core.md), and cascade the decision through
      every affected document, rule note, and status file.
Date: 2026-10-11
```

**Problem being solved:** ADR-001 was PROPOSED with a recommendation (Option A, pure-TS first).
The owner decided differently: **Option B — Rust/Wasm engine core from day one** (Figma-shaped).
The decision needed to become binding law: ADR acceptance, the new placement contract (what lives
in the core vs the shell and how they communicate), and consistency updates across rules, all 13
architecture docs, memory, and the beacon — the project's own Documentation Update Rule.

**What was inspected:** The PROPOSED ADR-001 (kept as the pre-decision record); all architecture
docs referencing "ADR-001" / "Option A" (grep-verified list); `rules/rendering.md` R1/R7 notes
("backend ADR-gated", "Wasm core remains a possible future"); fresh verification of the Option B
tooling facts: wgpu (native Vulkan/Metal/D3D12/OpenGL + wasm WebGPU pass-through + WebGL2 via
`webgl` feature/Naga WGSL→GLSL [PUBLIC: gfx-rs/wgpu]) and the wgpu-in-worker + OffscreenCanvas
pattern [PUBLIC: matthewjberger/webgpu-worker, 2026 — matches WebGPU Baseline + Firefox 141/
Chromium 113 support line].

**Approach selected:** (1) Rewrite ADR-001 as ACCEPTED with the decision block, sub-decisions
table (Rust over C++ with agent-safety rationale; wgpu; renderer inside core; state machine in
core; boundary protocol; worker target; testing pyramid), accepted costs + mitigations, and
updated open questions. (2) Author `docs/architecture/engine-core.md` — the placement law:
ownership table, boundary protocol, event→frame loop, threading/hosting (COOP/COEP implications),
memory discipline (wasm32 4 GB/grow-only), error/trap policy (panic-recovery ADR flagged
P0-class), testing pyramid, build/artifact rules (bindings never hand-edited, protocol version
handshake). (3) Cascade: rules/rendering.md R1+R7 corrected (no longer ADR-gated / no longer
"future"); all architecture headers; hub table + open questions; rendering backends +
WGSL decision; placement notes in input-system/selection-system/undo-redo; vector-engine and
file-format testability notes. (4) Memory discipline: CHANGELOG, roadmap, beacon, this task file.

**Why this approach:** Keeps every doc stack-agnostic where it was (nothing needed rewrites —
the contracts were written placement-neutral by design), concentrates Option B's consequences in
ONE new law document (engine-core.md) instead of smearing them, and preserves the pre-decision
analysis for permanence (ADRs are superseded, never deleted — D7).

**Alternatives rejected:** Leaving docs saying "PROPOSED/awaiting" (violates truth-hierarchy law —
docs must reflect accepted reality); rewriting all 13 contracts for Option B (unnecessary — only
placement changed, behavior law is identical); secretly keeping TS-first assumptions (boundary
duplication law now forbids editor logic on the TS side — the docs must say so).

**Files changed:** Rewrote `docs/architecture/ADR-001-technology-stack.md` (ACCEPTED);
created `docs/architecture/engine-core.md` (placement law) and this task file; edited
`docs/architecture/{README.md, canvas.md, rendering.md, vector-engine.md, file-format.md,
input-system.md, selection-system.md, undo-redo.md}` + sed-updated all remaining contract headers
(`document-tree, coordinate-system, transforms, layout-engine, code-generation`); edited
`rules/rendering.md` (R1, R7); updated `docs/memory/CHANGELOG.md`, `docs/memory/
08_foundation_roadmap.md`, `/home/user/GIPULO_AGENT_MEMORY.md`.

**Behavior changed:** None (design phase; no code exists). The governing architecture is now:
Rust/Wasm core (document, geometry, layout, coords, history, scene, wgpu renderer, text, session
facade) + TS/React shell (DOM/UI/a11y/IME/OS integration/code workspace), joined by the
wasm-bindgen boundary with the Boundary Duplication Law.

**How result was verified:** Post-edit grep sweep for stale phrases ("stack-gated",
"PROPOSED, awaiting", "Option A: WebGPU") — remaining hits only inside ADR-001's intentional
pre-decision record; header consistency across all contracts; cross-reference resolution
(engine-core.md referenced from hub + README + rules); CJK anomaly scan clean; file counts.

**Remaining risks / open items:** (1) Rust-vs-C++ sub-decision ACTIVE as Rust — superseding ADR
required before scaffold if ever changed. (2) Worker rollout + COOP/COEP hosting plan → scaffold
note. (3) **Panic/trap recovery policy is P0-class and MUST be ADR'd before any risky subsystem
ships** (document integrity). (4) Codegen split (IR in core, emitters in TS) → ADR with codegen
feature. (5) wasm load-time budget now includes module instantiation — measured at scaffold.

**Next steps:** (1) Repo scaffold: `core/` cargo workspace (9 crates per ADR-001 §Consequences)
+ `app/` shell + bindings pipeline + CI (both toolchains) + fixtures — foundation only, no editor
features. (2) ADR-002/003 bundles (now unblocked and needed before first format/transform code).
(3) Roadmap step 3 (Design System formalization) can proceed in parallel (stack-agnostic).
