# File Format (`.gip`) & Serialization — Architecture Contract

> Status: DESIGN v1 (2026-10-11), shaped by ADR-001 (ACCEPTED: Rust/Wasm core + TS shell). Binding rules: CAN-SER (all), CAN-NUM-2,
> G7, document-tree contracts. Hub: `canvas.md`.

## 1. Purpose & scope

Defines `.gip`: the on-disk/wire representation of the document model, its versioning/migration
system, precision policy, and asset handling. The format is a product surface (determinism law:
same file ⇒ same geometry & visuals everywhere, CAN-SER-1) and a code-generation input (vision
Problem 2) — not an implementation detail.

## 2. Design goals (from the sources, binding)

1. **Deterministic round-trip:** serialize → load → serialize ⇒ identical (byte or documented
   semantic identity) — golden-file law (CAN-SER, testing T2.3).
2. **Full-fidelity precision:** float64 values round-trip exactly; stored values are NEVER
   pre-rounded to display precision (CAN-SER-2, CAN-NUM-2).
3. **Versioned, forward-only migrations** with per-step tests; old documents always load via
   migration chain (CAN-SER-3, removal rules RM1.3).
4. **Transient-state exclusion:** selection/camera/hover/modes/overlay chrome are NOT document
   data (CAN-SER-4); per-view camera persistence only if an ADR makes it an explicit feature.
5. **Interchange-friendly:** structure maps cleanly to the codegen IR and (future) import/export
   (SVG etc. — future ADRs), because design-to-code is native (vision Solution 2).
6. **Asset sanity:** media referenced, never inlined in the tree; asset store with content-hash
   dedupe, missing-asset records (CAN-MEDIA-1 states survive save/load).

## 3. Structure (conceptual sketch — encoding ADR-gated)

```
.gip container
├── meta: format version · app version · schema id · [content hash]
├── document:                       ← document-tree.md model, exact mirror
│   ├── pages[] → node trees (child order explicit & total, CAN-Z-1)
│   ├── shared: components · styles/variables/tokens · [asset manifest refs]
│   └── meta: name · user metadata (explicit fields only)
├── assets: manifest (id → content hash, media kind, color-space info, missing markers)
└── future-gated: per-view settings (ADR) · collaboration metadata (future ADR)
```

- **[PROPOSED] encoding:** versioned JSON-based container (gzip/brotli) for v1 — human-debuggable,
  agent-friendly, diff-friendly — with a documented upgrade path to a compact binary encoding
  (Figma's public Kiwi precedent shows binary earns its place at scale; adopting it prematurely
  costs debuggability). Binary media blobs live in the asset store, not the JSON. ADR-003 decides.
- **Field policy:** explicit over inferred (VG1 law at format level): closed/open, winding rule,
  sizing modes, clip flags, angle convention (ADR-002 output) are stored fields. Unknown fields
  in newer files are PRESERVED on round-trip (forward-compat law) — never dropped silently.

## 4. Owns

- Schema definition (single source; TS types mirror it exactly — CQ5) + schema-version registry.
- Serializer/deserializer (pure core functions; natively testable via `cargo test` —
  ADR-001/engine-core.md).
- Migration framework: ordered steps `vN → vN+1`, each with fixtures + tests; no down-migrations
  (forward-only, CAN-SER-3); load path = detect version → run chain → validate.
- Float encoding policy: shortest round-trippable decimal representation (or binary encoding if
  ADR-003 chooses binary container) — exactness is the requirement, representation is the choice.
- ID policy enforcement at format level: unique, stable, never reused (CAN-SER-3; scheme ADR-003).
- Save orchestration contract: async, non-blocking, transaction-consistent snapshots (never
  serialize mid-transaction — history atomicity guarantees clean cut points); autosave policy
  (app-level feature; format guarantees consistency of any saved snapshot).

## 5. Must never

- Serialize derived caches (bounds, tessellations, layout results, shaped text) as truth —
  derived data recomputes on load (document-tree §3); [exception: none in v1; caches-in-file is
  an ADR-gated optimization if ever measured necessary].
- Store display-rounded values (CAN-SER-2) or locale-dependent number formatting (CAN-SER-1).
- Embed time/random in content fields (timestamps only in explicit meta fields, never affecting
  geometry/visual identity).
- Break old documents: any schema change ships with its migration + tests in the same task
  (documentation rules D2 — "architecture changed?" includes format changes).

## 6. Dependencies

- Consumes: document model (in-memory truth), asset store, version registry.
- Consumed by: app save/load, golden-file test suites, code generation (reads the same model;
  may read the container directly for headless pipelines — codegen contract), future import/
  export & collaboration layers.

## 7. Invariants

- Round-trip identity (goal 1) · cross-platform determinism (same bytes everywhere) ·
  migration totality (every version pair connected by tested steps) · asset-ref integrity
  (dangling refs load as documented missing-asset states, never crashes).

## 8. Open questions → resolved by ADR-003 (PROPOSED 2026-10-11)

1. ✅ **DECIDED:** `.gip` v0 = single-file UTF-8 JSON, schema-declared order, explicit fields,
   no compression in v0 (measured compression ADR later).
2. ✅ **DECIDED:** IDs = u64 `salt(24) | counter(40)`; salt in `meta`; `nextId` high-water stored;
   **cross-lineage merges MUST remap IDs** (merge law).
3. ✅ **DECIDED:** f64 as shortest round-trippable decimal (`ryu`); `parse(serialize(x)) == x`
   bit-for-bit; NaN/±Inf schema-invalid.
4. ✅ **DECIDED:** sidecar content-addressed asset store + manifest inside the `.gip`; assets
   immutable in v0; single-file distribution packaging = separate later ADR.
5. Color-space metadata policy (with rendering color-management ADR) — still open.
6. ⏳ **OWNER REVIEW:** public `.gip` spec publication — product decision; design is
   interchange-ready either way.
7. Unknown-field preservation implemented at bounded `ext` extension points (ADR-003 #9).

## 9. Test focus

Golden round-trips per node type incl. pathological values (fractional, negative, huge, tiny,
deep nesting, unicode names, RTL text, missing assets) · byte-identity across runs/platforms ·
migration chain tests (every step, fixture per version) · unknown-field preservation ·
mid-transaction save exclusion · load performance (10k-node doc ≤ budget, canvas §26) ·
save performance (≤1s serialize target, §26) · asset integrity (hash mismatch ⇒ documented
error state).
