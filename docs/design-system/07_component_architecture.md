# 07 — Component Architecture (composition law, registry, adding components)

> Requirements: Design System Brief §Component Architecture ("AI must NOT recreate slightly
> different versions of buttons, dropdowns, toggles, number fields, menus, tooltips, tabs,
> property rows, color pickers every time a feature is added") + `rules/ui.md` UI4. This doc is
> the structural law that makes the promise enforceable.

## 1. The composition law (binding)

1. Features **compose** registered components with their documented variants and states.
2. A feature may NOT restyle a component locally (no per-feature padding, radius, color).
   If the pattern genuinely differs: it is a NEW component or variant — designed here FIRST,
   then implemented (`06` change protocol, DS README §change protocol).
3. A feature may NOT reimplement a component class that exists (second dropdown system = bug).
4. Component behavior (focus, keyboard, Escape, placement) is owned by the component, not the
   caller. Callers pass data and receive events.
5. No "temporary" component survives its feature — temporary visual = unmerged visual.

## 2. Variant system (how components are parameterized)

Variants are explicit, enumerable, and documented — never ad-hoc style props:

| Axis | Values (examples) |
|---|---|
| `variant` | primary · secondary · ghost · destructive · icon |
| `size` | per density (`02` §10); controls define which sizes exist |
| `state` | driven by interaction + data: default/hover/pressed/focus/disabled/loading/selected/invalid/mixed/… |
| `surface` | panel · toolbar · menu · canvas (contexts change tokens, never structure) |
| `density` | ultra-compact/compact/comfortable where the component supports it |

State is NEVER a free-form prop list: the state matrix in `05`/`06` is the contract.

## 3. Component anatomy contract

Every component documents: purpose · anatomy diagram (parts + spacing from `02`) · metrics ·
variants · state matrix · tokens used · keyboard/behavior · a11y (roles, labels, focus) ·
do-not list. The docs in `05`/`06` are the canonical examples of this format.

## 4. State-completeness gate (merge blocker)

A component or variant is INCOMPLETE — and may not ship — until:
1. Every state in its matrix exists and is visually specified;
2. Keyboard + focus behavior works; a11y attributes are present;
3. Both themes pass (contrast checks `11` §2); reduced motion honored;
4. Hot-component paint budget respected (`01` §6);
5. It is registered in §5 with its doc anchor.

## 5. Component registry (status: ✅ = built in `app/src/design-system/` + tests · ⬜ = specified, code pending)

| Component | Spec | Code |
|---|---|---|
| Button (primary/secondary/ghost/icon/destructive/loading) | `05` §1 | ✅ `components/Button` |
| Segmented control | `05` §2 | ✅ `components/Segmented` |
| Text field · numeric field (scrub, mixed, invalid, disabled) | `05` §3 | ✅ `components/Field` |
| Search field | `05` §4 | ✅ `components/SearchField` |
| Toggle | `05` §5 | ✅ `components/Toggle` |
| Checkbox · Radio | `05` §6 | ✅ `components/Choice` |
| Slider | `05` §7 | ✅ `components/Slider` |
| Chip | `06` §11 | ✅ `components/Chip` |
| Menu · context menu · submenu | `06` §1 | ✅ `components/Menu` (v1: one submenu depth; deeper nesting with the first feature needing it) |
| Tooltip | `06` §4 | ✅ `components/Tooltip` |
| Tabs | `06` §6 | ✅ `components/Tabs` |
| Panel · collapsible section | `06` §7 | ✅ `components/Panel` (splitter chrome ⬜ — lands with panel resize feature) |
| Property row | `06` §8 | ✅ `components/PropertyRow` |
| Layer row · tree view | `06` §9–10 | ✅ `components/LayerRow` (virtualization lands with the layers feature) |
| Icon sprite | `04` | ✅ `components/Icon` — reference set + node types; feature icons ship with features (04 §6) |
| Scrollbar | `06` §12 | ✅ `globals.css` |
| Tokens + theme application | `02` | ✅ `tokens.css` · `tokens.ts` (drift-tested) |
| Color swatch · palette · picker | `05` §8 | ⬜ — with the color/fill feature |
| Dropdown/select | `06` §2 | ⬜ — composes Field + Menu; first consumer registers it |
| Popover | `06` §3 | ⬜ — with the first anchored-content feature (color picker / effects) |
| Toast | `06` §5 | ⬜ — with the first user-feedback flow (publish/export) |
| List (virtualizable) | `06` §10 | ⬜ — with the first long list (fonts/assets) |
| Empty/error/loading state patterns | `06` §13 | ⬜ — patterns defined; components when the first surface needs them |
| Badge (counts) | `06` §11 | ⬜ — with the first count surface |
| Canvas overlay primitives (outline, handle, pill, guides) | `08` | ⬜ — core-side (Rust); ships with the canvas phase |
| Cursor set | `09` | ⬜ — art + module ship with the canvas phase |
| Command palette | `06` §14 | ⬜ (future-gated) |
| Code Workspace surfaces | `06` §14 | ⬜ (feature-gated) |

**Implementation notes (2026-10-11 — DS code v1):** 17 registry entries implemented in
`app/src/design-system/` with 47 passing tests (state-matrix suites + token-drift guard +
gallery smoke). Deviations caught & fixed during implementation: Field's Escape-then-blur
could commit a discarded draft (fixed with an explicit cancel flag); pointer capture is
progressive (jsdom/older browsers); the reference's 7px vs 8px handle discrepancy remains
canon (7px, `08` §2).

## 6. Adding or changing a component (process — binding)

1. **Search first:** is this an existing component/variant? (registry §5 + `05`/`06` scan).
2. If new: write/update the spec in the owning DS doc — anatomy, variants, full state matrix,
   tokens, a11y, do-not.
3. Register it (§5) — same task. No component exists only in code or only in a feature doc.
4. Implement using tokens only; both themes; keyboard path; motion per `10`.
5. Add its golden-state coverage to the DS test surface (when DS code lands).
6. Documentation Update Rule sweep (`rules/documentation.md` D2) — DS docs, CHANGELOG, task
   summary.
7. If the change alters shared identity (guide colors, brand gradient, control metrics):
   ADR-level (`08` §9; `rules/canvas.md` CAN-GUIDE-1).

## 7. Hot vs cold tiers (performance contract for components)

- **Hot** (rendered by the thousand): layer rows, property rows, list rows, canvas overlay
  primitives, handles. Only flat paint + 1px strokes + color/opacity transitions.
- **Cold** (few instances): menus, popovers, dialogs, toasts, tooltips — may use elevation and
  translucent surfaces.
- A component's tier is declared in its spec; moving tiers upward (cold → hot) requires a
  budget-suite check (P3/P6).

## 8. Naming & file conventions (for the implementation phase)

- Component names are nouns from the DOMAIN vocabulary where one exists (`rules/code-quality.md`
  naming law): `LayerRow`, `PropertyRow`, `SelectionOutline` — not `ListItem2`.
- Variants are the §2 axes, not name suffixes (`Button variant="primary"` — never `PrimaryBtn`).
- Canvas overlay primitives mirror the canvas vocabulary (`08` §2–6) one-to-one, so rule IDs
  map to code names (CAN-GUIDE-*, CAN-SEL-*).
