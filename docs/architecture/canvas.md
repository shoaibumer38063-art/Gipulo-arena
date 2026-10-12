# Canvas Architecture — THE HUB DOCUMENT

> Status: DESIGN v1 (2026-10-11). No code exists yet. Shaped by ADR-001 (ACCEPTED — Option B:
> Rust/Wasm core + TS/React shell; placement law in `engine-core.md`).
> Binding law: `rules/canvas.md` (CAN-*). This document maps the subsystems and their contracts;
> each subsystem has its own architecture doc (linked below). Rules define behavior law; these
> docs define structure — if they ever disagree, rules win and this doc is fixed.

## 1. Subsystem map (CAN-ARCH-1 realized as documents/modules)

| Subsystem | Architecture doc | Primary rule sections |
|---|---|---|
| Document model (`.gip` tree) | `document-tree.md` | CAN-SER, SM1 |
| Engine core (Rust/Wasm) & shell boundary | `engine-core.md` | ADR-001, SM3, CAN-MODE-1 |
| Geometry engine (vector/paths/bounds) | `vector-engine.md` | CAN-GEOM, CAN-VEC, VG* |
| Coordinate conversion | `coordinate-system.md` | CAN-COORD |
| Transform system | `transforms.md` | CAN-TR |
| Camera/viewport | (in `coordinate-system.md` §camera) | CAN-CAM |
| Rendering engine | `rendering.md` | CAN-RENDER, R* |
| Selection system | `selection-system.md` | CAN-SEL |
| Hit-testing | (in `selection-system.md` §hit) | CAN-SEL-1, CAN-GEOM-4 |
| Input system & state machine | `input-system.md` | CAN-MODE, CAN-INPUT |
| Tool state machines | (in `input-system.md` §tools) | CAN-MODE |
| Snapping engine | (in `input-system.md` §snapping interface; engine contract in `vector-engine.md` §consumers) | CAN-SNAP |
| Layout engine (Auto Layout/constraints/RVS) | `layout-engine.md` | L* |
| Text engine | (contract in `document-tree.md` §text + `vector-engine.md`; full doc when feature starts) | CAN-TEXT |
| Guides/overlay engine | (in `rendering.md` §overlays) | CAN-GUIDE, CAN-OVL |
| History (undo/redo) | `undo-redo.md` | CAN-HIST |
| File format & serialization | `file-format.md` | CAN-SER |
| Export renderer | (in `rendering.md` §export) | R1 |
| Accessibility mirror | (in `document-tree.md` §a11y consumers; full doc when implemented) | CAN-A11Y |
| Code generation / Code Workspace | `code-generation.md` | vision Solution 2 |
| Performance/invalidation | (in `rendering.md` §invalidation) | CAN-PERF, P* |

## 2. Dependency direction (CAN-ARCH-2, binding)

```
tools / interaction (input-system)
   │ uses
   ▼
selection · hit-testing · snapping ──► geometry engine ──► document model
   │                                        ▲                   ▲
   ▼                                        │                   │
camera/coordinates ─────────────────────────┘                   │
   │                                                            │
   ▼                                                            │
rendering (scene ← geometry+document) · overlays · export ──────┘
layout engine ──► document model (writes via transactions only)
history ──► document model (sole mutator path)
a11y mirror · UI panels ──► document/selection stores (read + transactional writes)
```

- Arrows = "may depend on". Reverse imports are violations. Rendered pixels never flow back
  (CAN-ARCH-3). Camera and document never write each other (CAN-ARCH-5).

## 3. Frame lifecycle (the pointer→pixel loop, CAN-RENDER-4 / R3)

1. **Event in** — normalized pointer/key/wheel event (input-system).
2. **Route** — single-owner state machine: current tool → current child state (CAN-MODE-1).
3. **Resolve** — coordinate conversion (screen→world via canonical module), hit/hover/snap
   queries against geometry engine (+spatial index).
4. **Mutate (or preview)** — committed changes go through transactions (history); live gesture
   values stay in transient state (SM domain 4) and produce *previews* only.
5. **Invalidate** — dirty regions/nodes marked; layout reflow if layout inputs changed
   (subtree-scoped, L5).
6. **Draw** — scene update → culling/LOD → batched draws (document layer, then screen-space
   overlay layer anchored via world→screen conversions).
7. **Sync side channels** — selection store → panels; a11y mirror surgical updates; overlays'
   non-visual state (e.g. pinned measures).

Budget: steps 1–6 within frame budget on reference documents (§26); heavy work (booleans, big
flattens, decode, codegen preview) is async/worker-side with cancellation and joins at step 5–6
when ready.

## 4. Cross-cutting contracts (all subsystems)

- **Single sources of truth:** geometry (geometry engine) · positions/converts (coordinate
  module) · document data (document model via transactions) · camera (camera store) ·
  selection (selection store) · mode (input state machine) · tokens (Design System).
- **Determinism:** every subsystem's output is a pure function of (document, camera, input) —
  no time/random/iteration-order dependence in document paths (CAN-SER-1).
- **Screen-space chrome:** all interactive editor visuals obey CAN-SS laws (1px strokes,
  screen-constant handles/hit areas, zoom-visibility thresholds).
- **Testing:** each subsystem doc lists its Test Focus; the canvas matrix (CAN-TEST §25) binds.

## 5. Open questions (ADR-gated, tracked)

1. ADR-001 technology stack — **DECIDED (ACCEPTED): Option B — Rust/Wasm engine core + TS/React
   shell; renderer = wgpu inside the core (WebGPU primary, WebGL2 fallback).** Boundary law:
   `engine-core.md`.
2. ✅ **PROPOSED — ADR-002 (2026-10-11):** transform storage & matrix conventions (decomposed
   canonical fields; column-major 6-tuple matrices; degrees (−180,180]; flip flags; zero-crossing
   policy). Closes transforms.md §7 items 1–4; one owner review item (stroke scaling under resize).
3. ✅ **PROPOSED — ADR-003 (2026-10-11):** `.gip` v0 container/IDs/floats/assets/versioning.
   Closes file-format.md §8 items 1–4; owner review item: public spec publication.
4. ✅ **PROPOSED — ADR-004 (2026-10-11):** history entry storage = structured op logs; stack
   policy; in-memory v1. Closes undo-redo.md §7 items 1–3.
5. Spatial index structure choice (R-tree vs grid vs hybrid) → ADR with vector-engine impl.
6. WebGL2 fallback parity tiers → ADR with renderer impl.
7. Text engine shaping strategy (HarfBuzz-wasm vs platform shaping vs custom) → ADR when text
   feature starts.
8. RVS redistribution semantics → ADR-005 before RVS implementation (layout L3-4) — includes the
   affordance color language reserved in DS `08` §8.
9. Panic/trap recovery policy → P0-class ADR before risky subsystems (ADR-001 accepted cost 4).
