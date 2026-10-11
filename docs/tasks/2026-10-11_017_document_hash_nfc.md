# Task 017 — documentHash (SHA-256) and NFC normalization (ADR-009)

- **Date:** 2026-10-11 · **Roadmap:** step 16 · **Status:** DONE (ADR-009 ACCEPTED under owner delegation)
- **Owner delegation:** the owner authorized the dependency and design decisions for this task.

## Decisions
- Dependencies `sha2` 0.11 and `unicode-normalization` 0.1, registered in ADR-009 and `dependencies.md`; boundary allow-list for `gip-document` updated in `scripts/check-boundaries.py`.
- `documentHash` = lowercase SHA-256 over the compact, schema-ordered document section. Written on save (caller's value untouched). Load accepts `null`; a present hash must match.
- NFC is enforced, not applied: any non-NFC string (keys and values) is rejected on save and load, naming its JSON path.

## Changes
- `core/crates/gip-document/src/integrity.rs` (new): `document_hash`, `check_nfc`.
- `core/crates/gip-document/src/file.rs`: `to_json` stamps the hash and checks NFC; `from_json` checks NFC and verifies the hash.
- `core/crates/gip-document/tests/integrity.rs` (new, 7 tests): hash format, no mutation of caller value, round-trip stability, legacy `null` load, tamper detection, NFD rejection on save and load, NFC acceptance.
- `core/crates/gip-document/tests/golden/sample.gip`: regenerated; diff is only the `documentHash` line.

## Verification
- `cargo test --workspace`: 138 passed, 0 failed.
- `cargo clippy --all-targets -- -D warnings`, `cargo fmt --check`, wasm check for `gip-session`, boundary check: all pass.

## Open
- Owner review of ADR-009 (non-blocking).
- Push to GitHub is pending a token.
