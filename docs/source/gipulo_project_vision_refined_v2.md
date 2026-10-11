# Gipulo — Vision, Problems, Solutions, and AI-First Development Foundation

I am a UI/UX designer, and I want to create a **1-to-1, production-level alternative to Figma**, named **Gipulo (`.gip`)**.

I do **not** want to create just a simple design tool with a few Figma-like features. I want to build a huge, serious, production-level application with a proper custom engine for vector geometry, canvas interaction, selection, transforms, paths, layout, rendering, and all the other systems required in a professional design application.

I know Figma is an extremely complex software product. It is not something that would normally be created or maintained by one person. A product at this level normally requires a large team of highly experienced UI/UX designers, product designers, senior frontend engineers, graphics engineers, C++/WebAssembly engineers, rendering engineers, infrastructure engineers, QA engineers, and many other specialists.

But I am mainly a UI/UX designer, not a coder.

At the same time, I strongly believe that modern AI agents are now capable enough to help with a very large amount of the work that previously required a full team, **but only if the entire project is properly structured, documented, tested, and controlled**.

The goal is not to blindly tell AI to “build a Figma clone.”

The goal is to create such a strong foundation, documentation system, rule system, memory system, testing system, and architecture that multiple AI agents can work on a very large codebase without constantly forgetting context, duplicating logic, breaking old features, or creating unnecessary code.

---

# Problem Statement — Why I Want to Create Gipulo

Nowadays, as a UI/UX designer, I am facing many design problems because a large amount of modern product design is being created while keeping younger audiences, especially Gen Z, in mind.

A lot of users are tired of seeing the same traditional layouts, cards, sections, rectangles, and predictable UI structures everywhere. Modern products are increasingly using much more expressive interfaces.

As designers, we are now expected to create things that feel:

- more unique;
- more custom-crafted;
- more experimental;
- more visually impressive;
- more dynamic;
- more interactive;
- less template-like;
- and more “out of the box.”

Because of this, we often need to create complex layouts that break normal geometric rules and normal layout patterns.

For example:

- asymmetrical cards;
- irregular sections;
- non-rectangular containers;
- complex cutouts;
- inverted curves;
- concave corners;
- custom masks;
- overlapping layouts;
- broken-grid compositions;
- curved sections;
- irregular buttons;
- custom responsive shapes;
- 3D objects;
- motion-based layouts;
- scroll-based interactions.

Creating these kinds of designs as static graphics is possible in Figma.

The real problem appears when we need to make them **properly responsive**.

We are not creating static posters. We are creating websites, applications, dashboards, mobile applications, and digital products that must work across many different screen sizes.

This is where many complex visual ideas start breaking.

---

# Problem 1 — Complex Shapes Break Inside Responsive Layouts

It is easy to create a complex irregular shape in Figma.

For example, we can create:

- a card with a cutout;
- an asymmetrical card;
- a button with an inverted corner;
- a section with custom curved geometry;
- a container with an irregular silhouette.

But the problem starts when these shapes are placed inside Auto Layout or when their parent frame is resized.

If the full vector shape stretches, then the whole geometry stretches with it.

This means:

- custom curves become distorted;
- cutouts become wider or narrower;
- inverted curves lose their intended proportions;
- decorative corners become stretched;
- the visual balance starts breaking;
- the final responsive result no longer looks like the original design.

There are manual ways to handle some of these situations.

For example, designers can sometimes use:

- nested frames;
- constraints;
- multiple shapes;
- masks;
- absolutely positioned layers;
- different components for different breakpoints;
- 9-slice-like techniques;
- manual reconstruction of individual parts.

But these approaches become difficult when the shape itself is complex.

A small mistake can break the whole object.

For a designer who wants to create many different complex responsive shapes, this quickly becomes difficult to maintain.

---

## Solution 1 — Responsive Vector Segments

I want Gipulo to have a dedicated feature for this.

The basic idea is:

**Responsiveness should affect only those straight vector segments that the designer explicitly allows to stretch.**

For example:

1. The user selects a complex vector shape.
2. A toggle called something like **Responsive Vector Segments** is enabled.
3. Gipulo detects the eligible straight segments of that vector.
4. These straight segments are visually highlighted, for example in red.
5. The user clicks the exact straight segments that are allowed to respond to resizing.
6. Activated segments turn green.
7. Only those selected segments are allowed to expand or contract.
8. The rest of the shape remains geometrically preserved.

This means that:

- curves stay unchanged;
- cutout geometry stays unchanged;
- inverted corners stay unchanged;
- decorative parts remain solid;
- custom curves do not get stretched;
- only the selected straight paths absorb the dimensional change.

For example, imagine a card where the left side contains a complex curved cutout, but the middle area is just a long straight horizontal section.

Instead of stretching the whole card, Gipulo should stretch only that straight section.

The complex curved part would remain exactly the same.

This would allow designers to build unusual shapes that remain visually stable inside responsive layouts.

This idea should not be described as the same thing as Flutter `CustomClipper`.

`CustomClipper` is mainly used to define custom clipping paths. The Gipulo feature is closer to **selective stretch regions / constrained vector resizing / 9-slice-style behavior applied directly to editable vector segments**.

However, Gipulo should make this much more visual and easier to use than traditional 9-slice systems.

The user should not have to manually calculate slices.

They should simply select which parts of the vector are allowed to respond.

---

# Problem 2 — Design-to-Code Should Be Native, Not an Afterthought

Another major problem is design-to-code.

Figma today does have many developer and code-related features, so it would be wrong to say that Figma has no design-to-code system.

Figma currently has things such as:

- Dev Mode;
- code snippets;
- Code Connect;
- MCP workflows;
- Figma Make;
- code-based workflows;
- developer handoff features.

However, my goal for Gipulo is different.

I want design-to-code to be planned **from the beginning of the editor architecture**, not treated as something separate from the design engine.

When we create a feature inside Gipulo, we should also think at the same time:

> How will this exact feature be represented in production code?

This means every supported design feature should eventually have a clear code equivalent.

For example:

- Auto Layout behavior should have a code representation.
- Responsive vector behavior should have a code representation.
- Typography should have a code representation.
- Components should have a code representation.
- Variables should have a code representation.
- Interactions should have a code representation.
- Layout constraints should have a code representation.
- Scroll behavior should have a code representation.
- 3D placement should eventually have a code representation where supported.

The goal should not be to generate some random HTML code after the design is complete.

The goal should be that the design system and the code system are developed together.

---

## Solution 2 — Native Design-to-Code System

The code output should look like a real software project.

It should not just be:

- one HTML file;
- one CSS file;
- one huge generated component;
- or a large amount of unreadable generated code.

Depending on the selected output target, Gipulo should create a proper project structure with things such as:

- folders;
- components;
- styles;
- tokens;
- reusable elements;
- assets;
- files;
- routes where needed;
- dependencies where needed;
- structured naming.

There should also be a dedicated **Code Workspace**.

When the user opens the Code section, the interface could work something like this:

- design canvas on the left;
- code/runtime area on the right.

Inside the Code section there should be at least two main sub-sections:

### 1. Code

This should show the actual generated project structure.

For example:

- folders;
- files;
- components;
- assets;
- styles;
- configuration;
- reusable code.

### 2. Preview

This preview must be very important.

The preview should **not simply be another rendering of the design canvas**.

It should show the actual result of the generated/compiled code.

That means:

**Design Canvas = Gipulo's internal editor renderer**

**Code Preview = actual generated code running in a real runtime**

This is important because then the designer can compare:

> Does the actual coded result look and behave exactly like the design?

If there is a mismatch, it becomes immediately visible.

This also gives us a much stronger way to test whether code translation is accurate.

Eventually, the goal can become a more advanced design↔code relationship, where supported code-side changes can also be synchronized carefully back into the design system without corrupting the document structure.

---

# Problem 3 — 3D Objects Are Now Important in Modern UI

3D is becoming a very important part of modern websites and UI design.

Modern landing pages and product websites increasingly use:

- 3D product objects;
- floating 3D icons;
- abstract 3D shapes;
- interactive 3D scenes;
- metallic objects;
- glass objects;
- spatial product visualizations;
- scroll-driven 3D elements.

Figma itself is mainly a 2D design environment.

For Gipulo, I do not want to build a full modeling application like Blender, Cinema 4D, or Spline.

That would create unnecessary complexity.

---

## Solution 3 — Simple Native 3D Support + External App Connection

Gipulo should support 3D objects mainly as design assets.

The user should be able to import 3D assets from tools such as:

- Blender;
- Spline;
- Cinema 4D;
- and other supported 3D tools or formats.

Inside Gipulo, we only need the controls that are important for UI/UX design.

For example:

- move;
- rotate;
- scale;
- basic positioning;
- camera-related presentation controls where required;
- simple display/material controls where required;
- interaction behavior;
- animation/interaction settings where supported.

A very important feature should be a **Live Link** or **Source Link** system.

For example:

1. A 3D object is created inside Blender.
2. The object is linked or imported into Gipulo.
3. The designer later changes the original object in Blender.
4. The designer saves the Blender file.
5. Gipulo detects that the source has changed.
6. Gipulo updates the 3D object.
7. Existing Gipulo-side positioning, scale, interaction settings, and layout behavior should remain intact where possible.

The exact implementation can later use:

- plugins;
- file watchers;
- source-file linking;
- interchange formats;
- background bridge applications;
- or APIs where available.

The main point is:

**Gipulo should not try to replace professional 3D software. It should make 3D objects easy to use inside UI design.**

---

# Problem 4 — Advanced Parallax and Scroll-Based Design

Modern websites increasingly use scroll-driven interaction.

For example:

- parallax;
- scroll-linked transforms;
- pinned sections;
- object movement based on scroll progress;
- scale on scroll;
- rotation on scroll;
- blur on scroll;
- opacity changes;
- layered depth;
- scene transitions;
- scroll-controlled 3D movement.

Figma's ecosystem now has more scroll and parallax support than it used to, especially through Figma Sites.

So it would not be correct to say that Figma has no parallax support at all.

The opportunity for Gipulo is to make scroll interaction much deeper and more directly connected to the design canvas.

---

## Solution 4 — Native Scroll Interaction System

Gipulo should eventually have a proper visual scroll-interaction system.

A designer should be able to connect scroll progress to properties such as:

- X/Y position;
- scale;
- rotation;
- opacity;
- blur;
- mask position;
- path progress;
- 3D position;
- 3D rotation;
- layer depth;
- clipping;
- reveal states.

The designer should be able to define things like:

- start point;
- end point;
- trigger;
- scroll range;
- easing;
- pinning;
- direction;
- timeline;
- interaction sequence.

The important goal is to give designers strong control without requiring them to manually write JavaScript for normal scroll-based interactions.

---

# Problem 5 — Advanced Features Can Become Too Expensive

Figma has a free tier, but many advanced professional, developer, collaboration, organization, administration, and AI-related features require paid plans or specific seat types.

For professional teams this can make sense.

But not every designer, student, freelancer, small team, or user can afford expensive subscriptions.

Gipulo should try to keep as much of the **core professional design experience** accessible as possible.

Of course, Gipulo will still need a sustainable business model.

Expensive ongoing services such as:

- AI inference;
- cloud rendering;
- large cloud storage;
- enterprise collaboration;
- hosted build infrastructure;
- premium team features;

can still have paid limits or subscriptions.

The main idea is that basic professional design capabilities should not become artificially restricted.

---

# There Are Many Other Missing Opportunities

These are only some of the main problems.

There are many other things that can eventually be improved.

The goal of Gipulo should not simply be:

> Copy Figma exactly.

The goal should be:

> First reach the level where a serious Figma user can comfortably work inside Gipulo, then solve the important problems that still make advanced design unnecessarily difficult.

---

# Before Development — Build the AI Development Foundation

Before we start aggressively adding features, the first goal should be to create the **perfect agent setup, project memory system, rule system, and documentation system**.

This is extremely important.

If we start creating hundreds of features without this foundation, AI agents will eventually:

- forget previous decisions;
- recreate existing functions;
- add duplicate logic;
- create unnecessary files;
- add unnecessary dependencies;
- break old features;
- change architecture without understanding it;
- create inconsistent UI;
- create conflicting canvas logic;
- create temporary patches instead of real fixes;
- forget why something was implemented in a certain way.

For a project as large as Gipulo, that would become a disaster.

So the documentation system must be treated as part of the engineering architecture.

---

# Production-Level Documentation System

The documentation should be created like a real production company would document a large product.

Large software companies document:

- features;
- behavior;
- implementation;
- architecture;
- APIs;
- edge cases;
- limitations;
- testing;
- known issues;
- product decisions;
- onboarding information.

This is important for human teams because when a new developer or designer joins in the middle of production, that person needs to understand the system without asking the original team about every small detail.

In our case, this is even more important because we will be working heavily with AI agents.

Every new AI session should be able to understand the relevant part of the project without guessing.

---

# 1. Feature Documentation

Every major feature should have its own Markdown documentation.

For example:

```text
/docs/features/vector-editing/
/docs/features/auto-layout/
/docs/features/components/
/docs/features/groups/
/docs/features/frames/
/docs/features/text/
/docs/features/responsive-vectors/
```

Each feature document should explain things such as:

- what the feature is;
- why it exists;
- user behavior;
- UI behavior;
- interaction states;
- keyboard behavior;
- mouse/pointer behavior;
- selection behavior;
- expected canvas behavior;
- dependencies;
- related systems;
- implementation notes;
- important rules;
- edge cases;
- known bugs;
- limitations;
- test cases;
- regression risks.

Not every feature requires exactly the same headings, but the documentation should be detailed enough that another agent can understand the feature without reconstructing it from the code.

---

# 2. Feature Index / Feature Registry

There should be a separate central `.md` file that works as the **master index of all Gipulo features**.

This should include:

- completed features;
- partially completed features;
- planned features;
- experimental features;
- deprecated features;
- removed features.

For each feature, it should point to the related documentation and major code location.

This file must be updated whenever:

- a feature is added;
- a feature is removed;
- a feature changes status;
- a major feature behavior changes.

This will help the agent understand what already exists before creating something new.

---

# 3. Debugging and Fix Documentation

There should be a separate documentation system specifically for bugs, fixes, and regressions.

This is very important.

For example, suppose we spend many hours fixing a difficult canvas bug related to:

- transform coordinates;
- zoom;
- hit-testing;
- vector point selection;
- bounding boxes.

Six months later, a similar bug appears.

The AI agent should not start from zero.

It should be able to search the previous debugging history and understand:

- what the symptoms were;
- what caused the issue;
- what was attempted;
- what actually fixed it;
- which files were involved;
- what should not be changed;
- what regression test was added.

A useful bug record should include:

```text
Bug:
Symptoms:
Affected Feature:
Root Cause:
Fix:
Files Changed:
Tests Added:
Regression Risks:
Verification:
Related Bugs:
```

This creates a proper debugging memory for the project.

---

# 4. User Prompt Archive

There should also be a separate archive for important user prompts.

This can be `.txt`, `.md`, or another searchable plain-text format.

This is important because sometimes one prompt contains multiple details that are not fully represented by the final implementation.

For example, one prompt may contain:

- two bugs;
- one visual correction;
- one new feature request;
- one behavior rule;
- one comparison with Figma.

If the prompt disappears from project history, we may lose the original intent.

So important prompts should be stored and indexed with things such as:

- date;
- task ID;
- feature;
- subsystem;
- related bug;
- keywords;
- related documentation.

The prompt archive should not replace proper documentation.

It should be a historical reference showing **how and why I guided the AI toward a certain result**.

---

# 5. Agent Task Summary / Reasoning Summary

I originally wanted a separate documentation system for AI thinking.

The important part should not be storing private raw chain-of-thought.

Instead, after an important task, the agent should save a structured **Task Summary / Decision Summary**.

This should explain:

- what problem it was solving;
- what it inspected;
- what approach it selected;
- why that approach was selected;
- what alternatives were rejected;
- what files were changed;
- what behavior changed;
- how it verified the result;
- what risks remain;
- what should be done next if the problem returns.

This gives future agents the useful reasoning context without depending on hidden internal thought processes.

---

# 6. Architecture Documentation

There should be dedicated documentation for the core architecture.

This is separate from normal feature documentation.

For example:

```text
/docs/architecture/canvas.md
/docs/architecture/vector-engine.md
/docs/architecture/rendering.md
/docs/architecture/document-tree.md
/docs/architecture/selection-system.md
/docs/architecture/coordinate-system.md
/docs/architecture/transforms.md
/docs/architecture/layout-engine.md
/docs/architecture/input-system.md
/docs/architecture/undo-redo.md
/docs/architecture/file-format.md
/docs/architecture/code-generation.md
```

This is extremely important because multiple features may depend on the same core system.

For example, changing coordinate conversion may affect:

- selection;
- resize;
- rotate;
- vector editing;
- snapping;
- rulers;
- guides;
- comments;
- plugins;
- exports.

AI should always know which systems are foundational.

---

# 7. Architecture Decision Records

For important architecture decisions, there should be short permanent records explaining:

- what decision was made;
- why it was made;
- what alternatives were considered;
- what consequences it creates;
- whether the decision is still active.

This prevents a future agent from replacing an important system simply because it did not understand why it existed.

---

# 8. Testing and Regression Documentation

Every important feature and difficult bug fix should have tests wherever practical.

The project should maintain tests for systems such as:

- vector geometry;
- path editing;
- coordinate conversion;
- hit-testing;
- selection;
- resize;
- rotate;
- snapping;
- zoom;
- canvas input;
- Auto Layout;
- responsive behavior;
- undo/redo;
- file saving/loading;
- code generation;
- generated preview;
- performance.

When a bug is fixed, a regression test should be added whenever possible.

The purpose is:

> The same bug should not be allowed to silently return later.

---

# 9. Performance Documentation

Performance is extremely important in a design editor.

A feature is not truly working if it technically works but creates lag.

The project should define performance expectations for things such as:

- canvas FPS;
- pan;
- zoom;
- pointer response;
- selection;
- large documents;
- vector editing;
- drag operations;
- memory use;
- file loading;
- file saving;
- generated-code preview.

AI agents should not be allowed to solve one feature by creating a major performance problem somewhere else.

---

# Documentation Update Rule

This is one of the most important rules:

**Documentation must be updated at the end of every meaningful task.**

But this does not mean rewriting every document after every task.

The agent should update only the documents affected by the task.

For example, after completing a task, the agent should check:

- Did a feature change?
- Did behavior change?
- Did architecture change?
- Was a bug fixed?
- Was a new regression test added?
- Did a rule change?
- Did a file structure change?
- Did the feature index need updating?
- Should the task summary be recorded?
- Should the prompt be archived?

This documentation update should become part of the normal completion process for every task.

---

# After the Foundation — Build the Design System

After the multi-layer documentation, memory, testing, and rule foundation is properly created, the first major visible system should be the **Gipulo Design System**.

The UI should remain highly familiar to Figma users.

Why?

Because many designers have spent years learning Figma.

If Gipulo completely changes:

- toolbar locations;
- layer panel behavior;
- properties panel structure;
- selection behavior;
- navigation;
- editing patterns;

then users have to learn a completely new tool.

That creates unnecessary friction.

So the basic editor layout and interaction logic should remain familiar.

For example:

- layer panel on the left;
- canvas in the center;
- properties panel on the right;
- familiar toolbar placement;
- familiar selection interaction;
- familiar frame/group/component concepts where appropriate.

However, the visual identity should still be Gipulo.

The accent colors, details, styling, polish, branding, and certain interaction improvements can be different.

The goal is:

**Familiar workflow, different identity.**

The design system must be created before hundreds of UI features are implemented so that different AI agents do not create inconsistent buttons, panels, spacing, radii, icons, states, tooltips, menus, inputs, and dialogs.

---

# AI Rules System

The next critical foundation is the AI rule system.

There should not be only one giant `rules.md` file.

Different systems require different rules.

For example:

```text
/rules/global.md
/rules/code-quality.md
/rules/canvas.md
/rules/vector-geometry.md
/rules/rendering.md
/rules/ui.md
/rules/layout.md
/rules/icons.md
/rules/interactions.md
/rules/state-management.md
/rules/performance.md
/rules/feature-addition.md
/rules/bug-fixing.md
/rules/refactoring.md
/rules/feature-removal.md
/rules/testing.md
/rules/documentation.md
```

There may be more rule categories later.

That is expected.

---

# Canvas Rules Are Especially Important

The canvas rules need to be extremely strict because the canvas is one of the most dangerous areas to change.

A small bad change can affect many systems.

Canvas rules should cover things such as:

- world coordinates;
- screen coordinates;
- local coordinates;
- parent coordinates;
- transform matrices;
- zoom;
- pan;
- hit-testing;
- selection bounds;
- object bounds;
- vector bounds;
- pointer tolerance;
- screen-pixel-invariant interactions;
- resize handles;
- rotate handles;
- snapping;
- guides;
- overlays;
- input priority;
- nested transforms;
- viewport rendering;
- invalidation;
- rendering order.

When an AI agent is working on the canvas, it should first read the relevant canvas rules before touching the implementation.

---

# Code Quality Rules

The codebase should remain as minimal and clean as realistically possible.

But “minimal code” should **not** mean:

- forcing everything into one file;
- making unreadable one-line logic;
- avoiding useful abstractions;
- removing comments that explain important behavior;
- combining unrelated systems only to reduce line count.

The real goal is:

- no duplicated logic;
- no unnecessary abstractions;
- no unnecessary files;
- no unnecessary dependencies;
- no dead code;
- no temporary hacks left permanently;
- no duplicated geometry systems;
- no duplicate state;
- no random architecture;
- no unnecessary wrappers;
- no clutter.

Before creating new code, the AI should check whether the required logic already exists.

If it exists, reuse or extend it.

Do not create another implementation of the same idea in another part of the codebase.

---

# Feature Addition Rules

Before adding a feature, the agent should:

1. read the feature index;
2. inspect related existing features;
3. inspect relevant architecture documents;
4. inspect relevant rules;
5. search the codebase for reusable logic;
6. identify affected systems;
7. plan the smallest correct implementation;
8. implement it;
9. test it;
10. verify neighboring behavior;
11. update documentation.

---

# Bug Fixing Rules

For bug fixing, the agent should not immediately start changing code.

It should first:

1. reproduce the bug;
2. identify the exact expected behavior;
3. inspect related bug history;
4. identify the real root cause;
5. avoid symptom-only patches;
6. make the smallest safe fix;
7. test the original problem;
8. test related behavior;
9. add regression coverage where practical;
10. update the bug documentation.

---

# Removal Rules

When a feature or old implementation is removed, the agent should also check for:

- dead imports;
- dead styles;
- dead tests;
- dead documentation;
- unused dependencies;
- obsolete feature flags;
- old compatibility code;
- old state;
- old event listeners;
- unreachable code.

Removing a feature should not leave half of that feature scattered around the project.

---

# IMPORTANT — These Rules Must Be Extremely High Quality

These rule systems should be treated like internal engineering standards of a serious software company.

They should not contain generic AI instructions such as:

> Write clean code.

They should explain exactly what “clean” means inside Gipulo.

For example:

- how canvas coordinates must be handled;
- where geometry calculations should live;
- how UI state should be separated from document state;
- when to create dependencies;
- how to add features safely;
- what documentation must change;
- how to verify performance;
- how to avoid duplicate logic.

The rules should become a real engineering operating system for both AI agents and future human developers.

---

# Final Principle

There are two things I want at the same time.

## 1. Extremely Detailed Documentation

The documentation should be detailed enough that:

- a new AI agent;
- a new senior engineer;
- a new designer;
- or a future human team

can enter the project and understand what is happening without rebuilding the entire context from scratch.

## 2. Extremely Clean and Minimal Code

At the same time, I want the actual codebase to remain as minimal and clean as possible.

Not minimal only by line count.

Minimal in terms of:

- duplication;
- unnecessary logic;
- unnecessary files;
- unnecessary dependencies;
- unnecessary abstractions;
- dead code;
- clutter;
- conflicting systems.

The project may become extremely large.

That is expected.

But the complexity should come from the **capability of the product**, not from a messy codebase.

The final goal is to build Gipulo in a way where advanced features can continue to grow without the project becoming impossible for AI agents or human developers to understand and maintain.
