# Generated Wasm Bindings — DO NOT EDIT

Everything under `bindings/pkg/` is **generated** by `scripts/build-wasm.sh` (wasm-pack build of
`core/crates/gip-session`, `--target web`, output name `gip_engine`). It is never committed
(`.gitignore`) and never hand-edited — CI regenerates and uses it.

- The shell consumes the bindings **only** through the thin hand-written facade
  (`app/src/engine/client.ts`), which performs the protocol-version handshake
  (`PROTOCOL_VERSION` in `gip-session` ↔ shell mirror; engine-core.md §10).
- If the generated output is stale after an agent edits the core: re-run the script. Do not patch
  generated files — fix the Rust source or the facade.

Contract: `docs/architecture/engine-core.md` §3, §10.
