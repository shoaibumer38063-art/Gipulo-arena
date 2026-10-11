# 06 — Canvas Rules Requirements (spec for the future `rules/canvas.md`)

> Distilled from `docs/source/gipulo_canvas_rules_prompt_refined.md`. This document is the
> requirement spec for authoring the canvas rules file — **"the main law for anything that
> touches the Gipulo canvas."** When `rules/canvas.md` is authored, it must satisfy everything
> below. Until then, this file governs canvas thinking.

## Purpose & when it applies

- Any agent adding a canvas feature, fixing a canvas bug, or changing: geometry · selection ·
  handles · guides · snapping · zoom · transforms · rendering · pointer behavior · text editing ·
  masking · effects · adding a new shape · adding a new content type — **must first read the
  relevant Canvas Rules.**
- Core purpose: an agent must **never solve one canvas problem by creating another canvas problem
  elsewhere.** Canonical failure examples to prevent:
  - fixing a corner-radius handle but creating a second coordinate-conversion system;
  - fixing selection but creating separate selection geometry that no longer matches rendered
    geometry;
  - changing zoom so handles/hover areas/cursors/snapping thresholds/point controls grow/shrink
    incorrectly;
  - adding a new shape with its own separate transform or hit-testing implementation.
- End goal: as Gipulo grows, the canvas becomes **more stable, not more fragile** — extremely
  complex in capability, but with rules that remain clear, centralized, reusable, predictable,
  and difficult for an AI agent to accidentally violate.

## Required research before authoring the rules

- Research how world-class production tools build/control canvas systems — NOT generic frontend
  advice. Study: **Figma** (both user-observable behavior AND publicly explained architecture) ·
  Sketch · Adobe Illustrator · Affinity Designer · Penpot · tldraw · Spline (where relevant) ·
  Framer (where relevant) · professional infinite-canvas systems · modern GPU/WebGPU/WebGL/
  Skia-style rendering architectures (where relevant).
- Known public Figma facts to respect: canvas architecture is closer to a **game-engine-style
  architecture** than a normal web page; performance-critical canvas/rendering code uses
  **C++ compiled to WebAssembly**, surrounding UI can use React/TypeScript; renderer moved toward
  **WebGPU**. Figma also publicly discussed a synchronized semantic/**Mirror DOM** approach for
  canvas accessibility.
- Penpot/tldraw expose useful public concepts: viewport/tile rendering, hit-testing/snapping
  geometry, camera systems, coordinate conversion, screen-space overlays, interaction state
  machines.
- **This does NOT mean blindly copying Figma or writing every canvas feature in C++.** The goal
  is to absorb principles, boundaries, invariants, interaction systems, rendering architecture,
  geometry architecture, and performance rules that make a professional canvas feel stable.
- **Research vs assumption law:** separate findings into (a) publicly documented architecture,
  (b) directly observable product behavior, (c) recommended Gipulo architecture, (d) engineering
  inference. Never present inferences as Figma-confirmed facts. Do not invent Figma internals.

## Rule quality bar (applies to every rule written)

- NO useless generic rules ("keep the canvas performant", "write reusable code", "make
  interactions smooth", "use clean architecture").
- For every important canvas area define: **the rule · why it exists · the source of truth ·
  which systems are affected · what the AI may do · what the AI must NEVER do · what must remain
  invariant · what must be tested after changing it · which neighboring systems can regress.**
- The finished document must function as: (1) architectural contract, (2) interaction contract,
  (3) rendering contract, (4) geometry contract, (5) performance contract, (6) regression-
  prevention system for AI agents. Extremely detailed but not cluttered with repeated generic
  advice; structured so an agent can quickly locate rules for the subsystem being modified.

## Architecture boundaries (subsystems to keep separated)

- Research & define clear separation of: document model · scene/layer tree · geometry engine ·
  rendering engine · camera/viewport · coordinate conversion · transform system · selection
  system · hit-testing · interaction/input system · tool state machines · snapping engine ·
  guides/measurement engine · layout engine · text engine · effects/compositing · asset/media
  system · canvas overlays · history/undo-redo · export renderer · accessibility layer ·
  performance/caching/invalidation.
- Key invariant: **rendered pixels must not become the only source of geometry truth.**
  Selection bounds, snapping, hit-testing, exports, vector editing, and visual rendering need
  clearly defined SHARED sources of truth. Research how pro tools prevent geometry and rendering
  from slowly drifting apart.

## Coordinate systems (EXTREMELY IMPORTANT)

- Define all spaces Gipulo needs: screen/device space · viewport space · canvas/world/page space ·
  parent space · object/local space · vector/path-local space · text-local space (where required) ·
  transformed space · pixel/device-pixel space (where rendering requires).
- Agents must not casually mix coordinate systems. Rules must define: where each system is used ·
  where conversions happen · which conversion utilities are canonical · how zoom affects
  conversion · how pan affects conversion · nested transforms · rotation · scale · parent
  transforms affecting children · device-pixel-ratio effects on visual rendering · how overlays
  position correctly over transformed canvas content.
- **Never random conversion formulas scattered around the app — ONE single authoritative
  transformation/conversion system.**

## Screen-space vs canvas-space behavior

- Some things live in canvas geometry and scale with zoom; others follow canvas geometry but stay
  ~constant in SCREEN PIXELS. Research exactly how pro tools handle this. Applies to: selection
  outlines · blue bounding boxes · resize handles · rotation activation areas · corner-radius
  handles · Bézier handles · vector anchor points · midpoint/add-point controls · path hover
  areas · snapping thresholds · guide thickness · guide labels · measurement text · cursor
  activation zones · hover tolerances · selection tolerances.
- Usability invariant: a control intended to be ~8–10 screen px must NOT become a 1px hit area at
  low zoom or 100px at high zoom — while its position stays perfectly attached to the geometry.
- Rules must clearly distinguish: **canvas-space position · screen-space visual size ·
  screen-space hit area · zoom-scaled geometry.**

## Geometry ⇄ rendering synchronization

- Keep synchronized: actual shape geometry · rendered shape · hit-test geometry · snapping
  geometry · selection bounds · vector-edit geometry · export geometry · masking geometry.
  These must never become separate approximations that drift apart.
- Complex shapes: for a complex polygon/curve/cutout/compound path/open path, clicking empty
  space inside the rectangular bounding box must NOT behave as if that space were real geometry —
  unless a specific tool/selection rule requires it.
- Define when to use: precise path geometry · stroke geometry · fill geometry · visual bounds ·
  geometric bounds · axis-aligned bounds · rotated/oriented bounds · transformed bounds ·
  parent-relative bounds — correct for normal shapes AND extremely complex nested objects.

## Vector geometry (canonical operations)

- Cover: points/vertices · anchor points · straight segments · quadratic Béziers · cubic Béziers ·
  Bézier handles · smooth points · corner points · mirrored handles · asymmetric handles ·
  disconnected handles · closed paths · open paths · multiple subpaths · vector networks/branching
  paths (if supported) · winding/fill rules · stroke geometry · stroke alignment (where supported) ·
  joins · caps · miter limits · dash patterns · path direction · splitting · joining · closing ·
  reversing · inserting points · deleting points · moving points · curving paths ·
  flattening/converting · outline stroke · offset paths · path measurement · nearest-point-on-path.
- Research numerical-stability issues. Agents must NOT approximate Bézier calculations differently
  per feature — **canonical geometry operations only.**

## Vector/shape edit mode (explicit state machine)

- Cover entering/operating/exiting: double-click · Enter · Escape · direct selection · selecting
  one/multiple/all points · selecting paths · selecting Bézier handles · dragging points ·
  dragging handles · adding a point on hover · midpoint controls · deleting points · path closure ·
  creating new path branches (if supported) · changing point type · corner-radius editing ·
  temporary interaction states · what remains active while pointer is held · what stops
  immediately on release.
- **The state machine must be explicit. No random booleans scattered around independently
  deciding whether the editor is in shape-edit mode.**

## Selection system

- Cover: click selection · direct selection · deep selection · double-click · click-through ·
  modifier-key selection · multi-selection · marquee/box selection · selection inside groups ·
  inside frames · through masks · locked layers · hidden layers · components/instances · nested
  components · compound shapes · text · open paths · zero-size objects · rotated objects · tiny
  objects · objects at extremely low zoom.
- Define selection PRIORITY when multiple things overlap; when the system selects: parent · child ·
  top-most visible object · geometry underneath · an editable vector point · an overlay control.
- Define selection preservation when entering/exiting modes.

## Bounding boxes & selection overlays

- Cover: blue bounding boxes · multi-selection bounds · rotated bounds · resize handles · rotation
  zones · center points (where needed) · corner-radius controls · vector points · path outlines ·
  text bounds · frame bounds · component/instance bounds · locked-state visualization · hover
  outlines · editing-state outlines.
- Distinguish: **geometric bounds · visual/effect bounds · selection bounds · transform bounds.**
- Effects (blur, shadow, glow, shaders, stroke) must NOT accidentally change transform behavior
  unless intentionally designed to.

## Hovering

- Cover: shape hover · path hover · point hover · handle hover · resize-edge hover ·
  resize-corner hover · rotation-zone hover · corner-radius hover · snapping-target hover ·
  text-edit hover · guide hover · ruler hover · mask hover · nested-item hover.
- No flicker when activation zones overlap → clear priority; hysteresis/tolerance where
  appropriate so a 1px cursor move doesn't constantly switch states.

## Cursors & cursor states

- Cover design + behavior: cursor visual design · state · activation area · position of activation
  areas · orientation · rotation-aware cursors · resize direction · drag cursor · hand/pan ·
  grabbing · pen · point-add · point-delete · text · rotate · scale · duplicate/modifier ·
  forbidden/not-allowed · eyedropper (if relevant) · boolean/shape-builder states · mask states.
- Laws: for rotated objects, resize cursors follow the ACTUAL transform direction (never behave
  as if 0° rotation); the cursor represents the action that will happen **before pointer-down**;
  cursor activation zones designed separately from visible handle size where necessary.
- Research Figma/Sketch/Illustrator hover/activation-zone behavior.

## Input & interaction state machine

- Cover: pointer down/move/up · click · double-click · long press (where needed) · drag threshold ·
  pointer capture · pointer cancellation · Escape cancellation · mouse leaving viewport · window
  losing focus · modifier keys changing DURING drag · keyboard interactions · trackpad · touch ·
  stylus/pen (where supported).
- The system must always know whether the user is: idle · hovering · pressing · dragging ·
  resizing · rotating · panning · zooming · drawing · editing a path · editing text · creating a
  guide · snapping · duplicating · transforming.
- **Conflicting tools must never believe they own the same pointer interaction.**

## Transforms (move/drag/scale/resize/rotate/flip)

- Cover: move · drag · resize · proportional scaling · non-proportional resizing · center-based
  scaling · rotate · rotate around pivot · flip horizontal/vertical · negative scale (if the
  internal model allows) · transform of children · groups · frames · multi-selections · rotated
  selections · nested objects. Cover modifier keys and their EXACT behavior.
- **Numerical stability: repeated transform/decompose/recompose operations must not slowly corrupt
  geometry via floating-point rounding.**

## Positive/negative values, fractions, subpixels

- Gipulo must NOT assume every useful coordinate is a positive integer. Rules for: negative X/Y ·
  negative rotation · negative offsets · fractional coordinates · fractional width/height (where
  valid) · fractional stroke values · fractional gaps · fractional padding · values like
  `1.1px`, `1.5px`, `1.6px` · subpixel rendering · device-pixel alignment · high-DPI screens ·
  rounding for display vs rounding for geometry.
- **Do NOT round underlying geometry because a UI field displays fewer decimals. Displayed
  precision and internal precision are separate concepts.**

## Zoom, pan, scroll (camera)

- Cover: mouse wheel · trackpad scroll · trackpad pinch · keyboard zoom · zoom to selection · zoom
  to fit · zoom around pointer · spacebar panning · hand tool · middle-mouse pan (if supported) ·
  momentum/inertia (if supported) · min/max zoom · very high zoom · very low zoom.
- Laws: zoom feels stable — the point under the pointer must NOT drift during pointer-centered
  zoom; panning/zooming never modify document geometry; **camera state and document state stay
  separate.**

## Snapping

- Snap targets: object edges · centers · corners · anchor points · vector paths · frames · guides ·
  rulers · grids · layout guides · spacing/distribution · text baselines (if supported) · rotated
  geometry · paths/curves (where supported).
- Define: snap threshold · threshold in screen pixels vs canvas units · priority · competing snap
  targets · temporary snapping · disabling with modifier keys · snap release behavior · hysteresis ·
  snapping while resizing/moving/creating · snapping vector points.
- **Must not feel sticky or unpredictable.**

## Guides, measurements, visual feedback

- Cover every overlay type: blue guides · red/pink measurement lines · orange guides · smart
  alignment guides · spacing guides · distribution guides · snapping guides · frame guides ·
  layout guides · ruler guides · temporary guides · distance labels · gap labels · padding labels.
- **Do NOT assume a color means the same thing across tools** — research Gipulo's intended visual
  language and define exactly what each guide color/state means; prevent different features from
  randomly introducing new guide colors for the same meaning.
- Line thickness, label size, interaction areas remain usable across zoom levels.
- (Concretized meanings from the HTML reference: blue=geometry, magenta=spacing, orange=snap/
  measure, red=ruler guides — see `07_design_tokens_and_visual_specs.md`.)

## Alignment, distribution, gaps, spacing

- Cover: align left/right/top/bottom · horizontal/vertical center · distribute horizontally/
  vertically · equal spacing · smart spacing · Auto Layout gaps · padding · negative spacing
  (where supported) · multi-selection alignment · rotated object behavior · nested selection.
- Define whether calculations use object bounds, visual bounds, or another canonical bound.
  **No alignment command may invent its own bounds logic.**

## Grids & sub-grids

- Cover: canvas grids · frame-level layout guides · uniform grids · rows · columns · Auto Layout
  grid · sub-grids · fractional grid sizes · grid origin · offset · visibility · grid snapping ·
  rotated frames (if supported) · nested grids.
- **Clearly distinguish visual guides from layout systems — a visual grid must not silently become
  a layout constraint.**

## Rulers

- Cover: horizontal/vertical ruler · zero/origin behavior · coordinates · frame-relative ruler
  states (where required) · zoom · negative coordinates · guide creation from rulers ·
  dragging/removing guides · selection highlights · units · precision.
- **Rulers must always agree with the canonical coordinate system.**

## Shapes

- Cover: rectangle · rounded rectangle · ellipse · line · arrow · polygon · star · open vector ·
  closed vector · compound vector · complex path · parametric shapes · future custom procedural
  shapes.
- **A shape must not become a completely separate architecture.** Basic shapes may stay
  parametric/non-destructive where useful, with a clear transition when converted to editable
  vector geometry.

## Frames, groups, nesting

- Cover: frames · groups · nested groups · nested frames · groups inside frames · frames inside
  groups · clipping · transforms · child coordinate systems · selection · resizing · constraints ·
  Auto Layout · masks · components · z-order.
- **Groups and frames must NOT be treated as the same thing internally if their behavior differs.
  Define their contracts clearly.**

## Text

- Text is not just another rectangle. Cover: normal horizontal text · vertical text · text on a
  path · point text · area/fixed-size text · auto-width · auto-height · fixed-size boxes ·
  alignment · baseline · line height · letter spacing · text shaping · font fallback · variable
  fonts · text selection · editing · caret · composition/IME input · RTL · mixed scripts · nested
  text styles · transformed text · outlined text.
- **Text rendering geometry, editing geometry, selection geometry, and export geometry must remain
  consistent.**

## Boolean & compound shapes

- Cover: union · subtract · intersect · exclude · compound shapes · nested booleans ·
  editable/non-destructive boolean operations · flattening · path winding/fill rules · selection
  inside boolean groups · transform behavior · performance.
- **Never destructively flatten geometry unless the operation requires it or the user asks.**

## Components & instances

- Define interaction with: selection · transforms · nesting · overrides · clipping · masks ·
  Auto Layout · vector editing · detach · nested instances · variants/properties · component
  boundaries.
- The canvas must **visually communicate** whether the user is editing an instance, component,
  nested component, or normal layer.

## Masks & clipping

- Cover: frame clipping · vector masks · alpha masks (if supported) · luminance masks (if
  supported) · nested masks · transformed masks · masks inside components · hit-testing through
  masks · selection of masked content · export behavior.
- **Rendered visibility and editability need explicit rules: an object can exist in the document
  even if part of it is visually clipped.**

## Fills, strokes, gradients, patterns, color

- Cover: solid fills · multiple fills · strokes · multiple strokes (if supported) · gradients
  (linear · radial · angular/conic · diamond if supported) · image fills · pattern fills ·
  procedural fills · shader fills · opacity · blend modes · color spaces · alpha.
- Gradient-handle canvas behavior: activation · hover · dragging · stops · midpoint controls ·
  rotation · scaling · screen-space handle size · snapping where applicable.

## Effects & shaders

- Cover: drop shadows · inner shadows · layer blur · background blur · multiple effects · blend
  modes · noise/texture · shader effects · future custom effects. Research modern GPU-based
  effect pipelines.
- Separate: geometry · visual effects · visual/effect bounds · selection bounds · hit-testing.
- **An enormous blur must not make the user accidentally select an object hundreds of pixels away**
  (unless an intentional interaction rule). **Shader/effect failures must not corrupt the entire
  canvas renderer.**

## Images & videos

- Images: placement · image fills · crop · fit · fill · transform · masks · large images ·
  decoding · caching · color handling · missing assets · loading states.
- Video: playback on canvas · pause · scrub · poster frame · decoding · GPU texture/resource
  lifecycle · masking · transforms · performance · visibility/off-screen behavior.
- **Do not keep expensive off-screen media work running unnecessarily.**

## Parametric / procedural objects

- Cover: parameter source of truth · generated geometry · editable parameters · conversion to
  normal vectors · selection · bounds · hit-testing · snapping · rendering · undo · serialization ·
  performance.
- **Generated geometry must not become disconnected from the parameters that generated it.**

## Layer order, z-order, compositing

- Cover: front/back ordering · stacking · nested stacking · masks · clipping · blend modes ·
  isolated groups (if supported) · effect order · rendering order · selection priority.
- **Render order and interaction order need clearly defined relationships.**

## Rendering architecture & performance

- Based on real research, not generic advice. Study concepts from high-performance design/
  infinite-canvas tools: GPU rendering · WebGPU/WebGL · C++/Wasm where justified · render
  abstraction layers · retained scene data · tile rendering · viewport culling · dirty-region/
  invalidation systems · render caching · geometry caches · texture caches · GPU resource
  lifecycle · batching · draw-call reduction · level of detail · asynchronous asset loading ·
  worker/background processing where suitable.
- Do NOT force a specific implementation just because another app uses it. Instead define
  **performance invariants and architectural boundaries** that let Gipulo choose the correct
  implementation.
- Bar: the user can work on very large documents **without constantly thinking about the renderer.**

## Canvas overlays are NOT document geometry

- Selection outlines, handles, guides, collaborator cursors, measurements, hover effects,
  temporary tool previews, snapping lines, similar UI overlays = **editor UI**, architecturally
  separated from document content. They may follow document geometry but must:
  NOT export · NOT become selectable document objects · NOT modify layout · NOT change document
  bounds · NOT become part of saved design geometry (unless explicitly intended).
- Research screen-space overlay layers; define rules around them.

## History, undo/redo, transactions

- Cover: pointer drag = ONE history operation · continuous resize = one operation · continuous
  rotate = one operation · vector handle drag = one operation · text editing · multi-property
  changes · cancellation · failed operations · temporary previews.
- **Never create hundreds of undo steps while the pointer moves during one drag. Temporary
  hover/preview states normally never enter document history.**

## Determinism & serialization

- Canvas behavior must be deterministic: **the same saved `.gip` document reconstructs the same
  geometry and visual result.**
- Rules for: serialization precision · IDs · ordering · floating-point values · parametric data ·
  paths · transforms · effects · media references · migrations.
- **Do not save temporary editor UI states as document geometry** unless intentionally part of the
  file format.

## Accessibility & keyboard operation

- Canvas performance must not mean ignoring accessibility. Provide: keyboard navigation · focus
  management · semantic descriptions · screen-reader representations · accessible editing commands.
- Figma's public Mirror-DOM approach is a reference point (canvas rendering doesn't automatically
  get DOM accessibility) — don't blindly copy, but **accessibility is an architectural
  requirement, not something added at the end.**

## Testing rules for canvas changes

- Every canvas change must identify required tests. Coverage targets: geometry · hit-testing ·
  coordinate conversion · transforms · nested transforms · selection · hover · drag · resize ·
  rotate · snapping · vector points · Bézier handles · zoom · pan · high zoom · low zoom ·
  fractional values · negative coordinates · DPI · text · masks · effects · export parity ·
  undo/redo · performance.
- Interaction bugs → regression tests wherever practical.
- Governing rule: **"If a difficult canvas bug has been fixed once, do everything practical to
  stop exactly that bug from silently returning six months later."**

## Performance budgets

- Not just "keep 60 FPS." Create measurable budgets/benchmarking rules for: pan · zoom · hover ·
  pointer latency · dragging · resizing · vector editing · large selections · complex paths ·
  large documents · effects · images · videos · text · memory · file loading.
- **Performance tests must use representative heavy documents, not an empty canvas with one
  rectangle.**

## Interaction micro-details (the difference between "works" and "Figma-level")

- Rules must reach micro-level behavior: exact activation areas · whether thresholds are
  screen-pixel invariant · overlap priority between resize and rotate zones · when hover begins/
  ends · drag threshold before click becomes drag · what happens on pointer release · what Escape
  cancels · what modifier keys do while already dragging · whether a control remains active after
  mouse release · how snapping releases · how points behave at 5% zoom vs 3200% zoom · how tiny
  objects remain selectable · how fractional geometry is displayed · how nested transforms are
  resolved.

## Final requirement

- The finished `canvas-rules.md` must be extremely detailed (the canvas will support a huge number
  of features), yet structured for quick lookup and free of repeated generic advice.
- **No AI agent may touch a critical canvas system without first understanding the existing canvas
  architecture, canonical coordinate/geometry systems, interaction rules, and neighboring features
  that could be affected.**
