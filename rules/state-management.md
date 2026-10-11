# STATE MANAGEMENT RULES — One owner per state, four separate domains

> Status: ACTIVE (v1, 2026-10-11). Sources: canvas rules CAN-ARCH-5, CAN-SEL-4, CAN-SER-4;
> vision doc (UI state vs document state separation).

## SM1 — The four state domains (never merged, never duplicated across)

1. **Document state** — the `.gip` truth: scene graph, geometry, paints, text, layout params,
   components/overrides, user ruler guides/layout grids. ONLY mutated via transactions with
   history (canvas rules §22). ONLY this domain serializes.
2. **Camera state** — `{x, y, zoom}` per viewport. Mutated by camera commands/gestures only;
   never by document edits; never serialized as document geometry (per-view persistence is an
   ADR-gated file-format feature).
3. **Editor/UI state** — selection, hovered node/zones, active tool, panel states (widths,
   collapsed, tabs), density/theme prefs, modals/open menus, search queries. References document
   nodes by ID — never by object embedding, never serialized into `.gip`.
4. **Transient interaction state** — the input state machine's current node, gesture-start
   snapshots, live drag values, snap previews, marquee rect, IME composition buffer. Lives only
   for the gesture; NEVER enters history; NEVER persists past cancel/commit (canvas rules
   CAN-INPUT-4/5).

## SM2 — Single ownership

- Every piece of state has exactly one owning store/module; everything else reads through its API
  or subscribes. Two places holding "the selection" (or zoom, or active tool) is a critical
  violation (CQ1.8 duplicate state).
- Derived values (bounds unions, mixed-value detection, visible-layer lists) are computed on read
  or cached with explicit invalidation tied to the source store — never stored as second copies.

## SM3 — Mutation discipline

- Document mutations: transactional, intent-labeled (one user intent = one entry), inverse-able;
  no component mutates document nodes directly behind the transaction API.
- UI state mutations are synchronous and cheap; they never trigger document recomputation except
  through declared subscriptions (e.g. selection change → properties panel read).
- Gestures: capture start-state in transient domain (canvas rules CAN-TR-2); commit once at the
  end through a transaction; cancel restores from the snapshot with zero document writes.

## SM4 — Reactivity & rendering

- UI re-renders subscribe to the narrowest state slices possible (a layer row subscribes to its
  node + selection membership, not the whole document).
- Canvas redraws are driven by invalidation regions (rendering rules R2), NOT by UI-store
  diffing; UI stores and the render loop are decoupled — a panel state change must not repaint
  the canvas, and a canvas gesture must not re-render unaffected panels.

## SM5 — Lifecycle & hygiene

- Mode/tool switches define what happens to selection and transient state explicitly (canvas
  rules CAN-SEL-5); "whatever happens to survive" is not a design.
- No hidden globals: module-level mutable singletons are banned except the declared stores;
  feature code receives state via documented interfaces (testability).
- Window blur/visibility change: transient gestures cancel safely (canvas rules CAN-INPUT-4);
  expensive background work (video, preview builds) suspends (rendering rules R5).

## SM6 — Testing

- Store-level unit tests: ownership, subscription granularity, transaction atomicity,
  cancel-restores-exactly, serialization includes ONLY document domain (golden round-trips,
  canvas rules §23), camera changes never dirty document, UI prefs never leak into `.gip`.
