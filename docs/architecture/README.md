# Architecture Documentation & ADRs

Dedicated docs for core architecture — SEPARATE from feature docs.

**Status 2026-10-11: ALL 14 architecture contracts authored; repo scaffold REALIZED (core/ Rust
crates + app/ shell + CI — still zero editor features).** `canvas.md` is the HUB: subsystem map,
dependency law, frame lifecycle, cross-cutting contracts, open-question tracker. `engine-core.md`
is the placement law: core/shell ownership table + the Rust/Wasm ⇄ TypeScript boundary protocol.
`dependencies.md` is the dependency registry + workspace layout + toolchain versions.

| Doc | Scope |
|---|---|
| `canvas.md` | **Hub** — subsystem map, dependency direction, frame lifecycle, open questions |
| `engine-core.md` | **Placement law** — core/shell ownership table, Rust/Wasm ⇄ TS boundary protocol, memory/threading/error policy, testing pyramid |
| `document-tree.md` | `.gip` document model: node types, container contracts, components, text nodes |
| `coordinate-system.md` | The 8 spaces, canonical conversion module, camera store |
| `vector-engine.md` | Geometry engine: path model, one operations library, bounds, spatial index, caches |
| `rendering.md` | Pipeline, graphics abstraction, WebGPU/WebGL2 backends, invalidation, overlays, export |
| `selection-system.md` | Selection store, resolution priority, hit-testing, hover, marquee, preservation |
| `transforms.md` | Canonical transform data, gesture-start-delta pattern, pivots, commit cleanup |
| `layout-engine.md` | Auto Layout, constraints, sizing modes, **RVS stretch-region contract** |
| `input-system.md` | Event normalization, the one state machine, gestures, modifiers, cancellation, tools |
| `undo-redo.md` | Transactions, one-intent-one-entry, coalescing, stack policy |
| `file-format.md` | `.gip` container, precision, IDs, migrations, assets |
| `code-generation.md` | Native design-to-code: IR pipeline, mapping registry, Code Workspace, Preview law |
| `dependencies.md` | **Registry** — workspace layout, internal dependency law (enforced by `scripts/check-boundaries.py`), external dependency table with resolved versions, toolchain pins, package-manager decision |

(Subsystem contracts folded into these docs per the hub table: text engine, guides/overlays,
export renderer, a11y mirror, snapping, hit-testing, performance/invalidation — they get own docs
when their feature phase starts, if warranted.)

Why: many features depend on the same core systems (e.g. changing coordinate conversion affects
selection, resize, rotate, vector editing, snapping, rulers, guides, comments, plugins, exports).
Agents must always know which systems are foundational.

## Architecture Decision Records (ADRs)

Store ADRs here as `ADR-NNN-short-title.md`. Every important architecture decision gets a
permanent record containing:

```
Decision:
Date:
Status: (proposed | accepted | active | superseded by ADR-X)
Context:
Why:
Alternatives considered:
Consequences:
```

Purpose: prevent a future agent from replacing an important system simply because it did not
understand why it existed.

### ADR index

| ADR | Topic | Status |
|---|---|---|
| `ADR-001-technology-stack.md` | **ACCEPTED (2026-10-11 — Option B):** Rust/Wasm engine core (crates under `core/`) + thin TS/React shell; renderer = wgpu inside the core (WebGPU primary, WebGL2 via Naga fallback); boundary law in `engine-core.md` | **ACCEPTED — ACTIVE** |
| `ADR-002-transform-conventions.md` | Transform & numeric conventions: decomposed canonical fields (x·y·w·h·rotation·flips), column-major 6-tuple matrices (CSS/SVG/Canvas order), degrees normalized to (-180,180], flip flags, zero-crossing policy, no skew, one constants module | **PROPOSED (2026-10-11)** — binding as implementation default; owner review items § |
| `ADR-003-file-format.md` | `.gip` v0: single-file UTF-8 JSON, schema order, explicit fields, shortest round-trip f64; u64 IDs (24-bit doc salt + 40-bit counter) with merge remap law; sidecar content-addressed assets; forward-only migrations; bounded unknown-field preservation; no compression in v0 | **PROPOSED (2026-10-11)** — binding as implementation default |
| `ADR-004-history-storage.md` | History entries = structured op logs of field-level changes (never commands, never snapshots); undo replays recorded values; truncate redo tail; byte-budget cap; 500 ms text coalescing; in-memory v1; collab-non-precluding shape | **PROPOSED (2026-10-11)** — binding as implementation default |
| `ADR-007-segment-index.md` | Path-level spatial index: static BVH over tight segment bounds, rebuilt per committed transaction that touches the path; brute force remains the oracle | **PROPOSED (2026-10-11)** — binding as implementation default |
| `ADR-006-path-model.md` | Canonical path model: cubic-only segments, vertex = anchor + absolute handles, explicit closed flag and fill rule, straight-segment data definition; path DATA in `gip-document`, OPERATIONS in `gip-geometry` | **PROPOSED (2026-10-11)** — binding as implementation default |
| ADR-005 (queued) | RVS affordance colors (required before Responsive Vector Segments ship) — reserved color language decided in `docs/design-system/08_canvas_ui.md` §8 | queued |
| `ADR-005-rvs-affordance.md` | RVS affordances: eligible = magenta (guide/spacing) with dot; activated = success green with count pill; red stays reserved for rulers | **ACCEPTED (2026-10-11, owner delegation)** — reversible by superseding ADR |
| `ADR-009-document-hash-and-nfc.md` | documentHash = SHA-256 of the document section (written on save, verified on load); strings must be NFC (rejected, never rewritten); deps sha2 + unicode-normalization | **ACCEPTED (2026-10-11, owner delegation)** |
| `ADR-008-recovery-and-integrity.md` | P0-class: trap recovery from committed whole-file snapshots; autosave cadence (2 s debounce, 30 s max); 3 generations; telemetry-free local diagnostics; fault-injection feature (release-excluded) | **PROPOSED (2026-10-11)** — owner review on cadence, retention, clean-close, copy |

Full requirements: `../memory/03_documentation_system.md` §6–7.
