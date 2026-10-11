# Selection System & Hit-Testing — Architecture Contract

> Status: DESIGN v1 (2026-10-11), shaped by ADR-001 (ACCEPTED: Rust/Wasm core + TS shell). Binding rules: CAN-SEL, CAN-HOVER,
> CAN-GEOM-4/5, SM1–SM3. Hub: `canvas.md`.

**Placement (ADR-001 — engine-core.md):** the selection, hover and edit-target stores LIVE IN THE
CORE (Rust) — they remain UI-state (never document data, never serialized), but the renderer
reads them directly per frame; resolution algorithms run core-side; shell panels subscribe via
boundary events.

## 1. Purpose & scope

Owns "what is selected / hovered" and the resolution algorithms that decide it: click/dblclick/
deep-select, marquee, hover prediction, mode-transition preservation. Hit-testing is this
system's query layer over the geometry engine — one algorithm serves hover AND click so hover
always predicts click (CAN-HOVER-1).

## 2. Owns

- **Selection store** (UI-state domain, SM1.3): ordered set of node IDs + container context
  (page / frame-or-group being edited) + edit target (vector-edit/text-edit node, if any).
  References by ID only; never serialized (CAN-SEL-4).
- **Resolution algorithm** (CAN-SEL-1 priority, binding):
  1. active overlay control under cursor (only when its owner object/mode is active);
  2. vector-edit mode: anchors → segments → handles (screen-space tolerances, CAN-SS);
  3. top-most visible node whose **precise hit geometry** contains the point (geometry engine
     `hitQuery` — paths/strokes, never bbox-guessing, CAN-GEOM-4);
  4. empty.
  Filters: locked ⇒ never canvas-selectable; hidden ⇒ never hittable (CAN-SEL-1).
- **Click semantics** (CAN-SEL-2): click = top-level child of current container context;
  double-click = descend one level; ⌘/Ctrl-click = deep-select exact leaf (parent context shown
  dashed 40% per DS); Shift-click = toggle additive.
- **Marquee** (CAN-SEL-3): top-level objects whose bounds intersect (touch-select default
  [OBS Figma; ADR-confirm vs contain]); starts-inside-frame descent rule; never locked/hidden.
- **Hover state** (CAN-HOVER): hovered node + hovered zone (handle/segment/point/ruler-band…),
  resolved with the same priority chain + zone priority (CAN-HOVER-2) + hysteresis
  (CAN-HOVER-3 [PROPOSED] 2px release). Hover is transient state (SM1.4) — never history.
- **Preservation matrix** (CAN-SEL-5): tool switch ⇒ keep; enter vector-edit ⇒ shape becomes
  edit target, prior selection retained as context; exit ⇒ restore; text-edit entry/exit same
  pattern; document deletion of selected nodes ⇒ selection drops to nearest valid ancestor
  context (documented behavior).
- **Mixed selection** (CAN-SEL-7): legal; bounds = union of member transformBounds; property
  panel contract for mixed values lives in `rules/ui.md`/interactions I6.

## 3. Must never

- Store selection on nodes or in `.gip` (CAN-SEL-4, CAN-SER-4).
- Use rendered pixels or bbox-only tests for point selection (CAN-ARCH-3, CAN-GEOM-4).
- Diverge hover from click results (CAN-HOVER-1 — divergence is a bug by definition).
- Mutate the document (selection is read + UI state; edits flow through tools/transactions).
- Keep stale IDs: subscription to document deletions prunes selection immediately (SM5).

## 4. Public interface (conceptual)

- Reads: `getSelection()` · `getEditTarget()` · `getHover()` · `selectionBounds(kind, space)`
  (delegates bounds math to geometry engine) · `isDescendantContext(nodeId)`.
- Writes (UI-state only): `select(ids, context)` · `toggle(id)` · `descend()/ascend()` ·
  `deepSelect(id)` · `clear()` · `setEditTarget(id|null)` · `setHover(state)` — all idempotent,
  all emitting narrow subscriptions (panels, overlays, a11y mirror consume).
- Queries for input-system: `resolveClick(worldPoint, modifiers)` · `resolveHover(worldPoint)` ·
  `resolveMarquee(worldRect)` → results, not side effects (the state machine commits them).

## 5. Dependencies

- Consumes: geometry engine (`hitQuery`, bounds, spatial index), document model (visibility/
  lock/type checks), coordinate module (screen→world for queries), DS tokens (visual states are
  rendered by overlay engine from selection store reads).
- Consumed by: input-system/tools (commit selections), overlay rendering (outlines/handles per
  CAN-SEL-6 states), properties panel, transform ops (target set), a11y mirror (focus sync,
  CAN-A11Y-1), history (selection is NOT recorded — only document effects are).

## 6. Invariants

- Hover predicts click (CAN-HOVER-1); selection visuals always derive from the store (no second
  selection truth in overlay code); deep-select parent-context display never changes the stored
  selection set semantics; at every zoom 5%–3200% tiny/zero-size objects remain addressable
  (CAN-GEOM-5, CAN-SS-4).

## 7. Open questions (ADR-gated)

1. Touch-select vs contain-select marquee default (CAN-SEL-3 [OBS]) → confirm in ADR-002 bundle.
2. Click-through policies for special content (locked groups' children, mask content) beyond
   CAN-MASK-1 → feature-doc decisions when masking ships.
3. Selection limit behavior on huge marquee results (perf guardrails) → measured at impl
   (budgets bind).

## 8. Test focus

CAN-TEST §25 selection/hit rows: click/dblclick/deep/shift/marquee matrices × locked/hidden/
rotated/nested/tiny/zero-size/low-zoom · hover⇄click prediction property tests · hysteresis
(no 1px flicker) · preservation matrix per mode transition · mixed-selection bounds union ·
deletion-pruning · marquee inside-frame descent · performance: resolveClick/hover within pointer
latency budget on 10k-node reference doc.
