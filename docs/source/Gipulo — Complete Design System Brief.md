# Gipulo — Complete Design System Brief

I am creating a **1-to-1 Figma-level editor**, not just a simple editor with some Figma-like features.

The goal is to build a **full production-level design tool that replicates the depth, behavior, interaction quality, and overall capability of Figma**, while also adding more advanced features that Figma does not currently offer.

This is a huge project.

It may take months to complete, and I am a UX designer, not a coder, so I will be using AI agents to help build this project from scratch.

Because of that, before we start building the editor itself, I first need a **complete and extremely detailed Design System**.

This Design System will become the visual and interaction foundation of the whole application.

It should not look like a normal SaaS dashboard, simple web app, admin panel, or generic React application.

This is a **huge professional editor application**, so the Design System needs to be designed at the same level of depth as applications like:

- Figma;
- Framer;
- modern professional creative tools;
- modern 2026 productivity/editor applications.

The UI should feel very modern, highly polished, custom-crafted, and suitable for 2026.

The visual direction should be highly inspired by the level of refinement found in **Figma and Framer**, but Gipulo should still have its own identity.

---

# The Design System Must Cover EVERYTHING

This should not be a Design System that only defines:

- colors;
- typography;
- buttons;
- cards;
- inputs;
- spacing.

That would be far too basic for this project.

I need a Design System that controls **almost every visible and interactive part of the application, from major layouts down to extremely small micro-interactions**.

The goal is that we should almost never depend on browser defaults, React defaults, operating-system defaults, or random UI behavior.

Almost everything should have a purposely designed Gipulo version.

For example:

- buttons;
- icon buttons;
- cards;
- panels;
- sections;
- dropdowns;
- select menus;
- context menus;
- text fields;
- numeric fields;
- sliders;
- toggles;
- checkboxes;
- radio buttons;
- segmented controls;
- tabs;
- search fields;
- command palettes;
- tooltips;
- popovers;
- dialogs;
- modals;
- notifications;
- toasts;
- progress states;
- loaders;
- scrollbars;
- separators;
- resize handles;
- drag handles;
- property rows;
- lists;
- tree views;
- layer items;
- breadcrumbs;
- badges;
- chips;
- empty states;
- error states;
- disabled states;
- loading states;
- focus states;
- keyboard-focus states;
- hover states;
- pressed states;
- selected states;
- dragged states;
- active states.

Everything should have a clear custom design.

---

# Avoid Cheap Default Browser/UI Behavior

One of the most important goals is to avoid situations where most of the editor looks premium, but one small interaction suddenly shows some cheap default browser behavior.

For example:

when text is selected, browsers normally show a default blue text-selection color.

I do not want random default blue selection if that does not match Gipulo.

The Design System should define:

- text-selection background;
- text-selection foreground;
- focus rings;
- keyboard focus;
- active borders;
- native control replacement;
- default highlights;
- disabled styling;
- drag-selection styling.

If Gipulo has its own accent color, then these small system-level interactions should also follow that visual language where technically possible.

The same applies to native dropdowns.

We should not simply use a default HTML `<select>` and accept whatever browser UI appears.

We should have our own dropdown system that matches Gipulo's:

- typography;
- spacing;
- radius;
- iconography;
- hover states;
- active states;
- search behavior;
- scroll behavior;
- keyboard behavior;
- selected states.

The same principle should apply across the whole editor.

---

# The Design System Must Work From Macro Level to Micro Level

Because this is not a small application, the Design System needs multiple levels.

It should cover things from the biggest application structure all the way down to 1-pixel-level details.

For example:

## Macro Level

- overall editor layout;
- top toolbar;
- left sidebar;
- right properties panel;
- canvas;
- floating toolbars;
- bottom controls;
- code workspace;
- preview workspace;
- modal environments;
- fullscreen modes;
- responsive editor layouts.

## Component Level

- buttons;
- fields;
- cards;
- controls;
- panels;
- tabs;
- menus;
- dropdowns;
- layer rows;
- property rows;
- tool groups;
- inspectors;
- dialogs.

## Micro Level

- border thickness;
- divider opacity;
- hover backgrounds;
- text-selection color;
- cursor icons;
- resize zones;
- scrollbar thickness;
- icon optical alignment;
- tooltip offsets;
- focus rings;
- active indicators;
- guide thickness;
- snapping markers;
- control hit areas;
- tiny spacing values;
- corner radii;
- shadows;
- blur;
- gradients;
- state transitions.

The small details are extremely important.

In an editor this large, users interact with thousands of these micro-details every day.

---

# Modern 2026 Visual Direction

The UI should feel like a modern 2026 product.

I do not want an outdated flat dashboard style.

The Design System should explore modern visual treatments where appropriate, such as:

- dynamic gradients;
- subtle chromatic accents;
- premium gradient buttons;
- modern translucent surfaces;
- restrained glass effects;
- layered surfaces;
- soft elevation;
- refined shadows;
- subtle border lighting;
- intelligent hover effects;
- modern active states;
- smooth state transitions.

But this should **not** mean turning the entire interface into a flashy neon UI.

Most of the editor should remain clean, readable, neutral, and professional.

The more expressive visual treatments should be used intelligently for:

- primary actions;
- active tools;
- important states;
- highlighted controls;
- selected states;
- special features;
- branded moments.

The editor must remain comfortable to use for many hours.

---

# Typography

Typography needs its own complete system.

It should define:

- primary UI typeface;
- secondary typeface if required;
- numeric typography;
- code typography;
- heading styles;
- labels;
- property values;
- input text;
- tooltips;
- menu text;
- tabs;
- captions;
- status text;
- shortcuts;
- error messages;
- helper text.

It should also define:

- sizes;
- weights;
- line heights;
- letter spacing;
- optical hierarchy;
- truncation behavior;
- ellipsis behavior;
- wrapping;
- numeric alignment.

Typography inside a professional editor should be extremely controlled because a few pixels of unnecessary height repeated across hundreds of controls can make the entire UI feel wrong.

---

# Icon System

The Design System should have a complete icon language.

AI agents should not randomly pick different icons from different icon packs every time they add a feature.

The icon system should define:

- icon style;
- stroke width;
- corner treatment;
- optical size;
- bounding box;
- padding;
- filled vs outlined usage;
- active state;
- inactive state;
- disabled state;
- hover state;
- selected state;
- warning state;
- destructive state.

Different UI areas may also need different icon sizes.

For example:

- toolbar icons;
- panel icons;
- small property icons;
- menu icons;
- status icons;
- canvas controls.

---

# Spacing and Density

This application will contain a huge number of controls, so spacing must be extremely carefully designed.

The Design System should define a complete spacing system for:

- panel padding;
- section gaps;
- property rows;
- icon gaps;
- label/value gaps;
- toolbar spacing;
- nested indentation;
- menu spacing;
- popovers;
- dialogs;
- canvas overlays.

It should also define different density levels if required.

For example:

- comfortable;
- compact;
- ultra-compact.

But compact should never mean difficult to click.

Visual size and interactive hit area should be treated separately where necessary.

---

# Complete State System

Every component must be designed for all relevant states.

Do not only design the normal state.

For example, a button may need:

- default;
- hover;
- pressed;
- active;
- selected;
- focus;
- keyboard focus;
- disabled;
- loading;
- destructive;
- success.

A layer item may need:

- default;
- hover;
- selected;
- multi-selected;
- editing name;
- dragged;
- drag target;
- locked;
- hidden;
- component;
- instance;
- nested;
- warning/error.

A property field may need:

- default;
- hover;
- focus;
- editing;
- invalid;
- mixed values;
- disabled;
- inherited value;
- overridden value.

This state logic should exist throughout the system.

---

# Canvas UI Is Part of the Design System

This is extremely important.

The Design System is not only about panels, buttons, cards, and application layout.

The **canvas itself has a huge visual UI system**.

The Design System must define how canvas controls look and behave visually.

For example:

- object bounding boxes;
- selection outlines;
- hover outlines;
- resize handles;
- rotate zones;
- corner-radius handles;
- vector anchor points;
- Bézier handles;
- path outlines;
- midpoint controls;
- gradient handles;
- measurement labels;
- spacing indicators;
- snapping indicators;
- ruler guides;
- grid overlays;
- text-editing outlines.

These are all part of the product's visual language.

---

# Guide System

There should be a complete visual system for different guide types.

For example:

## Blue Guides / Geometry UI

These may be used for things such as:

- selected object geometry;
- bounding boxes;
- vector paths;
- anchor points;
- Bézier handles;
- object dimensions;
- active geometry states.

## Pink / Magenta Guides

These may represent things such as:

- spacing;
- equal spacing;
- distribution;
- padding;
- Auto Layout measurements;
- smart spacing relationships.

## Orange Guides

These may represent certain snapping, alignment, measurement, or special interaction states depending on the final Gipulo language.

But the meaning of each color must be researched and defined clearly.

We should not randomly use blue, orange, pink, red, or green for different canvas features.

Every guide color should have a consistent meaning.

The Design System should define:

- line thickness;
- color;
- opacity;
- dash pattern;
- endpoint;
- labels;
- label backgrounds;
- active state;
- inactive state;
- hover state;
- locked state;
- temporary state.

Guide appearance should also remain usable at different zoom levels.

---

# Cursor Design System

Cursor design needs its own proper section.

AI should never randomly use whatever cursor seems convenient.

The Design System should define the cursor for:

- default selection;
- pointer;
- move;
- grab;
- grabbing;
- resize horizontal;
- resize vertical;
- resize diagonal;
- rotate;
- scale;
- pen;
- vector edit;
- point add;
- point delete;
- Bézier editing;
- text;
- hand tool;
- eyedropper;
- zoom;
- duplicate;
- mask;
- forbidden;
- drag-copy;
- crop;
- 3D orbit if later required.

But not just the cursor icon itself.

Also define:

- cursor hotspot;
- exact visual size;
- outline;
- contrast;
- behavior on light canvas;
- behavior on dark canvas;
- state changes;
- rotation-aware cursor direction;
- cursor state while dragging;
- cursor state before activation;
- cursor state while modifier keys are held.

For example, if a rotated rectangle is being resized, the resize cursor should visually match the real resize direction instead of always showing a fixed browser cursor.

---

# Left Layer Panel

The layer panel needs a complete design system of its own.

It should define:

- panel width;
- minimum width;
- maximum width;
- resized state;
- collapsed state;
- expanded state;
- mobile/small-window behavior;
- section headers;
- pages;
- layers;
- nested layers;
- indentation;
- expand/collapse arrows;
- layer icons;
- visibility;
- locking;
- component states;
- instance states;
- text layers;
- vector layers;
- frames;
- groups;
- masks.

It should also define every interaction state:

- normal;
- hover;
- selected;
- multi-selected;
- dragged;
- drag destination;
- renaming;
- collapsed;
- expanded;
- locked;
- hidden;
- disabled;
- error state.

Dragging a layer between other layers should have a purposely designed insertion indicator.

Dragging inside a frame/group should have a different clear state.

---

# Right Properties Panel

The properties panel also requires its own full system.

It should define:

- widths;
- resizing;
- collapsed state;
- responsive state;
- sections;
- headers;
- property rows;
- labels;
- values;
- numeric fields;
- linked values;
- chain controls;
- sliders;
- color controls;
- gradient controls;
- effect controls;
- stroke controls;
- Auto Layout controls;
- constraints;
- typography;
- component properties;
- prototype/interaction controls.

Different selection types should have properly designed property states.

For example:

- nothing selected;
- shape selected;
- frame selected;
- group selected;
- text selected;
- vector selected;
- component selected;
- instance selected;
- image selected;
- multiple objects selected;
- mixed-property selection.

The UI should adapt intelligently instead of showing the same panel for everything.

---

# Responsive Application UI

Gipulo itself also needs to work across different screen sizes.

The Design System should define what happens when the editor window becomes:

- very wide;
- normal desktop;
- laptop;
- small laptop;
- tablet-sized;
- narrow window.

For example:

- when does the layer panel shrink?
- when can it collapse?
- when does the properties panel become narrower?
- when do labels turn into icons?
- when do tool groups collapse?
- when do floating controls move?
- when does horizontal scrolling become acceptable?
- which areas must preserve minimum width?

The editor should not simply compress everything until the controls become unusable.

---

# Toolbar System

The toolbar should have proper rules for:

- placement;
- grouping;
- spacing;
- separators;
- tool states;
- active tool;
- hover;
- selected;
- submenu;
- long-press;
- keyboard shortcut display;
- compact layouts;
- responsive layouts.

Different tools may also require contextual controls.

These should follow the same Design System instead of creating one-off floating UIs.

---

# Menus, Popovers and Context Menus

These are extremely common in an editor and must not use generic defaults.

Define:

- menu width;
- padding;
- item height;
- icons;
- shortcuts;
- nested menus;
- separators;
- destructive items;
- selected items;
- disabled items;
- search;
- checkmarks;
- submenus;
- hover;
- keyboard navigation;
- scrolling;
- viewport collision behavior.

The same visual language should apply to:

- dropdowns;
- context menus;
- toolbar menus;
- command palettes;
- property menus.

---

# Motion and Interaction Design

The Design System should also define motion.

Not large decorative animation everywhere.

I mean small interface motion such as:

- hover transitions;
- button press;
- panel opening;
- dropdown opening;
- tooltip appearance;
- menu transition;
- drag feedback;
- snapping feedback;
- selected-state transition;
- toggle transition;
- slider movement.

Motion should feel fast and responsive.

It should never make the editor feel slow.

The Design System should define:

- duration ranges;
- easing;
- spring usage;
- when animation should not be used;
- reduced-motion behavior.

---

# Light and Dark Surfaces

Even if Gipulo initially launches with one primary theme, the Design System architecture should not hardcode colors everywhere.

Colors should be tokenized.

Define tokens for:

- canvas;
- panels;
- raised surfaces;
- menus;
- borders;
- separators;
- primary text;
- secondary text;
- disabled text;
- accent;
- hover;
- selected;
- focus;
- success;
- warning;
- error;
- guides.

This makes future theming possible without rebuilding every component.

---

# Design Tokens and Variables

The final Design System should be implemented through proper tokens/variables.

For example:

- colors;
- typography;
- spacing;
- radius;
- border widths;
- elevation;
- opacity;
- icon sizes;
- control heights;
- animation durations;
- easing;
- z-index/layering.

AI agents should not randomly write values like:

```css
border-radius: 7px;
padding: 9px;
color: #3478ff;
```

unless those values come from the Design System or there is a valid feature-specific reason.

The system should make visual consistency difficult to accidentally break.

---

# Component Architecture

Every common visual pattern should become a reusable component.

AI should not recreate slightly different versions of:

- buttons;
- dropdowns;
- toggles;
- number fields;
- menus;
- tooltips;
- tabs;
- property rows;
- color pickers;

every time a new feature is added.

The Design System should define the reusable component and its variants.

Then features should compose those existing components.

---

# Accessibility

The Design System must also define accessibility rules.

For example:

- minimum readable contrast;
- keyboard focus;
- keyboard navigation;
- hit target size;
- screen-reader labels;
- reduced motion;
- disabled states;
- error communication;
- color-independent feedback.

A very advanced visual editor should still be usable without depending only on color.

---

# Performance Must Also Affect UI Design

This is not only a visual problem.

Some UI designs can become expensive when thousands of elements are rendered.

The Design System should avoid unnecessarily heavy effects in frequently repeated components.

For example, using expensive blur, multiple shadows, or complex filters on thousands of layer rows would be a bad design decision even if it looks beautiful.

The UI should remain premium but practical for a very large application.

---

# Research What I Have Missed

This prompt still cannot list every component, state, interaction, and visual rule required by an editor as large as Gipulo.

There will be many things I have forgotten.

Before creating the final Design System, deeply research the interface and interaction systems of professional tools such as:

- Figma;
- Framer;
- Sketch;
- Adobe Illustrator;
- Affinity Designer;
- Spline;
- other modern professional editor applications.

Do not blindly copy them.

Study:

- what UI systems they require;
- what states exist;
- which controls repeat throughout the application;
- which canvas overlays exist;
- how panels resize;
- how nested layers work;
- how cursor states work;
- how properties change depending on selection;
- how menus behave;
- how dense professional interfaces remain readable.

Then add important missing categories yourself.

---

# Extremely Important — Attention to Detail

The scale of Gipulo should be treated like a very high-budget professional software project.

The Design System should not accept the attitude of:

> This small thing does not matter.

In an application this large, those small things combine into the quality of the entire product.

Attention should be given to details such as:

- 1px alignment;
- icon centering;
- cursor hotspots;
- hover timing;
- focus state;
- separator opacity;
- property-row height;
- label alignment;
- dropdown position;
- drag indicator;
- guide thickness;
- text baseline;
- tooltip offset;
- resize hit area;
- scrollbar behavior;
- selected-text color.

These details matter.

---

# Final Goal

I want this Design System to be detailed enough that when an AI agent later creates any UI inside Gipulo, it should almost never have to guess:

> What should this look like?

The answer should already exist in the Design System.

The AI should be able to understand:

- which component to use;
- which color token to use;
- which spacing to use;
- which icon style to use;
- which state to show;
- how it behaves on hover;
- how it behaves when pressed;
- how it behaves when selected;
- how it behaves while dragging;
- how it behaves on different window sizes;
- how its cursor looks;
- how its canvas guides look;
- how its animation behaves.

This should become the **single visual and interaction source of truth for the entire Gipulo application**.

The design should be extremely detailed because the project itself is extremely large.

I want a Design System that can scale from the smallest 1px canvas guide to the complete editor layout without losing consistency.