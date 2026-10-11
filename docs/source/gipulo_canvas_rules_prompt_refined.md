# Prompt — Create the Gipulo Canvas Rules Documentation

Now create a **separate `.md` file only for Canvas Rules**.

But before writing the rules, **deeply research how world-class, production-level design tools build and control their canvas systems**.

Do not just write generic rules based on normal frontend development.

This canvas will become one of the most important and most dangerous systems inside Gipulo. Almost every major design feature will eventually touch the canvas in some way. If the canvas foundation becomes inconsistent, then later we will keep fixing one feature and breaking another feature.

So before creating `canvas-rules.md`, deeply research the current state of the art, especially **Figma**, but also other serious canvas/design applications and canvas architectures such as:

- Figma;
- Sketch;
- Adobe Illustrator;
- Affinity Designer;
- Penpot;
- tldraw;
- Spline where relevant;
- Framer where relevant;
- professional infinite-canvas systems;
- modern GPU/WebGPU/WebGL/Skia-style rendering architectures where relevant.

For Figma, research both:

1. **how the canvas behaves from the user's point of view**, and
2. **what Figma has publicly explained about the architecture behind that behavior**.

Use current Figma documentation, engineering articles, interaction behavior, and other reliable sources.

Figma has publicly described its canvas architecture as being much closer to a **game-engine-style architecture** than a normal web page. Its performance-critical canvas/rendering code uses C++ compiled to WebAssembly, while the surrounding application UI can use technologies such as React/TypeScript. Figma also moved its renderer toward WebGPU.

This does **not** mean Gipulo should blindly copy Figma's implementation or that every canvas feature must be written in C++.

The goal of the research is to understand the **principles, boundaries, invariants, interaction systems, rendering architecture, geometry architecture, and performance rules** that make a professional design canvas feel extremely stable.

Also research systems like Penpot and tldraw because they expose some useful architecture concepts publicly—for example viewport/tile rendering, geometry used for hit-testing and snapping, camera systems, coordinate conversion, screen-space overlays, and interaction state machines.

---

# The Main Purpose of `canvas-rules.md`

This file should become the **main law for anything that touches the Gipulo canvas**.

Whenever an AI agent is:

- adding a canvas feature;
- fixing a canvas bug;
- changing geometry;
- changing selection;
- changing handles;
- changing guides;
- changing snapping;
- changing zoom;
- changing transforms;
- changing rendering;
- changing pointer behavior;
- changing text editing;
- changing masking;
- changing effects;
- adding a new shape;
- adding a new content type;

it should first read the relevant Canvas Rules.

The purpose is that an AI agent should **never solve one canvas problem by creating another canvas problem somewhere else**.

For example, if the agent fixes a corner-radius handle, it should not accidentally create a second coordinate-conversion system.

If it fixes selection, it should not create separate selection geometry that no longer matches the rendered geometry.

If it changes zoom, it should not make handles, hover areas, cursors, snapping thresholds, or point controls grow and shrink incorrectly.

If it adds a new shape, it should not create a completely separate transform or hit-testing implementation only for that shape.

The canvas rules should make all of this extremely clear.

---

# IMPORTANT — Do Not Make Generic AI Rules

Do **not** give me useless rules such as:

> Keep the canvas performant.

> Write reusable code.

> Make interactions smooth.

> Use clean architecture.

These are too generic.

I need rules that explain **what these things actually mean inside Gipulo**.

For every important canvas area, define:

- the rule;
- why the rule exists;
- the source of truth;
- which systems are affected;
- what the AI is allowed to do;
- what the AI must never do;
- what must remain invariant;
- what needs to be tested after changing it;
- which neighboring systems can regress because of the change.

These should feel like internal engineering rules from a world-class graphics/design software company.

---

# Research and Define the Canvas Architecture Boundaries

Before writing interaction-level rules, first understand and define the major canvas subsystems.

Research whether Gipulo should clearly separate systems such as:

- document model;
- scene/layer tree;
- geometry engine;
- rendering engine;
- camera/viewport;
- coordinate conversion;
- transform system;
- selection system;
- hit-testing;
- interaction/input system;
- tool state machines;
- snapping engine;
- guides/measurement engine;
- layout engine;
- text engine;
- effects/compositing;
- asset/media system;
- canvas overlays;
- history/undo-redo;
- export renderer;
- accessibility layer;
- performance/caching/invalidation.

The Canvas Rules should prevent these systems from becoming mixed together.

For example:

**rendered pixels should not become the only source of geometry truth.**

Selection bounds, snapping, hit-testing, exports, vector editing, and visual rendering need clearly defined shared sources of truth.

Research how professional tools avoid geometry and rendering slowly drifting apart.

---

# Coordinate Systems — EXTREMELY IMPORTANT

Create very strict rules for coordinate systems.

This is one of the most important parts of the entire canvas.

Research and properly define all coordinate spaces Gipulo needs, for example:

- screen/device space;
- viewport space;
- canvas/world/page space;
- parent space;
- object/local space;
- vector/path-local space;
- text-local space where required;
- transformed space;
- pixel/device-pixel space where rendering requires it.

Do not let agents casually mix these coordinate systems.

The rules need to define:

- where each coordinate system is used;
- where conversions happen;
- which conversion utilities are canonical;
- how zoom affects conversion;
- how pan affects conversion;
- how nested transforms affect conversion;
- how rotation affects conversion;
- how scale affects conversion;
- how parent transforms affect children;
- how device-pixel-ratio affects visual rendering;
- how overlays are positioned correctly over transformed canvas content.

There should never be random formulas for coordinate conversion scattered around the application.

There should be a **single authoritative transformation/conversion system**.

---

# Screen-Space vs Canvas-Space Behavior

This needs its own very strict rules.

Some things exist in the canvas geometry and should scale with zoom.

Other things visually follow canvas geometry but should remain almost constant in **screen pixels**.

Research exactly how professional design tools handle this.

This includes things such as:

- selection outlines;
- blue bounding boxes;
- resize handles;
- rotation activation areas;
- corner-radius handles;
- Bézier handles;
- vector anchor points;
- midpoint/add-point controls;
- path hover areas;
- snapping thresholds;
- guide thickness;
- guide labels;
- measurement text;
- cursor activation zones;
- hover tolerances;
- selection tolerances.

For example, if a control is intended to remain approximately 8–10 screen pixels for usability, zooming the canvas should **not** make its interactive hit area become 1 pixel at low zoom and 100 pixels at high zoom.

At the same time, its position must stay perfectly attached to the actual geometry.

The rules must clearly distinguish:

- canvas-space position;
- screen-space visual size;
- screen-space hit area;
- zoom-scaled geometry.

This is extremely important.

---

# Geometry and Visual Rendering Must Stay Synchronized

Research and create strict rules around the relationship between:

- actual shape geometry;
- rendered shape;
- hit-test geometry;
- snapping geometry;
- selection bounds;
- vector-edit geometry;
- export geometry;
- masking geometry.

These cannot become separate approximations that slowly stop matching one another.

If a shape has a complex polygon, curve, cutout, compound path, or open path, clicking empty space inside its rectangular bounding box should not automatically behave as if that empty space were real geometry unless that specific tool/selection rule requires it.

Define when the system should use:

- precise path geometry;
- stroke geometry;
- fill geometry;
- visual bounds;
- geometric bounds;
- axis-aligned bounds;
- rotated/oriented bounds;
- transformed bounds;
- parent-relative bounds.

This should work correctly for normal shapes and extremely complex nested objects.

---

# Vector Geometry

Create extremely detailed rules for vector geometry.

This includes:

- points/vertices;
- anchor points;
- straight segments;
- quadratic Bézier curves;
- cubic Bézier curves;
- Bézier handles;
- smooth points;
- corner points;
- mirrored handles;
- asymmetric handles;
- disconnected handles;
- closed paths;
- open paths;
- multiple subpaths;
- vector networks/branching paths if Gipulo supports them;
- winding/fill rules;
- stroke geometry;
- stroke alignment where supported;
- joins;
- caps;
- miter limits;
- dash patterns;
- path direction;
- path splitting;
- joining;
- closing;
- reversing;
- inserting points;
- deleting points;
- moving points;
- curving paths;
- flattening/converting;
- outline stroke;
- offset paths;
- path measurement;
- nearest-point-on-path calculations.

Research numerical-stability issues as well.

Agents should not repeatedly approximate Bézier calculations differently in different features.

There should be canonical geometry operations.

---

# Shape Editing / Vector Edit Mode

Create rules for entering, operating inside, and exiting vector/shape edit mode.

Cover:

- double-click;
- Enter;
- Escape;
- direct selection;
- selecting one point;
- multiple points;
- all points;
- selecting paths;
- selecting Bézier handles;
- dragging points;
- dragging handles;
- adding a point on hover;
- midpoint controls;
- deleting points;
- path closure;
- creating new path branches if supported;
- changing point type;
- corner-radius editing;
- temporary interaction states;
- what remains active while mouse/pointer is held;
- what stops immediately when pointer is released.

The state machine should be explicit.

Do not allow random booleans scattered around the code to independently decide whether the editor is in shape-edit mode.

---

# Selection System

Research world-class selection behavior deeply.

Canvas rules should cover:

- click selection;
- direct selection;
- deep selection;
- double-click;
- click-through behavior;
- modifier-key selection;
- multi-selection;
- marquee/box selection;
- selection inside groups;
- selection inside frames;
- selection through masks;
- selection of locked layers;
- hidden layers;
- components/instances;
- nested components;
- compound shapes;
- text;
- open paths;
- zero-size objects;
- rotated objects;
- tiny objects;
- objects at extremely low zoom.

Define selection priority when multiple things overlap.

Define when the system selects:

- the parent;
- the child;
- the top-most visible object;
- geometry underneath;
- an editable vector point;
- an overlay control.

Also define selection preservation when entering/exiting modes.

---

# Bounding Boxes and Selection Overlays

Create specific rules for:

- blue bounding boxes;
- multi-selection bounds;
- rotated bounds;
- resize handles;
- rotation zones;
- center points where needed;
- corner-radius controls;
- vector points;
- path outlines;
- text bounds;
- frame bounds;
- component/instance bounds;
- locked-state visualization;
- hover outlines;
- editing-state outlines.

Research the difference between:

- geometric bounds;
- visual/effect bounds;
- selection bounds;
- transform bounds.

Effects such as blur, shadow, glow, shader effects, or stroke should not accidentally change transform behavior unless intentionally designed to do so.

---

# Hovering

Hover behavior needs proper rules.

Research and define:

- shape hover;
- path hover;
- point hover;
- handle hover;
- resize-edge hover;
- resize-corner hover;
- rotation-zone hover;
- corner-radius hover;
- snapping-target hover;
- text-edit hover;
- guide hover;
- ruler hover;
- mask hover;
- nested-item hover.

Hovering should not flicker when two activation zones overlap.

There should be clear priority.

There also needs to be hysteresis/tolerance where appropriate so the cursor does not constantly switch states when moving by one pixel.

---

# Cursors and Cursor States

Create a dedicated cursor section.

Do not only list cursor icons.

Research:

- cursor visual design;
- cursor state;
- activation area;
- position of activation areas;
- orientation;
- rotation-aware cursors;
- resize direction;
- drag cursor;
- hand/pan cursor;
- grabbing cursor;
- pen cursor;
- point-add cursor;
- point-delete cursor;
- text cursor;
- rotate cursor;
- scale cursor;
- duplicate/modifier cursor;
- forbidden/not-allowed cursor;
- eyedropper if relevant;
- boolean/shape-builder states;
- mask states.

For rotated objects, resize cursors must follow the actual transform direction instead of always behaving as if the object has 0° rotation.

The cursor must represent the action that will actually happen **before pointer-down**.

Cursor activation zones should be designed separately from visible handle size where necessary.

Research Figma, Sketch, Illustrator, and other professional tools for hover/activation-zone behavior.

---

# Input and Interaction State Machine

Canvas input must have a proper state-machine architecture.

Cover:

- pointer down;
- pointer move;
- pointer up;
- click;
- double-click;
- long press where needed;
- drag threshold;
- pointer capture;
- pointer cancellation;
- Escape cancellation;
- mouse leaving viewport;
- window losing focus;
- modifier keys changing during drag;
- keyboard interactions;
- trackpad;
- touch;
- stylus/pen where supported.

The system must clearly know whether the user is:

- idle;
- hovering;
- pressing;
- dragging;
- resizing;
- rotating;
- panning;
- zooming;
- drawing;
- editing a path;
- editing text;
- creating a guide;
- snapping;
- duplicating;
- transforming.

Do not allow conflicting tools to believe they own the same pointer interaction.

---

# Moving, Dragging, Scaling, Resizing, Rotating and Flipping

Create rules for all transform operations.

Cover:

- move;
- drag;
- resize;
- proportional scaling;
- non-proportional resizing;
- center-based scaling;
- rotate;
- rotate around pivot;
- flip horizontal;
- flip vertical;
- negative scale if the internal model allows it;
- transform of children;
- transform of groups;
- transform of frames;
- transform of multi-selections;
- transform of rotated selections;
- transform of nested objects.

Cover modifier keys and their exact behavior.

The transformation must remain numerically stable after repeated operations.

Do not allow repeated transform/decompose/recompose operations to slowly corrupt geometry because of floating-point rounding.

---

# Positive Values, Negative Values, Fractions and Subpixels

Gipulo must not assume every useful coordinate is a positive integer.

Research and create rules for:

- negative X/Y positions;
- negative rotation;
- negative offsets;
- fractional coordinates;
- fractional width/height where valid;
- fractional stroke values;
- fractional gaps;
- fractional padding;
- values such as `1.1px`, `1.5px`, `1.6px`, etc.;
- subpixel rendering;
- device-pixel alignment;
- high-DPI screens;
- rounding for display vs rounding for geometry.

Do **not** round the underlying geometry simply because the UI property field displays fewer decimal places.

Displayed precision and internal precision should be separate concepts.

---

# Zooming, Panning and Scrolling

Research how world-class infinite canvases handle camera behavior.

Create rules for:

- mouse wheel;
- trackpad scroll;
- trackpad pinch;
- keyboard zoom;
- zoom to selection;
- zoom to fit;
- zoom around pointer;
- spacebar panning;
- hand tool;
- middle-mouse pan if supported;
- momentum/inertia if supported;
- min/max zoom;
- very high zoom;
- very low zoom.

Zoom should feel stable.

The point under the pointer should not unexpectedly drift when pointer-centered zoom is used.

Panning and zooming should not modify document geometry.

Camera state and document state must stay separate.

---

# Snapping

Create an extremely detailed snapping section.

Research professional snapping systems.

Cover snapping to:

- object edges;
- centers;
- corners;
- anchor points;
- vector paths;
- frames;
- guides;
- rulers;
- grids;
- layout guides;
- spacing/distribution;
- text baselines if supported;
- rotated geometry;
- paths/curves where supported.

Define:

- snap threshold;
- threshold in screen pixels vs canvas units;
- snapping priority;
- competing snap targets;
- temporary snapping;
- disabling snapping with modifier keys;
- snap release behavior;
- snap hysteresis;
- snapping while resizing;
- snapping while moving;
- snapping while creating;
- snapping vector points.

The system must not feel sticky or unpredictable.

---

# Guides, Measurements and Visual Feedback

Research and document every type of canvas guide/overlay.

This includes:

- blue guides;
- red/pink measurement lines;
- orange guides;
- smart alignment guides;
- spacing guides;
- distribution guides;
- snapping guides;
- frame guides;
- layout guides;
- ruler guides;
- temporary guides;
- distance labels;
- gap labels;
- padding labels.

Do **not** assume that a color always means the same thing across tools.

Research Gipulo's intended visual language and define exactly what each guide color/state means.

The Canvas Rules should prevent different features from randomly introducing new guide colors for the same meaning.

Guide line thickness, label size, and interaction areas should remain usable across zoom levels.

---

# Alignment, Distribution, Gaps and Spacing

Create rules for:

- align left/right/top/bottom;
- horizontal center;
- vertical center;
- distribute horizontally;
- distribute vertically;
- equal spacing;
- smart spacing;
- Auto Layout gaps;
- padding;
- negative spacing where supported;
- multi-selection alignment;
- rotated object behavior;
- nested selection behavior.

Define whether calculations use object bounds, visual bounds, or another canonical bound.

Do not allow each alignment command to invent its own bounds logic.

---

# Grids and Sub-Grids

Cover:

- canvas grids;
- frame-level layout guides;
- uniform grids;
- rows;
- columns;
- Auto Layout grid;
- sub-grids;
- fractional grid sizes;
- grid origin;
- grid offset;
- grid visibility;
- grid snapping;
- rotated frames if supported;
- nested grids.

Clearly distinguish **visual guides** from **layout systems**.

A visual grid should not silently become a layout constraint.

---

# Rulers

Create rules for:

- horizontal ruler;
- vertical ruler;
- zero/origin behavior;
- coordinates;
- frame-relative ruler states where required;
- zoom;
- negative coordinates;
- guide creation from rulers;
- dragging/removing guides;
- selection highlights;
- units;
- precision.

Rulers must always agree with the canonical coordinate system.

---

# Shapes

Cover all basic and advanced shape categories.

For example:

- rectangle;
- rounded rectangle;
- ellipse;
- line;
- arrow;
- polygon;
- star;
- open vector;
- closed vector;
- compound vector;
- complex path;
- parametric shapes;
- future custom procedural shapes.

A shape should not become a completely separate architecture.

Basic shapes can remain parametric/non-destructive where useful, but there must be a clear transition when the user converts them to editable vector geometry.

---

# Frames, Groups and Nesting

Create strict rules for:

- frames;
- groups;
- nested groups;
- nested frames;
- groups inside frames;
- frames inside groups;
- clipping;
- transforms;
- child coordinate systems;
- selection;
- resizing;
- constraints;
- Auto Layout;
- masks;
- components;
- z-order.

Groups and frames must not be treated as the same thing internally if their behavior is different.

Define their contracts clearly.

---

# Text

Canvas rules must also cover text because text is not just another rectangle.

Research and define rules for:

- normal horizontal text;
- vertical text;
- text on a path;
- point text;
- area/fixed-size text;
- auto-width;
- auto-height;
- fixed-size boxes;
- alignment;
- baseline;
- line height;
- letter spacing;
- text shaping;
- font fallback;
- variable fonts;
- text selection;
- editing;
- caret;
- composition/IME input;
- RTL;
- mixed scripts;
- nested text styles;
- transformed text;
- outlined text.

Text rendering geometry, editing geometry, selection geometry, and export geometry must remain consistent.

---

# Boolean and Compound Shapes

Cover:

- union;
- subtract;
- intersect;
- exclude;
- compound shapes;
- nested booleans;
- editable/non-destructive boolean operations;
- flattening;
- path winding/fill rules;
- selection inside boolean groups;
- transform behavior;
- performance.

Do not destructively flatten geometry unless the operation specifically requires it or the user asks for it.

---

# Components and Instances

Canvas rules should define how components interact with:

- selection;
- transforms;
- nesting;
- overrides;
- clipping;
- masks;
- Auto Layout;
- vector editing;
- detach;
- nested instances;
- variants/properties;
- component boundaries.

The canvas must visually communicate when the user is editing an instance, component, nested component, or normal layer.

---

# Masks and Clipping

Create separate rules for:

- frame clipping;
- vector masks;
- alpha masks if supported;
- luminance masks if supported;
- nested masks;
- transformed masks;
- masks inside components;
- interaction/hit-testing through masks;
- selection of masked content;
- export behavior.

Rendered visibility and editability need explicit rules.

An object can exist in the document even if part of it is visually clipped.

---

# Fills, Strokes, Gradients, Patterns and Color

Canvas rules must cover rendering and interaction for:

- solid fills;
- multiple fills;
- strokes;
- multiple strokes if supported;
- gradients;
- linear gradients;
- radial gradients;
- angular/conic gradients;
- diamond gradients if supported;
- image fills;
- pattern fills;
- procedural fills;
- shader fills;
- opacity;
- blend modes;
- color spaces;
- alpha.

Also define gradient-handle behavior on the canvas:

- activation;
- hover;
- dragging;
- stops;
- midpoint controls;
- rotation;
- scaling;
- screen-space handle size;
- snapping where applicable.

---

# Effects and Shaders

Research modern GPU-based effect pipelines.

Cover:

- drop shadows;
- inner shadows;
- layer blur;
- background blur;
- multiple effects;
- blend modes;
- noise/texture;
- shader effects;
- future custom effects.

The canvas rules must separate:

- geometry;
- visual effects;
- visual/effect bounds;
- selection bounds;
- hit-testing.

An enormous blur should not make the user accidentally select an object hundreds of pixels away unless that is an intentional interaction rule.

Shader/effect failures should not corrupt the entire canvas renderer.

---

# Images and Videos

Create rules for:

- image placement;
- image fills;
- crop;
- fit;
- fill;
- transform;
- masks;
- large images;
- image decoding;
- caching;
- color handling;
- missing assets;
- loading states.

For video:

- playback on canvas;
- pause;
- scrub;
- poster frame;
- decoding;
- GPU texture/resource lifecycle;
- masking;
- transforms;
- performance;
- visibility/off-screen behavior.

Do not keep expensive off-screen media work running unnecessarily.

---

# Parametric / Procedural Objects

Since Gipulo may support advanced parametric objects, create rules for:

- parameter source of truth;
- generated geometry;
- editable parameters;
- conversion to normal vectors;
- selection;
- bounds;
- hit-testing;
- snapping;
- rendering;
- undo;
- serialization;
- performance.

Generated geometry should not become disconnected from the parameters that generated it.

---

# Layer Order, Z-Order and Compositing

Cover:

- front/back ordering;
- stacking;
- nested stacking;
- masks;
- clipping;
- blend modes;
- isolated groups if supported;
- effect order;
- rendering order;
- selection priority.

The render order and interaction order need clearly defined relationships.

---

# Rendering Architecture and Performance

This section must be based on real research, not generic advice.

Study concepts used by high-performance design/infinite-canvas tools such as:

- GPU rendering;
- WebGPU/WebGL;
- C++/Wasm where justified;
- render abstraction layers;
- retained scene data;
- tile rendering;
- viewport culling;
- dirty-region/invalidation systems;
- render caching;
- geometry caches;
- texture caches;
- GPU resource lifecycle;
- batching;
- draw-call reduction;
- level of detail;
- asynchronous asset loading;
- worker/background processing where suitable.

Do not force a specific implementation just because another application uses it.

Instead define **performance invariants and architectural boundaries** that let Gipulo choose the correct implementation.

The user should be able to work on very large documents without constantly thinking about the renderer.

---

# Canvas Overlays Must Not Become Part of Document Geometry

Selection outlines, handles, guides, collaborator cursors, measurements, hover effects, temporary tool previews, snapping lines, and similar UI overlays should be architecturally separated from actual document content.

They may follow document geometry, but they are editor UI.

They should:

- not export;
- not accidentally become selectable document objects;
- not modify layout;
- not change document bounds;
- not become part of saved design geometry unless explicitly intended.

Research screen-space overlay layers and define rules around them.

---

# History, Undo/Redo and Transactions

Every canvas mutation needs clear history rules.

Cover:

- pointer drag as one history operation;
- continuous resize as one operation;
- continuous rotate as one operation;
- vector handle drag as one operation;
- text editing;
- multi-property changes;
- cancellation;
- failed operations;
- temporary previews.

Do not create hundreds of undo steps while the pointer moves during one drag.

Temporary hover/preview states should normally not enter document history.

---

# Determinism and Serialization

Canvas behavior should be deterministic.

The same saved `.gip` document should reconstruct the same geometry and visual result.

Create rules for:

- serialization precision;
- IDs;
- ordering;
- floating-point values;
- parametric data;
- paths;
- transforms;
- effects;
- media references;
- migrations.

Do not save temporary editor UI states as document geometry unless intentionally part of the file format.

---

# Accessibility and Keyboard Operation

Canvas performance must not mean ignoring accessibility.

Research how canvas-based applications can provide:

- keyboard navigation;
- focus management;
- semantic descriptions;
- screen-reader representations;
- accessible editing commands.

Figma has publicly discussed using a synchronized semantic/Mirror DOM approach because canvas rendering does not automatically receive normal DOM accessibility.

We do not need to blindly copy that implementation, but Canvas Rules should define accessibility as an architectural requirement instead of something added at the end.

---

# Testing Rules for Canvas Changes

This section is extremely important.

Every canvas-related change should identify which tests are required.

Tests should eventually cover:

- geometry;
- hit-testing;
- coordinate conversion;
- transforms;
- nested transforms;
- selection;
- hover;
- drag;
- resize;
- rotate;
- snapping;
- vector points;
- Bézier handles;
- zoom;
- pan;
- high zoom;
- low zoom;
- fractional values;
- negative coordinates;
- DPI;
- text;
- masks;
- effects;
- export parity;
- undo/redo;
- performance.

For interaction bugs, regression tests should be created wherever practical.

The rule should be:

> If a difficult canvas bug has been fixed once, we should do everything practical to stop exactly that bug from silently returning six months later.

---

# Performance Budgets

Do not only write “keep 60 FPS.”

Research realistic state-of-the-art performance expectations and create measurable budgets or benchmarking rules for:

- pan;
- zoom;
- hover;
- pointer latency;
- dragging;
- resizing;
- vector editing;
- large selections;
- complex paths;
- large documents;
- effects;
- images;
- videos;
- text;
- memory;
- file loading.

Performance tests should use representative heavy documents, not only an empty canvas with one rectangle.

---

# IMPORTANT — Research Interaction Micro-Details

The canvas rules must go down to micro-level behavior.

For example:

- exact activation areas;
- whether thresholds are screen-pixel invariant;
- overlap priority between resize and rotate zones;
- when hover begins;
- when hover ends;
- drag threshold before click becomes drag;
- what happens on pointer release;
- what Escape cancels;
- what modifier keys do while already dragging;
- whether a control remains active after mouse release;
- how snapping releases;
- how points behave at 5% zoom vs 3200% zoom;
- how tiny objects remain selectable;
- how fractional geometry is displayed;
- how nested transforms are resolved.

These details are what make the difference between a canvas that technically “works” and a canvas that feels like Figma-level professional software.

---

# Rule About Research vs Assumption

Do not invent Figma internals that are not publicly known.

Separate findings into:

- **publicly documented architecture**;
- **directly observable product behavior**;
- **recommended Gipulo architecture**;
- **engineering inference**.

If something is an inference, clearly treat it as an inference.

Do not present assumptions as if Figma officially confirmed them.

---

# Final Requirement

The final `canvas-rules.md` should be **extremely detailed**, because the Gipulo canvas will eventually support a huge number of features.

However, it should not become cluttered with repeated generic advice.

It should be structured so an AI agent can quickly locate the exact rules for the subsystem it is modifying.

The document should function as:

1. **an architectural contract;**
2. **an interaction contract;**
3. **a rendering contract;**
4. **a geometry contract;**
5. **a performance contract;**
6. **a regression-prevention system for AI agents.**

Most importantly:

**No AI agent should be allowed to touch a critical canvas system without first understanding the existing canvas architecture, canonical coordinate/geometry systems, interaction rules, and the neighboring features that could be affected.**

The goal is that as Gipulo becomes larger and larger, the canvas becomes **more stable**, not more fragile.

The canvas can become extremely complex in capability, but its underlying rules must remain extremely clear, centralized, reusable, predictable, and difficult for an AI agent to accidentally violate.
