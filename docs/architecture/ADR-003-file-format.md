# ADR-003 — `.gip` File Format v0: Container, Identifiers, Floats & Assets

```text
Decision:   `.gip` v0 = a SINGLE-FILE, UTF-8 JSON container, schema-ordered, all fields written
            explicitly, floats as shortest round-trippable decimals (exact f64). Identifiers are
            64-bit: 24-bit per-document salt | 40-bit monotonic counter; any cross-document merge
            RE-ALLOCATES ids (remap law). Assets are EXTERNAL, content-addressed (sidecar store
            + manifest inside the file); no blobs inside the JSON. Versioned, forward-only
            migrations; unknown fields preserved at bounded extension points. No compression in
            v0 (a measured, separate decision later). History/camera/selection are NOT in the
            file. Public spec publication: deferred product decision.
Date:       2026-10-11 (authored)
Status:     PROPOSED — awaiting product-owner review. Binding as the implementation default
            (D7 pattern). On acceptance, this ADR closes file-format.md §8 and
            coordinate-system.md §8.2 (per-view camera persistence), and fixes the ID item in
            document-tree.md §8.1.
Refines:    file-format.md §8 (all six items) · CAN-SER (all) · CAN-MEDIA-1. Interacts with:
            ADR-002 (transform field shape), ADR-004 (history is NOT persisted in v0).
```

## Context

- `file-format.md` already fixes the laws: deterministic round-trip, full-fidelity float64,
  forward-only versioned migrations, transient-state exclusion, explicit-over-inferred fields,
  assets referenced (never inlined in the tree). This ADR picks the concrete representation the
  laws demand — it does not reopen them.
- The format is a product surface (one-click mental model "a file is a file") and a codegen input
  (vision Solution 2 reads the same model); it must be readable by humans and AI agents during
  this project's development, diffable in review, and testable as golden bytes natively
  (`cargo test` — ADR-001).
- Budgets that constrain the choice (canvas §26): 10k-node open ≤ 3 s (incl. wasm instantiation),
  save ≤ 1 s, memory ≤ 1.5 GB. v0 documents are far smaller than the point where binary encoding
  pays for itself; premature binary costs the debuggability this project depends on daily.
- Precedent: Figma's public Kiwi encoding earns its place at Figma's scale [PUBLIC, figma.com
  kiwi blog]; Penpot's early SVG-native model shows interchange strengths but needed a renderer
  rebuild for effects [PUBLIC]. Gipulo's model is richer than SVG's (components, layout, masks,
  explicit fields) — JSON mirrors the schema directly.

## The decision (sub-decisions; each changeable only by a superseding ADR)

| # | Sub-decision | Choice | Rationale |
|---|---|---|---|
| 1 | Container | **Single `.gip` file, UTF-8 JSON**, top-level shape `{ format: "gip", version: 1, meta, document, assets }`. No compression in v0 | Human/agent-readable, diff-friendly, migration-friendly; load/save budgets are dominated by parse+validate, not disk I/O at v0 scale; compression is a cheap later win (see §Open questions) |
| 2 | Determinism | Serializers emit **schema-declared field order** (Rust struct order), arrays preserve order exactly, maps are `BTreeMap` (sorted) — never hash-map iteration; **all fields written explicitly** (no omit-when-default); strings NFC-normalized; no timestamps/random values in content sections | Byte-identical output across runs/platforms is the golden-file law (CAN-SER-1, T2.3); explicit fields make diffs reviewable and make "unknown field" handling unambiguous |
| 3 | Floats | f64 stored as JSON numbers using **shortest round-trippable decimal** (Rust `ryu` via serde_json) — `parse(serialize(x)) == x` bit-for-bit for every finite f64; NaN/±Inf are schema-invalid; no display rounding, no fixed precision | CAN-SER-2 exactness; shortest form keeps files small and readable; property test over random bit patterns incl. subnormals locks it in |
| 4 | Identifiers | `NodeId` = u64: **`salt(24)` in the high bits, `counter(40)` in the low bits**. Salt = random per document lineage, stored in `meta` (hex); counter = monotonic high-water mark stored as `meta.nextId` | IDs are unique+stable+never-reused within a lineage (CAN-SER-3); sequential low bits give debuggable, human-traceable IDs; random salt makes accidental cross-lineage collisions improbable (~1/16.7M per pair) and the remap law (below) makes them harmless |
| 5 | Merge/paste law | **Any operation that brings nodes from one document lineage into another MUST re-allocate IDs** (remap table applied atomically; `nextId` advances past the incoming maximum) | Keeps salt+counter safe without 128-bit IDs; paste and future collab have ONE documented mechanism; collision never requires detection-and-repair |
| 6 | Assets | **Sidecar store + manifest**: files live beside the document in a content-addressed store (`<name>.assets/sha256/<xx>/<hash>`); `assets` section in the `.gip` maps `assetId → { hash, kind, bytes, colorSpace?, missing? }`; nodes reference `assetId` only | CAN-MEDIA-1: media referenced, never inlined; dedupe by content hash is free; missing assets are explicit records that survive save/load; single-file packaging (zip-like bundle) is a separate later ADR when sharing/export needs it |
| 7 | Asset identity | `assetId` is a separate stable u64 (same allocation rules as NodeId); the content hash (SHA-256) is the store key; hash mismatch on load ⇒ documented error state, never a crash. Assets are **immutable in v0**: replacing media creates new content (new hash); the manifest entry for the `assetId` is updated, nodes are untouched | Content addressing gives integrity + dedupe; separating `assetId` from content hash means node references are stable across re-imports |
| 8 | Versioning | Integer `version` (starts at 1) + ordered forward-only migration steps in `gip-document`, each with fixtures + tests; load = detect → validate → migrate chain → validate; **no down-migrations** | CAN-SER-3; old documents always open; every schema change ships its migration in the same task (documentation rule D2) |
| 9 | Unknown fields | Preserved at **bounded extension points**: each serializable struct carries an `ext` map (omitted when empty) capturing unrecognized keys; load→save preserves them; tests pin the behavior | Forward-compat law without unbounded `#[serde(flatten)]` everywhere; deliberate, auditable surface |
| 10 | NOT in the file | Selection, camera, hover, modes, overlay chrome (CAN-SER-4); undo/redo stack (ADR-004, in-memory v1); collaboration metadata (future ADR); per-view camera persistence **deferred** — multi-viewport camera stays app-level workspace state, not document data | Transient-state law is absolute; camera-in-file would dirty documents on navigation (CAN-CAM-1's spirit) |
| 11 | `meta` fields | `name` · `docSalt` · `nextId` · `appVersion` (information only) · `documentHash` = SHA-256 over the canonical `document` section bytes (integrity + fast dirty checks; computed by save; never influences rendering or geometry identity) | Deterministic, explicit, no time/random in content; hash enables golden-byte tests to report meaningfully |
| 12 | TS mirroring | Rust serde structs are the single schema source; TS types are hand-mirrored and guarded by a **cross-language fixture test** (Rust emits fixtures; the shell test parses + validates them in CI). Schema→TS codegen is deferred until churn justifies it (CQ1.2) | CQ5 "TS types mirror the schema exactly" enforced by CI, not by discipline; no codegen machinery before it earns its place |
| 13 | Public spec | **Deferred** — a product decision for the owner (is `.gip` documented publicly?). Interchange-ready design stands regardless; publication, if chosen, is written from the same schema source | Product decision, not engineering; nothing in v0 blocks it |

## Accepted costs & mitigations (honest record)

1. **JSON is not the smallest possible file.** Mitigation: budgets are met at v0 scale; the
   compression decision (below) is measured, and a binary container remains a supported upgrade
   path behind the same migration framework (the container is versioned, so encoding swaps ship
   as migrations, not rewrites).
2. **24-bit salt is not collision-proof across many merged lineages.** Mitigation: the remap law
   (sub-decision 5) makes correctness independent of collision probability; full 64-bit random
   IDs were considered and rejected only because they lose debug-friendly sequencing (see Options).
3. **Hand-mirrored TS types can drift.** Mitigation: the cross-language fixture test fails CI on
   drift; the mirror is small (schema types only, no logic).
4. **`ext` preservation adds per-struct noise.** Mitigation: bounded to one optional field per
   struct, omitted when empty; tests pin preservation so agents cannot "simplify" it away.

## Options considered (pre-decision record)

- **Binary-first container (Kiwi-like)**: rejected for v0 — loses daily debuggability/diffability
  for an agent-built project; Figma's precedent is at a scale Gipulo has not reached; remains the
  documented upgrade path when measurements demand it.
- **SVG-native document (Penpot-like)**: rejected — cannot express components/layout/masks as
  first-class truth and drags web-platform semantics into the model; SVG is an interchange
  format for us, not the truth.
- **SQLite/ZIP multi-file container**: rejected for v0 — adds a dependency and a second file model
  for no v0 benefit; the sidecar asset store covers the multi-file need with zero dependencies.
- **Full 64-bit random NodeIds**: safe against collisions but unordered/unreadable in logs and
  files (agents diff these); salt+counter + remap law chosen instead.
- **Embedded assets (blobs in the file)**: rejected for v0 — breaks the "media referenced, never
  inlined" law and bloats every save; packaging for distribution is a separate later ADR.

## Consequences

1. **`gip-document` implementation order:** schema structs with serde (explicit fields, `ext`
   maps, BTreeMaps) → golden-fixture suites (all node kinds, pathological values) → migration
   registry skeleton → `documentHash` computation.
2. **`file-format.md` §8 closes** (items 1–4 decided; item 5 color-space metadata remains linked
   to the color-management ADR; item 6 public spec stays an open product question).
3. **`coordinate-system.md` §8.2 closes:** per-view camera persistence is app-level workspace
   state, not `.gip` data, in v0.
4. **Fixtures:** the five canvas §26 reference documents become `.gip` v1 files generated by the
   serializer (never hand-written), doubling as golden round-trip tests.
5. **Dependency registry updated (deliberate):** `dependencies.md` §2/§3 + `scripts/check-boundaries.py`
   register the ONE layer-1 exception — `gip-document` may depend on `serde` + `serde_json`
   (derive is compile-time; serde_json used as a pure function library; version pinned when the
   document phase first imports them). The boundary check passes with the allow-list entry in
   place, so the document phase can proceed without touching CI policy.
6. **ADR-004 interaction:** history is not serialized; patch payloads reference canonical field
   values (ADR-002), so a save mid-session needs no history awareness beyond transaction cut
   points (already guaranteed).
7. **Save/load UX note (product-visible later):** because history is in-memory (ADR-004), closing
   a file discards the undo/redo stack; autosave/version-history is a separate future feature with
   its own ADR (must not be promised as v1 behavior).

## Owner review items (product-visible — non-blocking, recommended defaults included)

1. **Public `.gip` spec:** should the format be publicly documented (fits the "own the file"
   position of a Figma alternative)? Recommended: yes eventually — design is interchange-ready;
   defer the publication decision until the format is stable (v1+).

## Open questions (deferred, tracked)

1. **Compression:** gzip/brotli wrapper — decided by measurement when a reference document exceeds
   a save/open budget threshold. (Deterministic settings required — no timestamps in headers.)
2. **Color-space metadata policy** — with the rendering color-management ADR.
3. **Single-file distribution packaging** (document + assets bundled for sharing/export) — ADR
   when share/export is scheduled.
4. **Collaboration metadata** (server-mediated workflows) — future ADR; the format reserves the
   `meta.ext` point rather than inventing speculative fields (CQ1.2).
5. **Variables/tokens model shape** — ADR with the variables feature (document-tree §8.3);
   format will mirror that schema when it exists.

## Sources

- Figma file format (Kiwi) public blog: figma.com/blog (the Kiwi encoding announcement) [PUBLIC]
- JSON number precision / shortest round-trip (`ryu`, serde_json) — docs.rs/ryu [PUBLIC]
- SHA-256 content addressing (standard practice; no citation needed beyond FIPS 180-4) [PUBLIC]
- Penpot SVG-native model & renderer rebuild — collected in `docs/design-system/13_research_notes.md`
  and ADR-001 §Precedent research [PUBLIC]
- tldraw/Excalidraw document shapes (JSON-first editors) — public repos, surveyed in
  `docs/design-system/13_research_notes.md` [PUBLIC]

## Implementation amendments (document phase, task 011)

Recorded when `gip-document` was built. These refine, and where stated replace, the sub-decisions above.

- **A1 — IDs are 16-digit lowercase hex strings, not JSON numbers (refines #4, #5).** JSON
  numbers are lossy above 2^53 in JavaScript and salt<<40 values are mostly above it. Every
  `NodeId`, `AssetId`, `meta.docSalt`, and `meta.nextId` is written as `"ab12cd0000000002"`.
  The decoder is strict: exactly 16 lowercase hex digits; uppercase and short forms are rejected.
- **A2 — `ext` is limited to `GipFile`, `Meta`, `Document`, `Page`, `Node`, `AssetRecord` (replaces
  the "each serializable struct" wording of #9).** `Transform` (Copy, hot) and the path types
  (`Path`, `Subpath`, `Vertex`) carry no `ext`; they use `deny_unknown_fields`, so an unknown key
  is an error, not silent data loss. Reason: the surface stays bounded and auditable.
- **A3 — Asset ids share the lineage id space.** They use the same salt/counter rules, must be below
  `nextId`, and must not collide with any node or page id.
- **A4 — (CLOSED by ADR-009, 2026-10-11) `documentHash` is written as SHA-256 on save; the hashing dependency is registered.** Originally: it is
  never silently omitted; SHA-256 is a separate dependency decision.
- **A5 — Serialization is validate-then-write.** `to_json` validates before writing, so NaN/Inf
  (which serde_json would write as `null`) can never reach disk. Numbers use `float_roundtrip`
  for bit-exact f64 round trips.
- **A6 — Migrations are forward-only and table-driven** (`file::MIGRATIONS`, empty at v1).
