# Input System & Tool State Machines — Architecture Contract

> Status: DESIGN v1 (2026-10-11), shaped by ADR-001 (ACCEPTED: Rust/Wasm core + TS shell). Binding rules: CAN-MODE, CAN-INPUT,
> CAN-MICRO (§27 registry), interactions I1–I2 (scopes), SM1.4. Hub: `canvas.md`.

**Placement (ADR-001 — engine-core.md):** this system LIVES IN THE CORE (Rust). The shell
normalizes DOM events, resolves focus scopes against the shortcut registry, applies cursor CSS
from core-reported cursor tokens, and forwards canonical events across the boundary
(engine-core.md §4). Shell-side input code is transport only; the machine is the single owner.

## 1. Purpose & scope

Normalizes all input (pointer/keyboard/wheel/touch/stylus) and routes it through ONE hierarchical
state machine so exactly one state owns the interaction at any time (CAN-MODE-1, tldraw's public
pattern). Owns gesture lifecycles, mode transitions, modifier semantics, cancellation — and the
snapping engine's activation interface during gestures.

## 2. Owns

- **Event normalization:** pointer events (down/move/up/cancel, capture on drag start), wheel +
  ctrl-wheel(pinch), keyboard (with focus scopes from interactions I2), touch/stylus where
  supported; positions converted screen→world ONLY via the coordinate module; DPR-independent.
- **The state machine:** root → active tool → active child state. Machine states include (CAN-
  MODE-1): idle · hovering · pressing · dragging/moving · resizing · rotating · panning · zooming ·
  drawing · marquee · vector-edit(point/handle/segment substates) · text-edit · creating-guide ·
  snapping-active · duplicating · transforming-multi. Mode = current node; readable by all,
  writable by the machine only (CAN-MODE-2 — boolean constellations banned).
- **Gesture lifecycle:** press → (threshold: 3px [PROPOSED, CAN-MICRO]) → drag → commit |
  cancel. Gesture-start snapshots live in transient state (transforms §2 pattern). Commit = one
  transaction (CAN-HIST-1); cancel = exact restore, zero writes (CAN-INPUT-4).
- **Cancellation paths (one funnel):** Escape (innermost-first: drag → mode → selection,
  CAN-MICRO) · pointercancel · window blur · pointer-capture loss · pointerleave policies
  (documented per state). All restore pre-gesture state; none create history.
- **Modifier semantics (live, CAN-INPUT-3):** the binding matrix from CAN-MICRO §27 is the single
  source (Shift constrain/axis · ⌥ center/duplicate/measure · ⌘ deep-select/snap-off/nested-
  measure · Space pan · ⇧M pin · ⌘; guides · ⌥⇧ nudge-measure). Re-evaluated every move event;
  changes apply mid-gesture without restart.
- **Snapping activation interface:** during move/resize/create gestures, queries the snapping
  engine (candidates via geometry engine) with screen-px thresholds (4px activation, ~6px release
  hysteresis [PROPOSED], ⌘ disables at 20% ghost per DS); applies exact snapped targets
  (CAN-SNAP-7) into the gesture computation; submits snap-guide overlay descriptors (never draws
  directly, CAN-OVL-2).
- **Tool contracts:** each tool = a state-node subtree declaring: cursor mapping (via cursor
  engine, CAN-CUR-1), hover behavior, click/drag semantics, committed mutations (through
  transactions), overlay descriptors, Escape behavior, and which selection/mode transitions it
  causes (CAN-SEL-5). Adding a tool = adding a subtree; NEVER parallel input paths.

## 3. Must never

- Allow two states to believe they own the pointer (CAN-MODE-1); routing is exclusive top-down.
- Mutate the document outside commit transactions; mutate during preview (transient only).
- Persist transient affordances past pointerup (CAN-INPUT-5) or enter previews into history
  (CAN-HIST-2).
- Implement private coordinate/hit/snap math (delegate: coordinate module, selection system,
  snapping via geometry engine).
- Swallow events silently: every event in every state has defined handling or explicit pass-through.

## 4. Public interface (conceptual)

- `feed(event)` — sole entry point (app shell forwards normalized DOM events).
- Reads: `getMode()` · `getActiveTool()` · `getGesture()` (debug/a11y/panels; e.g. properties
  panel disables during gestures per documented behavior).
- Tool registration: declarative subtree mounting (root owns registry); `setCurrentTool(id)`
  with documented side effects on selection/mode (CAN-SEL-5).
- Keyboard command routing: shortcut registry (interactions I1) resolves bindings per active
  focus scope; canvas-scope commands enter the machine as synthetic intents (same ownership laws).

## 5. Dependencies

- Consumes: coordinate module, selection system (resolution queries), geometry engine (via
  snapping/hit), transforms (gesture ops), history (commit/cancel), DS tokens (overlay
  descriptors reference tokens), cursor engine.
- Consumed by: app shell (event forwarding), tools (all), a11y mirror (mode announcements,
  coalesced per CAN-A11Y-2), tests (machine is directly driveable headless — events in,
  state/transactions out).

## 6. Invariants

- Single ownership at all times; mode readable deterministically (no hidden sub-booleans).
- Gesture atomicity: commit-once or restore-exactly; no partial writes observable.
- Hover predicts click (delegated to selection system's shared resolution, CAN-HOVER-1).
- Same event sequence ⇒ same state trajectory & transactions (determinism, testable headless).

## 7. Open questions (ADR-gated)

1. Drag threshold 3px, hysteresis values → confirm ADR-002 bundle (CAN-MICRO [PROPOSED] items).
2. Long-press semantics (touch) and stylus button mapping → ADR when touch/stylus support starts.
3. Text-edit input path: hidden textarea vs contenteditable vs custom IME pipeline → ADR with
   text feature (IME/composition law CAN-TEXT-3 binds either way).
4. Multi-touch gestures beyond pinch (two-finger rotate?) → product decision, ADR-gated.

## 8. Test focus

State-trajectory tests (event scripts → expected modes/transactions) headless · threshold/
hysteresis edge cases (2.9px vs 3.1px) · all cancellation funnels restore exactly, zero history ·
modifier mid-gesture application (CAN-INPUT-3) · Escape layering (drag→mode→selection) ·
multi-owner attempts (regression guards) · snap exactness & ⌘-disable · wheel/pinch routing to
camera (document never dirtied, CAN-CAM) · touch/stylus parity where supported · a11y: full
keyboard operation of canvas commands (CAN-A11Y-2).
