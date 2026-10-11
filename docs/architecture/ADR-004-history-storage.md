# ADR-004 — History Entry Storage & Stack Policy

```text
Decision:   History entries are STRUCTURED OP LOGS of field-level changes — never command
            objects, never snapshots. One entry = { intent label, ordered edit ops }
            (SetField · InsertNode · RemoveNode · MoveNode · ReorderChildren · SetShared ·
            SetMeta). Undo applies recorded `before` payloads in reverse order; redo applies
            `after` payloads in order — mutators are never re-run. In-memory only in v1 (no
            serialization). Stack policy: truncate redo tail on edit-after-undo; byte-budget
            cap with oldest-drop degradation; text sessions coalesce on a 500 ms pause or focus
            loss. Nested transactions fold into the parent entry. Entries carry no timestamps.
Date:       2026-10-11 (authored)
Status:     PROPOSED — awaiting product-owner review. Binding as the implementation default
            (D7 pattern). On acceptance, this ADR closes undo-redo.md §7.1/§7.2 and confirms the
            §2 coalescing [PROPOSED] values.
Refines:    undo-redo.md §7 (items 1–4) · CAN-HIST (all) · SM3. Interacts with: ADR-002
            (patched values are canonical transform fields), ADR-003 (history is not persisted,
            IDs are stable).
```

## Context

- `undo-redo.md` fixes the laws: transactions are the sole write door (SM3, physically enforced
  by the boundary — the shell has NO document write API); one user intent = one entry; transient
  state never enters history; history must guarantee `undo(transact(x)) ≡ pre-x` exactly
  (byte-level for golden tests), atomicity (failed transactions change nothing), and gesture
  atomicity (observers never see intermediate writes — previews are transient state).
- What was open: the storage model (inverse patches vs commands), branching policy, memory cap,
  text coalescing window, and multiplayer non-preclusion. This ADR closes them.
- The workspace already fixes: `gip-history` owns `IntentLabel` (data, not formatting);
  transactions live in the Wasm core (ADR-001 boundary); invalidation publishes affected node
  IDs to scene/geometry/spatial-index subscribers.
- Determinism (CAN-SER-1) bans time/random in stored data — history entries included.

## The decision (sub-decisions; each changeable only by a superseding ADR)

| # | Sub-decision | Choice | Rationale |
|---|---|---|---|
| 1 | Entry model | **Ordered op log**: `Entry { intent: IntentLabel, ops: Vec<EditOp> }`; `EditOp` is a small closed enum (table below). Each op stores both `before` and `after` payloads for the fields it touches | A generic, verifiable representation: undo = replay `before` in reverse, redo = replay `after` in order — exactness provable by construction and testable byte-for-byte, without re-running feature logic |
| 2 | Why not commands | Commands (mutator closures with an undo method) are rejected | They duplicate logic (mutator + inverse frequently drift), cannot be generically proven to restore exact state, and break when feature code changes between sessions. `rules/` single-mechanism law prefers one verifiable mechanism agents cannot improvise around |
| 3 | Why not snapshots / persistent trees | Full-document snapshots and immutable-tree (structural sharing) storage are rejected | Snapshots multiply memory by document size × depth; structural sharing is a whole different document architecture with wasm-side cost and no invalidation granularity for free. Op logs also give invalidation data (affected node IDs) as a by-product |
| 4 | `EditOp` vocabulary (closed set, v0) | `SetField { node, field: FieldKey, before, after }` · `InsertNode { parent, index, nodeSnapshot }` · `RemoveNode { node, subtreeSnapshot }` · `MoveNode { node, fromParent, fromIndex, toParent, toIndex }` · `ReorderChildren { parent, before, after }` · `SetShared { collection, key, before, after }` · `SetMeta { field, before, after }`. Deletions store one subtree snapshot (not one op per node) so a subtree delete is atomic and cheap to invert | Keeps entries compact for the common case (a drag = a handful of `SetField`s on a few nodes) and atomic for structural changes; `FieldKey` is a closed enum mirroring the document schema (no string paths in hot paths — string paths exist only in diagnostics/tests) |
| 5 | Undo/redo mechanics | Undo: apply `before` payloads in reverse op order. Redo: apply `after` payloads in forward order. Mutators are NEVER re-executed | Deterministic, session-independent, and immune to feature-code drift; makes the byte-identity test (undo ⇒ exactly the pre-transaction document) a direct comparison |
| 6 | Atomicity & rollback | A transaction collects ops while its mutator runs; a failure (validation, invariant violation) applies collected `before` payloads in reverse and commits nothing | Atomicity law (undo-redo §2); fault-injection test required |
| 7 | Nested folding | Nested transactions (e.g. layout reflow triggered inside a resize) fold into the outermost entry (CAN-HIST-4); nesting is a core-internal mechanism, not a shell-facing API | One user intent = one entry, regardless of internal fan-out |
| 8 | Branching | Edit-after-undo **truncates the redo tail** (standard editor behavior) | Confirms the [PROPOSED] in undo-redo §2; a history TREE is not v1 — it is a different product promise (version history) already planned as its own future feature |
| 9 | Memory cap & degradation | Cap by **estimated payload bytes** of stored ops (default 64 MiB [PROPOSED — measured and tuned at implementation]); beyond budget, drop oldest entries and advance a documented `canUndo` boundary; `markSaved`/`isDirty` unaffected | Entries are small in the common case; the cap protects the 1.5 GB budget against pathological sessions (many large subtree deletes); behavior is explicit and testable, never silent |
| 10 | Coalescing | Pointer gestures & scrub/slider drags = one entry each (commit at gesture end — already law). Text sessions = one entry per edit session, broken on **pause > 500 ms** or focus loss. Repeated discrete intents (arrow-key nudges) = separate entries | Confirms undo-redo §2 [PROPOSED] values; the 500 ms window is re-confirmed with the text feature when real typing telemetry exists |
| 11 | Persistence | History is **in-memory only** in v1; not serialized (ADR-003 keeps it out of `.gip`); `markSaved()` stores a save-point marker (entry index + document hash) for dirty tracking; closing the file discards history | Avoids speculative version-history machinery; the architecture does not preclude it (entries are intent-labeled + node-scoped) |
| 12 | Invalidation | Committing an entry publishes affected node IDs (union of ops) + document version to subscribers: scene, geometry caches, spatial index, a11y mirror, autosave dirty state | Already contracted (undo-redo §4); the op log makes the affected-set exact and cheap |
| 13 | Collaboration non-preclusion | No speculative CRDT/OT code. The op-log shape (intent-labeled, node-scoped, field-addressed, ordered) is the shape any future server-mediated sync needs; the future ADR maps it | CQ1.2 (no speculative code); the option value of this decision is real but unused today |

## The op vocabulary, precisely

```text
SetField       — one field of one node (or shared record) changes: transform fields (ADR-002),
                 opacity, visible, locked, name, style refs, text runs, component overrides…
InsertNode     — a node enters the tree at (parent, index); snapshot carries the full node
RemoveNode     — a node leaves the tree; snapshot carries the entire subtree (atomic inverse)
MoveNode       — reparent + index change for an existing node (identity preserved)
ReorderChildren— sibling order changes without reparenting (explicit before/after orders)
SetShared      — components / styles / assets manifest entries (collection + key addressed)
SetMeta        — document-level fields (name, etc.)
```

Rules: ops execute in stored order; applying an entry's ops is schema-validated; `before`/`after`
payloads contain ONLY the fields an op is responsible for — never whole-node dumps inside
`SetField` (that would resurrect snapshot-like memory costs).

## Accepted costs & mitigations (honest record)

1. **Every mutator must emit ops correctly** — the one place where a bug can surface as "undo
   didn't restore". Mitigation: the document crate's write API emits ops structurally (agents
   cannot write fields without going through it — SM3); golden tests assert `undo ⇒ byte-identical
   pre-state` for every operation family, plus fault-injection for rollback.
2. **FieldKey enum maintenance** as the schema grows. Mitigation: `FieldKey` is generated from the
   same schema source as serialization (single definition, CQ5) — not hand-maintained twice.
3. **Subtree snapshots make large deletes heavier than patches.** Accepted; measured; if a
   pathological case appears, the mitigation (payload cap + drop-oldest) already bounds it, and a
   future ADR can add compression of payloads in memory without changing the model.
4. **Text coalescing window is a guess until telemetry.** Marked [PROPOSED]; re-confirmed with the
   text feature — a wrong window is a tuning constant, not an architecture risk.

## Consequences

1. **`gip-history` implementation:** `EditOp` enum + `Entry` + transaction collector + apply/undo
   engine + stack policy. `FieldKey` comes from `gip-document` (single schema source).
2. **`undo-redo.md` §7.1 and §7.2 close**; the §2 [PROPOSED] coalescing values are confirmed
   (text window re-confirmed later); §7.3 (memory cap) has its default; §7.5 (multiplayer)
   remains a future ADR but the shape is fixed as non-precluding.
3. **Dirty tracking:** `markSaved()` uses (entry index, document hash per ADR-003) — no history
   serialization involved.
4. **Tests (from undo-redo §8) get their fixtures:** 1000-move-event drag ⇒ 1 entry · cancel ⇒ 0
   entries + exact restore · nested fold · rollback under fault injection · byte-identity
   undo/redo on golden documents · memory-cap degradation · invalidation sets exact (no ghost
   geometry after undo).
5. **No scaffold changes required**; `gip-history` currently holds `IntentLabel` only — the
   transaction engine lands with the history phase, against these decisions.

## Owner review items (product-visible — non-blocking, recommended defaults included)

1. **Undo memory cap:** default 64 MiB [PROPOSED]. Effect for a designer: in practice thousands
   of steps; if a session does something pathological, the oldest steps silently age out (the
   undo menu simply shows fewer steps). Recommended; will be tuned with measurements.
2. **Closing a file discards undo/redo history** (in-memory v1). Recommended for v1: it avoids
   speculative persistence machinery; persistent version history is a separate future feature —
   not promised until scheduled.

## Open questions (deferred, tracked)

1. Payload compression in memory (only if measurements demand it) — no model change required.
2. Version-history / autosave persistence — its own feature + ADR; will read the same entries.
3. Collaboration sync protocol — future ADR; op-log shape reserved.
4. Text coalescing window re-confirmation with the text feature (tuning constant).

## Sources

- Standard editor practice for inverse-command vs patch-log undo: surveyed in ADR-001/`13_research_notes.md`
  precedents (Figma public engineering material; tldraw/Excalidraw public repos) [PUBLIC]
- Gipulo binding laws: `undo-redo.md`, `canvas.md` CAN-HIST/SM3, `rules/canvas.md` [internal]
- No claim is made about any tool's non-public internal representation (rules: never present
  inferences as facts).

## Implementation amendments (history phase, task 012)

Recorded when `gip-history` was built. Each one is a deliberate, tested narrowing or
clarification of the sub-decisions above.

- **A1 — ID allocation is outside history.** `alloc_id` advances `meta.nextId` and is never rolled
  back, by undo or by rollback. Ids are never reused (CAN-SER-3). Consequence: undo restores the
  document section exactly, but `meta.nextId` is higher than before. "Byte-identical" in the tests
  means the document section, compared by exact `PartialEq`, not the whole file.
- **A2 — Dirty tracking is positional, not hash-based, in v1.** `markSaved` records the history
  version. `isDirty` is true whenever the version differs, so undoing past the save point is dirty.
  The hash-based check (sub-decision 11) waits for the SHA-256 dependency decision.
- **A3 — `MoveNode` is implemented; `SetShared` is deferred.** Shared collections (components,
  styles, assets manifest) do not exist yet. Adding them is a schema change plus a new op.
- **A4 — Rollback is the same mechanism as undo.** A failed mutator or a failed validation replays
  the recorded ops backward. There is no second undo path.
- **A5 — Replay failure is an internal error, not a user error.** `TxError::Internal` means the
  recorded state no longer matches the document. The engine is then inconsistent and must be
  reloaded. No test should reach it except by deliberately corrupting a document.
- **A6 — CLOSED (task 012, owner decision "move into gip-document").** The op vocabulary
  (`EditOp`), `apply`, `snapshot`, and the affected-set helpers now live in `gip-document::edit`,
  the write door. `GipFile` and `Document` fields are crate-private; other crates read them only
  through accessors (`meta()`, `document()`, `assets()`, `pages()`, `nodes()`) and allocate ids
  through `GipFile::allocate_id()`. Only `gip-history` may name `gip_document::edit`, and
  `scripts/check-boundaries.py` enforces that (a probe file was verified to fail the check). Residual
  caveat, stated plainly: the rule is per crate, not per module. Code inside `gip-document` can
  still write fields, and `gip-history` could call `apply` outside `transact`. Both are reviewed
  and covered by the boundary check. A capability token would be the stricter alternative if the
  owner wants it later.
- **A7 — Budget eviction keeps the newest entry, always.** `enforce_budget` never evicts the entry
  just committed, even when it alone exceeds the budget. The documented `canUndo` boundary moves
  forward when entries age out.
