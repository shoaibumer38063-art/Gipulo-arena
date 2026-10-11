# ADR-008 — Trap Recovery, Autosave & Document Integrity (P0-class)

```text
Decision:   A wasm trap or panic ends the core instance. Nothing inside it is trusted afterwards.
            Recovery therefore rests on COMMITTED, VALIDATED WHOLE-FILE SNAPSHOTS kept by the shell
            in local storage. The core only produces and checks snapshot bytes (pure functions in
            gip-document); the shell owns storage, timing, and the recovery UI. Diagnostics stay on
            the user's machine. Nothing is sent anywhere (no telemetry).
Date:       2026-10-11 (authored)
Status:     PROPOSED — binding implementation default (D7 pattern). Product-visible values are
            marked for owner review below.
Closes:     ADR-001 accepted cost 4 · engine-core.md §8 and §11.2 · canvas.md §10 item 9 ·
            architecture/README P0 queue entry. Refines: file-format.md §4 (save orchestration).
            Interacts with: ADR-003 (file = snapshot unit), ADR-004 (history not persisted).
```

## Context

- ADR-001 accepted the risk: a trap kills the module. Engine-core §8 requires recovery from the last
  autosave snapshot, and forbids risky subsystems (booleans, huge imports, codegen builds) until this
  ADR exists.
- Trap = unrecoverable instance state. Partial writes from a dying instance must never reach storage.
- History is in-memory (ADR-004 #11), so undo history is lost on any restart. That is accepted.
- The core has no filesystem and no clock. Those belong to the shell.

## The decision (sub-decisions)

| # | Sub-decision | Choice | Rationale |
|---|---|---|---|
| 1 | Recovery unit | The **whole committed file** (`gip-document::file::to_json` bytes), never a delta and never the in-memory history | Matches ADR-003 (one file, one truth). Deltas on a dying instance could be torn. Whole files are verifiable on their own |
| 2 | Cut points | Snapshots are taken only at commit boundaries: after `History::transact` records an entry, and after undo/redo. Never mid-transaction (file-format §4) | Transactions are atomic, so a commit boundary is always a consistent state. A trap mid-transaction loses only that uncommitted intent |
| 3 | Who writes | The core serializes (pure, deterministic, validated); the **shell** writes to local storage (IndexedDB) in one transaction per generation | The core has no storage. The shell already holds all DOM and persistence state (engine-core §1) |
| 4 | Validation before write | `to_json` validates first, so an invalid state can never be saved (ADR-003 A5) | A corrupt document cannot become the recovery source |
| 5 | Validation after read | `from_json` validates on every restore. An invalid generation is skipped, and the next older one is tried | Storage corruption is caught, not trusted |
| 6 | Generations | Keep the last **3** generations per document. Each write is atomic: write the new generation, then prune the oldest | Bounded storage; one bad generation never loses everything |
| 7 | Autosave cadence [PROPOSED — owner review] | Debounce **2 s** after the last commit; force a snapshot at most every **30 s** while dirty | Bounds the worst-case loss window to ~30 s, and typical loss to ~2 s |
| 8 | Dirty & save | `History::is_dirty` (task 012) drives the cadence. An explicit save writes the file to the user's chosen location and marks it saved. Autosave never replaces the user's file | Autosave is recovery, not "save as". The user's file stays the user's |
| 9 | Diagnostics | A panic hook in `gip-session` (native + wasm) records the panic message and location into a fixed-size in-memory buffer. On restart, the shell reads it once and stores it in a local crash record shown to the user on request. Never transmitted | Telemetry-free, reviewable, and useful for bug reports the user chooses to send |
| 10 | Restore flow | (a) Trap detected → shell shows the recovery surface → new core instance starts → shell loads the newest valid generation → `from_json` → new history starts clean → user sees "restored to <time>" | Explicit, testable, no silent data loss claims |
| 11 | No valid generation | Shell offers the last explicitly saved file and states that no recovery snapshot was usable. It never fabricates a document | Honest failure beats silent data loss |
| 12 | Clean close | After a successful explicit save with no later edits, the document's generations are deleted. If dirty, close prompts (save / discard / cancel) and the generations are kept until the user decides [PROPOSED — owner review] | Storage stays clean; unsaved work is never silently dropped |
| 13 | Version guard | A generation with a newer container version is refused, with a message to update the app (ADR-003 #8) | Forward-only migrations; never downgrade data |
| 14 | Fault injection | A test-only trap command exists behind a cargo feature `fault-injection`, which is OFF in release builds. A CI check fails if the feature is enabled for release | Trap recovery is tested for real, not simulated, without shipping a crash button |
| 15 | Integrity hash | When the SHA-256 dependency is registered (ADR-003 #11), each generation stores `documentHash`. Until then, content validation (rule 5) is the integrity check | Known gap, tracked, not hidden |
| 16 | Loss statement | The UI copy states what is lost: undo history, and any edits after the last snapshot | Honest recovery UX |

## Recovery copy (DS error pattern — 11_accessibility §error: icon + message + recovery action)

- Surface title: "Gipulo stopped unexpectedly"
- Body: "Your work was restored to <time>. Undo history was not kept. Changes made in the last few
  seconds may be missing."
- Actions: "Continue" (primary) · "View details" (shows local crash record).
- Never show raw panic text as the message; it goes only in "View details".

## Trap injection test plan

- Native: transaction atomicity under a failing mutator (already, task 012).
- `wasm-bindgen-test` (headless browser): with `fault-injection`, trigger a trap at a random
  committed point; restart; assert the restored document equals the last snapshot and validates.
- Corrupt-generation tests: truncated JSON, wrong hash later, invalid field → fallback to the older generation.
- Release gate: `fault-injection` must be absent from release artifacts (CI check).

## Consequences

1. `gip-session` gains the panic hook and the fault-injection feature (later task, with the first boundary code).
2. `gip-document::file` snapshot bytes are the only persisted form. No new format is introduced.
3. Risky subsystems may now be scheduled, each behind its own tests, once the restore path is tested.
4. The shell gets a storage module (IndexedDB) and a recovery surface (DS pattern); both are shell-only.

## Owner review items (product-visible, non-blocking; recommended defaults included)

1. Autosave cadence: 2 s debounce, 30 s maximum while dirty (rule 7).
2. Generations kept: 3 (rule 6).
3. Clean-close behaviour when dirty: prompt, and keep generations until the user decides (rule 12).
4. Recovery copy wording (above) — needs the content designer's review.

## Open questions (deferred, tracked)

1. Storage quota handling when IndexedDB is full (rule-level policy; measure first).
2. Snapshot compression (ADR-003 defers compression; revisit only if measured).
3. Multi-tab conflicts (two tabs on one document) — its own ADR when multi-tab is in scope.
