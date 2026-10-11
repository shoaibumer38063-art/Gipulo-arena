# ADR-001 — Technology Stack & Renderer Architecture

```text
Decision:   OPTION B — Wasm engine core from day one. A Rust core compiled to WebAssembly owns
            all document/geometry/layout/state/rendering; a thin TypeScript + React shell owns
            DOM/UI. Renderer = wgpu inside the core (WebGPU primary; WebGL2 fallback via Naga
            WGSL→GLSL translation).
Date:       2026-10-11 (authored) · 2026-10-11 (ACCEPTED by product owner — Option B selected)
Status:     ACCEPTED — ACTIVE and binding. Implementation shape (ownership split, boundary
            protocol, memory/threading/testing rules): docs/architecture/engine-core.md
Note:       The prior PROPOSED state of this ADR recommended Option A. That analysis is kept
            below as the pre-decision record; it is overridden by the accepted decision.
```

## Context

- Gipulo is a 1:1 production-level Figma alternative built primarily by AI agents under the
  `rules/` system, for a founder who is a UI/UX designer, not a coder.
- Binding constraints from the rules (unchanged by this decision): renderer behind an abstraction
  (`rules/rendering.md` R1); determinism & `.gip` fidelity (CAN-SER); performance budgets on
  heavy reference documents (canvas §26: 10k nodes, 1k-point vectors, effects-heavy files at
  ≥60fps pan/zoom/drag); pure headless-testable engines (VG tests, T2); Mirror-DOM accessibility
  layer (canvas §24); native design-to-code reading the same document truth (vision Solution 2).
- 2026 browser reality [PUBLIC]: WebGPU Baseline since Jan 2026 (Chrome/Edge 113+, Safari 26+,
  Firefox 141+/145+), ~82% global; WebGL2 still required as fallback for older Safari /
  Linux-Firefox. (utsubo.com frontier-web-apis-2026; vr.org WebGPU-baseline-2026.)
- Product owner's explicit direction: maximum engine performance headroom from day one (Option B),
  accepting the documented costs below.

## Precedent research (tagged)

| Tool | Stack (public) | Lesson applied |
|---|---|---|
| **Figma** | C++ engine → Wasm (Emscripten) + React/TS UI; WebGL → WebGPU via one graphics abstraction with dynamic fallback; same C++ compiles natively for server-side render/tests [PUBLIC] | The accepted architecture is Figma-shaped; native-compile perk and abstraction discipline are copied deliberately |
| **wgpu** | Rust graphics API: native (Vulkan/Metal/D3D12/OpenGL) + wasm (WebGPU pass-through; WebGL2 via `webgl` feature using Naga WGSL→GLSL) [PUBLIC-verified] | wgpu IS Gipulo's R1 graphics abstraction — one API, both browser backends |
| **webgpu-worker** | wgpu + wasm app in a Web Worker driving an OffscreenCanvas (`SurfaceTarget::OffscreenCanvas`), main thread only forwards events [PUBLIC, 2026] | Validates the target threading architecture (core in worker, UI never janks) |
| **Penpot** | SVG-DOM → full renderer rebuild on Rust+Wasm+Skia when effects exceeded the DOM pipeline [PUBLIC] | Why deferring the engine costs a rewrite later — the case for B over A |
| **tldraw** | TS+React DOM rendering; maintainers publicly document the React bottleneck at extreme zoom; migrations to Canvas2D + R-tree [PUBLIC] | DOM rendering is rejected for Gipulo's document layer (Option C); tldraw's camera/state patterns remain adopted (stack-independent) |

## The decision (accepted)

**Option B — Wasm engine core from day one.** Sub-decisions (ACTIVE; each changeable only via a
superseding ADR):

| Sub-decision | Choice | Rationale |
|---|---|---|
| Core language | **Rust** (not C++) | Memory safety for agent-authored code; cargo workspaces = enforced crate boundaries (the purity law, physically); wasm-bindgen/wasm-pack maturity; native builds for headless tests + future server-side rendering (Figma pattern). C++ remains the Figma-shaped alternative — rejected for agent-safety; revisitable before scaffold finalization. |
| Renderer | **wgpu inside the core** | Satisfies R1 (abstraction = wgpu); WebGPU pass-through in browsers; WebGL2 fallback via `webgl` feature + Naga translation; one rendering codebase, two backends, no TS graphics code. |
| Renderer location | **Inside the Wasm core** | Scene, tessellation, draw lists and overlay rendering all core-side; shell only hands over the canvas surface. No pixels cross the boundary. |
| Tool/gesture state machine | **In the core** | Hit-testing, selection, snapping, edit contexts determine rendered output — placing the machine where the truth lives makes CAN-MODE-1 ("exactly one owner") structural, not disciplinary. Details: engine-core.md §2. |
| Boundary | **wasm-bindgen; intents down, events/snapshots up** | Thin generated JS client, never hand-edited; typed-array zero-copy for bulk data; IDs/handles (u64), not object graphs. Details: engine-core.md §3. |
| Threading target | **Core in a Web Worker + OffscreenCanvas** (v1 may start main-thread; pattern verified [PUBLIC]) | UI never janks under engine load; threaded wasm (rayon) later needs COOP/COEP — hosting requirement recorded (engine-core.md §5). |
| Testing | **cargo test (native) = majority · wasm-bindgen-test = boundary · Vitest = shell · Playwright = E2E/golden-images · budget suite** | The core is natively testable — faster and more deterministic than any browser harness. |
| Tooling | rustup `wasm32-unknown-unknown` · wasm-bindgen + wasm-opt · pnpm + Vite for shell · CI runs both toolchains | Standard, agent-fluent; pinned versions decided at scaffold (no speculative pinning now). |

**Addendum (2026-10-11 — scaffold):** realized tooling pinned in `dependencies.md` §3–4: shell
package manager = **npm** (not pnpm — single shell package; revisit with workspaces later);
Rust stable 1.99 (MSRV 1.88, set by wgpu 30), target `wasm32-unknown-unknown`; wgpu 30.0.1;
wasm-bindgen 0.2.129; wasm-pack 0.13.1; Node ≥20; scripts in `scripts/`.

## Accepted costs & mitigations (honest record)

1. **Two toolchains / two test regimes** [known con of B] → Mitigation: strict ownership split +
   **Boundary Duplication Law** (no logic implemented on both sides — engine-core.md §1); the
   shell stays thin by design.
2. **Slower iteration than pure TS** → Mitigation: the majority of logic (geometry/layout/history/
   format/machine) is `cargo test`-able natively — faster than any browser test loop; dev-profile
   builds skip heavy optimization; the shell is small enough to iterate with Vite HMR.
3. **wasm32 memory ceiling (4 GB, grow-only)** [PUBLIC, long-standing] → Budget already ≤1.5 GB
   (canvas §26); memory monitored in the budget suite; native builds unaffected (future
   server-side work has no such cap).
4. **Trap/panic kills the module** → Recovery policy is open-question §, P0-class ADR before any
   risky subsystem ships (autosave snapshots + session recovery UI).
5. **AI-agent debugging across FFI** → Native compilation of the same crates enables normal Rust
   debugging of engine logic; boundary kept narrow and typed; rich logs core-side.

## Options considered (pre-decision record — kept for permanence)

- **Option A — TS monorepo + pure-TS engine + WebGPU-first renderer + wasm escalation path**
  (previously RECOMMENDED): fastest agent velocity; rejected by the owner in favor of day-one
  headroom. Its escalation path (“extract hot modules to Wasm behind the package interface”)
  describes exactly what Option B ships immediately.
- **Option C — DOM/SVG React rendering (tldraw-style):** rejected — publicly documented rendering
  ceiling (tldraw #8307) below Gipulo's effect/scale requirements.
- **Option D — Canvas2D primary (Excalidraw-style):** rejected — no compute/effects headroom;
  predicts a renderer rewrite (Penpot evidence). Canvas2D remains debug/bootstrap-only.

## Consequences

1. **Repo shape (scaffold-time):** `core/` cargo workspace — crates `gip-document` ·
   `gip-geometry` · `gip-layout` · `gip-coords` · `gip-history` · `gip-scene` · `gip-render`
   (wgpu) · `gip-text` · `gip-session` (engine API facade) — plus `app/` (React shell),
   `bindings/` (generated wasm output, never hand-edited), `fixtures/`.
2. **engine-core.md becomes THE boundary law** — every architecture contract written before this
   decision stays valid (they were stack-agnostic); placements are now resolved there.
3. **ADR-002 bundle** additionally picks Rust numeric-lib conventions (e.g. glam/euclid classes)
   for matrix/angle representation — the product-visible laws (angle range, flip storage, marquee
   default) are unchanged.
4. **ADR-003** unchanged in substance; serializer/deserializer now core-side.
5. **Rules notes updated:** `rules/rendering.md` — backend no longer "ADR-gated" (decided: wgpu);
   "Wasm core possible future" → it IS the architecture. No rule restructuring required.
6. **Load-time budget:** canvas §26 open-time target (≤3s) now includes Wasm module instantiation
   + first frame; measured at scaffold; budget kept, module size tracked (engine-core.md §10).
7. **Upside banked:** same core compiles natively ⇒ CI golden/determinism suites without a
   browser, future server-side rendering/export, and a clean path to a native desktop shell later
   (future ADR — not committed).

## Open questions (post-decision, tracked)

1. ~~Technology stack~~ — **DECIDED (this ADR, Option B).**
2. Rust-vs-C++ language sub-decision — ACTIVE as Rust; superseding ADR required to change before
   scaffold finalization.
3. Worker rollout timing + COOP/COEP hosting plan (dev server + preview environments must set
   headers if SAB/threads are used) → recording at scaffold; ADR if threads deferred.
4. Panic/trap recovery policy & document-integrity guarantees → P0-class ADR at impl.
5. Codegen split: IR generated in core, project emitters in TS [REC] → ADR with codegen feature.
6. Carried over (unaffected): shader strategy — now constrained to WGSL-first via wgpu/Naga ·
   text shaping crate choice · tiling strategy · color management · WebGL2 parity tiers.

## Sources

- Figma WebGPU migration (C++/Wasm, abstraction, fallback): figma.com/blog/figma-rendering-powered-by-webgpu
- Figma Wasm load-time 3×: figma.com/blog/webassembly-cut-figmas-load-time-by-3x
- Figma Mirror DOM a11y: figma.com/blog/building-accessibility-into-a-canvas-based-product
- WebGPU Baseline Jan 2026: utsubo.com/blog/frontier-web-apis-2026-production-ready · vr.org/articles/webgpu-baseline-2026-three-js-webxr-default
- wgpu (native + wasm WebGPU/WebGL2, Naga WGSL→GLSL): github.com/gfx-rs/wgpu
- wgpu + wasm in worker with OffscreenCanvas: github.com/matthewjberger/webgpu-worker
- tldraw rendering bottleneck: github.com/tldraw/tldraw/issues/8307 · tldraw.dev/blog/tldraw-sdk-4.4
- Penpot Rust/Wasm/Skia rebuild: designtools.fyi/tools/penpot · pyshine.com Penpot overview
