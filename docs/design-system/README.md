# Gipulo Design System — the single visual + interaction source of truth

> Status: **FORMALIZED (2026-10-11)** — requirements distilled from the three source documents,
> all specs authored, guide-color language closed. Code implementation (token module + component
> library) is the next phase; until it exists these docs ARE the system.
> Authority: `docs/source/` (ultimate truth) > `docs/memory/` 05/07 (distilled specs) >
> `rules/ui.md` + `rules/icons.md` + `rules/canvas.md` (behavior law) > **these docs (visual +
> structural law)** > ADRs. A rule always wins over a spec here; if they conflict, this doc is
> corrected in the same task.

## Mission (from the Design System Brief)

The DS is the visual and interaction foundation of the whole application — the answer to "what
should this look like?" for EVERY surface, from the smallest 1px canvas guide to the complete
editor layout. It must never look like a generic SaaS dashboard: the depth benchmark is
**Figma · Framer · modern 2026 professional creative tools**. Gipulo keeps its own identity
(brand gradient, guide language, signature features) while reaching that depth.

## The eight DS laws (binding)

1. **Tokens only.** Every color, size, radius, spacing, duration, easing, z-layer comes from
   `02_tokens.md` in code form (CSS custom properties / core theme push). No magic numbers
   beyond these docs (`rules/ui.md` UI1).
2. **Compose, never recreate.** Features use DS components with their defined variants
   (`07_component_architecture.md`). No "slightly different button" ever.
3. **No browser/OS/React defaults.** Everything visible gets a purposely designed Gipulo
   version where technically possible (`01_foundations.md` §Defaults checklist).
4. **Complete states.** Every component ships ALL relevant states — a component without its
   full state matrix is unfinished (`05`–`06`, `rules/ui.md` UI3).
5. **Reserved treatment.** Neutral everywhere; accent for primary actions/selection/focus;
   gradient for the ONE primary action in a view; purple = components (`01_foundations.md` §4).
6. **Fixed guide meanings.** The canvas color language is CLOSED: blue geometry · magenta
   spacing/layout relationships · orange snap+measure · red ruler guides/grids · purple
   components. New guide meaning = ADR-level change (`08_canvas_ui.md` §9).
7. **Motion is feedback, not decoration.** Fast, restrained, purposeful
   (`10_motion.md`); reduced-motion honored everywhere.
8. **Premium but practical.** Performance is a design constraint: no expensive effects in
   repeatedly-rendered components (layer rows, property rows) — `01_foundations.md` §6.

## File index

| Doc | Covers |
|---|---|
| `01_foundations.md` | Visual direction, macro→micro scope, reserved-treatment laws, no-defaults checklist, quality bar, performance-in-UI law |
| `02_tokens.md` | THE token layer: color (light/dark), typography scale, spacing, radii, borders, elevation, opacity, motion, z-layers, density, control heights, icon sizes |
| `03_typography.md` | Fonts, full type scale, numeric rules, truncation, label laws, accessibility alignment |
| `04_icons.md` | Icon spec (16 grid, 1.25 stroke), sizes per surface, colors/states, initial inventory, discipline |
| `05_components_controls.md` | Buttons, fields, segmented controls, toggles, checkboxes, radios, sliders, color control |
| `06_components_surfaces.md` | Menus, popovers, tooltips, toasts, dialogs, tabs, panels, property rows, layer rows, lists, scrollbars, empty/error/loading states |
| `07_component_architecture.md` | Variants, composition law, state-completeness gate, component registry, adding a component |
| `08_canvas_ui.md` | Canvas overlay law: universal zoom laws, all four guide systems with every state, handles/hit areas, **closed color language** |
| `09_cursors.md` | Full cursor inventory, rotation-aware law, hotspots, drag states, modifier variants |
| `10_motion.md` | Durations, easing, per-pattern motion table, reduced motion, when NOT to animate |
| `11_accessibility.md` | Contrast, focus, keyboard, hit targets, screen readers, non-color feedback |
| `12_responsive.md` | Editor breakpoints, panel behaviors incl. Minimize UI, label→icon folding, minimum widths |
| `13_research_notes.md` | The mandated "research what I missed" record: sources studied, categories added, decisions |

## Tagging convention inside these docs

`[DS]` = value/behavior taken from the Design System HTML reference (binding) · `[UI3]` =
researched from professional editors (source in `13`) · `[PROPOSED]` = authored beyond the
reference, binding until the owner changes it (same convention as the canvas rules).

## Change protocol (DS changes are law changes)

1. **New/changed component or state** → update the owning DS doc + registry
   (`07` §5) in the same task; code follows the doc, never leads it.
2. **New token** → add to `02_tokens.md` with a named consumer; tokens without consumers are
   deleted (no speculative tokens, CQ1.2 spirit).
3. **New guide color or canvas overlay meaning** → ADR-level change (`rules/canvas.md`
   CAN-GUIDE-1; requires updating `08_canvas_ui.md` §9 + the rules).
4. **Everything DS** → task summary + CHANGELOG via the Documentation Update Rule
   (`rules/documentation.md` D2).
5. **Rendered ground truth** stays the source HTML (`docs/source/Gipulo — Design System.html`,
   light/dark states live) — if a value must change, this doc set and the HTML diverge are
   resolved by: source wins until the owner approves a DS revision (then HTML is revised, not
   silently overridden).

## What exists in code (as of 2026-10-11)

- **DS code v1**: `app/src/design-system/` — token layer (`tokens.css`/`tokens.ts`, drift-tested),
  global resets, self-hosted Inter + JetBrains Mono, 17 registry entries (15 components + tokens + scrollbar) with 47 state tests
  (47 passing), and the **gallery** story page (`gallery/Gallery.tsx`) rendering every component
  in its states + an illustrative editor frame. Run `npm run dev` in `app/` to view it.
  Registry with per-entry status: `07_component_architecture.md` §5.

## What does NOT exist yet (honesty)

- Remaining registry entries (color picker, dropdown, popover, toast, list virtualization,
  badge, canvas overlay primitives, cursors, command palette, Code Workspace surfaces) ship
  with the first feature that needs them — see the registry.
- RVS affordance color semantics: recommended + flagged for a small ADR before that feature
  ships (`08_canvas_ui.md` §8).
- Components for future features (comments/presence/version history) are specified at
  requirements level in `06` §14 and `13`, not fully designed — their feature phases own the
  details.
