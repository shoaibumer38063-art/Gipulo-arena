# Agent Task / Decision Summaries

After every important task, the agent saves a structured summary here (NOT private raw
chain-of-thought) as `YYYY-MM-DD_NNN_short-slug.md`, using this template:

```text
Task:
Date:
Problem being solved:
What was inspected:
Approach selected:
Why this approach:
Alternatives rejected (and why):
Files changed:
Behavior changed:
How result was verified:
Remaining risks:
What to do next if the problem returns:
```

Purpose: give future agents the useful reasoning context behind decisions.

Entries so far:
- `2026-10-11_001_document-ingestion.md` — creation of the memory system.
- `2026-10-11_002_rules-system.md` — all 17 rule files authored (v1, ACTIVE).
- `2026-10-11_003_architecture_contracts.md` — 13 architecture contracts + ADR-001 prepared (PROPOSED at the time).
- `2026-10-11_004_adr001_accepted_option_b.md` — owner decision recorded (Option B) + `engine-core.md` boundary law + full cascade.
- `2026-10-11_005_repo-scaffold.md` — repo scaffold: core crates, shell, wasm pipeline, CI, fixtures (zero editor features).
- `2026-10-11_006_design_system_formalization.md` — 14 Design System docs: tokens, components with all states, canvas UI + closed guide-color language, cursors, motion, a11y audit, responsive, research notes.
- `2026-10-11_007_design_system_implementation.md` — DS code v1: drift-tested token layer, 17 registry entries (15 components + tokens + scrollbar) with 47 state tests, gallery story page, editor-frame demo.
- `2026-10-11_017_document_hash_nfc.md` — ADR-009: documentHash (SHA-256) written and verified; NFC enforced on save/load. Deps sha2 + unicode-normalization registered.
- `2026-10-11_015_rvs_affordance.md` — ADR-005 accepted under owner delegation: RVS uses magenta (eligible) and success green (activated) with shape cues; red stays for rulers. Docs-only.
- `2026-10-11_014_recovery_policy.md` — ADR-008 (P0-class, PROPOSED): trap recovery from committed whole-file snapshots; autosave cadence; 3 generations; local-only diagnostics; release-excluded fault injection. Docs-only; no code yet.
- `2026-10-11_013_segment_index.md` — `gip-geometry::SegmentIndex` (ADR-007, PROPOSED): static BVH per path; equal to brute force; 1.13 ms vs 142 ms per query on a dense 1000-segment fixture (fixture-specific; caveat in ADR). 131 workspace tests green.
- `2026-10-11_012_history_transactions.md` — `gip-history`: op log (ADR-004), atomic transactions, exact undo/redo, byte budget, dirty tracking; FieldKey accessors in gip-document; 128 workspace tests green. Write door CLOSED (A6): `gip-document::edit` is the only op vocabulary; boundary check enforces it.
- `2026-10-11_011_document_serializer.md` — `gip-document` schema + `.gip` v1 container, validation rules, hex-string IDs (ADR-003 A1–A6), `gip-geometry` transform bridge; 114 workspace tests green.
- `2026-10-11_010_geometry_phase.md` — `gip-geometry` realized: cubic math, bounds, flatten, hit geometry, VG2 ops subset; ADR-006 path model; 88 workspace tests green; measured hover baseline ≈10 ms on 1000 segments (spatial index is the next item).
- `2026-10-11_009_coordinates_phase.md` — `gip-coords` realized: Mat2/Trs/Camera/viewport conversions/angles/epsilon module; 38 crate tests (property suites, deterministic PRNG); workspace 48 Rust tests green; clippy/fmt/boundaries/wasm check green.
- `2026-10-11_008_adr_bundle_002_003_004.md` — ADR bundle: transforms/numerics (002), `.gip` v0 format (003), history storage (004) — all PROPOSED, binding as implementation defaults; contract open questions closed.
