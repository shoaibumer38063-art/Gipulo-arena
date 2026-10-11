# History / Undo-Redo — Architecture Contract

> Status: DESIGN v1 (2026-10-11), shaped by ADR-001 (ACCEPTED: Rust/Wasm core + TS shell). Binding rules: CAN-HIST (all), SM3,
> testing T2/T3. Hub: `canvas.md`.

**Placement (ADR-001 — engine-core.md):** transactions live in the core, and the boundary makes
the sole-write-door law PHYSICAL: the TS shell has no document write API at all — only typed
intents. No shell code path exists that could bypass history.

## 1. Purpose & scope

The sole mutation path for the document model: transactional, intent-labeled, undoable. Guarantees
the two history laws: **one user intent = one entry** (a whole drag is one undo, not hundreds)
and **transient state never enters history** (CAN-HIST-1/2).

## 2. Owns

- **Transaction API:** `transact(intent, mutator)` — collects all document writes of one intent
  into one entry; nested intents (layout reflow triggered by a resize) fold into the parent entry
  (CAN-HIST-4); failed mutators roll back fully (atomicity).
- **Entry model:** inverse-patch vs command-object storage — **ADR-gated** (CAN-HIST-3 [REC]);
  whichever is chosen: canvas/feature code NEVER hand-rolls private undo (single mechanism law);
  entries carry: intent label (for UX copy + a11y announcements), affected node IDs (for
  invalidation + subscriptions), inverse data, timestamp-free determinism (no time dependence in
  stored data, CAN-SER-1).
- **Coalescing rules (binding):** pointer gesture = 1 entry (commit at gesture end — previews
  never land); slider/scrub drags = 1 entry; text editing = 1 entry per edit session
  ([PROPOSED] break on pause >500ms or focus loss — ADR-confirm); repeated same-intent discrete
  commands (e.g. nudges) = separate entries (each is a distinct intent) but a11y announcements
  coalesce (CAN-A11Y-2).
- **Stack management:** undo/redo pointers; branching policy on edit-after-undo (**[PROPOSED]
  truncate redo tail — standard editor behavior; ADR-confirm**); memory bounds (entry cap /
  patch-size budget with documented degradation, e.g. drop oldest beyond N — ADR at impl);
  save-point markers (dirty state for window title / save UX).
- **Cancellation path:** gesture cancels bypass history entirely — restore from transient
  snapshot (input-system §2), zero entries (CAN-HIST-2).

## 3. Must never

- Record: selection changes, camera changes, hover, mode switches, panel/UI prefs, previews
  (CAN-HIST-2, SM1 — only the document domain is historical).
- Create multiple entries for one gesture, or split one intent across entries (CAN-HIST-1/4).
- Store time/random-dependent data in entries (determinism, CAN-SER-1).
- Allow document writes that bypass transactions (SM3 — enforced at the document model API level:
  there is no other write path).

## 4. Public interface (conceptual)

- `transact(intent, mutator)` (the only write door) · `undo()` / `redo()` / `canUndo/canRedo` ·
  `getIntentLabel(depth)` (UX/a11y) · `markSaved()` / `isDirty()` · subscriptions: entry
  committed / stack changed (panels, a11y announcements, autosave triggers).
- Invalidation contract: committed entries publish affected node IDs + geometry versions →
  scene/geometry caches and spatial index update incrementally (vector-engine §4 versioning).

## 5. Dependencies

- Consumes: document model (write API), transient gesture snapshots (restore-on-cancel).
- Consumed by: every mutation source (tools, panels, layout commits, imports, codegen-sync
  [future, vision Solution 2 back-sync must also transact]), autosave/serialization (save-point),
  a11y (announcement of intent labels), tests (transaction records as assertions).

## 6. Invariants

- `undo(transact(x)) ≡ pre-x` exactly (byte-level document identity for golden tests);
  `redo` re-applies exactly. Atomicity: failed transaction ⇒ no observable change. Gesture
  atomicity: during a gesture, observers see either pre-gesture or committed state — never
  intermediate document writes (previews are transient-state renders).

## 7. Open questions → resolved by ADR-004 (PROPOSED 2026-10-11)

1. ✅ **DECIDED:** entries are **structured op logs** of field-level changes
   (`SetField/InsertNode/RemoveNode/MoveNode/ReorderChildren/SetShared/SetMeta`) with recorded
   `before`/`after` payloads; undo replays recorded values — mutators are never re-executed.
2. ✅ **DECIDED (confirms [PROPOSED]):** edit-after-undo truncates the redo tail.
3. ✅ **DECIDED (default):** byte-budget cap (64 MiB [PROPOSED], measured at impl) with
   oldest-drop degradation + documented `canUndo` boundary.
4. ✅ **DECIDED (re-confirm later):** text sessions coalesce on pause > 500 ms or focus loss;
   re-confirmed with the text feature (tuning constant).
5. Multiplayer-era history: reserved — op-log shape is the future sync shape; no speculative
   code now (CQ1.2); its own future ADR.

## 8. Test focus

Gesture=1-entry law (drag with 1000 move events ⇒ single entry) · cancel=zero-entries + exact
restore · undo/redo byte-identity on golden documents · nested-intent folding (resize ⇒ reflow
⇒ one entry) · atomic rollback (fault-injected mutator failure) · coalescing windows (text
sessions) · branch policy · memory-cap behavior · invalidation correctness (caches/spatial index
consistent after undo — no ghost geometry) · determinism (no time/ordering dependence).
