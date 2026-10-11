# 05 — Design System Brief (complete requirements distillation)

> Distilled from `docs/source/Gipulo — Complete Design System Brief.md`. This is the requirement
> spec for the Gipulo Design System. Concrete values (colors/sizes/timings) live in
> `07_design_tokens_and_visual_specs.md` (extracted from the Design System HTML reference).

## Mission

- Gipulo = **1-to-1 Figma-level editor** — a full production-level design tool replicating the
  **depth, behavior, interaction quality, and overall capability of Figma**, PLUS advanced
  features Figma does not offer.
- Huge project, possibly months; user is a UX designer using AI agents → the **complete,
  extremely detailed Design System comes BEFORE building the editor itself**.
- The Design System = the **visual and interaction foundation of the whole application** and the
  **single visual+interaction source of truth for the entire Gipulo application.**
- It must NOT look like a normal SaaS dashboard, simple web app, admin panel, or generic React
  application. Depth benchmark: **Figma · Framer · modern professional creative tools · modern
  2026 productivity/editor applications.**
- Feel: very modern, highly polished, custom-crafted, suitable for **2026**. Visual direction
  highly inspired by Figma/Framer refinement — but **Gipulo keeps its own identity**.

## Scope: EVERYTHING, macro → micro

- Not just colors/typography/buttons/cards/inputs/spacing — that is far too basic.
- The DS controls **almost every visible and interactive part, from major layouts down to
  extremely small micro-interactions.**
- Goal: **almost never depend on browser defaults, React defaults, OS defaults, or random UI
  behavior.** Almost everything gets a purposely designed Gipulo version.
- Component coverage list (every item needs a clear custom design):
  buttons · icon buttons · cards · panels · sections · dropdowns · select menus · context menus ·
  text fields · numeric fields · sliders · toggles · checkboxes · radio buttons · segmented
  controls · tabs · search fields · command palettes · tooltips · popovers · dialogs · modals ·
  notifications · toasts · progress states · loaders · scrollbars · separators · resize handles ·
  drag handles · property rows · lists · tree views · layer items · breadcrumbs · badges · chips ·
  empty states · error states · disabled states · loading states · focus states · keyboard-focus
  states · hover states · pressed states · selected states · dragged states · active states.

### Three levels
- **Macro:** overall editor layout · top toolbar · left sidebar · right properties panel · canvas ·
  floating toolbars · bottom controls · code workspace · preview workspace · modal environments ·
  fullscreen modes · responsive editor layouts.
- **Component:** buttons · fields · cards · controls · panels · tabs · menus · dropdowns · layer
  rows · property rows · tool groups · inspectors · dialogs.
- **Micro:** border thickness · divider opacity · hover backgrounds · text-selection color ·
  cursor icons · resize zones · scrollbar thickness · icon optical alignment · tooltip offsets ·
  focus rings · active indicators · guide thickness · snapping markers · control hit areas · tiny
  spacing values · corner radii · shadows · blur · gradients · state transitions.
- "The small details are extremely important. In an editor this large, users interact with
  thousands of these micro-details every day."

## Avoid cheap default browser/UI behavior

- Nightmare scenario: the editor looks premium, then one small interaction shows cheap default
  browser behavior (e.g. default blue text selection).
- The DS must define: text-selection background · text-selection foreground · focus rings ·
  keyboard focus · active borders · native control replacement · default highlights · disabled
  styling · drag-selection styling. If Gipulo has its own accent color, these system-level
  interactions follow that visual language where technically possible.
- **No default HTML `<select>`**: Gipulo has its own dropdown system matching its typography ·
  spacing · radius · iconography · hover states · active states · search behavior · scroll
  behavior · keyboard behavior · selected states. Same principle across the whole editor.

## Modern 2026 visual direction

- Explore modern treatments where appropriate: dynamic gradients · subtle chromatic accents ·
  premium gradient buttons · modern translucent surfaces · restrained glass effects · layered
  surfaces · soft elevation · refined shadows · subtle border lighting · intelligent hover
  effects · modern active states · smooth state transitions.
- **NOT a flashy neon UI.** Most of the editor stays clean, readable, neutral, professional.
- Expressive treatments reserved for: primary actions · active tools · important states ·
  highlighted controls · selected states · special features · branded moments.
- The editor must remain **comfortable to use for many hours**.

## Typography system

- Define: primary UI typeface · secondary typeface if required · numeric typography · code
  typography · heading styles · labels · property values · input text · tooltips · menu text ·
  tabs · captions · status text · shortcuts · error messages · helper text.
- Also define: sizes · weights · line heights · letter spacing · optical hierarchy · truncation
  behavior · ellipsis behavior · wrapping · numeric alignment.
- Rationale: typography in a professional editor must be extremely controlled — a few pixels of
  unnecessary height repeated across hundreds of controls makes the whole UI feel wrong.

## Icon system

- One complete icon language; agents must NOT randomly pick icons from different packs per feature.
- Define: icon style · stroke width · corner treatment · optical size · bounding box · padding ·
  filled vs outlined usage · active · inactive · disabled · hover · selected · warning ·
  destructive states.
- Different UI areas need different icon sizes: toolbar icons · panel icons · small property
  icons · menu icons · status icons · canvas controls.

## Spacing & density

- Complete spacing system for: panel padding · section gaps · property rows · icon gaps ·
  label/value gaps · toolbar spacing · nested indentation · menu spacing · popovers · dialogs ·
  canvas overlays.
- Density levels if required: **comfortable · compact · ultra-compact** — but **compact must
  never mean difficult to click**: treat visual size and interactive hit area separately where
  necessary.

## Complete state system

- Every component designed for ALL relevant states — never only the normal state.
- Button: default · hover · pressed · active · selected · focus · keyboard focus · disabled ·
  loading · destructive · success.
- Layer item: default · hover · selected · multi-selected · editing name · dragged · drag target ·
  locked · hidden · component · instance · nested · warning/error.
- Property field: default · hover · focus · editing · invalid · mixed values · disabled ·
  inherited value · overridden value.
- This state logic exists throughout the system.

## Canvas UI is part of the Design System (extremely important)

- The canvas itself has a huge visual UI system. The DS defines the look+behavior of:
  object bounding boxes · selection outlines · hover outlines · resize handles · rotate zones ·
  corner-radius handles · vector anchor points · Bézier handles · path outlines · midpoint
  controls · gradient handles · measurement labels · spacing indicators · snapping indicators ·
  ruler guides · grid overlays · text-editing outlines.

## Guide system (color meanings must be fixed, never random)

- **Blue guides / geometry UI:** selected object geometry · bounding boxes · vector paths ·
  anchor points · Bézier handles · object dimensions · active geometry states.
- **Pink/magenta guides:** spacing · equal spacing · distribution · padding · Auto Layout
  measurements · smart spacing relationships.
- **Orange guides:** certain snapping, alignment, measurement, or special interaction states
  (final meaning per Gipulo's language).
- Rule: every guide color must have a researched, clearly defined, consistent meaning. Never
  randomly use blue/orange/pink/red/green across canvas features.
- Define per guide type: line thickness · color · opacity · dash pattern · endpoint · labels ·
  label backgrounds · active · inactive · hover · locked · temporary states.
- Guide appearance must remain usable at different zoom levels.
- (The HTML reference concretized these: blue=geometry `#0d99ff`, magenta=spacing `#ec2fd0`,
  orange=snap/measure `#ff8a00`, red=ruler guides `#f24822` — see `07`.)

## Cursor design system

- Agents must never randomly use whatever cursor seems convenient.
- Define cursors for: default selection · pointer · move · grab · grabbing · resize horizontal ·
  resize vertical · resize diagonal · rotate · scale · pen · vector edit · point add · point
  delete · Bézier editing · text · hand tool · eyedropper · zoom · duplicate · mask · forbidden ·
  drag-copy · crop · 3D orbit (if later required).
- Also define per cursor: hotspot · exact visual size · outline · contrast · behavior on light
  canvas · behavior on dark canvas · state changes · rotation-aware cursor direction · cursor
  while dragging · before activation · while modifier keys are held.
- Example requirement: resizing a ROTATED rectangle → the resize cursor must visually match the
  real resize direction, not a fixed browser cursor.

## Left layer panel (its own complete system)

- Structure: panel width · minimum width · maximum width · resized state · collapsed · expanded ·
  mobile/small-window behavior · section headers · pages · layers · nested layers · indentation ·
  expand/collapse arrows · layer icons · visibility · locking · component states · instance
  states · text layers · vector layers · frames · groups · masks.
- Every interaction state: normal · hover · selected · multi-selected · dragged · drag
  destination · renaming · collapsed · expanded · locked · hidden · disabled · error state.
- Dragging a layer between other layers → purposely designed **insertion indicator**.
- Dragging INSIDE a frame/group → a different clear state.

## Right properties panel (its own full system)

- Structure: widths · resizing · collapsed · responsive state · sections · headers · property
  rows · labels · values · numeric fields · linked values · chain controls · sliders · color
  controls · gradient controls · effect controls · stroke controls · Auto Layout controls ·
  constraints · typography · component properties · prototype/interaction controls.
- Properly designed property states per selection type: nothing selected · shape · frame · group ·
  text · vector · component · instance · image · multiple objects · mixed-property selection.
- The UI **adapts intelligently** instead of showing the same panel for everything.

## Responsive application UI (the editor itself)

- Define behavior when the window is: very wide · normal desktop · laptop · small laptop ·
  tablet-sized · narrow.
- Answer: when does the layer panel shrink · when can it collapse · when does the properties panel
  narrow · when do labels turn into icons · when do tool groups collapse · when do floating
  controls move · when is horizontal scrolling acceptable · which areas must preserve minimum
  width.
- The editor must NOT compress everything until controls become unusable.

## Toolbar system

- Rules for: placement · grouping · spacing · separators · tool states · active tool · hover ·
  selected · submenu · long-press · keyboard shortcut display · compact layouts · responsive
  layouts.
- Contextual controls for specific tools must follow the same DS — no one-off floating UIs.

## Menus, popovers, context menus

- Must not use generic defaults. Define: menu width · padding · item height · icons · shortcuts ·
  nested menus · separators · destructive items · selected items · disabled items · search ·
  checkmarks · submenus · hover · keyboard navigation · scrolling · viewport collision behavior.
- Same visual language across: dropdowns · context menus · toolbar menus · command palettes ·
  property menus.

## Motion & interaction design

- Small interface motion (NOT large decorative animation everywhere): hover transitions · button
  press · panel opening · dropdown opening · tooltip appearance · menu transition · drag feedback ·
  snapping feedback · selected-state transition · toggle transition · slider movement.
- Motion must feel **fast and responsive — never make the editor feel slow**.
- Define: duration ranges · easing · spring usage · when animation should NOT be used ·
  reduced-motion behavior.

## Light & dark surfaces (theming architecture)

- Even if Gipulo launches with one primary theme, **colors must be tokenized — never hardcoded
  everywhere.**
- Tokens for: canvas · panels · raised surfaces · menus · borders · separators · primary text ·
  secondary text · disabled text · accent · hover · selected · focus · success · warning · error ·
  guides.
- Makes future theming possible without rebuilding every component.

## Design tokens & variables (implementation form)

- Final DS implemented through proper tokens/variables: colors · typography · spacing · radius ·
  border widths · elevation · opacity · icon sizes · control heights · animation durations ·
  easing · z-index/layering.
- Agents must NOT randomly write values like `border-radius: 7px; padding: 9px; color: #3478ff;`
  unless from the DS or with a valid feature-specific reason.
- The system should make visual consistency **difficult to accidentally break**.

## Component architecture

- Every common visual pattern becomes a reusable component with defined variants.
- AI must NOT recreate slightly different versions of buttons, dropdowns, toggles, number fields,
  menus, tooltips, tabs, property rows, color pickers every time a feature is added.
- Features **compose existing DS components**.

## Accessibility

- Define: minimum readable contrast · keyboard focus · keyboard navigation · hit target size ·
  screen-reader labels · reduced motion · disabled states · error communication ·
  color-independent feedback.
- A very advanced visual editor must still be usable without depending only on color.

## Performance affects UI design

- Some UI designs become expensive when thousands of elements render. Avoid unnecessarily heavy
  effects in frequently repeated components (e.g. expensive blur, multiple shadows, complex
  filters on thousands of layer rows = bad decision even if beautiful).
- The UI stays **premium but practical** for a very large application.

## Research what the user missed

- The brief cannot list every component/state/interaction/visual rule needed. Before finalizing
  the DS, deeply research interface+interaction systems of: **Figma · Framer · Sketch · Adobe
  Illustrator · Affinity Designer · Spline · other modern professional editors.**
- Do NOT blindly copy. Study: what UI systems they require · what states exist · which controls
  repeat throughout · which canvas overlays exist · how panels resize · how nested layers work ·
  how cursor states work · how properties change per selection · how menus behave · how dense
  professional interfaces remain readable. Then **add important missing categories yourself**.

## Attention to detail (non-negotiable attitude)

- Treat Gipulo like a **very high-budget professional software project**. Never accept "this small
  thing does not matter" — small things combine into the quality of the entire product.
- Details that matter: 1px alignment · icon centering · cursor hotspots · hover timing · focus
  state · separator opacity · property-row height · label alignment · dropdown position · drag
  indicator · guide thickness · text baseline · tooltip offset · resize hit area · scrollbar
  behavior · selected-text color.

## Final goal

- When an AI agent later creates ANY UI inside Gipulo, it should almost never have to guess
  "what should this look like?" — the answer already exists in the Design System.
- The agent can always determine: which component to use · which color token · which spacing ·
  which icon style · which state to show · hover behavior · pressed behavior · selected behavior ·
  dragging behavior · behavior at different window sizes · cursor appearance · canvas guide
  appearance · animation behavior.
- The DS scales **from the smallest 1px canvas guide to the complete editor layout** without
  losing consistency.
