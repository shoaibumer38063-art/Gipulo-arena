# 03 — Typography System

> Requirements: Design System Brief §Typography (a few pixels of extra height repeated across
> hundreds of controls makes the whole UI feel wrong). Values `[DS]` from the HTML reference.

## 1. Typefaces

| Token | Family | Weights | Rationale |
|---|---|---|---|
| `font/ui` | **Inter** (variable) | 400 light · **450 default** · 500 medium · 600 semibold | Professional editor UI benchmark (Figma-class), exceptional at 11px, tabular figures support |
| `font/mono` | **JetBrains Mono** | 400 · 500 | Code workspace, hex values, numeric verification |

Font loading: variable font, `font-display: swap`, fallbacks `-apple-system, "Segoe UI",
sans-serif` / `ui-monospace, monospace`. Antialiasing on (`-webkit-font-smoothing: antialiased`)
`[DS]`. Never rasterize UI text; never fake-bold (`font-synthesis: none`).

## 2. The scale (complete — nothing else exists)

| Style token | Spec | Usage |
|---|---|---|
| `type/title` | `600 13px/20px` | panel headers, dialog titles, major group titles |
| `type/section` | `600 11px` | section headings (properties sections, panels) |
| `type/body` | `450 11px/16px` | THE default: labels, values, menu items, buttons, tooltips, tabs |
| `type/body-medium` | `500 11px/16px` | emphasis within body (field labels, active tab, toolbar file name) |
| `type/caption` | `500 10px/16px`, `letter-spacing:.02em`, `text/tertiary` | frame titles, helper text, section captions |
| `type/code` | `500 10px/14px` mono, `text/secondary` | hex values, code workspace, shortcut tokens `[PROPOSED for shortcut tokens]` |
| `type/pill` | `500 10px` Inter | canvas pills/labels (size, angle, gap, coordinates) |
| `type/counter` | `500 10px` tabular `[PROPOSED]` | counts/badges where present |

Laws:
- **11px is the UI default** — a 12px "default" creeping in is a DS violation. 13px only via
  `type/title`. 10px only for captions/code/canvas pills.
- Line heights: 16px for 10–11px text (control rhythm); never `normal`/browser default.
- Letter spacing: 0 for body; `.02em` captions; code 0 `[DS]`.

## 3. Hierarchy & weights

- Weight is the primary hierarchy tool (450 → 500 → 600 within the same size). Size jumps are
  reserved (10/11/13) because vertical rhythm is precious.
- One emphasis per row maximum. Never combine weight + color + underline for emphasis.
- Section headings are quiet (`600 11px` with generous padding per `06` §7), not banners.

## 4. Numeric typography `[DS]`

- All numeric values (property fields, position/size, rotation, opacity, guide pills,
  coordinates) use **`font-variant-numeric: tabular-nums`** — digits never jitter while
  scrubbing or animating.
- Numeric precision display: property fields show up to 2 decimals when fractional (`28.37`);
  values trail `.00` only where the reference does (`X 128.00 · Y 64.00` inspector style).
- Guide/measure values: 0–2 decimals, shortest form (`08` §5).
- Units inside fields are part of the label prefix (`W`, `H`, `X`, `Y`) — never inline suffixes
  in the input.

## 5. Truncation, wrapping, overflow `[DS]`

- Layer names: single line, `…` ellipsis, `white-space: nowrap` (indentation + actions keep
  their zones; the NAME truncates first).
- Property values: never truncate mid-digit — the field scrolls/selects instead.
- Menu items: truncate label before shortcut; shortcut never truncates.
- Tooltips: may wrap at 240px max width `[PROPOSED]`; never truncate a tooltip.
- Frame names (`type/caption`): truncate with `…`; rename mode shows full value in input.
- Code workspace: no wrapping by default; horizontal scroll (the one place horizontal scroll is
  acceptable — `12` §6).

## 6. Label & value laws (professional density)

- Property rows: label column fixed (reference uses a 26px prefix for field labels), value text
  `type/body`, baseline-aligned; label `[DS]`.
- Field label prefix (`W`, `X`, `R`…): 26px wide, centered, `text/secondary`, weight 500,
  scrubbable cursor (`05` §3).
- Mixed values: placeholder text "Mixed" rendered in `text/primary` (intentionally readable —
  not dimmed) `[DS]`.
- Visible labels and aria-labels MUST match (e.g. tooltip "vertical padding" and the accessible
  name "vertical padding") `[UI3]` — no separate screen-reader dialect.
- All-caps: never (except nothing). Title-case for panel titles, sentence case for menu items
  and buttons `[PROPOSED]` (reference is sentence case).

## 7. Special text surfaces

- Toolbar: white text, file name `500`, "Gipulo /" prefix at `450` opacity .5 `[DS]`.
- Menus: `type/body`; shortcuts right-aligned, `text/secondary`-equivalent opacity .5, same size.
- Tooltips/toasts: `type/body` white on `tip/surface`.
- Canvas pills: `type/pill` white on colored backgrounds (`08` §7).
- Code workspace: `type/code`; tokens/identification driven by a DS code-theme map
  `[PROPOSED — authored with the Code Workspace feature]`.

## 8. Accessibility & typography (`11` expands)

- Minimum contrast: `text/secondary` is the lightest text used for meaning; `text/tertiary`
  only for non-essential captions and must still meet AA for its size `[check in 11 §2]`.
- Text scales with OS/browser zoom; layouts are tested at 80%–200% (`12` §7).
- No text in images; no icon-only meaning without aria-label.
- Reduced motion affects transitions, never text legibility.

## 9. Do-not list

- Never use the browser default font stack anywhere (UI or canvas).
- Never mix typefaces beyond Inter + JetBrains Mono (no "display font" appears in the editor).
- Never center-align multi-line UI text.
- Never letter-space uppercase UI text (we don't use uppercase UI text).
- Never justify text.
- Never apply shadows/gradients to text (pill backgrounds may be colored; the text is flat).
