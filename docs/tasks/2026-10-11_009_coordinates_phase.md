# Task 009 — Coordinates phase (`gip-coords`)

**Date:** 2026-10-11 · **Status:** COMPLETE (native + wasm checks green) · **Governs:** `docs/architecture/coordinate-system.md` §10 · `ADR-002` (PROPOSED — binding implementation default)

## Goal

First real engine subsystem: the single coordinate-space / transform / camera module, built
test-first against ADR-002. No editor features, no document types (structural: `gip-coords`
does not depend on `gip-document`).

## What was built (`core/crates/gip-coords/`)

| Module | Contents | ADR anchor |
|---|---|---|
| `eps.rs` | `EPS_ROUNDTRIP`, `EPS_ANGLE`, `EPS_MATRIX` (1e-9), `EPS_REL` (1e-12) — the ONE tolerance module | ADR-002 §10, CAN-NUM-5 |
| `angle.rs` | `normalize_deg` → `(−180, 180]`; `−0.0` collapses; NaN/Inf pass through unlaundered | ADR-002 §5 |
| `vec2.rs` | `Vec2` value type, arithmetic, finiteness, approx equality | — |
| `mat2.rs` | `Mat2` column-major 6-tuple (CSS/SVG/Canvas2D order), composition, inverse (singular ⇒ `None`), local↔world, decompose | ADR-002 §2–4 |
| `trs.rs` | `Trs` canonical fields, `T·R·S` build, decomposition (singular/skew ⇒ `None`), canonical math-frame normalization | ADR-002 §1, §3, §6, §7, §8 |
| `camera.rs` | `Camera {x,y,zoom}`, world⇄viewport, screen-constant pan, pointer-anchored zoom with clamp-safe invariance, 2%–4096% | CAN-CAM-2/3 |
| `viewport.rs` | `ViewportRect`, screen⇄viewport, viewport⇄device (DPR), composite screen⇄world | CAN-COORD-2/3 |
| `lib.rs` | `Space` enum (8 canonical spaces, names pinned by test), re-exports | CAN-COORD-1 |

## Decisions made during the phase

1. **Canonical math frame ≠ stored frame.** `Trs::normalized()` folds negative sizes into flips
   and expresses horizontal mirror as `flip_v` + 180°. Stored data keeps the user-authored flags
   (a horizontal flip is a first-class user action and must not be rewritten in the panels).
   Decomposition returns the normalized frame.
2. **Zero-size nodes are legal:** `Mat2::invert` and `decompose` return `None` for singular
   matrices; forward application stays finite. No hidden minimums (ADR-002 §7).
3. **Deterministic property testing without a framework.** Dev-dependencies are subject to the
   registry law too. A 20-line splitmix64 generator with fixed seeds gives reproducible failures
   and zero supply chain. Recorded in `dependencies.md` §5 rule 5. Revisit only if shrinking is
   needed.
4. **Camera invariance uses the clamped zoom.** Anchor-preservation is derived from the clamped
   zoom, so it holds even at the 2% / 4096% limits.

## Bugs caught during the phase (all fixed, none shipped)

- **Test-only:** an `f64` literal array needed an explicit type for `to_bits()` (E0689).
- **Test expectation wrong:** the zero-crossing test asserted a field frame that contradicts the
  canonical math frame. Rewritten to assert the real invariant — matrix equivalence + canonical
  frame — rather than a field guess.
- **Test law misstated:** the pointer-zoom test applied pans *inside* the anchor-invariance law.
  Pan and zoom laws split into separate assertions (pan = screen-constant shift).
- **Test generator divergence:** the 1000-cycle test multiplied 1000 random scales, which
  overflows. Corrected to true apply→revert cycles (the stated CAN-TR-6 law).
- **Test magnitude:** normalization-rotation equivalence was tested at |deg| up to 1e9, beyond
  where radian precision supports 1e-9. Bounded to 1e6 with the reason written in the test.
- **Hygiene:** a 1-char typo in a hex seed literal (rejected by rustc) fixed to `0x5EED_0000`.

**Lesson recorded:** when a property test fails, first ask whether the *law as written* is
right, not only whether the implementation is. Three of the four failures here were
mis-stated laws, not code bugs.

## Verification (2026-10-11)

- `cargo test` (whole workspace): **48 Rust tests pass** — `gip-coords` 38 (25 unit + 12
  property suites + 1 doctest), others 1–2 each.
- `cargo clippy --all-targets -- -D warnings`: clean (including the wgpu dependency graph build).
- `cargo fmt --all -- --check`: clean.
- `scripts/check-boundaries.py`: PASS (9 crates).
- `cargo check -p gip-session --target wasm32-unknown-unknown`: clean.

Toolchain: Rust **1.99.0** stable (rustup reinstalled into `~/.cache`; not snapshotted —
reinstall each session, see memory §4/errors). CI remains the durable gate.

## Deferred (named owners)

- `zoomToFit` / `zoomToSelection` → geometry phase (needs bounds).
- Viewport DOM observation → shell measures, core consumes numbers (renderer/boundary phase).
- Momentum panning → off in v1 (CAN-CAM-4).
- Conversion micro-benchmarks → budget suite (canvas §26).

## Next steps

1. **Geometry phase (`gip-geometry`)** — bounds, path primitives, straight-segment identification
   (the RVS foundation), with golden fixtures. RVS color law still needs ADR-005 before the RVS
   feature itself.
2. **Document schema/serializer (`gip-document`)** — first use of serde per the registered
   exception; `document::Transform` mirrors `Trs` field-for-field with a drift test; golden
   round-trip fixtures from ADR-003.
3. Testing system step 6 (Playwright E2E) when the shell first imports the wasm bindings.
4. Owner review queue unchanged (stroke scaling, rotation display range, public spec, undo cap).
