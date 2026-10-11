# Task Summary 005 — Repository Scaffold (Foundation Tooling, No Editor Features)

```text
Task: Create the real repository foundation per ADR-001 (ACCEPTED — Option B) and the product
      owner's explicit choice: core/ Rust workspace (9 crates), app/ TS/React shell, wasm build
      pipeline, CI running real gates from day one, reference-document fixture spec.
Date: 2026-10-11
```

**Problem being solved:** ADR-001 acceptance unblocked code, but starting editor features on an
unscaffolded repo would violate the foundation-first law. The scaffold must make the
architecture's laws PHYSICAL (purity boundaries, sole-write-door, boundary duplication) and give
every future agent a correct place to put things — with real CI gates so "tests green" is
verifiable from the first commit.

**What was inspected:** ADR-001 §Consequences (crate list, tooling), `engine-core.md` (ownership
table, boundary protocol, testing pyramid, artifact discipline), `dependencies.md` (written this
task), canvas §26 (budget fixtures), testing rules T2/T5, and current toolchain facts: wgpu
(WebGPU pass-through + WebGL2 via Naga), wasm-pack/bindgen, WebGPU Baseline 2026 (prior turns).

**Approach selected (all verified live in the sandbox):**
1. **`core/` cargo workspace** — nine crates exactly per ADR-001 (`gip-document`, `gip-coords`,
   `gip-geometry`, `gip-layout`, `gip-text`, `gip-history`, `gip-scene`, `gip-render`,
   `gip-session`), each a *documented skeleton*: module docs citing contract + rule IDs, one
   structural vocabulary item per crate (e.g. the eight spaces, six bounds kinds, three sizing
   modes), harness smoke tests. `unsafe_code = forbid`, `missing_docs = warn`, clippy `-D
   warnings` policy, `[profile.dev] debug = 0`.
2. **Dependency law made enforceable**: `scripts/check-boundaries.py` (Python 3 + tomllib) parses
   every crate manifest and fails on illegal internal edges or unregistered externals —
   **verified passing on the real repo and verified FAILING on a deliberately violated copy**
   (both violation classes named correctly). Only `gip-render`→wgpu and
   `gip-session`→wasm-bindgen (wasm32) are permitted externals; layer-1 crates have zero deps.
3. **`app/` shell** — React 19 + Vite 8 + strict TypeScript (7.0.2) + Vitest; foundation status
   page (placeholder, DS phase replaces it); `src/engine/client.ts` = the hand-written facade
   with the protocol-version handshake (`assertProtocolVersion`) + 3 passing unit tests.
4. **Wasm pipeline** — `scripts/build-wasm.sh` (wasm-pack, `--target web`, output
   `bindings/pkg`, out-name `gip_engine`); run live: 31s build, generated package validated with
   Node's WebAssembly engine (valid module, 132,424 bytes, exports `protocol_version:function`).
   Output is gitignored; bindings README documents the never-hand-edit rule.
5. **CI from day one** (`.github/workflows/ci.yml`): `core` (fmt --check, clippy -D warnings,
   cargo test, boundary check) · `wasm` (wasm32 check, wasm-pack build via
   taiki-e/install-action, artifact existence) · `shell` (npm ci, typecheck, vitest, build).
6. **Fixtures spec** — `fixtures/reference_documents.json`: the five canvas §26 reference
   documents as machine-readable targets ([PROPOSED] concrete parameters, categories are law) +
   lifecycle README (generator lands with `.gip` v0 / ADR-003).
7. **Docs**: new `docs/architecture/dependencies.md` (registry: internal edges, externals with
   resolved versions + justification per CQ1.4, toolchain pins, npm-over-pnpm decision);
   ADR-001 addendum (tooling realized); `engine-core.md` §10 updated to realized paths.

**Live verification results (this sandbox, Rust 1.99.0 / Node 20.20.2):**
- `cargo test --workspace` — **11 unit tests, 9 crates, all green** (incl. gip-render linking
  wgpu 30.0.1 and gip-session facade).
- `cargo fmt --all -- --check` — clean. `cargo clippy --workspace --all-targets -- -D warnings`
  — clean (one lint-priority config fix needed: group priority for `clippy::all`).
- `python3 scripts/check-boundaries.py` — PASS (edge-by-edge printout); negative test on a
  violated copy — FAIL with both violations correctly identified.
- `./scripts/build-wasm.sh` — success; `bindings/pkg/gip_engine_bg.wasm` = **valid WebAssembly
  module (Node-validated), 132 KB, exports the protocol handshake**.
- Shell: `npm run typecheck` clean · `npm test` 3/3 · `npm run build` success (vite 8.3.4).
- Resolved versions: wgpu 30.0.1 · naga 30.0.1 · wasm-bindgen 0.2.129 · wasm-pack 0.13.1 ·
  react 19.3.0 · vite 8.3.4 · typescript 7.0.2 · vitest 4.1.11.

**Errors found & fixed during the task (recorded per bug-fix honesty law):**
1. `clippy::all` in `[workspace.lints.clippy]` needed `priority = -1` to allow specific-lint
   overrides (dbg_macro deny) — fixed, then clippy clean.
2. `scripts/build-wasm.sh` initially non-executable (Permission denied) — chmod +x.
3. **`wasm-pack --out-dir` resolves relative to the CRATE directory, not the invoking cwd** —
   two misplaced output trees (`core/bindings`, `core/crates/bindings`) were created and removed;
   script now passes an absolute `"$ROOT/bindings/pkg"` and the output location is verified.
4. wgpu initially resolved to 26.0.1 because cargo honored the `rust-version = "1.85"` MSRV —
   bumped MSRV to 1.88 and pinned wgpu 30.0.1 (current); recorded in `dependencies.md` §3.

**Alternatives rejected:** TS-only scaffold (contradicts the accepted ADR-001); making wgpu an
off-by-default feature to dodge compile cost (hides the real build; the graph compiles fine);
pnpm from the start (single shell package — npm now, recorded as revisitable); committing
`bindings/pkg` (generated output must never be committed); skipping CI until "real code"
(tests-from-day-one was the explicit ask — and it caught a config bug immediately).

**Remaining risks:** (1) CI actions/yaml unverified against a real GitHub runner (no repo
connected) — first push will confirm; structure follows standard action usage. (2) wasm-pack
warns a newer version exists (0.15.0 vs 0.13.1) — pin deliberately; upgrade is a chore, not a
task. (3) `unreachable_pub` allow in `wasm.rs` is deliberate (wasm-bindgen requires pub) —
documented inline. (4) P0-class panic/trap recovery ADR still pending before risky subsystems.

**Next steps:** ADR-002 (transform/matrix conventions) · ADR-003 (`.gip` format v0 — unblocks
fixture generation and the document crate's real implementation) · Design System formalization
(roadmap step 3) · then the first real subsystems (coords → geometry → document) each with their
golden-fixture suites per testing T2.
