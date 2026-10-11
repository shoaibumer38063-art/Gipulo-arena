# 01 — Foundations: visual direction, scope, and quality laws

> Sources: Design System Brief (`docs/source/Gipulo — Complete Design System Brief.md` §modern
> direction, defaults, attention to detail, performance) + the HTML reference. This doc states
> the PHILOSOPHY and the non-negotiables; values live in `02_tokens.md`.

## 1. Identity & benchmark

- Gipulo = a 1-to-1 Figma-level professional editor. The DS must feel **very modern, highly
  polished, custom-crafted — suitable for 2026** — with its own identity, not a Figma clone.
- Forbidden references: generic SaaS dashboards, admin panels, "basic React app" look, neon
  flash. Permitted and required: Figma/Framer-class refinement, professional creative-tool depth.
- Identity carriers (what makes it *Gipulo*): the brand gradient (blue→purple→magenta), the
  closed guide-color language, the toolbar's dark ribbon `[DS]`, the corner-radius language,
  restrained precision everywhere.

## 2. Scope ladder — everything, macro to micro `[DS]`

- **Macro:** editor shell (top toolbar 40px · layer panel 240px · canvas · properties panel
  240px) · floating/contextual toolbars · code workspace (canvas | Code + Preview) · modal
  environments · fullscreen & minimize modes · responsive layouts (`12`).
- **Component:** every control in `05`–`06` — buttons → fields → menus → layer rows → property
  rows → tabs → dialogs — all custom, all complete.
- **Micro (the soul of quality):** border thickness (1px `[DS]`) · divider opacity (`--bd`) ·
  hover backgrounds · text-selection color · cursor hotspots · resize zones · scrollbar
  thickness (10px) · icon optical alignment · tooltip offsets · focus rings · active indicators ·
  guide thickness (1px at every zoom) · snap markers · hit areas (≥24px min, 16px on canvas
  handle zones) · tiny spacing values · radii · shadows · blur · gradients · transitions.
- Law: **small details are not small.** Users touch thousands of micro-details daily; each one
  is either designed or it cheapens the product.

## 3. Defaults checklist — "cheap browser behavior" must never appear `[DS]`

| Default risk | Gipulo replacement (token/spec) |
|---|---|
| Blue text selection | `::selection` = `--sel` (accent 30%), color inherit (`02` §5) |
| Native focus outline | `:focus-visible` = 2px `--accent` ring, offset 1px; components use their own (`05`) |
| `<select>` dropdowns | DS dropdown/select system (`06` §2) — search, scroll, keyboard, states |
| `<input type=number>` spinners | DS numeric fields with scrubbable labels, tabular values (`05` §3) |
| OS scrollbars | DS scrollbars: 10px, thumb `--bd2`, radius 5 (`06` §12) |
| Browser context menu | DS context menu everywhere on canvas and layers (`06` §1) |
| Default drag ghost (HTML5 DnD images) | DS drag ghosts: opacity .7 + `--shadow` + panel bg (`06` §8) |
| Default button cursor (`pointer`) | `cursor: default` over chrome; canvas uses the custom cursor system (`09`) |
| Tap highlight / overscroll glow | Suppressed OS-side; Gipulo feedback only |
| Native tooltips (`title=`) | DS tooltips (`06` §4) — 400ms delay, `--tip` surface |
| Form validation bubbles | DS invalid states + inline error text (`05` §3, `11` §7) |
| Unstyled checkboxes/radios/sliders | All redesigned (`05` §5–7) |
| Browser zoom-looking scrollbars in code view | Same DS scrollbar language |
| IME composition default underline | DS dashed `2 1` 1.5px composition underline (`08` §3) |

Rule: if a platform default would ever be VISIBLE, it is on this list and replaced.

## 4. Reserved treatment — where expressiveness is allowed `[DS]`

- Neutral, readable surfaces are the norm. Expressive treatments ONLY at: primary actions ·
  active tool · selected states · important status · highlighted controls · branded moments.
- **Accent reservation:** `--accent` = primary actions, selection, focus, active tool. Not
  decoration.
- **Gradient law:** gradient reserved for the ONE primary action in a view (e.g. Share button);
  a second gradient element in the same view is a DS violation.
- Permitted modern techniques (restrained): subtle chromatic accents · translucent surfaces for
  floating chrome · soft layered elevation · refined 3-layer shadow (`--shadow`) · hover
  luminosity shifts · brand-gradient logo mark. Forbidden: decorative blur behind frequently
  rendered rows, stacked shadows on list items, gradients on passive chrome.
- Components: purple `--comp` is reserved for component/instance identity — never decorative.

## 5. Empty states, errors, loading — treated as designed surfaces, not fallbacks `[DS]`

- Empty states: short statement + one primary action (the only gradient allowed on that view).
- Errors: `--danger` text/icon + plain-language message; never color alone (`11` §7).
- Loading: instant feedback (<100ms anything) else DS loader/skeleton; never layout shift.

## 6. Performance is a design constraint `[DS]`

- The DS must survive thousands of rendered elements: layer rows, property rows, canvas chrome.
- **Hot components** (repeated by the thousand: layer rows, property rows, canvas overlay
  primitives) use only cheap paint: flat fills, 1px strokes, opacity/transform transitions
  (80ms). No per-row blur, no multiple shadows, no filtered backdrops.
- **Cold components** (menus, dialogs, popovers) may use elevation/translucency.
- Animation on hot paths limited to color/opacity/transform; geometry feedback is direct
  (cursor/drag response is never animated — `10` §6).
- Design decisions that fight the budget suite (canvas §26) are wrong even if beautiful.

## 7. The quality bar — non-negotiable details `[DS]`

1px alignment of chrome edges · icons optically centered (not just mathematically) · cursor
hotspots exact · hover timing consistent (80ms) · focus rings never missing · separator opacity
from tokens only · property-row heights consistent (28px) · label/value alignment shared
baseline · dropdown/popover positions never clipped (collision handling `06` §3) · drag
insertion indicators precise (`06` §8) · guide thickness 1px at every zoom · text baselines
aligned (tabular numerics) · tooltip offsets consistent (8px gap) · resize hit areas ≥ targets ·
scrollbar behavior (overlay, not layout-shifting) · selected-text colors custom and
contrast-safe.

**Attitude law:** "this small thing does not matter" is never an acceptable engineering
decision. If it's visible, it was designed.
