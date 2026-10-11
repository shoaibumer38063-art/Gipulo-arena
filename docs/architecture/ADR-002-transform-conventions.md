# ADR-002 — Transform Representation, Matrix Conventions & Numeric Policy

```text
Decision:   Canonical per-node transforms are DECOMPOSED FIELDS, never stored matrices:
            x · y (parent space) · w · h (>= 0) · rotation (degrees) · flipH · flipV — all f64.
            Rotation is stored in degrees normalized to (-180, 180] everywhere (storage, UI,
            tests, files — one convention). Flips are boolean flags, never negative sizes.
            Matrices are derived, held in the browser-native 6-tuple order (a b c d e f /
            column-major), composed T·R·S with the node's top-left as rotation origin.
            No skew in v1. No external math crate for 2D. All epsilons live in ONE constants
            module in gip-coords.
Date:       2026-10-11 (authored)
Status:     PROPOSED — awaiting product-owner review. Binding as the implementation default
            (D7 pattern: [PROPOSED] values are binding until the owner revises). On acceptance,
            this ADR closes the open questions in transforms.md §7 and coordinate-system.md §8.1.
Refines:    transforms.md §7 (all five items) · coordinate-system.md §8.1 · CAN-TR-5/6 ·
            CAN-COORD-5 · CAN-NUM-2/5. Interacts with: ADR-003 (serialized transform fields),
            ADR-004 (history patches record canonical fields).
```

## Context

- Every spatial consumer depends on ONE answer to "what is a node's transform?": the coordinate
  module (conversion chains), the transform system (move/resize/rotate/flip), geometry (bounds,
  snapping, hit-testing), layout (frame resize handoff), history (inverse exactness), panels
  (numeric fields), snapping, a11y mirror, export, and the `.gip` format. Leaving the shape
  ambiguous is what produces duplicated math — the exact failure CAN-COORD-2 exists to prevent.
- Existing binding laws this ADR must satisfy: never round/quantize stored values (CAN-NUM-2);
  decomposition always from the canonical stored form, never raw matrix round-trips (CAN-TR-6);
  preview/commit equivalence and 1000×-cycle stability (transforms §6); exact undo identity
  (undo-redo §6); explicit-over-inferred fields at the format level (file-format §3).
- The workspace already fixes: world coordinates are f64 throughout the engine (ADR-001 boundary
  §3); bounds kinds are owned by geometry (6 kinds); node ids/handles are u64.
- Timing: this ADR blocks the coordinates phase, the transform solver, the `.gip` transform
  field shape (ADR-003), and history patch identity (ADR-004). It is authored before any of
  those subsystems exist, as intended.

## Precedent research (tagged)

| Source | Fact [tag] | Consequence for this ADR |
|---|---|---|
| CSS Transforms Level 1 / MDN `matrix()` | The 6 values a b c d e f are **column-major**; the 2D matrix is `[[a, c, e], [b, d, f]]`; W3C spec §14 states the 3×2→4×4 equivalence [PUBLIC, verified 2026-10-11] | Adopting the SAME 6-tuple order means zero re-ordering at every browser handoff: `ctx.setTransform(a,b,c,d,e,f)`, CSS `matrix(...)`, SVG `matrix(...)` all take it verbatim |
| SVG 1.1 transform attribute | `matrix(a b c d e f)` — same six values, same meaning [PUBLIC, well-known spec] | SVG import/export later is a straight copy, not a conversion risk |
| Canvas2D `setTransform` | Same 6-tuple order [PUBLIC, well-known API] | Overlay/fallback rendering paths need no adapter |
| Rust math crates (glam, euclid) | Public crates; glam is 3D/game-oriented with f32-first defaults; euclid encodes spaces in the type system [PUBLIC] | 2D affine here is ~10 functions on f64; a hand-rolled `Mat2` keeps the zero-dependency law and stays auditable. Revisit only if 3D arrives (its own ADR) |
| Figma internal matrix/rotation representation | **Not public — not claimed.** What is public product behavior: rotation is a first-class field in the properties panel; size is numeric W/H; flip is a visible state [PUBLIC product behavior] | The decision below is derived from Gipulo's own laws (field-exact history, numeric UI, explicit format), not from imitating unverifiable internals |

## The decision (sub-decisions; each changeable only by a superseding ADR)

| # | Sub-decision | Choice | Rationale |
|---|---|---|---|
| 1 | Stored transform | **Decomposed fields**: `x, y` (parent space, f64) · `w, h` (f64, ≥ 0) · `rotation` (degrees, f64, normalized) · `flipH, flipV` (bool). Matrices are derived caches only | Numeric fields are a product surface; field-identical values make history patches and format diffs exact; matrix decomposition is ambiguous under skew and lossy under repetition — the canonical form must be the input, not an output (CAN-TR-6) |
| 2 | Matrix layout | `Mat2 { a, b, c, d, tx, ty }` (f64), **column-vector convention**: `[[a, c, tx], [b, d, ty], [0, 0, 1]]`, point `p' = M · p` | Identical to the CSS/SVG/Canvas2D 6-tuple order (table above) ⇒ zero handoff bugs; the formula is documented ONCE (coordinate-system.md §4/§5) |
| 3 | Composition | `local = T(x, y) · R(θ) · S(sx, sy)`; `world = parentWorld · local`; `sx = flipH ? -w : w`, `sy = flipV ? -h : h` | Rotation-then-scale ⇒ W/H are pre-rotation extents along the node's OWN axes — what panels, resize handles, and DS affordances assume; ancestor accumulation is the single chain CAN-COORD-3 mandates |
| 4 | Rotation origin & pivots | The stored rotation is about the node's local origin = **top-left of its box** (CSS `transform-origin: 0 0` semantics). Gesture pivots (center default, opposite-corner anchoring) are computed by the ONE pivot calculator and expressed as adjusted `x, y` + `θ` | Minimal stored state; keeps the "one pivot calculator, no per-tool variants" law (transforms §2) valid for every pivot without storing pivot state |
| 5 | Angle convention | **Degrees**, f64, normalized at write to **(-180, 180]** via `((θ + 180) mod 360) − 180`, with −180 mapped to +180. UI displays the same range; radians appear only inside trig calls in `gip-coords` | One number in docs, UI, tests, and files; degrees are the DS-facing unit and diff-friendly in `.gip`; the normalization is exact for snapped values (0, ±90, 180 agree bit-for-bit after commit cleanup) |
| 6 | Flip storage | **Flags** `flipH/flipV`; the matrix derives the sign. Stored `w`, `h` are NEVER negative | Booleans are exact and serialization-trivial; negative sizes would poison layout, bounds, hit-testing, and every validators' invariants; confirms CAN-TR-5 [PROPOSED: retain flip flags] |
| 7 | Zero-crossing (resize past zero) | The flip toggles **during preview** at the instant the dragged size crosses 0; the committed values are the **absolute magnitudes** of that size with the corresponding flag updated (never a negative stored size). No hidden minimums; 0-size nodes are legal (render nothing, remain selectable by their outline/handles) | Preview/commit equivalence holds by construction (commit adds no correction beyond the exact-snap cleanup law); avoids unexplained magic constants (CQ) |
| 8 | Skew | **Not representable in v1.** A future skew feature requires its own ADR + a `.gip` migration | Keeps compose/decompose total and exact; no hidden matrix-only state that panels couldn't display |
| 9 | Math dependency | **None** — a hand-rolled `Mat2`/`Vec2` f64 module inside `gip-coords` | Zero-dependency law; 2D affine is ~10 audited functions; glam/euclid add API surface (and glam's f32-first defaults invite precision bugs). 3D's 4×4 lives with the 3D feature ADR |
| 10 | Epsilons & constants | ONE module in `gip-coords` (CAN-NUM-5): `EPS_ROUNDTRIP = 1e-9` world units (CAN-COORD invariant) · `EPS_ANGLE = 1e-9` degrees (equality after snapped commits) · `EPS_MATRIX = 1e-9` (max-abs entry difference for equality). No other epsilon may be introduced outside this module; display rounding is a shell formatting function only (CAN-NUM-2) | One auditable place for numerics; prevents agent-invented tolerances; test laws cite constants instead of literals |
| 11 | Serialized shape (ADR-003 cross-ref) | Per node: explicit `x, y, w, h, rotation, flipH, flipV` fields (written explicitly, never inferred) | Explicit-over-inferred at format level; stable diffs; agents reading files see exactly what the UI shows |
| 12 | 3D readiness | 2D math stays affine 2×3. Future 3D nodes carry their own 4×4 transform resolved by the 3D feature ADR; a 3D node contributes only a projected bounds rect to 2D systems | No speculative 4×4 now (CQ1.2); clean seam later |
| 13 | Bounds interplay | Authority stays with geometry (six `BoundsKind`s). Transforms supply canonical fields; rotation-aware operations use the full-matrix path; axis-aligned fast paths are allowed only where documented + tested (CAN-COORD-5) | This ADR fixes the INPUT shape to geometry, not the bounds model |

## Accepted costs & mitigations (honest record)

1. **Corner-drag on a rotated node is not expressible as a screen-axis delta.**
   Mitigation: the transform solver documents ONE rule — inverse-rotate the pointer delta into the
   node's local axes, then solve `w/h` there (rotation preserved). This is a fast matrix path,
   tested in the ops × rotation matrix; no per-tool variants.
2. **Hand-rolled math = we own its correctness.**
   Mitigation: property-based tests (canonical ⇄ matrix round-trips over sampled angles incl.
   exact snapped values; 1000× cycle stability; JSON round-trip of all seven fields) — all
   native `cargo test`, cheap and deterministic.
3. **Degrees in the hot path** (one `sin`/`cos` per compose).
   Mitigation: matrices materialize once per node per frame and are cached; the conversion
   happens in exactly one place; the budget suite measures conversion and compose costs.
4. **`(-180, 180]` display** means a user typing 270 sees −90.
   Mitigation: recommended as ONE convention everywhere anyway (zero display transforms = zero
   display bugs); the alternative (0–360 display formatting) is an owner review item below —
   storage would be unaffected either way.

## Consequences

1. **coordinate-system.md** §3 ("rotate 2D affine") gains the concrete `Mat2` type; §8.1 closes.
2. **transforms.md** §7.1–7.4 close as decided; §7.5 (stroke/effects scaling) becomes an owner
   review item with a recommended default — it stays a product decision, not engineering taste.
3. **ADR-003** serializes the seven explicit fields; tests get a golden table of angle
   normalization + flip/zero-crossing cases.
4. **ADR-004** history patches record canonical field values (e.g. `rotation` after cleanup), so
   undo restores exactly the pre-gesture numbers — no matrix inference anywhere in history.
5. **No scaffold changes required.** `gip-coords` gains `Mat2` + the constants module when the
   coordinates phase starts (next); `gip-document`'s transform field shape lands with the
   document phase against these decisions.
6. **DS note (no edit needed now):** nothing in the DS contradicts these conventions; when the
   transform features land, `05`/`06` panels show the same (-180, 180] range and W/H ≥ 0.

## Owner review items (product-visible — non-blocking, recommended defaults included)

1. **Plain resize vs style scaling:** recommended — plain resize NEVER scales strokes/effects/
   corner radii (it is an exact numeric W/H change; RVS works on geometry, unaffected by style
   scaling); an explicit **Scale tool** scales geometry + the documented style list. This will be
   written into DS `05`/`06` when those features start. (Alternative: Figma-style — I cannot
   verify Figma's exact behavior from public sources, so it is not used as an argument.)
2. **Rotation display range:** recommended `(-180, 180]` everywhere (one convention). Alternative:
   display 0–360 (formatting-only exception, storage unchanged) — owner's call.

## Open questions (deferred, tracked)

1. Skew support — future ADR + format migration (not in v1).
2. 3D transform composition (4×4) + projection handoff — 3D feature ADR (vision Solution 3).
3. Transform-during-layout interplay specifics (frame resize ⇒ reflow) — already CAN-TR-3; the
   solver detail lands with the layout phase tests.
4. Marquee → per-child transform decomposition for smart-select/scale gestures — CAN-TR-1
   covers the law; concrete decomposition helpers land with the transform implementation.

## Sources

- CSS Transforms Module Level 1 — `matrix()` six values a–f, column-major, 3×2→4×4 equivalence:
  w3.org/TR/css-transforms-1/ · MDN `transform-function/matrix()` (fetched 2026-10-11)
- SVG 1.1 coordinate systems & `matrix(a b c d e f)` — w3.org/TR/SVG11/coords.html [spec, well-known]
- Canvas2D `setTransform(a, b, c, d, e, f)` — HTML Standard / MDN [API, well-known]
- wgpu 30 / Naga conventions (matrices in shaders are the renderer's concern; no WGSL matrix is
  authored until the rendering phase) — github.com/gfx-rs/wgpu [PUBLIC]
- glam / euclid crate documentation — docs.rs (public, evaluated then declined)
