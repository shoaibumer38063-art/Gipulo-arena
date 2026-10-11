# ADR-009 — documentHash (SHA-256) and NFC Normalization: Dependency Decisions

```text
Decision:   Register two dependencies for gip-document, and the rules that use them.
            · sha2 0.11 (RustCrypto, pure Rust, no_std-capable) → documentHash = SHA-256
            · unicode-normalization 0.1 (pure Rust, Unicode tables) → NFC checks on every string
Date:       2026-10-11 (authored)
Status:     ACCEPTED under the owner's standing delegation of design/dependency decisions (2026-10-11).
            Reversible by a superseding ADR.
Closes:     ADR-003 A4 (documentHash null until registered) · ADR-003 §2 determinism ("strings NFC-
            normalized") · roadmap step 11 "next" · task 011 open items.
Amends:     dependencies.md §2 — gip-document external allow-list grows from {serde, serde_json}
            to {serde, serde_json, sha2, unicode-normalization}.
```

## Context

- ADR-003 #11 requires `documentHash` = SHA-256 over the canonical `document` section bytes, computed
  by save, never influencing rendering or identity. A4 forbids silently omitting it.
- ADR-003 #2 requires strings to be NFC-normalized, so the same text always produces the same bytes
  and hash.
- Both needs are about bytes and text identity, which belongs in the serializer (gip-document), not in
  the shell.
- Neither crate touches I/O, time, or randomness, so the purity rationale of `dependencies.md` §2 holds.

## Options considered

| Need | Option | Verdict |
|---|---|---|
| SHA-256 | **sha2** (RustCrypto) | ✅ chosen: pure Rust, widely audited, compiles to wasm32, no unsafe in our code |
| SHA-256 | Hand-rolled | ❌ crypto must not be hand-written (CQ1.4 justification) |
| SHA-256 | Web Crypto from the shell | ❌ would move a core invariant across the boundary; hash must be identical in tests and core |
| NFC | **unicode-normalization** | ✅ chosen: the reference algorithm, `is_nfc` check without allocation for clean strings |
| NFC | Normalize on write (rewrite strings) | ❌ silently changes user content; contradicts validate-then-write (A5) |
| NFC | Ignore | ❌ breaks byte-identical output across platforms |

## Decisions (sub-decisions)

| # | Rule | Choice |
|---|---|---|
| 1 | Hash input | `serde_json::to_string(&file.document)` — compact, schema field order, `BTreeMap` order (ADR-003 #2). Exact bytes, not pretty-printed |
| 2 | Hash output | lowercase hex, 64 digits (same form as asset hashes, ADR-003 #7) |
| 3 | On save (`to_json`) | `meta.documentHash` is written as the computed hash. The caller's `GipFile` is not mutated |
| 4 | On load (`from_json`) | `null` is accepted (file predates the hash). A present hash MUST equal the recomputed hash; a mismatch is `GipError::Invalid("document hash mismatch")` |
| 5 | NFC (save and load) | Every string in the file (keys and values, all sections) must satisfy `is_nfc`. A non-NFC string is `GipError::Invalid` naming the JSON path. Never auto-rewritten |
| 6 | Scope | Only `gip-document` uses these crates. Other layers stay dependency-free (dependencies.md §2 unchanged) |
| 7 | Version | No file-format version bump: the `documentHash` field already exists in v1 (ADR-003 #11) |

## Consequences

1. Golden fixture `tests/golden/sample.gip` gains a real `documentHash`; it is regenerated with
   `GIP_UPDATE_GOLDEN=1` and the diff must be only that field (checked in task 016).
2. Files written before this ADR (hash `null`) still load.
3. Tampering with the document section is detected on load.
4. The hash enables fast dirty checks in the shell (ADR-008 autosave) without a second serializer.

## Owner review (non-blocking)

Decided under delegation. Override any row by a superseding ADR.
