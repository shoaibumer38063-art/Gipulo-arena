# CANVAS RULES — The Main Law of the Gipulo Canvas

> **Status: ACTIVE (v1, 2026-10-11).** This file is the main law for ANYTHING that touches the
> Gipulo canvas. No agent may modify canvas systems without reading the relevant sections first.
> Requirements this file must satisfy: `docs/memory/06_canvas_rules_requirements.md`.
> Concrete visual values come from the Design System: `docs/design-system/08_canvas_ui.md`
> (canvas chrome, guides, handles — incl. the closed color language §9) + `docs/design-system/02_tokens.md`
> (values); distilled record: `docs/memory/07_design_tokens_and_visual_specs.md` §12.

## 0. How to use this file (agent protocol)

**BEFORE any canvas work:** (1) read §1–§5 always; (2) read the section(s) for the subsystem you
touch; (3) read §27 (micro-details) and §28 (change protocol); (4) check `docs/bugs/` for related
history; (5) check `docs/architecture/canvas.md` (when written) for the current implementation map.

**Core purpose:** an agent must NEVER solve one canvas problem by creating another canvas problem
elsewhere. Canonical violations this file exists to prevent:
- fixing a corner-radius handle by creating a second coordinate-conversion system;
- fixing selection by creating selection geometry that no longer matches rendered geometry;
- changing zoom so handles/hover areas/cursors/snap thresholds grow or shrink with it;
- adding a new shape with its own private transform or hit-testing implementation.

**Source tags used below:**
- **[PUBLIC]** — publicly documented architecture of real tools (see §1 sources).
- **[OBS]** — directly observable product behavior of professional tools.
- **[DS]** — established by the Gipulo Design System reference (binding).
- **[REC]** — recommended Gipulo architecture (binding until an ADR changes it).
- **[PROPOSED]** — provisional value; must be confirmed by an ADR before implementation hardens.
- **[INF]** — engineering inference; never presented as fact about other products.

---

## 1. Research basis & source classification

**Publicly documented (Figma):**
- Renderer written in **C++ compiled to WebAssembly (Emscripten)** for the browser; the same C++
  also compiles **natively (x64/arm64) for server-side rendering, testing, debugging**.
  A **graphics-interface abstraction layer** in C++ has separate WebGL and WebGPU implementations;
  WebGPU shipped Sep 2025 with **dynamic fallback to WebGL**, explicit draw-call arguments,
  a custom GLSL→WGSL shader processor, batched uniform uploads. (figma.com/blog/figma-rendering-powered-by-webgpu)
- Earlier: asm.js → **WebAssembly (2017) cut load time 3×** regardless of document size. Wasm is
  32-bit addressed (4 GB max, grow-only memory). (figma.com/blog/webassembly-cut-figmas-load-time-by-3x)
- App UI around the engine: **React + TypeScript**. Multiplayer: Rust servers, WebSockets, Kiwi
  binary format, S3 checkpoints + write-ahead log. [PUBLIC]
- Accessibility: **Mirror DOM** — an internal accessibility tree caching per-layer accessible
  summaries with surgical updates; a hidden React DOM mirroring the scene graph, positioned with
  **affine transforms** (needed by screen magnifiers/voice control); **bidirectional sync between
  canvas selection and keyboard focus**; ARIA live-region announcements that **coalesce rapid
  events** (e.g. arrow nudges). (figma.com/blog/building-accessibility-into-a-canvas-based-product)

**Publicly documented (tldraw):** camera = {x, y, zoom}; distinct **screen / viewport / page**
coordinate spaces with canonical `screenToPage()` / `pageToScreen()` conversions; viewport culling;
tools implemented as a **hierarchical state machine** (root → active tool → active child state,
e.g. idle → pointing-handle → resizing); exactly **one active tool owns input routing**. (tldraw.dev docs)

**Publicly documented (Penpot):** SVG-native document model; real CSS Flexbox/Grid layout
semantics; new renderer built on **Rust + WebAssembly + Skia (WebGL)** to support effects like
background blur; exporter uses headless Chromium. [PUBLIC]

**Gipulo's stance [REC]:** absorb the principles (engine/UI split, abstraction layer, state
machines, canonical conversions, mirror-DOM accessibility), **do not blindly copy implementations**.
Gipulo does NOT commit to C++ now: the architecture must keep a **renderer/engine boundary** that
would ALLOW a Wasm core later without redesign (see §21). Every [PUBLIC] item above is context,
not a mandate.

---

## 2. Subsystem map & boundaries (CAN-ARCH)

- **CAN-ARCH-1 [REC]** The canvas is split into these subsystems, each with one owning module and
  an explicit public interface: Document model · Scene/layer tree · Geometry engine · Rendering
  engine · Camera/viewport · Coordinate conversion · Transform system · Selection system ·
  Hit-testing · Input system (state machine) · Tool state machines · Snapping engine ·
  Guides/overlay engine · Layout engine · Text engine · Effects/compositing · Asset/media system ·
  History (undo/redo) · Export renderer · Accessibility layer · Performance/caching/invalidation.
- **CAN-ARCH-2** Dependencies flow one way: tools/interaction → selection/hit-testing/snapping →
  geometry engine → document model; rendering consumes geometry + document, never the reverse.
  **NEVER:** a subsystem reaching into another's internals; circular imports between subsystems.
- **CAN-ARCH-3 [REC]** **Rendered pixels are never a source of truth.** Geometry, bounds,
  hit-testing, snapping, export, and vector editing all read from the geometry engine/document
  model. Pixel readback is allowed only for user-facing pickers (e.g. eyedropper) — never for
  layout, selection, or serialization logic.
- **CAN-ARCH-4** Any new canvas feature must declare (in its feature doc) which subsystems it
  touches. A feature that "needs" a private copy of an existing subsystem's job is a design bug:
  extend the owner instead.
- **CAN-ARCH-5 [REC]** Camera state, document state, UI/editor state, and transient interaction
  state are four separate stores (see `rules/state-management.md`). Panning/zooming never mutates
  the document; document edits never mutate the camera.
- **TEST after boundary changes:** import/dependency lint; round-trip render-vs-geometry parity (§25).

## 3. Coordinate systems (CAN-COORD) — EXTREMELY IMPORTANT

- **CAN-COORD-1 [REC]** Gipulo's coordinate spaces, canonically named (use these exact names in
  code and docs):
  - `screen` — browser/device-independent pixels from the viewport origin (clientX/Y world).
  - `viewport` — screen pixels relative to the canvas viewport element's top-left.
  - `world` (= canvas/page space) — the infinite canvas coordinate system of a page.
  - `parent` — a node's coordinate space relative to its parent's transform.
  - `local` — object-local space (origin at object's own untransformed top-left/anchor).
  - `path` — vector/path-local space (pre-transform path coordinates).
  - `text` — text-layout-local space (shaping/run coordinates) where required.
  - `device` — physical pixels (`viewport × devicePixelRatio`), used only inside rendering.
- **CAN-COORD-2 [REC]** ALL conversions happen in ONE authoritative module (the coordinate system,
  part of camera/viewport). Canonical API shape (mirrors tldraw's public pattern [PUBLIC]):
  `screenToWorld`, `worldToScreen`, `worldToParent`, `parentToWorld`, `worldToLocal`,
  `localToWorld`, plus matrix helpers. **NEVER:** conversion formulas (manual `* zoom + pan`
  arithmetic) anywhere else in the codebase — a second conversion implementation is a critical
  violation even if "just for one feature."
- **CAN-COORD-3** Conversions must account for, in this order: devicePixelRatio (device only) →
  camera (pan, zoom) → accumulated ancestor transforms (translation, rotation, scale, flip) →
  object transform. Nested transforms resolve by matrix accumulation from root down; no feature
  may re-derive a child's world position by partial math.
- **CAN-COORD-4 [DS/OBS]** Overlays (selection, handles, guides) are positioned by converting
  their anchor geometry to `screen` space each frame; they never store baked screen positions.
- **CAN-COORD-5 [REC]** Rotation/scale-aware conversions must use the full affine matrix (no
  axis-separated shortcuts) except in explicitly documented fast paths that are tested against
  the matrix path.
- **INVARIANT:** for any point p and any space pair (A,B): `B→A(A→B(p)) ≈ p` within 1e-9 world
  units. **TEST:** property-based round-trip tests across zoom levels 0.05–32, rotations, flips,
  nested parents, fractional coordinates, negative coordinates.

## 4. Screen-space vs canvas-space (CAN-SS)

- **CAN-SS-1 [DS]** Screen-pixel-invariant UI (constant visual size at every zoom, positions
  attached to geometry): selection outlines (1px) · bounding boxes · resize handles (7–8px visual,
  **16px hit area**) · rotation zones · corner-radius handles · Bézier handles/anchor points ·
  midpoint/add-point controls · snapping thresholds · guide lines (1px) · guide labels/pills
  (10/16 text) · measurement text · cursor activation zones · hover/selection tolerances.
- **CAN-SS-2** Four distinct concepts must never be conflated in code: **canvas-space position ·
  screen-space visual size · screen-space hit area · zoom-scaled geometry.** Hit areas are defined
  in screen px and converted into world space at query time (`hitPx / zoom`) — never stored in
  world units.
- **CAN-SS-3 [DS]** Zoom-visibility laws: guide **labels hide below 25% zoom; handles hide below
  12% zoom**; strokes/lines stay 1px on screen at every zoom; pixel-crisp rendering uses half-pixel
  offsets for 1px lines.
- **CAN-SS-4 [OBS/REC]** A control intended to be ~8–10 screen px must have the SAME screen size
  and hit area at 5% and 3200% zoom. Its world-space footprint therefore changes with zoom —
  that is correct; world-constant hit areas are forbidden for editor chrome.
- **TEST:** interaction tests at zoom = 5%, 25%, 100%, 400%, 3200% asserting handle screen sizes,
  hit behavior, label visibility.

## 5. Geometry truth, bounds & synchronization (CAN-GEOM)

- **CAN-GEOM-1 [REC]** One geometry engine owns: path storage, canonical bounds computation,
  flattening, hit-geometry, snapping geometry. Rendering, selection, hit-testing, snapping,
  export, and vector editing all consume it. Separate approximations that could drift are
  forbidden; cached derivations must invalidate with their source.
- **CAN-GEOM-2 [REC]** Bounds vocabulary (every consumer must name which it uses):
  - `geometricBounds` — fill/path geometry, no stroke, no effects (transform bounds source).
  - `strokeBounds` — geometry + stroke extent per stroke alignment.
  - `visualBounds` — geometry + strokes + effects (blur/shadow spread).
  - `selectionBounds` — what the selection outline/handles show = `geometricBounds` [PROPOSED —
    ADR needed: whether strokes are included; Figma-observable default excludes outside strokes
    from the transform box [OBS]].
  - `transformBounds` — what resize/rotate operate on (== selectionBounds unless an ADR says more).
  - `hitBounds` — precise geometry used for pointer hit-testing (paths, not boxes).
  Each is available in `world`, `parent`, and axis-aligned/oriented forms as needed.
- **CAN-GEOM-3 [REC]** Effects (blur, shadow, glow) extend `visualBounds` ONLY. They must never
  change `transformBounds`, layout, or the document's spatial indexing — unless an intentional,
  ADR-recorded interaction rule says otherwise. An enormous blur must not make an object
  accidentally selectable hundreds of pixels away.
- **CAN-GEOM-4 [REC]** Hit-testing uses **precise path geometry** (fill rule for fills, stroked
  path for strokes). Clicking empty space inside the bounding box of a complex polygon/cutout/
  open path must NOT select it — except where a specific tool rule requires bbox behavior
  (documented per tool; e.g. marquee uses bounds intersection, not point containment).
- **CAN-GEOM-5** Zero-size and tiny objects remain addressable: hit-testing falls back to a
  minimum screen-space tolerance [DS: tiny <8px objects render outline + center dot; hit area
  never below the 16px-handle-equivalent tolerance for points] so they stay selectable at any zoom.
- **CAN-GEOM-6 [REC]** Geometry/rendering parity is testable: for golden documents, rendered
  outline snapshots and geometry-engine bounds must agree within tolerance. Drift = P1 bug.
- **TEST:** bounds-type unit tests per shape class (rect, rounded rect, ellipse, open path,
  compound, boolean, text, masked, effects, rotated, nested, flipped); hit-test matrix
  (inside/outside/on-stroke/in-cutout); parity snapshots.

## 6. Vector geometry on canvas (CAN-VEC)

Full law: `rules/vector-geometry.md`. Canvas-specific bindings:

- **CAN-VEC-1 [REC]** All Bézier math (nearest point, flattening, bbox, splitting, offsetting,
  curvature) uses the canonical geometry module with fixed tolerances — features never
  re-approximate Béziers privately.
- **CAN-VEC-2 [DS]** Vector edit visuals: path stroke 1.5px, handle lines 0.75px, anchors = 7px
  hollow squares (white fill, blue stroke), hover = 9px light fill `#b3dfff`, selected = solid,
  Bézier handles = r3 circles; close-path ring r7; marquee dashed `3 2` + 8% fill; lasso dashed
  6% fill; region hover 12% fill; pen rubber-band dashed 50%.
- **CAN-VEC-3 [REC]** Numerical stability: editing operations mutate stored control points
  directly; no repeated flatten→rebuild cycles on live geometry; repeated transform operations
  must not accumulate error (see CAN-TR-6).
- **CAN-VEC-4 [OBS/REC]** Add-point on segment hover shows a ghost point; deleting a point
  preserves path continuity per standard behavior (segment rejoins); open/closed path state is
  explicit in the data model, never inferred from coincident endpoints alone.

## 7. Modes & input state machine (CAN-MODE, CAN-INPUT)

- **CAN-MODE-1 [REC/PUBLIC-pattern]** Canvas input is ONE hierarchical state machine (pattern
  public in tldraw): root → active tool → active child state. Exactly one state owns the pointer
  at any time. Known states include: `idle · hovering · pressing · dragging · moving · resizing ·
  rotating · panning · zooming · drawing · marquee · vector-edit(point/handle/segment) ·
  text-edit · creating-guide · snapping-active · duplicating · transforming-multi`.
- **CAN-MODE-2** **NEVER:** scattered booleans (`isEditing`, `dragging2`, …) independently
  deciding modes. Mode = the state machine's current node; any code may READ it, only the machine
  may WRITE it.
- **CAN-MODE-3 [OBS/REC]** Mode transitions are explicit and documented: vector-edit entered by
  double-click or Enter on a selected vector/point; exited by Escape (first to selection, second
  to idle), clicking outside, or switching tools. Text-edit entered by double-click/Enter on text
  or typing with a text layer selected. Escape ALWAYS cancels the innermost active operation
  first (drag → mode → selection) and never destroys committed document state.
- **CAN-INPUT-1 [REC]** Event coverage: pointerdown/move/up/cancel, click, dblclick, wheel,
  trackpad pinch (ctrl+wheel), key events, focus/blur of window, pointerleave of viewport, touch,
  stylus where supported. Pointer capture is taken on drag start and released on end/cancel.
- **CAN-INPUT-2 [PROPOSED]** Click→drag threshold: **3 screen px** of movement before a press
  becomes a drag (below threshold, release = click). ADR to confirm.
- **CAN-INPUT-3 [REC]** Modifier keys pressed/released DURING a drag apply immediately (constrain,
  center, duplicate, snap-off per §27 table) — the drag operation re-evaluates modifiers on every
  move event; it must not require restart.
- **CAN-INPUT-4** Cancellation semantics: Escape, `pointercancel`, window blur, and pointer
  capture loss all route through ONE cancel path that restores pre-gesture state and creates NO
  history entry.
- **CAN-INPUT-5 [OBS]** While a pointer is held, transient affordances (add-point ghosts, snap
  previews, ghost copies) remain live; on release they vanish immediately — nothing transient
  survives the gesture except committed mutations.

## 8. Selection (CAN-SEL)

- **CAN-SEL-1 [OBS/REC]** Click selection priority (top wins): (1) an active overlay control
  under the cursor (handle, gradient stop, guide) — only when its owner object/mode is active;
  (2) in vector-edit mode: anchor points → segments → handles (screen-space tolerance);
  (3) top-most visible object whose **precise hit geometry** contains the point; (4) nothing.
  Locked layers are never selected by canvas click; hidden layers are never hit; selecting them
  happens via layers panel only.
- **CAN-SEL-2 [OBS]** Click = select top-level child of the current container context (page or
  frame/group being edited); **double-click** descends one level into groups/frames; **⌘/Ctrl-click
  (deep select)** selects the exact clicked leaf and shows parent context dashed 40% [DS].
- **CAN-SEL-3 [OBS]** Shift-click toggles additive multi-selection. Marquee selects top-level
  objects whose bounds **intersect** the marquee (touch-select) [OBS Figma default; ADR to confirm
  vs contain-select]. Marquee never selects locked/hidden; never descends into frames unless the
  marquee starts inside that frame [OBS].
- **CAN-SEL-4 [REC]** Selection is UI state: an ordered set of node IDs + container context, owned
  by the selection system — NOT stored on nodes, NOT serialized into `.gip` (except where the file
  format intentionally records last-selection per view — ADR-gated).
- **CAN-SEL-5** Selection preservation: entering vector-edit keeps the shape selected (edit
  target = that shape); exiting restores prior selection; switching tools preserves selection
  unless the tool requires otherwise (documented per tool).
- **CAN-SEL-6 [DS]** Visual states: hover outline 60% 1px · selected 100% + 8 handles + size
  pill · multi-select children 60% + group box 100% · parent-context dashed 40% `3 2` · locked
  dashed `2 2` 60% + lock badge · component/instance purple `--comp` · clipped child outline
  continues 50% dashed past clip.
- **CAN-SEL-7 [REC]** Mixed-type selection is legal; property panel and transform operations must
  handle it (see `rules/ui.md`); bounds = union of member `transformBounds`.

## 9. Hover (CAN-HOVER)

- **CAN-HOVER-1** Hover resolution uses the SAME priority order and geometry as click selection
  (CAN-SEL-1/2) so hover always predicts click. Divergence between hover highlight and click
  result is a bug.
- **CAN-HOVER-2 [REC]** Competing activation zones (resize corner vs rotate zone vs radius handle
  vs anchor point) resolve by fixed priority: active-mode controls > corner handles > edge handles
  > rotate zones > radius handles > geometry. No flicker: one winner per pointer position.
- **CAN-HOVER-3 [PROPOSED]** Hysteresis: once a zone is hovered, it releases only after the
  pointer exits by ≥2 screen px, preventing 1px state oscillation. ADR to confirm.
- **CAN-HOVER-4 [DS]** Hover visuals: object outline 60% · point hover 9px light fill · segment
  hover thicken + add-point ghost · handle hover per DS · ruler guide hover 9px band 15% ·
  spacing label hover ring r6 40%.

## 10. Cursors (CAN-CUR)

- **CAN-CUR-1 [REC]** One cursor engine maps (tool, mode, hovered zone, modifiers, drag state) →
  cursor. Components never set `document.body.style.cursor` ad hoc. Cursor must represent the
  action that WILL happen **before pointer-down** [OBS law].
- **CAN-CUR-2 [OBS]** Rotation awareness: resize cursors on rotated objects follow the object's
  actual axis directions (computed from the transform, quantized to the 8-direction cursor set in
  object space), never the fixed screen-axis cursors.
- **CAN-CUR-3 [DS]** Activation zones are designed separately from visible handle size (7px
  visual / 16px hit). Cursor changes at zone entry, not visual-handle entry.
- **CAN-CUR-4** Drag-state cursors: grab/grabbing for pan, move-cursor while dragging objects,
  forbidden cursor for illegal drop targets, drag-copy cursor while duplicating (modifier held).
  Full cursor inventory & visuals: `rules/ui.md` + DS §cursor system (to be formalized as custom
  cursor assets — until then, closest standard cursors + documented mapping table).
- **CAN-CUR-5** Cursors must remain legible on light and dark canvas (white halo/dark outline per
  DS casing rules); custom cursor assets are [PROPOSED] and need DS sign-off.

## 11. Transform operations (CAN-TR)

- **CAN-TR-1 [REC]** Move/resize/rotate/flip all go through the ONE transform system operating on
  node transform data in `parent` space; multi-selection transforms apply in the selection's
  common space with correct per-child decomposition (rotation of the group rotates children's
  positions AND orientations).
- **CAN-TR-2 [OBS]** Gesture math: compute deltas from **gesture-start values**, not accumulated
  frame-to-frame results. The pre-gesture transform is captured once; each move applies
  `start + f(currentPointer)`. This prevents drift and makes cancel/restore trivial.
- **CAN-TR-3** Resizing: proportional with Shift (aspect locked from gesture start), from-center
  with ⌥/Alt, corner vs edge semantics per handle; frames resize children per constraints/Auto
  Layout (layout engine owns child reflow — transform system must not hand-move laid-out children).
- **CAN-TR-4 [OBS]** Rotation: default pivot = object center; rotate handle outside corners [DS:
  arc zone r≈14 dashed]; Shift constrains to **15° increments** [DS: angle tick marks every 15°];
  displayed angle normalizes (e.g. −180…180 or 0…360 — pick ONE convention, ADR-gated, used
  everywhere including serialization display).
- **CAN-TR-5** Flips: horizontal/vertical flip are scale(−1,1)/scale(1,−1) applied about the
  object center; the internal model must define whether negative scale is decomposed immediately
  into rotation+positive-scale or retained [PROPOSED: retain flip flags; ADR needed].
- **CAN-TR-6 [REC]** Numerical stability: repeated transform/decompose/recompose must not corrupt
  geometry — decompose from the stored canonical form (position, size, rotation, flip) rather than
  from a raw matrix wherever possible; snap-to-intent (e.g. exact 90° after a 15°-constrained
  rotate) is applied at gesture END, never mid-gesture rounding of stored values.
- **CAN-TR-7** Children of transformed parents never store baked parent transforms; nesting is
  resolved by the coordinate system at read time.

## 12. Numbers: negatives, fractions, subpixels (CAN-NUM)

- **CAN-NUM-1** Gipulo must NOT assume positive integers: negative X/Y, negative rotation,
  fractional coordinates/sizes/strokes/gaps/padding (1.1px, 1.5px…) are all legal document values.
  Validation may warn, never silently clamp or round stored geometry.
- **CAN-NUM-2 [REC]** Internal precision: IEEE-754 double (float64 in a Wasm core if/when it
  exists). **Displayed precision ≠ internal precision**: property fields may display rounded
  values (2 dp per DS measure pills: values 0–2 decimals), but committing a displayed value only
  overwrites what the user actually edited.
- **CAN-NUM-3 [REC]** Subpixel rendering: geometry renders at fractional positions (no forced
  pixel snapping of document content); EDITOR CHROME (guides, outlines) uses half-pixel alignment
  for 1px crispness [DS]. Pixel-grid snapping of CONTENT is an explicit user feature (snap toggle),
  never automatic.
- **CAN-NUM-4** devicePixelRatio affects only `device`-space rendering (line widths, texture
  scale); it never changes world geometry or hit math in world units.
- **CAN-NUM-5 [PROPOSED]** Epsilon policy: world-space comparisons use ε = 1e-6; screen-space use
  0.5px; path coincidence tolerance 1e-4 world units. One shared constants module; ADR to confirm.

## 13. Camera: zoom, pan, scroll (CAN-CAM)

- **CAN-CAM-1 [REC/PUBLIC-pattern]** Camera = `{x, y, zoom}` in one store; conversions only via
  CAN-COORD-2 module. Camera never modifies document geometry; document edits never move camera
  (except explicit commands: zoom-to-fit/selection).
- **CAN-CAM-2 [OBS]** Pointer-centered zoom invariant: the world point under the cursor stays
  under the cursor (within 0.5 screen px) through the whole zoom gesture — wheel, pinch, and
  keyboard zoom (keyboard zooms around viewport center or selection center — documented per
  command). No drift across repeated gestures.
- **CAN-CAM-3 [PROPOSED]** Zoom range: clamp **2%–4096%**; the system must be verified working
  across 5%–3200% (the range named in the user's requirements doc). Zoom steps: multiplicative
  (≈1.1×/notch) for wheel/pinch; preset stops (2, 5, 10, 25, 50, 75, 100, 150, 200, 400, 800,
  1600, 3200%) for UI zoom menu. ADR to confirm.
- **CAN-CAM-4 [OBS]** Pan: spacebar-hold (temporary hand; restores previous tool on release) ·
  hand tool · middle-mouse drag · trackpad two-finger scroll = pan · pinch = zoom. Momentum/
  inertia optional [PROPOSED: off in v1 for determinism; ADR].
- **CAN-CAM-5** High-DPI/resize: viewport changes recompute screen-space derivations immediately;
  nothing caches screen positions across resizes.
- **TEST:** zoom-invariance tests (point-under-cursor), pan/zoom-does-not-dirty-document tests,
  min/max clamp, rapid wheel + pinch concurrency.

## 14. Snapping (CAN-SNAP)

- **CAN-SNAP-1 [DS]** Snap activation threshold: **within 4 screen px while dragging**; snap line
  = 1px orange dashed `4 3` + × markers at snap points; approach preview = 40% dotted `2 3`;
  release fade = 120ms; ⌘/Ctrl held = snapping disabled (guides shown at 20% + "⌘ held" chip).
- **CAN-SNAP-2 [REC]** Snap targets: object edges · centers · corners · anchor points · vector
  paths (nearest point) · frame edges/centers · guides · ruler guides · grid intersections ·
  layout-grid lines · equal-spacing/distribution candidates · text baselines [where supported].
  All candidates derive from canonical bounds/geometry (CAN-GEOM-2) — no feature-private snap
  geometry.
- **CAN-SNAP-3 [REC]** Priority when competing: guides/rulers > equal-spacing > edges/centers of
  nearby objects > grid. One winning snap per axis; ties broken by screen distance then priority.
- **CAN-SNAP-4 [PROPOSED]** Hysteresis: once snapped, release requires moving ≥1.5× the activation
  threshold away (≈6px) so snapping doesn't stutter at the boundary. ADR to confirm.
- **CAN-SNAP-5 [DS]** Multi-object drag: the **group bounding box** snaps (group box shown dashed
  `3 2` 60%); child-edge snapping is an explicit case [DS: "line to a child, not the group"].
- **CAN-SNAP-6** Snapping applies during move, resize (edge being dragged snaps), and creation
  (drawing shapes). Vector points snap with the same engine at point level.
- **CAN-SNAP-7** Snap results must be exact: when snapped, the committed value equals the target
  value (no 0.33px residue). Pixel-grid snap [DS: cell highlight 35% + crosshair] applies only
  when enabled and zoom ≥ threshold [PROPOSED: ≥200% for pixel grid; ADR].

## 15. Guides & overlays (CAN-GUIDE, CAN-OVL)

- **CAN-GUIDE-1 [DS]** Guide color language is FIXED and closed:
  **blue `--g-geo` = geometry/selection/paths · magenta `--g-space` = spacing/padding/
  distribution · orange `--g-snap` = snapping & measurement · red `--g-ruler` = user ruler guides
  & layout grids (red is NEVER an error color on canvas) · purple `--comp` = components/instances.**
  Adding a new guide meaning requires updating the Design System + this file (ADR-level change).
  Features must NOT invent new guide colors for existing meanings.
- **CAN-GUIDE-2 [DS]** Measurement system: measure = hold ⌥ (selected→hovered distances, 1px
  solid orange + 6px end ticks + pill 10/16 r3 min-width 18px) · nested measure ⌘⌥ · pinned
  measure ⇧M (hollow pill until Esc) · nudge ⌥⇧arrows shows live value · **measures bounds, not
  strokes** · multi-selection uses group bbox · values 0–2 decimals · tight gaps (<16px) lift the
  label with a leader · zero distance = dot + lifted "0" · label collisions stagger above/below ·
  zoom <25% = lines only, no labels.
- **CAN-OVL-1 [REC]** ALL editor overlays (selection, handles, guides, measurements, collaborator
  cursors, hover, tool previews, snap lines, text caret chrome) live in a dedicated overlay layer,
  rendered in screen space anchored via CAN-COORD conversions. Overlays: **never export · never
  become selectable document objects · never modify layout · never change document bounds · never
  serialize into `.gip`** (unless the file format intentionally stores a user-created object like
  a ruler guide — ruler guides ARE document objects; transient chrome is not).
- **CAN-OVL-2** Only the guides/overlay engine draws overlays; tools submit overlay *descriptors*
  (typed, tokenized) — never raw draw calls. This keeps visuals consistent and cullable.
- **CAN-OVL-3 [DS]** Ruler guides: red 1px, hit band 9px, hover 15% band + resize cursor,
  selected 2px + position pill, locked dashed `4 2` + badge, toggle ⌘; (hidden = no hit area);
  dragging from ruler shows live position pill. Layout grids: columns 12% fill / square 25%
  opacity, frame-scoped.

## 16. Alignment, grids, rulers (CAN-ALIGN, CAN-GRID, CAN-RULER)

- **CAN-ALIGN-1 [REC]** Align/distribute commands operate on `selectionBounds` (CAN-GEOM-2) of
  each target in the common parent space — ONE shared bounds accessor; **no command may invent its
  own bounds logic.** Rotated objects align by their oriented bounds' extents in parent space
  (documented behavior; ADR if a dispute arises — default: axis-aligned extent of the rotated
  shape [OBS Figma]).
- **CAN-ALIGN-2** Distribution = equal gaps between bounds; equal-spacing guides (magenta) reflect
  the same computation — visuals and math share one source.
- **CAN-GRID-1** **Visual grids are not layout systems.** Canvas/page grids and frame layout grids
  are rendering + snapping aids; only the layout engine (Auto Layout etc.) positions content.
  A visual grid must never silently constrain children.
- **CAN-RULER-1 [REC]** Rulers render from camera + canonical coordinates; they always agree with
  world coordinates (zero/origin, negative values, zoom labels). Guide creation = drag from ruler;
  guides are document objects (persisted, per-page/frame context documented in feature doc).

## 17. Object model contracts (CAN-SHAPE / FRAME / COMP / MASK / BOOL / PARAM)

- **CAN-SHAPE-1 [REC]** Basic shapes (rect, rounded rect, ellipse, line, arrow, polygon, star)
  are **parametric and non-destructive** while unedited; each shape type implements ONE interface
  (`getGeometry()`, `getBounds(kind)`, `hitTest()`) served to the geometry engine — **a new shape
  type must NOT bring its own transform, selection, or hit-testing implementation.**
- **CAN-SHAPE-2** Conversion to editable vector geometry is explicit (user action), one-way, and
  recorded in history; after conversion the node is a vector path like any other.
- **CAN-FRAME-1 [REC]** Frames and groups are DIFFERENT contracts: **Frame** = own coordinate
  space for children, clipping, constraints, optional Auto Layout, independent background, can be
  resized (reflows children per constraints/layout). **Group** = no clipping, no own layout,
  bounds derived from children (recomputed on child change), transforms distribute to children.
  Code must not treat them interchangeably; shared behavior lives in a common container interface.
- **CAN-FRAME-2 [DS]** Visual contracts: frame shows name tab (hover chip 15%, rename = white
  input); group shows box only, no name; clipped child outline continues 50% dashed outside clip.
- **CAN-COMP-1** Components/instances: canvas must visually communicate edit context (purple
  chrome for component/instance [DS]); instance editing enters the instance's override scope;
  vector-editing an instance's child creates overrides per the component system's rules (feature
  doc authoritative); detach is explicit. Component sets: purple dashed `4 2` + name tab [DS].
- **CAN-MASK-1 [REC]** Clipping/masking affects rendering AND hit-testing (clipped-away regions
  are not hittable [OBS Figma]) but NOT document existence — masked content remains fully in the
  document, editable, serializable. Mask outlines render dashed [DS `4 2`].
- **CAN-BOOL-1** Boolean ops (union/subtract/intersect/exclude) are **non-destructive by default**:
  operands persist, result geometry is cached and invalidated on operand change. Flatten =
  explicit user command only. Winding/fill rules stored explicitly in the path model.
- **CAN-PARAM-1** Parametric/procedural objects: parameters are the source of truth; generated
  geometry is a cache (deterministic regeneration); selection/bounds/hit/snapping all consume the
  generated geometry via the standard interfaces; conversion to plain vectors is explicit.

## 18. Text on canvas (CAN-TEXT)

- **CAN-TEXT-1 [REC]** The text engine owns shaping, line breaking, caret geometry, and text
  selection geometry. Rendering, editing, selection, and export all read the SAME layout result —
  four copies of text metrics is a critical violation.
- **CAN-TEXT-2 [DS]** Box modes: auto-width (box hugs text, 8 handles) · fixed-width (wraps) ·
  auto-height (width handles only). Mode is explicit document data. Overflow on fixed height:
  `…` indicator pill [DS].
- **CAN-TEXT-3 [DS/OBS]** Editing visuals: 1px caret, `--sel` range fill, IME composition =
  dashed underline `2 1` 1.5px; transformed text edits in place with correct screen mapping;
  RTL/mixed scripts follow the shaping engine's logical order (caret moves logically).
- **CAN-TEXT-4 [REC]** Text-on-path is a proposal-stage feature [DS marks it "proposal"]: its
  geometry contract (start/end handles, offset %, flip) must be ADR'd before implementation;
  until then, no partial text-on-path code lands.
- **CAN-TEXT-5** Font loading/fallback: missing fonts render with a documented fallback + error
  state in layers panel [DS: error row `--danger`]; metrics switches never silently reflow a
  saved document without history entry.

## 19. Paints, effects, media (CAN-PAINT / CAN-FX / CAN-MEDIA)

- **CAN-PAINT-1** Fills/strokes/gradients render from document paint data; multiple fills/strokes
  supported per layer stack order. Stroke alignment (inside/outside/center) affects `strokeBounds`
  and hit geometry, not `transformBounds` [per CAN-GEOM-2 ADR outcome].
- **CAN-PAINT-2 [DS/REC]** Gradient handles are overlay controls: screen-constant size, anchored
  to gradient geometry in object space, draggable stops + midpoint add controls, hover/drag states
  per DS; gradient geometry (angle/extent) is document data, handles are chrome.
- **CAN-FX-1 [REC]** Effects pipeline is isolated: an effect/shader failure degrades that layer
  (fallback paint + error badge) and NEVER crashes or corrupts the whole renderer. Effect regions
  are computed into `visualBounds` for caching/culling only.
- **CAN-MEDIA-1 [REC]** Images decode asynchronously with placeholders + loading states; missing
  assets show a documented missing-asset state (never silent blank). Large images are tiled/LOD'd
  by the renderer; color-managed per file-format spec (ADR pending).
- **CAN-MEDIA-2** Video: canvas playback only for visible, in-viewport instances; off-screen or
  deselected video pauses and releases expensive resources; poster frame when paused; scrubbing
  seeks without corrupting timeline state.

## 20. Z-order & compositing (CAN-Z)

- **CAN-Z-1** Document child order = paint order = hit-test order (reverse: topmost first). One
  ordering source; rendering, hit-testing, layer panel, and export all read it.
- **CAN-Z-2 [REC]** Blend modes composite within their parent stacking context; groups with
  blends/effects become isolated compositing layers [OBS standard behavior]; opacity applies to
  the flattened group result (group opacity ≠ per-child opacity).

## 21. Rendering architecture & performance invariants (CAN-RENDER / CAN-PERF)

- **CAN-RENDER-1 [REC/PUBLIC-pattern]** Rendering follows a retained-scene architecture: document
  → scene representation → renderer. The renderer sits behind a **graphics abstraction layer**
  (Figma's public pattern: one C++ interface, WebGL+WebGPU implementations) so the backend
  (WebGL2 today, WebGPU/Wasm-core later) can change without touching canvas logic. Backend choice
  itself is ADR-gated; these rules bind regardless of backend.
- **CAN-RENDER-2 [REC]** Invalidation: dirty-region tracking — editing one node must not
  re-render/re-tessellate the whole page. Caches (tessellation, textures, text runs) invalidate
  with their sources and are keyed deterministically.
- **CAN-RENDER-3 [REC]** Viewport culling: only visible content (+ margin) tessellates/draws;
  LOD: flattening tolerance scales with zoom (coarser at low zoom) for DISPLAY ONLY — canonical
  geometry keeps full precision (export/hit use fixed tolerances, §CAN-VEC-1).
- **CAN-RENDER-4 [REC]** Heavy computations (boolean ops, large flattening, image decode,
  code-preview builds) run off the main interaction path (workers/async) with cancellation on
  gesture change; the pointer→pixel loop stays synchronous and lean.
- **CAN-PERF-1** No canvas change may regress §26 budgets. Performance work requires measurement
  on the reference heavy documents (§26), never an empty canvas.
- **CAN-PERF-2 [REC]** Draw-call discipline: batch by material/texture; overlays batch separately
  from content; per-frame allocations in the render loop are minimized (pools/reuse).

## 22. History & transactions (CAN-HIST)

- **CAN-HIST-1** Every document mutation goes through a transaction. **One gesture = one history
  entry**: a whole drag/resize/rotate/handle-drag/marquee, continuous slider drags, and text
  editing (coalesced per edit session [PROPOSED: break on pause >500ms or focus loss; ADR]).
  NEVER hundreds of undo steps from one pointer movement.
- **CAN-HIST-2** Transient previews (hover, snap previews, ghost copies, uncommitted drags) never
  enter history. Cancelled gestures produce NO entry and restore pre-gesture state exactly
  (CAN-INPUT-4).
- **CAN-HIST-3 [REC]** History entries store inverse patches or command objects — decided by the
  undo-redo architecture doc + ADR; whichever is chosen, canvas code never hand-rolls private
  undo for a feature.
- **CAN-HIST-4** Multi-property changes from one user intent (e.g. paste with layout reflow) =
  one entry. Failed operations roll back fully.

## 23. Determinism & serialization (CAN-SER)

- **CAN-SER-1** The same `.gip` document must reconstruct identical geometry, paint, layout, and
  visual result on any machine/session. Sources of nondeterminism (map iteration order, time,
  random IDs, locale number formatting) are banned from serialization paths.
- **CAN-SER-2 [REC]** Serialization precision: round-trip-safe representation of float64 values
  (shortest round-trippable decimal or binary encoding — ADR-gated); stored values are NEVER
  pre-rounded to display precision.
- **CAN-SER-3** IDs are stable, unique, and never reused; child order is explicit; migrations are
  versioned and forward-only with test coverage per version step.
- **CAN-SER-4** Transient editor state (selection, hover, camera, mode, scroll positions,
  transient overlays) is NOT document geometry and is not saved as such. Per-view camera
  persistence, if wanted, is an explicit file-format feature (ADR).
- **TEST:** golden-file round-trips (serialize → load → serialize ⇒ byte-identical or
  semantically-identical per format spec) for every shape class incl. nested transforms,
  fractional/negative values, masks, booleans, text.

## 24. Accessibility (CAN-A11Y)

- **CAN-A11Y-1 [PUBLIC-pattern/REC]** Accessibility is architectural, from day one: Gipulo
  maintains a **semantic mirror layer** synchronized with the scene graph (Figma's Mirror DOM is
  the public precedent): accessible summaries per layer with surgical updates, spatial positioning
  via transforms (for magnifiers/voice control), and **bidirectional sync between canvas selection
  and keyboard focus**.
- **CAN-A11Y-2 [REC]** Announcements coalesce rapid events (nudge spam → one announcement).
  Canvas commands are keyboard-operable: selection traversal, transform by keyboard (arrows/⇧/⌥
  per §27), mode entry/exit (Enter/Escape), zoom/pan by keyboard.
- **CAN-A11Y-3** Color-independent feedback: states conveyed by more than hue (dashes, badges,
  icons, text) per DS; canvas focus indicators visible on light and dark content.

## 25. Testing requirements (CAN-TEST)

Every canvas change identifies required tests from this matrix (minimum):

| Area | Required coverage |
|---|---|
| Coordinates | round-trip all space pairs; zoom 0.05–32; rotation/flip/nesting; DPR variants |
| Bounds | all bounds kinds × shape classes × rotated/nested/effect/masked cases |
| Hit-testing | inside/outside/on-stroke/cutout/tiny/zero-size/low-zoom; priority overlaps |
| Selection | click/dblclick/deep/shift/marquee; locked/hidden; mode entry/exit preservation |
| Transforms | move/resize/rotate/flip incl. modifiers; repeated-operation stability (1000×) |
| Snapping | thresholds, priority, hysteresis, ⌘-disable, exactness, multi-object group box |
| Camera | pointer-zoom invariance; pan/zoom never dirties document; clamps |
| Vector edit | point/segment/handle ops; add/delete; close; pen; marquee; lasso |
| Text | box modes; caret/selection geometry == render geometry; IME states |
| History | gesture=1 entry; cancel=no entry+exact restore; multi-property coalescing |
| Serialization | golden round-trips; fractional/negative; determinism across runs |
| Guides/overlays | color-meaning snapshots; zoom visibility laws; never exported/serialized |
| Performance | §26 budgets on reference documents; no regression merges |
| Export parity | exported pixels/geometry match canvas within tolerance |

- **CAN-TEST-1** Interaction bugs get regression tests wherever practical. Governing law: **if a
  difficult canvas bug was fixed once, everything practical is done so exactly that bug cannot
  silently return six months later** (test + bug record cross-reference mandatory).
- **CAN-TEST-2 [REC]** Tests are written against subsystem interfaces (geometry engine, coordinate
  module, state machine) so they survive renderer backend swaps.

## 26. Performance budgets (CAN-BUDGET)

[PROPOSED v1 budgets — calibration ADR required with real hardware/documents; until then these
are the acceptance targets for canvas work.]

| Metric | Budget |
|---|---|
| Pan/zoom/drag frame rate (reference docs) | ≥60 fps sustained; no frame >32ms p95 |
| Pointer→visual latency (drag/resize) | ≤2 frames (~33ms) p95 |
| Hover highlight response | ≤1 frame |
| Snap computation per move event | ≤2ms main-thread p95 (offload if exceeded) |
| Open 10k-node document | ≤3s to interactive |
| Save 10k-node document | ≤1s serialize (async, non-blocking UI) |
| Memory (10k-node doc, steady state) | ≤1.5GB incl. caches [PROPOSED] |
| Vector edit on 1k-point path | ≥60fps handle drag |

- **CAN-BUDGET-1** Reference documents (committed as fixtures when code exists): (a) 10k mixed
  nodes, (b) 1k-point complex vector, (c) 500-frame document with images, (d) deep nesting
  (10 levels × 500 groups), (e) heavy-effects document (blurs/shadows on 100 layers).
  Performance tests NEVER use an empty canvas with one rectangle.
- **CAN-BUDGET-2** Any feature whose honest implementation breaks a budget requires an ADR with
  measured alternatives before merging — not a silent regression, not a fake fix.

## 27. Micro-detail registry (CAN-MICRO)

Binding interaction micro-values (sources tagged). Changing any value here = ADR + DS update.

| Micro-detail | Value / rule | Source |
|---|---|---|
| Drag threshold (click→drag) | 3 screen px | [PROPOSED] |
| Hover hysteresis (zone release) | ≥2 screen px beyond zone | [PROPOSED] |
| Snap activation | 4 screen px while dragging | [DS] |
| Snap release hysteresis | ≈6 screen px | [PROPOSED] |
| Guide strokes | 1px at every zoom, half-pixel aligned | [DS] |
| Handle visual / hit | 7–8px / 16px screen px | [DS] |
| Labels hidden below | 25% zoom | [DS] |
| Handles hidden below | 12% zoom | [DS] |
| Snap line style | dashed `4 3`; approach dotted `2 3` @40%; release fade 120ms | [DS] |
| Measure trigger | hold ⌥; nested ⌘⌥; pin ⇧M; values 0–2 dp; bounds not strokes | [DS] |
| Rotate constraint | Shift = 15° ticks | [DS/OBS] |
| Resize modifiers | Shift = proportional; ⌥/Alt = from center; ⌘ during drag = deep-select context | [OBS] |
| Move modifiers | Shift = axis lock; ⌥/Alt-drag = duplicate; ⌘/Ctrl-drag = copy cursor | [OBS] |
| Pan | Space-hold, hand tool, middle-drag, trackpad scroll; pinch = zoom | [OBS] |
| Snap disable | ⌘/Ctrl held during drag (guide @20% + "⌘ held") | [DS] |
| Escape | cancels innermost: drag → mode → selection; never destroys committed state | [REC] |
| Deep select | ⌘/Ctrl-click selects leaf; parent dashed 40% | [DS] |
| Tiny object visuals | <24px: 4 corner handles only; <8px: outline + center dot r1.5 | [DS] |
| Guides toggle | ⌘; show/hide (hidden = no hit area) | [DS] |
| Nudge measure | ⌥⇧arrow shows live value + step arrow | [DS] |
| Pill edge behavior | size/measure pills flip above object near viewport edge | [DS] |
| Dark canvas | same blue guides; white handles; white halo casing on saturated content | [DS] |
| Control remains active after release? | No — transient affordances vanish on pointerup except committed mutations | [REC] |

## 28. Change protocol (mandatory)

**Before touching canvas code:**
1. Read this file's relevant sections + §27–28.
2. Read `docs/architecture/canvas.md` + the touched subsystem's architecture doc (when they exist).
3. Search `docs/bugs/` for prior art on the symptom/area.
4. Identify affected neighbors from the subsystem map (§2) and the regression list per rule.
5. If a value in §27 or a color meaning in CAN-GUIDE-1 must change → stop; that is an ADR.

**After any canvas change:**
1. Run/extend tests per §25 matrix for touched areas + neighbors.
2. Verify §26 budgets if rendering/input/geometry paths changed (reference documents).
3. Update architecture docs + feature docs + this file (if a rule's detail changed) + bug record
   (if a fix) + FEATURE_REGISTRY (if status changed) — the Documentation Update Rule applies
   (`docs/memory/03_documentation_system.md`).
4. Record the task summary in `docs/tasks/`.

---
*v1 authored 2026-10-11 from `docs/memory/06_canvas_rules_requirements.md` + research (§1).
[PROPOSED] items must be confirmed by ADRs during implementation design; until an ADR exists,
the proposed value is the binding default so behavior stays consistent.*
