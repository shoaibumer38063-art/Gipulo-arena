# Task 020 — Editor Slice 0: plan and acceptance tests

- **Date:** 2026-10-11 · **Roadmap:** step 19 · **Status:** PLAN — awaiting owner review before code
- **Owner go-ahead:** given 2026-10-11 ("plan first, then build"). No editor code exists yet.
- **Rules read for this plan:** `rules/canvas.md` §0, §3, §4, §5, §7, §8, §11, §22, §25, §26, §28.
  **Not yet read (must be read before the implementing task):** `rules/canvas.md` §1–§2, §6, §9–§10,
  §12–§21, §23–§24, §27 (micro-details); `rules/interactions.md`; `rules/state-management.md`;
  `rules/ui.md`; `docs/architecture/input-system.md`, `selection-system.md`, `undo-redo.md`,
  `coordinate-system.md`, `engine-core.md`.

## 1. What exists today (verified from the code)

| Layer | State |
|---|---|
| Document model (`gip-document`) | Node kinds incl. Rectangle, Ellipse, VectorPath; pages; typed fields; validation; `.gip` save/load with hash + NFC |
| History (`gip-history`) | Transactions (`set_field`, `insert`, `remove`, `move_node`, `reorder`), undo/redo, dirty flag, one entry per `transact` call |
| Coordinates (`gip-coords`) | `Camera` (pan, zoom around point), `ViewportRect`, screen↔viewport↔world, Trs/Mat2 |
| Geometry (`gip-geometry`) | Paths, bounds, precise hit geometry (point-in-fill, stroke distance), `SegmentIndex` (BVH) |
| Scene / render (`gip-scene`, `gip-render`) | Vocabulary only (RenderLayer). **No draw list, no pixels** |
| Session / boundary (`gip-session`, `app/src/engine/client.ts`) | Protocol v0: version handshake only. No intents, events, or snapshots |
| Shell | Design-system gallery + engine status line. **No canvas** |

**Consequence:** Slice 0 must add (a) typed intents and snapshots at the boundary, (b) a draw list
from the scene layer, (c) the interaction state machine, and (d) a canvas host in the shell that
paints the draw list and forwards pointer events. Nothing in (a)–(d) exists yet.

## 2. Scope of Slice 0 — "a page you can draw rectangles on"

**In scope (user-visible):**
1. Infinite canvas with **pan** (space-drag / middle-drag / two-finger trackpad) and **zoom**
   (wheel and ⌘/Ctrl-wheel, pointer-anchored) — zoom limits 5 %–3200 % (CAN-CAM / CAN-SS-4).
2. **Rectangle tool** (`R`): press-drag-release creates a Rectangle node with the drag box as its
   transform; press-without-drag creates nothing.
3. **Selection**: click selects the top-most visible rectangle whose precise fill contains the point
   (CAN-SEL-1, CAN-GEOM-4). Click on empty canvas clears selection. Shift-click toggles (CAN-SEL-3).
   Escape clears selection.
4. **Move**: drag a selected rectangle. Gesture = one history entry (CAN-HIST-1). A 3 px threshold
   separates click from drag (CAN-INPUT-2, PROPOSED default).
5. **Cancel**: Escape or `pointercancel` during a drag restores the exact pre-gesture transform and
   creates no history entry (CAN-INPUT-4, CAN-HIST-2).
6. **Undo / redo** (`⌘/Ctrl-Z`, `⌘/Ctrl-Shift-Z`) for create and move.
7. **Selection visuals**: 1 px outline at 100 % opacity, hover outline at 60 % (CAN-SEL-6, DS tokens).
   **Resize/rotate handles are NOT in Slice 0** (see §6).

**Out of scope (explicit — each needs its own task or ADR):** resize, rotate, flip, vector edit,
text, marquee, multi-object transform, snapping and guides (CAN-SNAP / CAN-GUIDE), layers panel,
properties panel, grouping/frames, copy/paste/duplicate, file open/save UI, export, layouts (RVS).

## 3. Architecture placement (proposed)

```
pointer/key events (shell, DOM)
   │  raw events with viewport coords + modifiers         (shell: transport only)
   ▼
gip-session facade  ── typed intents DOWN, snapshot UP     (engine-core §3, protocol v1)
   │
   ├─ interaction state machine (ONE machine; CAN-MODE-1/2)  ← home: see decision D2
   │     states: idle · hovering · pressing · dragging-move · drawing-rect · panning
   ├─ hit-testing  (gip-geometry; precise geometry; screen-px tolerance converted at query time)
   ├─ transform   (gip-coords Trs; gesture-start values, CAN-TR-2)
   ├─ history     (gip-history transact: one gesture = one entry)
   └─ scene       (gip-scene): draw list for Document layer + Overlay layer (CAN-OVL)
   ▼
snapshot: { draw list, selection, camera, can_undo/redo, dirty }
   │
shell canvas host: paints the draw list; reports pointer events; NO geometry, NO conversions
```

**Coordinate rule (CAN-COORD-2):** the shell sends viewport-space pointer positions only. Every
conversion (viewport → world → parent → local) happens in `gip-coords`. The shell contains no `* zoom + pan`
arithmetic.

**Render path (decision D1):** the core emits a draw list; the shell paints it. Slice 0 uses a Canvas2D
painter in the shell as the bootstrap surface, which rendering.md §3 permits for debug/bootstrap
only. It has no geometry or conversion logic. The wgpu renderer stays the target for the next slice.

## 4. Acceptance criteria (testable, numbered)

**Coordinates & camera**
- AC-1 Round-trip `viewport → world → viewport` within 1e-9 at zooms 0.05, 0.25, 1, 4, 32 and at fractional and negative coordinates (CAN-COORD invariant).
- AC-2 Pointer-anchored zoom keeps the anchor point's world position fixed within 1e-9.
- AC-3 Pan and zoom never change the document (history length and document hash unchanged).
- AC-4 Camera clamps at 5 % and 3200 % zoom.

**Creation**
- AC-5 Press-drag-release with the rectangle tool creates exactly one Rectangle whose world box equals the drag box (with normalized negative-direction drags).
- AC-6 A press without drag (below 3 px) creates no node and no history entry.
- AC-7 A creation gesture is one history entry; undo removes the node; redo restores it with the same node ID.

**Selection & hit-testing**
- AC-8 Clicking inside a rectangle selects it; clicking outside selects nothing. Hit uses precise geometry (rotated rectangle: the corner outside the rotated shape is not hit).
- AC-9 Overlapping rectangles: the top-most (last in z-order) wins.
- AC-10 Inside a filled rectangle is a hit with no tolerance; for stroked hits the tolerance is a screen-space pixel value; at zoom 5 % and 3200 % the same screen tolerance applies (CAN-SS-2/4 tests).
- AC-11 Zero-size rectangles remain addressable (minimum screen tolerance, CAN-GEOM-5).
- AC-12 Shift-click toggles membership; Escape clears selection.

**Move & cancel**
- AC-13 Dragging a selected rectangle moves it by exactly the pointer delta in world units, at every tested zoom.
- AC-14 Move computes from gesture-start values; 1000 repeated move gestures produce no drift (`start + f(pointer)`, CAN-TR-2 and §25 "repeated-operation stability").
- AC-15 Escape or pointercancel during a move restores the exact pre-gesture transform (bitwise equal) and adds no history entry.
- AC-16 Pressing without the 3 px threshold does not move the rectangle.
- AC-17 While dragging, nothing transient enters history; only the final gesture commits (CAN-INPUT-5, CAN-HIST-2).

**History & persistence**
- AC-18 Undo/redo for create and move restores exact document state; `can_undo` / `can_redo` in the snapshot match history.
- AC-19 A document edited in Slice 0 saves and reloads with a valid hash, and the reloaded document renders identically (golden-style test).

**Boundary & architecture**
- AC-20 Shell modules contain no coordinate formulas (`* zoom`, `+ pan`, matrix math). Enforced by a grep-style lint in the test suite.
- AC-21 The interaction state machine is the only writer of mode; no boolean mode flags in the shell (CAN-MODE-2).
- AC-22 Protocol version bumps to 1 and the shell mirror is updated in the same change (engine-core §10).
- AC-23 `scripts/check-boundaries.py` passes; any new crate has a registry row and an ADR.

**Performance (CAN-BUDGET; reference documents, not an empty canvas — CAN-BUDGET-1)**
- AC-24 A committed fixture with 1,000 rectangles (the 10k fixture is a follow-up) meets: pointer→visual ≤ 2 frames p95 during drag; no frame > 32 ms p95 during pan/zoom. Measured headless and recorded with the fixture name.
- AC-25 Snap budget: N/A in Slice 0 (no snapping).

**Accessibility & visuals**
- AC-26 Selection and hover visuals use design tokens only; no hard-coded colors or sizes (rules/ui.md, rules/canvas.md §27).
- AC-27 Selection outline stays 1 px on screen at every zoom (CAN-SS-3).
- AC-28 Keyboard: `R` selects the rectangle tool, Escape cancels, Delete removes the selection (undoable, one entry). Keyboard operability is tested.

**Tests required by CAN-TEST §25 for touched areas:** Coordinates, Hit-testing, Selection, Transforms,
History, Camera, Serialization, Performance. Playwright E2E: create → select → move → cancel → undo →
save/reload round trip in a real browser.

## 5. Rule-compliance checklist (for the implementing task)

- [ ] CAN-ARCH-2: dependencies flow one way (tools → selection/hit-test → transform → history → scene → shell).
- [ ] CAN-COORD-2: no conversion formulas outside `gip-coords`.
- [ ] CAN-SS-2: hit areas stored in screen px, converted at query time.
- [ ] CAN-MODE-1/2: one state machine; shell reads, machine writes.
- [ ] CAN-HIST-1..4: gesture = one entry; cancel = no entry; no private undo.
- [ ] CAN-SEL-4: selection is UI state, not serialized, not on nodes.
- [ ] CAN-OVL: overlays are never in the Document layer (never exported).
- [ ] §28: regression list checked: selection geometry = rendered geometry.
- [ ] §27 micro-details for every touched control (read before implementing).

## 6. Deliberate deferrals (not violations)

- **Handles (CAN-SEL-6: 8 handles on selection):** handles belong to resize, which is the next slice.
  Slice 0 shows the 1 px outline only and says so in the task summary. No fake handles.
- **Snapping (CAN-SNAP) and guides:** the snap budget (§26) is measured when snapping lands.
- **Text, vector edit, layers, properties:** separate slices with their own ADRs (CAN-TEXT-4 keeps
  text-on-path proposal-stage).

## 7. Decisions needed from the owner (blocking the build)

| ID | Question | Recommendation |
|---|---|---|
| **D1** | Bootstrap painter: Canvas2D painter in the shell over a core draw list (fast to build, replaced by wgpu later) **or** wgpu from the start (no shell painter; harder; headless tests may lack WebGPU) | **Canvas2D bootstrap**, documented as temporary, replaced by the wgpu renderer in the next slice. Permitted by rendering.md §3 for bootstrap. |
| **D2** | Where the interaction state machine lives: inside `gip-session` (simplest) **or** a new crate `gip-interaction` (clearer boundary; needs a registry row and an ADR) | **New crate `gip-interaction`** with an ADR, so the state machine stays testable without the Wasm facade. |
| **D3** | Protocol: intents as typed Rust enums serialized through wasm-bindgen (JSON-free) **or** JSON strings across the boundary | **Typed wasm-bindgen methods** for hot paths (pointer move), with snapshot as a plain struct; no JSON on the hot path. |
| **D4** | Include Shift-click multi-select and Delete key in Slice 0 | **Yes** (both are small and covered by the matrix). |

## 8. Documentation updates the implementing task must make

- New ADR for D2 (and D3 if it changes the boundary contract, `engine-core.md` §3).
- `docs/architecture/canvas.md` (implementation map) — created for the first canvas slice.
- `docs/architecture/input-system.md` and `selection-system.md` — statuses from DRAFT to implementation.
- Feature doc for rectangle creation/selection/move; FEATURE_REGISTRY update.
- Bug record cross-references if any canvas bug is found (CAN-TEST-1).
- Roadmap step, changelog, beacon, task summary.

## 9. Test plan summary

| Area | Where |
|---|---|
| Coordinates & camera | Rust: `gip-coords` property tests (round trip, anchor invariance, clamps) |
| Hit-testing & tolerance | Rust: `gip-geometry` / `gip-interaction` matrix (inside/outside/tiny/zero/overlap/zoom) |
| Gestures & cancel | Rust: `gip-interaction` state machine scripts (press, drag, cancel, threshold) |
| History | Rust: `gip-history` gesture=1 entry; exact restore |
| Boundary | Rust: facade tests; shell lint for no-conversion rule (AC-20) |
| Visuals | Vitest: token-only styles; draw-list snapshot tests |
| End to end | Playwright: create → select → move → cancel → undo → save/reload |
| Performance | Rust bench + Playwright timing on the 1k fixture (recorded with fixture name) |

## 10. Estimated size

Slice 0 is the largest single slice so far (roughly the size of tasks 009–012 combined). It should be
split into three sub-tasks, each green before the next:
1. **020a** — boundary v1 + draw list + Canvas2D host (pan, zoom, empty canvas, AC-1..4, AC-20..22).
2. **020b** — rectangle create, select, move, cancel, undo (AC-5..19, AC-23).
3. **020c** — performance fixture, keyboard, E2E round trip (AC-24, AC-26..28).

## 11. Status

Awaiting owner answers to D1–D4. Build starts with 020a after approval.
