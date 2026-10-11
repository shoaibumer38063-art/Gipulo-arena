# INTERACTION RULES — App-level (non-canvas) interaction law

> Status: ACTIVE (v1, 2026-10-11). Canvas interactions: `rules/canvas.md`. This file governs
> panels, menus, dialogs, keyboard, focus, shortcuts, drag-and-drop outside the canvas.

## I1 — Keyboard & shortcuts

- Shortcut language follows the familiar Figma-style base (familiar-workflow principle). Values
  already fixed by the DS reference [DS]: copy ⌘C · paste-in-place ⌘⇧V · rulers ⇧R ·
  bring-to-front ⌘] · ungroup ⌘⇧G · toggle guides ⌘; · pin measure ⇧M · measure hold ⌥ ·
  nested measure ⌘⌥ · nudge-measure ⌥⇧arrows · deep select ⌘-click · snap-off hold ⌘.
- New shortcuts: must be registered in ONE shortcut registry (single source: id → binding →
  context → description), shown in menus/tooltips/command palette from that registry, and
  conflict-checked per context (canvas vs panel vs modal focus scopes). No listener- scattered
  `keydown` handlers.
- Every shortcut-reachable action is also discoverable (menu item, tooltip hint, or palette).
- Platform mapping: ⌘ on macOS / Ctrl elsewhere — resolved by the registry, never per-handler.

## I2 — Focus management

- Focus scopes: one active scope (canvas | panel | modal | menu | palette). Escape closes/returns
  from the innermost scope; Tab cycles within scope; focus returns to the invoker on close
  (menus, dialogs, popovers).
- Modals trap focus; opening a modal stores the restore target. Canvas selection ⇄ keyboard focus
  stay in bidirectional sync via the semantic mirror layer (canvas rules §24).
- Visible focus everywhere: DS `:focus-visible` (2px accent, offset 1px) / button double ring —
  never suppressed for "cleanliness."

## I3 — Menus, dropdowns, popovers, tooltips, toasts (behavior)

- All use DS components (ui rules UI9); identical behavior family: hover-open submenus with
  standard open/close delays [PROPOSED: open 150ms / close 250ms; ADR], click-toggle top menus,
  keyboard arrow/Enter/Escape/typeahead, viewport-collision flip/shift, scroll-lock policy
  documented per component.
- Tooltips: appear after hover dwell [PROPOSED: 400ms; DS: tooltip look fixed], dismiss on
  pointer-down/leave; never on touch; never over the cursor hotspot; shortcut hints from the
  registry (I1).
- Toasts: transient confirmations, auto-dismiss [PROPOSED: 3–4s], stack with limit, announce to
  screen readers (ARIA live, coalesced — canvas rules CAN-A11Y-2 pattern); never modal-blocking.
- Destructive actions: confirmation via DS dialog (never native confirm), danger-styled primary,
  undo offered via toast where the action is document-mutating.

## I4 — Drag & drop (panels, lists, assets)

- Layer tree DnD follows DS states exactly (`07` §9): dragged row 70% + shadow; insertion
  indicator 2px accent line + 6px circle; drop-inside frame = inset 1.5px accent ring + selected
  bg; indent determines depth on drop; locked/hidden targets show forbidden cursor.
- One drag controller per draggable family (layers, assets, guides-from-rulers, panel resize
  handles are separate controllers); drag threshold + cancel semantics match canvas rules
  (3px [PROPOSED], Escape cancels, blur cancels).
- Drops into the canvas from panels/assets follow canvas snapping/overlay rules — the canvas
  state machine owns the interaction once the pointer is over canvas (one owner, CAN-MODE-1).

## I5 — Panel behaviors

- Resizable panels: drag handles with DS hit areas (min 24px hit), min/max widths enforced
  (240px default per `07` §4; min/max [PROPOSED: 200/480px; ADR]), double-click resets to
  default [OBS common], persisted as UI prefs (SM domain 3), never in `.gip`.
- Collapsible sections remember state per user prefs; collapsing never destroys field edit state
  mid-edit (commit or cancel explicitly).
- Scrubbable numeric fields: drag on label (ew-resize cursor) adjusts value; step = 1 by default,
  0.1 with modifier [PROPOSED: ⌥ fine-step; ADR]; commit on release/Enter, cancel on Escape
  restoring the original value exactly.

## I6 — Text inputs & forms (app chrome)

- Enter commits, Escape cancels-and-restores, Tab commits-and-advances (field navigation order =
  visual order). Invalid input: DS invalid border + inline message; never silent revert, never
  native validation bubbles.
- Mixed values in multi-selection show "Mixed" placeholder (`--t1` color per DS); editing commits
  only the edited property to all selected targets; unedited mixed properties stay untouched.

## I7 — Hover & feedback timing

- Global motion budget (DS): hover transitions 80ms · toggles 120ms · menus 160ms · ease
  `cubic-bezier(.2,0,0,1)`; reduced-motion ⇒ none. Feedback for user actions ≤100ms perceived;
  longer operations show DS progress/loading states — never a frozen unexplained UI.

## I8 — Testing

- Interaction tests: shortcut registry conflicts, focus scope traversal/restore, menu keyboard
  navigation, DnD state snapshots (all DS states), field commit/cancel/mixed semantics, panel
  resize persistence. Bug fixes here follow the same regression law (canvas rules CAN-TEST-1
  principle applies app-wide).
