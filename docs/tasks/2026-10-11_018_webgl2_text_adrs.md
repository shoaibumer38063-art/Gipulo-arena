# Task 018 — WebGL2 parity tiers (ADR-010) and text shaping stack (ADR-011)

- **Date:** 2026-10-11 · **Roadmap:** step 17 · **Status:** DONE (both ADRs ACCEPTED under owner delegation)

## Verified facts used
- WebGL2 / GLES 3.0 have no compute shaders and no vertex or fragment storage buffers (wgpu `DownlevelFlags`).
- rustybuzz was archived 2026-07-26; its README recommends harfrust. ttf-parser is flagged unmaintained (RUSTSEC-2026-0192); skrifa/read-fonts is the recommended successor.

## Decisions
- ADR-010: three tiers (T1 WebGPU reference, T2 WebGL2 parity for core canvas features, T3 declared degraded fallbacks with a visible badge). Shared path is render-only.
- ADR-011: harfrust for shaping; skrifa/read-fonts for font access; `gip-text` only; gates before shipping text (shape-regression corpus, wasm size, benches).

## Not decided (explicit)
- Text rasterization path (SDF/atlas vs CPU raster) and text-on-path: deferred to their own ADRs with their features.

## Changes
- `docs/architecture/ADR-010-webgl2-parity.md`, `docs/architecture/ADR-011-text-shaping.md` (new); architecture index updated.
- No code changed. No dependency added.
