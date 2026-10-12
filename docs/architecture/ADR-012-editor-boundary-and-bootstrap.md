# ADR-012 — Editor Boundary, Interaction Placement, and Bootstrap Painter

```text
Decision:   (D1) Slice 0 paints the core's draw data with a Canvas2D BOOTSTRAP painter in the shell,
                 replaced by the wgpu renderer in a later slice. The shell paints; it computes nothing.
            (D2) The interaction state machine lives in a NEW crate, gip-interaction, separate from
                 the browser bridge. Slice 0a has a transitional shell-side pan gesture; it moves into
                 gip-interaction in Slice 0b.
            (D3) Shell↔core messages cross the boundary as TYPED calls (wasm-bindgen methods) with
                 plain snapshot structs. No JSON on the pointer path.
Date:       2026-10-11 (authored)
Status:     ACCEPTED — the owner approved the recommended option for each of D1–D4 on 2026-10-11
            (task 020 plan). D4 (multi-select and Delete in Slice 0) is scheduled for Slice 0b.
            Reversible by a superseding ADR.
Closes:     plan task 020, decisions D1–D3. rendering.md §8 item 1 (partly: shader language unchanged).
Binds:      engine-core.md §1 (Boundary Duplication Law), §3 (boundary), §10 (protocol version) ·
            rendering.md §3 (bootstrap clause) · canvas rules CAN-COORD-2, CAN-MODE-1/2, CAN-CAM-1.
```

## Context

- The core had only a protocol-version handshake. There was no view state, no pointer path, and
  no canvas. The renderer crate is vocabulary only.
- rendering.md §3 allows a Canvas2D surface "ONLY for debug/bootstrap". Slice 0 is that case.
- Canvas law requires one coordinate module and one mode machine. Putting gesture logic in the
  shell would create a second mode system, which CAN-MODE-2 forbids.

## Decisions

### D1 — Bootstrap painter
| Rule | Choice |
|---|---|
| Who paints | The shell paints view data that the core produced (`ViewState`, later draw lists). |
| What the shell may compute | Pixel alignment for 1 px lines and device-pixel backing-store size only. No zoom, pan, or conversion arithmetic. |
| Lifetime | Temporary. Replaced by the wgpu renderer behind the same `gip-scene` draw-list contract. |
| Guard | Tests scan canvas sources for zoom and camera arithmetic (`app/src/canvas/paint.test.ts`). |

### D2 — Interaction state machine placement
| Rule | Choice |
|---|---|
| Home | New crate `gip-interaction` (layer 2, depends on gip-coords, gip-geometry, gip-history). Not in gip-session. |
| Testing | Gesture scripts run natively in cargo, without a browser. |
| Transitional state | Slice 0a's shell-side pan gesture is one variable (`idle` or `panning`). It is a known CAN-MODE-2 deviation, scheduled to move into `gip-interaction` in Slice 0b before any tool work. |

### D3 — Boundary shape
| Rule | Choice |
|---|---|
| Calls | Typed methods on `Engine` (`set_viewport`, `zoom_at`, `pan_by`, `view`). Results are `bool` for validation or plain structs. |
| Protocol | `PROTOCOL_VERSION` bumped 0 → 1. The shell mirror changes in the same commit (engine-core §10). |
| Copies | The shell copies `ViewState` into a plain object and frees the Wasm handle. |

## Consequences

1. Shell input is raw: client pixels and deltas go in, and core-computed view state comes out.
2. Slice 0a has NO document edits. Pan and zoom are not history entries (CAN-CAM-1).
3. Two transitional items are recorded for Slice 0b: the shell pan gesture (D2) and the absence
   of keyboard shortcuts (I1 requires a single shortcut registry before any shortcut ships).
4. The protocol bump is the first boundary change; `scripts/check-boundaries.py` and the shell
   handshake test both enforce the version.

## Not decided here

- The wgpu renderer's draw-list format (own ADR with the renderer slice).
- Keyboard shortcut registry shape (I1; decided when the first shortcut ships).
- Space-hold temporary hand tool (CAN-CAM-4): Slice 0b, with the state machine.

## Owner review (non-blocking)

Decided with the owner's approval of the recommended options. Override any row by a superseding ADR.
