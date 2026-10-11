# Document Tree / Document Model — Architecture Contract

> Status: DESIGN v1 (2026-10-11), shaped by ADR-001 (ACCEPTED: Rust/Wasm core + TS shell). Binding rules: CAN-SER, CAN-FRAME,
> CAN-COMP, CAN-MASK, SM1/SM3, G7. Hub: `canvas.md`.

## 1. Purpose & scope

The `.gip` document model is the single source of truth for user content: the node tree, all
design data on nodes, shared collections (components, styles/variables, media references), and
document metadata. Everything else (geometry caches, scene, UI) is derived.

## 2. Owns

- Node types (initial set; additions are feature-doc + registry events): `document → page →
  {frame | group | section[future] | rectangle | rounded-rect | ellipse | line | arrow | polygon |
  star | vector-path | boolean-group | text | image | video | component | component-set |
  instance | mask-container | 3d-asset[planned, vision Solution 3]}`.
- Per-node common data: stable ID · name · transform data (per `transforms.md`) · opacity ·
  blend mode · visible · locked · export settings [as supported].
- Container contracts: **frame** (own child coordinate space, clip flag, constraints/Auto-Layout
  data, background) vs **group** (no clip, no layout, bounds derived) — different types, never
  interchangeable (CAN-FRAME-1).
- Component system data: component definitions, instance→main references, override records
  (property-level, scoped), component-set/variant structure (CAN-COMP-1).
- Masking data: mask kind (vector/alpha/luminance[as supported]), mask→content association.
- Shared collections: styles/variables/tokens (code-representable per vision Problem 2), media
  asset references (never inline blobs in the tree — refs + asset store, `file-format.md`).

## 3. Must never

- Contain derived/cached data as truth (bounds, layout results, tessellations are caches owned by
  consumers, invalidatable, never serialized as authoritative geometry).
- Contain transient/editor state (selection, camera, hover, modes — CAN-SER-4, SM1).
- Be mutated except through transactions (SM3, `undo-redo.md`) — no direct node writes from UI,
  tools, or layout.
- Infer structure from ambiguity: closed/open paths, sizing modes, clip on/off, winding rules are
  explicit fields (VG1).

## 4. Public interface (conceptual)

- Read API: typed getters by node ID (O(1) index), child iteration in document order (= paint/
  hit order, CAN-Z-1), ancestor chain, subtree walks with stable ordering.
- Write API: transaction-scoped operations only (create/delete/reparent/reorder/set-field with
  intent labels); schema-validated; every write path records enough for inverse (history).
- Observation API: narrow subscriptions per node/field for UI + a11y mirror + scene (SM4).
- Schema: versioned; the schema IS the file-format contract (`file-format.md`); TS types mirror
  it exactly (CQ5).

## 5. Text nodes (contract; full text-engine doc when feature starts)

Text node stores: content (logical characters + marks), box mode (auto-width/auto-height/fixed —
CAN-TEXT-2), style runs (font family/size/weight, line height, letter spacing, alignment),
direction/RTL data, font references (with fallback records). Shaped layout/caret geometry is
computed by the text engine — never stored in the document (CAN-TEXT-1: one layout result feeds
render/edit/select/export).

## 6. Invariants

- IDs unique, stable, never reused (CAN-SER-3); child order explicit and total.
- Same document ⇒ same derived everything (determinism, G7).
- Reparenting preserves node identity and (per rules) recomputes affected layout/bounds only.
- Instances never mutate their main component; overrides live on the instance (CAN-COMP-1).
- Legacy-data loading is migration-gated, never improvised (`file-format.md`).

## 7. Dependencies

- Consumes: nothing (root of the dependency graph).
- Consumed by: geometry engine, layout engine, scene/rendering, selection, history, a11y mirror,
  panels, code generation, export — all read-only except transactional writes via history.

## 8. Open questions (ADR-gated)

1. ✅ **DECIDED — ADR-003 (PROPOSED 2026-10-11):** IDs = u64 (`salt(24)` high bits + `counter(40)`
   low bits), salt per document lineage in `meta`, `nextId` high-water mark persisted;
   cross-lineage merges remap IDs.
2. Override record granularity (per-field vs per-subtree patches) → ADR with component feature.
3. Variables/tokens model shape (collections, modes, aliasing) → ADR with variables feature
   (must satisfy code-representation duty, vision Problem 2).
4. 3D asset node contract (source-link data for Live Link, vision Solution 3) → ADR when 3D starts.

## 9. Test focus

Golden documents per node type; round-trip identity (with `file-format.md`); transaction
atomicity/cancel-restore (with `undo-redo.md`); subscription granularity (SM6); determinism
across runs/machines; legacy-migration fixtures (once versions exist).
