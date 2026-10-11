# ADR-010 — WebGL2 Fallback Parity Tiers

```text
Decision:   WebGPU is the primary backend; WebGL2 is a supported fallback with a defined parity
            contract. The renderer's shared path uses ONLY render pipelines (no compute, no
            storage buffers). Every renderer feature declares its tier; effects that cannot run on
            WebGL2 degrade by a documented rule, never silently.
Date:       2026-10-11 (authored)
Status:     ACCEPTED under the owner's standing delegation of design decisions (2026-10-11).
            Reversible by a superseding ADR.
Closes:     rendering.md §8 open questions 1 (shader source-of-truth, partly) and 2 (parity tiers).
Binds:      rendering.md (R1–R7) · CAN-RENDER-1 · CAN-FX-1 (effect isolation) · canvas §25 (parity tests).
```

## Context (verified facts, 2026-10-11)

- WebGL2 and GLES 3.0 have **no compute shaders**; indirect execution is also absent (wgpu
  `DownlevelFlags::COMPUTE_SHADERS`, `INDIRECT_EXECUTION`).
- WebGL2 does **not support storage buffers in vertex or fragment shaders** (`VERTEX_STORAGE`,
  `FRAGMENT_STORAGE`), and does not support read-only depth-stencil or depth-texture copies.
- wgpu targets WebGL2 by translating WGSL to GLSL (Naga), with the `webgl` path; the feature set
  is therefore the intersection of WebGL2 and what wgpu downlevel exposes.
- WebGPU Baseline (Jan 2026): Chrome/Edge 113+, Safari 26+, Firefox 141+/145+ (~82% global, per
  task 003 research). WebGL2 remains needed for the rest.

## Decision (tiers)

| Tier | Backend | What it must do | Rule |
|---|---|---|---|
| **T1 — Full** | WebGPU | All renderer features, including compute-based work | Reference output. Golden pixels are generated here first |
| **T2 — Parity** | WebGL2 | Every feature that is in the **core canvas set** (vector fills/strokes, images, clipping, selection and guide overlays, text quads, export) | Must match T1 within the documented tolerance (below) |
| **T3 — Degraded** | WebGL2 | Effects that need compute or storage buffers (heavy blur paths, large GPU-side sorting, OIT-style transparency, GPU tiling caches) | Degrade by a DECLARED fallback (CPU path or simpler render-only pass). The degraded state is shown in the layers panel as a documented badge, never silent |

### Rules

1. **P1 — Render-only shared path.** Shared renderer code uses render pipelines, uniforms, textures,
   and vertex buffers only. Compute and storage buffers are allowed ONLY behind a feature gate that
   has a T3 fallback declared in the same task.
2. **P2 — Declared tier per feature.** Every renderer feature doc states its tier, its T3 fallback,
   and its parity tolerance. A feature without a declared tier cannot merge.
3. **P3 — Parity tolerance.** T2 must match T1 within a per-feature tolerance recorded in the
   feature doc (initial default: mean absolute difference ≤ 2/255 per channel on the reference
   fixtures, with no visible geometry shift). The tolerance is a measured value, not a guess; the
   first renderer task measures and records it.
4. **P4 — Fallback is visible.** Running in T3 for an effect shows its badge; the doc is never
   silently degraded (CAN-FX-1 isolation still applies: a failing effect degrades only itself).
5. **P5 — Same truth.** Both tiers consume the same scene and the same pixel-rule inputs (R4):
   same document + camera + DPR ⇒ same output within tier tolerance. Backend choice never changes
   document, geometry, or selection state (R1).
6. **P6 — Shader authoring.** Shaders are written in WGSL and translated for WebGL2 by Naga (the
   path ADR-001 chose). A shader that Naga cannot translate for WebGL2 is a T3 shader by definition.

## Testing

- CI runs the parity suite on both backends where the runner supports them. Where a runner lacks
  GPU/WebGL2, the suite records "not run" explicitly; it never reports a pass it did not execute.
- Pixel parity is a canvas §25 requirement; the renderer's first task must add the harness.

## Consequences

- No renderer feature may rely on compute or storage buffers without a declared T3 path.
- Budget work (canvas §26) must be measured on T1 and on T2 separately before it is claimed.
- Export uses the T1 path when available; if export runs on T2, export parity uses the same tolerance (rendering.md §3).

## Owner review (non-blocking)

Decided under delegation. The numeric default tolerance (P3) is provisional until the first
renderer task measures it. Override any row by a superseding ADR.
