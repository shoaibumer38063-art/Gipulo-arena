# Gipulo Engine Core (Rust → WebAssembly)

The engine core per **ADR-001 (ACCEPTED — Option B)**: a Rust workspace compiled to WebAssembly.
All editor logic lives here; the TypeScript shell (`../app`) is DOM/UI transport only. Boundary
law: `../docs/architecture/engine-core.md`. Dependency registry: `../docs/architecture/dependencies.md`.

## Crates (ADR-001 §Consequences) — scaffold status: compiling skeletons, no editor logic yet

| Crate | Owns (contract) | Internal deps |
|---|---|---|
| `gip-document` | `.gip` document model (`docs/architecture/document-tree.md`) | — |
| `gip-coords` | coordinate spaces, conversions, matrices, camera math (`coordinate-system.md`) | — |
| `gip-geometry` | vector/geometry engine, bounds, spatial index (`vector-engine.md`) | document, coords |
| `gip-layout` | Auto Layout, constraints, RVS stretch (`layout-engine.md`) | document, geometry, coords |
| `gip-text` | text shaping/layout (`document-tree.md` §5) | document, geometry, coords |
| `gip-history` | transactions, undo/redo (`undo-redo.md`) | document |
| `gip-scene` | retained scene, invalidation (`rendering.md`) | document, geometry, coords |
| `gip-render` | wgpu renderer: document + overlay layers, export (`rendering.md`) | document, geometry, coords, scene, **wgpu** |
| `gip-session` | engine facade + wasm-bindgen boundary (`engine-core.md`) | all + **wasm-bindgen** (wasm32 only) |

The dependency law above is **enforced**: `python3 ../scripts/check-boundaries.py` (CI runs it).

## Commands

```bash
cargo test --workspace                 # native tests (most engine tests run here — headless)
cargo clippy --workspace --all-targets -- -D warnings
cargo check -p gip-session --target wasm32-unknown-unknown   # wasm compatibility
../scripts/build-wasm.sh               # wasm-pack → ../bindings/pkg (generated)
```

## Build notes

- `[profile.dev] debug = 0` in the workspace keeps builds fast (the wgpu dependency graph is
  large). Temporarily raise to `debug = 1` when stepping through code with a debugger.
- On resource-constrained machines, `CARGO_BUILD_JOBS=1` avoids OOM during the first cold build.
- `target/` and generated bindings are gitignored; a cold build downloads/compiles the graph once.
