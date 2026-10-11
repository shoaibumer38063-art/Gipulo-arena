# Gipulo

A 1:1, production-level alternative to Figma — built with AI agents under a strict
memory / rules / architecture system. Document format: `.gip`.

**Foundation status (2026-10-11):** memory system ✓ · rules system (17 files) ✓ · architecture
contracts (14 docs + ADRs) ✓ · technology decision **ACCEPTED — ADR-001 Option B** · **repo
scaffold ✓ (this phase)** · no editor features yet (by design).

## Repository map

| Path | What it is |
|---|---|
| `docs/source/` | The four canonical source documents (verbatim, read-only — ultimate truth) |
| `docs/memory/` | Fast-recall memory: index, canvas requirements, tokens, roadmap, CHANGELOG |
| `docs/architecture/` | 14 architecture contracts + ADRs (`ADR-001` = accepted stack; `engine-core.md` = core/shell boundary law; `dependencies.md` = dependency registry) |
| `docs/features/` | Feature docs + registry (empty by design — features not started) |
| `docs/bugs/` · `docs/prompts/` · `docs/tasks/` | Bug records · prompt archive · task summaries |
| `rules/` | 18 binding rule files — `canvas.md` is the main law of the canvas |
| `core/` | Rust engine core (crates under `core/crates/`) compiled to WebAssembly — scaffold |
| `app/` | TypeScript/React shell (DOM/UI only — never editor logic, per `engine-core.md`) |
| `bindings/` | Generated wasm bindings package (never hand-edited) |
| `fixtures/` | Reference documents for the performance budget suite (canvas §26) |
| `scripts/` | Boundary checker (dependency law) · wasm build script |
| `.github/workflows/ci.yml` | CI: core fmt/clippy/tests · wasm build · shell typecheck/tests/build |

## Build & test

```bash
# Engine core (Rust)
cd core
cargo test --workspace              # native tests — the majority of engine tests
cargo clippy --workspace --all-targets -- -D warnings

# Dependency-law check (from repo root)
python3 scripts/check-boundaries.py

# Shell (TypeScript/React)
cd app
npm install
npm run dev                         # dev server
npm test                            # vitest
npm run build                       # typecheck + production build

# Wasm bindings (generated — never edit the output)
./scripts/build-wasm.sh
```

## The rules of the house

Read `AGENTS.md` (operating contract) and `docs/memory/00_INDEX.md` before doing anything.
Canvas work: read `rules/canvas.md` FIRST. Everything else: the session-start list in
`rules/global.md`. Every meaningful task ends with documentation updates (`rules/documentation.md`).
