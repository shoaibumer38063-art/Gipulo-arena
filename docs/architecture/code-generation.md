# Code Generation & Code Workspace — Architecture Contract

> Status: DESIGN v1 (2026-10-11), shaped by ADR-001 (ACCEPTED: Rust/Wasm core + TS shell). Binding sources: vision Problem/
> Solution 2 (`docs/memory/02` — NATIVE design-to-code is a founding differentiator), feature-
> addition step 8 (code-equivalent duty), layout L3-6 (RVS code representation). Hub: `canvas.md`.

## 1. Purpose & scope

Turn `.gip` documents into REAL software projects (not one giant HTML blob): the generation
pipeline, the feature→code mapping registry, and the Code Workspace (design canvas left; Code +
Preview right, where Preview runs the actual generated code in a real runtime). Design-to-code is
planned INTO every feature from the start — the design system and code system are developed
together (vision Solution 2).

## 2. The two founding laws (from the vision doc — restated as architecture)

- **LAW 1 — Co-development:** every supported design feature must have a clear code equivalent
  defined while the feature is built (feature docs carry a "code equivalent" section —
  feature-addition step 8 enforces). Features without a defined code representation are
  incomplete by definition.
- **LAW 2 — Two renderers, one truth:** **Design Canvas = Gipulo's internal editor renderer.
  Code Preview = actual generated code running in a real runtime.** The preview is NEVER another
  rendering of the design canvas — the whole point is that mismatches between design and coded
  result become immediately visible (and testable).

## 3. Pipeline (stages are module boundaries)

```
document (.gip model) ─► IR (semantic intermediate: layout intent, components, tokens,
                          interactions, assets — target-independent)
                     ─► TARGET EMITTERS (project structure per target: folders · components ·
                          styles · tokens · assets · routes · deps · structured naming)
                     ─► BUILD/RUNTIME (compile/bundle as target needs)
                     ─► PREVIEW (sandboxed real runtime — iframe/worker per security ADR)
                     ─► EXPORT (downloadable project / repo output)
```

- IR is the stability layer: emitters never read the raw document tree ad hoc; the document →
  IR mapping is versioned with the schema (file-format §3). Determinism: same document + same
  target config ⇒ same generated project (diff-stable output — no timestamp/ordering noise).

## 4. Feature → code mapping registry (the living table LAW 1 fills)

Single registry (doc + machine-readable source) mapping every design capability to its code
representation. Seed mappings (from the sources; expanded as features ship):

| Design feature | Code equivalent (initial direction — ADRs refine per target) |
|---|---|
| Auto Layout | flexbox (direction/gap/padding/align/hug→auto, fill→flex-grow, fixed→basis) |
| Constraints | responsive positioning strategy per target (CSS anchors/percent/grid — ADR) |
| Responsive Vector Segments | stretch-region strategy: generated assets/SVG + 9-slice-style CSS or target-native equivalent (**L3-6: must be ADR'd before RVS ships**) |
| Typography (styles/scales) | font-face/CSS type styles or target tokens |
| Variables/tokens | design tokens export (CSS custom properties / JSON tokens / target-native) |
| Components/instances | framework components + props; variants → props/unions |
| Frames | containers/sections/routes (per target semantics) |
| Interactions/prototype | event bindings/animation code per target |
| Scroll interactions (vision Solution 4) | runtime scroll driver bindings (property→scroll-range maps) |
| 3D placement (vision Solution 3) | target 3D runtime embedding where supported |
| Masks/booleans/effects | CSS/SVG/target-native equivalents or pre-rendered assets (documented fidelity tiers) |

Fidelity is honest: where a target cannot represent something exactly, the registry records the
degradation + the preview makes it VISIBLE (that is LAW 2 working as designed).

## 5. Code Workspace (UI contract; DS-governed like everything)

- Layout: design canvas left · code/runtime area right (vision Solution 2). Two sub-sections:
  **Code** (generated project browser: folders/files/components/assets/styles/config with a real
  editor surface — DS tokens/typography incl. mono font) and **Preview** (the real runtime).
- Preview runtime: sandboxed (iframe with strict CSP / worker — security model ADR-gated), runs
  the ACTUAL built output of the generated project; device/viewport presets; console/error
  surfacing with DS error states; refresh-on-regenerate with documented debounce.
- Comparison affordances: side-by-side canvas vs preview (the designer's accuracy check);
  automated parity checking (see §7) feeds the same mismatch story.

## 6. Back-sync (future-gated, vision Solution 2)

Eventual goal: supported code-side changes synchronize carefully BACK into the design without
corrupting document structure. Architecture preconditions recorded NOW (no speculative code —
CQ1.2): IR round-trip annotations (provenance: which document node produced which code unit),
stable IDs in generated output, mutation-classification framework. Full design = future ADR when
the forward pipeline is proven.

## 7. Must never

- Emit "random HTML after design completion" (vision's explicit anti-goal) — structure/naming/
  tokens/components are first-class output.
- Fake the preview: any "preview" that re-renders via the canvas engine violates LAW 2.
- Read document truth privately: codegen consumes the same model/geometry via documented
  interfaces (single-source law); its own layout math is a duplication violation (L1).
- Produce nondeterministic output (diff-unstable projects destroy trust and testing).
- Inline secrets/telemetry or unexpected network calls in generated projects (generated code is
  user-facing product code — clean-code rules apply to OUTPUT too, incl. CQ1 prohibitions).

## 8. Open questions (ADR-gated)

1. v1 target list (recommendation: static HTML/CSS/JS project first — universal, previewable,
   exportable; React/TS emitter second; others per demand) → product-owner decision.
2. Preview sandbox security model (iframe CSP vs worker vs both) → ADR at impl.
3. IR schema versioning strategy with `.gip` schema → ADR with file-format ADR-003.
4. Parity automation: pixel-diff canvas-vs-preview thresholds + tolerance policy (this becomes a
   first-class test of code-translation accuracy — vision Solution 2's stated benefit) → ADR at impl.
5. RVS code representation (registry row above) → blocking ADR before RVS feature ships.
6. Dependency policy for generated projects (minimal-by-default; CQ1.4 spirit applies to output).

## 9. Test focus

Golden-project snapshots (document ⇒ generated tree, byte-stable) · IR round-trip/provenance
integrity · per-mapping fidelity tests (each registry row has a case: Auto Layout ⇒ flexbox
computed styles match layout engine results within tolerance) · preview runtime boots generated
project (E2E) · **canvas-vs-preview parity suite** (LAW 2's measurement: documented tolerances;
mismatches are codegen bugs) · determinism across platforms · large-document generation within
performance budgets (P2 surface: generated-code preview build+run).
