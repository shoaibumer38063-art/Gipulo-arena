# 02 — The 5 Problems & 5 Solutions (Gipulo's differentiators)

> Distilled from `docs/source/gipulo_project_vision_refined_v2.md`. These define WHY Gipulo
> exists beyond parity with Figma. Every future feature decision should respect these intents.

---

## Problem 1 — Complex shapes break inside responsive layouts

- Easy to create complex irregular shapes in Figma: card with a cutout · asymmetrical card ·
  button with an inverted corner · section with custom curved geometry · container with an
  irregular silhouette.
- The problem starts when these shapes sit inside **Auto Layout** or their **parent frame is
  resized**: if the full vector stretches, the whole geometry stretches with it, so:
  - custom curves become distorted;
  - cutouts become wider/narrower;
  - inverted curves lose intended proportions;
  - decorative corners become stretched;
  - visual balance breaks;
  - the responsive result no longer looks like the original design.
- Manual workarounds exist but fail on complex shapes: nested frames · constraints · multiple
  shapes · masks · absolutely positioned layers · different components per breakpoint ·
  9-slice-like techniques · manual reconstruction of parts. A small mistake can break the whole
  object; maintaining many complex responsive shapes this way becomes difficult.

### Solution 1 — Responsive Vector Segments (signature feature)

Core idea: **responsiveness affects ONLY the straight vector segments the designer explicitly
allows to stretch.**

Behavior flow:
1. User selects a complex vector shape.
2. Enables a toggle called something like **"Responsive Vector Segments"**.
3. Gipulo **detects the eligible straight segments** of that vector.
4. Eligible straight segments are **visually highlighted (e.g. red)**.
5. User **clicks the exact straight segments** allowed to respond to resizing.
6. Activated segments **turn green**.
7. **Only selected segments expand/contract**; the rest of the shape is geometrically preserved.

Guarantees: curves stay unchanged · cutout geometry stays unchanged · inverted corners stay
unchanged · decorative parts remain solid · custom curves never stretch · only selected straight
paths absorb the dimensional change.

Example: a card whose left side has a complex curved cutout and whose middle is a long straight
horizontal section — Gipulo stretches only the straight section; the curved part stays identical.

Positioning rules (user's explicit instructions):
- **Do NOT describe this as Flutter `CustomClipper`** (that is for custom clipping paths).
- It IS closer to: **selective stretch regions / constrained vector resizing / 9-slice-style
  behavior applied directly to editable vector segments.**
- It must be **much more visual and easier** than traditional 9-slice: the user never calculates
  slices manually — they simply select which parts of the vector may respond.

---

## Problem 2 — Design-to-code should be native, not an afterthought

- Honest baseline (user's words): Figma DOES have dev/code features today — Dev Mode · code
  snippets · Code Connect · MCP workflows · Figma Make · code-based workflows · developer
  handoff. It would be wrong to claim Figma has no design-to-code system.
- Gipulo's different goal: **design-to-code planned from the beginning of the editor
  architecture**, not separate from the design engine. When creating ANY feature, simultaneously
  ask: *"How will this exact feature be represented in production code?"*
- Every supported design feature must eventually have a clear code equivalent, e.g.:
  Auto Layout behavior · responsive vector behavior · typography · components · variables ·
  interactions · layout constraints · scroll behavior · 3D placement (where supported).
- The goal is NOT random generated HTML after design completion. **Design system and code system
  are developed together.**

### Solution 2 — Native Design-to-Code System + Code Workspace

- Output must look like a **real software project**, not one HTML file / one CSS file / one huge
  generated component / unreadable generated code. Depending on output target, Gipulo creates a
  proper structure: folders · components · styles · tokens · reusable elements · assets · files ·
  routes where needed · dependencies where needed · structured naming.
- **Code Workspace** layout: design canvas on the LEFT, code/runtime area on the RIGHT.
- Two main sub-sections inside Code:
  1. **Code** — the actual generated project structure (folders, files, components, assets,
     styles, configuration, reusable code).
  2. **Preview** — CRITICAL: **not another rendering of the design canvas**. It shows the actual
     result of generated/compiled code running in a real runtime.
- The distinction that must be preserved forever:
  - **Design Canvas = Gipulo's internal editor renderer.**
  - **Code Preview = actual generated code running in a real runtime.**
- Why: the designer can compare "does the coded result look and behave exactly like the design?"
  Mismatches become immediately visible → a much stronger way to test code-translation accuracy.
- Eventual goal: a more advanced **design↔code relationship** where supported code-side changes
  can be synchronized carefully BACK into the design system **without corrupting the document
  structure**.

---

## Problem 3 — 3D objects are now important in modern UI

- Modern landing pages/product sites increasingly use: 3D product objects · floating 3D icons ·
  abstract 3D shapes · interactive 3D scenes · metallic objects · glass objects · spatial product
  visualizations · scroll-driven 3D elements.
- Figma itself is mainly a **2D** design environment.
- Constraint: Gipulo must **NOT become a full modeling application** (like Blender, Cinema 4D, or
  Spline) — that would create unnecessary complexity.

### Solution 3 — Simple native 3D support + external app connection

- 3D objects are supported **mainly as design assets**, imported from tools such as **Blender,
  Spline, Cinema 4D**, and other supported 3D tools/formats.
- Inside Gipulo, only UI/UX-relevant controls: move · rotate · scale · basic positioning ·
  camera-related presentation controls where required · simple display/material controls where
  required · interaction behavior · animation/interaction settings where supported.
- **Live Link / Source Link system** (very important feature). Flow:
  1. 3D object created inside Blender.
  2. Object linked/imported into Gipulo.
  3. Designer later changes the original object in Blender.
  4. Designer saves the Blender file.
  5. Gipulo **detects the source changed**.
  6. Gipulo **updates the 3D object**.
  7. Existing Gipulo-side positioning, scale, interaction settings, and layout behavior
     **remain intact where possible**.
- Implementation options (decide later): plugins · file watchers · source-file linking ·
  interchange formats · background bridge applications · APIs where available.
- Governing principle: **Gipulo should not try to replace professional 3D software. It should
  make 3D objects easy to use inside UI design.**

---

## Problem 4 — Advanced parallax and scroll-based design

- Modern websites increasingly use scroll-driven interaction: parallax · scroll-linked
  transforms · pinned sections · object movement based on scroll progress · scale on scroll ·
  rotation on scroll · blur on scroll · opacity changes · layered depth · scene transitions ·
  scroll-controlled 3D movement.
- Honest baseline: Figma's ecosystem has MORE scroll/parallax support than it used to (especially
  via Figma Sites). It is not correct to say Figma has no parallax support at all.
- Gipulo's opportunity: make scroll interaction **much deeper and more directly connected to the
  design canvas**.

### Solution 4 — Native Scroll Interaction System

- A proper **visual** scroll-interaction system. Designer connects scroll progress to properties:
  X/Y position · scale · rotation · opacity · blur · mask position · path progress · 3D position ·
  3D rotation · layer depth · clipping · reveal states.
- Designer can define: start point · end point · trigger · scroll range · easing · pinning ·
  direction · timeline · interaction sequence.
- Goal: **strong control WITHOUT manually writing JavaScript** for normal scroll-based interactions.

---

## Problem 5 — Advanced features can become too expensive

- Figma has a free tier, but many advanced professional, developer, collaboration, organization,
  administration, and AI-related features require paid plans or specific seat types.
- For professional teams that can make sense — but not every designer, student, freelancer, small
  team, or user can afford expensive subscriptions.

### Solution 5 — Keep the core professional experience accessible

- Gipulo should keep as much of the **core professional design experience** accessible as possible.
- Gipulo still needs a **sustainable business model**. Expensive ongoing services MAY have paid
  limits/subscriptions: AI inference · cloud rendering · large cloud storage · enterprise
  collaboration · hosted build infrastructure · premium team features.
- Main idea: **basic professional design capabilities must not become artificially restricted.**

---

## Beyond the five

- The user states these are only SOME of the main problems; **many other things can eventually be
  improved.** Gipulo's goal is not to copy Figma exactly, but: first match Figma to a
  comfortable working level, then solve the problems that make advanced design unnecessarily
  difficult (see `01_project_identity_and_vision.md`).
