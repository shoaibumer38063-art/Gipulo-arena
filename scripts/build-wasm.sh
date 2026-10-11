#!/usr/bin/env bash
# Builds the Gipulo engine core (Rust) to WebAssembly and emits the generated bindings package.
#
# Contract: docs/architecture/engine-core.md §10 — the output under bindings/pkg/ is GENERATED,
# never hand-edited, and never committed. CI runs this same script (see .github/workflows/ci.yml).
#
# Requires: rustup toolchain (see rust-toolchain.toml) + wasm-pack
#   install wasm-pack: https://rustwasm.github.io/wasm-pack/installer/
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

if ! command -v wasm-pack >/dev/null 2>&1; then
  echo "ERROR: wasm-pack not found." >&2
  echo "Install: https://rustwasm.github.io/wasm-pack/installer/  (or: cargo install wasm-pack)" >&2
  exit 1
fi

cd "$ROOT/core"
# --out-dir is resolved relative to the CRATE directory by wasm-pack, so pass an absolute
# path to make the destination unambiguous: <repo>/bindings/pkg.
wasm-pack build crates/gip-session \
  --target web \
  --dev \
  --out-dir "$ROOT/bindings/pkg" \
  --out-name gip_engine

echo ""
echo "OK: bindings/pkg generated (NEVER edit or commit this output — it is regenerated)."
echo "The shell consumes it through a thin hand-written facade (app/src/engine/client.ts)."
