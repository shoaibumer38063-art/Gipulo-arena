# Dependencies, Workspace Layout & Toolchain — Registry

> Status: ACTIVE (2026-10-11, scaffold phase). Laws this doc serves: **CQ1.4** (no dependency
> without written justification) · **engine-core.md §1** (Boundary Duplication Law) · **ADR-001
> ACCEPTED (Option B)** — Rust/Wasm core + TS/React shell. Enforcement: `scripts/check-boundaries.py`
> (CI block) for the Rust side; lockfiles (`core/Cargo.lock`, `app/package-lock.json`) for exact
> versions. **Every new dependency requires a row in §3 before it may be used.**

## 1. Realized workspace layout

```
gipulo/
├── core/                 Rust workspace → WebAssembly (ADR-001 Option B)
│   ├── Cargo.toml        workspace: lints (unsafe forbidden, docs required), dev profile
│   └── crates/           the nine decided crates (below)
├── app/                  TS/React shell — DOM/UI only (Boundary Duplication Law)
├── bindings/             GENERATED wasm package (bindings/pkg — never edited, never committed)
├── fixtures/             reference documents for the budget suite (canvas §26)
├── scripts/              check-boundaries.py · build-wasm.sh
└── .github/workflows/    CI: core · wasm · shell (all three gate merges)
```

## 2. Internal dependency law (enforced — `scripts/check-boundaries.py`)

| Crate | May depend on (internal) | May depend on (external) |
|---|---|---|
| `gip-document` | — | **`serde` + `serde_json`** only (ADR-003 schema serialization — the ONE deliberate layer-1 exception; see §3) |
| `gip-coords` | — | **none** |
| `gip-geometry` | document, coords | **none** |
| `gip-layout` | document, geometry, coords | **none** |
| `gip-text` | document, geometry, coords | **none** |
| `gip-history` | document | **none** |
| `gip-scene` | document, geometry, coords | **none** |
| `gip-render` | document, geometry, coords, scene | **wgpu only** |
| `gip-session` | all of the above | **wasm-bindgen only** (wasm32 target only) |

Rationale: layer-1 crates stay dependency-free so the geometry/document/math truth is pure,
headless-testable, and immune to ecosystem churn; wgpu sits behind the only crate that may draw;
the facade is the only boundary. Violations fail CI with the exact edge named.

**Registered exceptions for `gip-document` (ADR-009 extends ADR-003):** `sha2` and `unicode-normalization` join serde/serde_json (documentHash and NFC). Earlier text: `gip-document` may depend on
`serde` (derive) + `serde_json` for schema serialization. Justification: the schema is the serialization contract (CQ5 single definition), serde derive is compile-time only, and serde_json used purely as a function library preserves the purity/headless-testing rationale. Hand-rolling JSON to keep the letter of "zero external" would trade a trusted audit target for maintenance burden — not a good trade. Any further layer-1 exception requires its own registered decision (same process as any new dependency).

## 3. External dependency registry (resolved versions, 2026-10-11)

### Rust core

| Dependency | Resolved | Used by | Why allowed |
|---|---|---|---|
| `wgpu` (+ `naga`) | 30.0.1 | `gip-render` only | The graphics abstraction itself: browser WebGPU pass-through + WebGL2 via Naga WGSL→GLSL (ADR-001; rendering rules R1). Major-version bumps: note in task summary + full budget suite run (renderer is budget-critical). |
| `wasm-bindgen` | 0.2.129 | `gip-session` (wasm32 only) | The boundary transport (engine-core.md §3): intents down, events/snapshots up. Generated code never hand-edited. |
| `serde` 1.0.229 + `serde_json` 1.0.152 *(pinned by Cargo.lock; document phase)* | `gip-document` | ADR-003: the schema IS the serialization contract — derive-based (compile-time) with serde_json as a pure function library; shortest-round-trip f64 via ryu. The single deliberate layer-1 exception (§2). |

| `sha2` 0.11 | `gip-document` | ADR-009: documentHash = SHA-256 over the canonical document section. Pure Rust (RustCrypto), no I/O, wasm-compatible; hand-rolled crypto is forbidden. |
| `unicode-normalization` 0.1 | `gip-document` | ADR-009: NFC check on every string (ADR-003 #2). Pure Rust Unicode tables; checks only, never rewrites strings. |

### Shell

| Dependency | Resolved | Why allowed |
|---|---|---|
| `react` + `react-dom` | 19.3.0 | UI shell (ADR-001 Option B; Figma's public pattern: React around an engine — the engine here is Wasm). |
| `vite` + `@vitejs/plugin-react` | 8.3.4 / 6.1.2 | Dev server + production bundling for the shell. |
| `typescript` | 7.0.2 | Strict typing law (CQ5) — shell must typecheck with `--strict`-class settings. |
| `vitest` | 4.1.11 | Shell test runner (engine-core.md §9 layer 3). |
| `@types/react`, `@types/react-dom` | 19.3.0 | Type packages for React (non-runtime). |
| `@playwright/test` (+ Chromium via `playwright install`) | dev only | Browser E2E of the shell ↔ engine handshake (`app/e2e`, roadmap step 6). Runs against the production build. Added with task 019. |

### Toolchain (not dependencies, but version-locked)

Rust stable **1.99.0** (`rust-version = "1.88"` MSRV in `core/Cargo.toml` — set by wgpu 30's
MSRV; bumped only via ADR-worthy note) · target `wasm32-unknown-unknown` · `wasm-pack` 0.13.1
(bindings builds) · Node **20** (engines ≥20) · Python 3 (boundary checker; no third-party wheels).

## 4. Package-manager decision

**npm** for the shell (deviation from ADR-001's provisional "pnpm" wording, recorded 2026-10-11):
the shell is currently a single package; npm is already present everywhere Node is; zero extra
toolchain. Revisit (pnpm workspaces) when `bindings/` becomes a consumed workspace member or a
second shell package appears. The lockfile is committed and CI installs with `npm ci`.

## 5. Process rules

1. New dependency ⇒ add a §3 row (name, used-by, why) in the same task — then it may be imported.
2. Core-side dependencies additionally pass `scripts/check-boundaries.py` (internal edges are
   least-privilege; unknown externals fail the check).
3. Lockfiles are committed; CI uses frozen installs (`npm ci`; cargo uses `Cargo.lock`).
4. Dependency removal follows `rules/feature-removal.md` (no orphan deps left behind).
5. **Dev-dependencies obey the same law** — the checker reads them too. Property/round-trip
   suites therefore use an in-crate deterministic PRNG (splitmix64, seeds in the tests) instead
   of a framework: reproducible failures, zero supply chain (decision 2026-10-11, task 009).
   Revisit only if a failure needs input shrinking.
