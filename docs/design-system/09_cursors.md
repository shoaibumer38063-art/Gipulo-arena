# 09 — Cursor System

> Requirements: Design System Brief §Cursor Design System (`rules/canvas.md` CAN-CUR is behavior
> law). Principle: **agents must never randomly use whatever cursor seems convenient.** Cursor
> appearance is part of the product's feel — in a professional editor the cursor is the user's
> hand.

## 1. Policy

- **Chrome** (panels, menus, buttons): `cursor: default` — Gipulo deliberately does NOT use
  `pointer` over buttons `[DS]` (matches professional-tool convention); text inputs use the
  native I-beam; resize splitters use native col-resize equivalents.
- **Canvas**: every cursor is part of the Gipulo cursor set below — all custom, all spec'd,
  all rotation-aware where geometry demands it. Native browser cursors never appear over the
  canvas.
- **Dragging never changes the hit model:** the cursor while dragging is part of the gesture's
  spec (every drag state below names its cursor).

## 2. Visual construction `[PROPOSED]` (authored — reference defines behavior, not the art)

- Drawn in a 24×24 canvas box, anchored at an exact **hotspot** (specified per cursor).
- Line weight 1.25px at 1x (matches icon language `04`); white fill with a 1px dark outline
  (+ 1px white outer rim where placed over dark guide colors) so cursors are visible on ANY
  canvas content — light, dark, colored, and over guides.
- Rotation: cursors that express direction (resize/rotate/scale) are ROTATED to match the
  object's transform (CAN-CUR-1) — a rotated rectangle's corner cursor points along its real
  diagonal, never a fixed browser arrow.
- No animated cursors; no tooltip text inside cursors (badges are glyphs only).
- Badges (modifier context) render at the cursor's lower-right: 12px circle or 10px glyph,
  `tip/surface` fill, white glyph (e.g. `+` duplicate, `Ø` forbidden, `⌘` override).

## 3. Inventory (chart below = the complete allowed set)

| Cursor | When shown | Notes |
|---|---|---|
| `default-arrow` | chrome; canvas idle with no tool/hover | native over chrome; custom over canvas |
| `text-ibeam` | over text input; text tool over text; entering text edit | native in fields |
| `hand-open` | hand tool active; Space held (pan-ready) | space = temporary hand (canvas §27) |
| `hand-grabbing` | panning (hand tool or space-drag) | closed fist |
| `move` | move tool over object; dragging objects | solid arrow glyph `[DS]` icon |
| `resize-h` / `resize-v` | edge handles, axis-aligned | rotation-aware (rotates with object) |
| `resize-diagonal` | corner handles (4 directions via rotation) | one glyph, rotated |
| `rotate` | rotate zone (arc outside corners) | curved-arrow glyph; tangent direction |
| `scale` | proportional corner handle (with modifier contexts) | corner arrows + `⌗`-class glyph `[PROPOSED]` |
| `crosshair` | shape tools while drawing; precision contexts (snap off with ⌘ over marquee?) | thin cross + center dot `[PROPOSED]` |
| `pen` | pen tool ready | nib glyph; preview states in `08` §3 |
| `pen-add-point` | pen over segment (add point) | nib + `+` badge |
| `pen-delete-point` | pen over existing point (remove) | nib + `−` badge |
| `bend` | bend tool over segment | curve glyph `[PROPOSED]` |
| `vector-select` | vector edit mode idle (no specific point) | hollow arrow `[PROPOSED]` |
| `anchor-grab` `[PROPOSED]` | over an anchor in edit mode | hollow square glyph |
| `eyedropper` | eyedropper active | dropper glyph; live color chip beside cursor `[PROPOSED]` |
| `zoom` | zoom tool active | magnifier glyph with `+`/`−` per modifier |
| `duplicate` | ⌥ held during object drag (canvas §27) | move glyph + `+` badge |
| `drag-copy` `[PROPOSED]` | dragging FROM assets/panel onto canvas | move glyph + `+` badge + ghost preview |
| `forbidden` | illegal drop (locked/own-child destination, invalid parent) | `Ø` badge on move glyph |
| `drop-inside` `[PROPOSED]` | dragging over a frame/group that will adopt the drop | move glyph + inset-ring badge hint |
| `mask` | mask tool/drag context | mask glyph `[PROPOSED]` |
| `crop` `[PROPOSED]` | image crop mode (feature-gated) | crop brackets glyph |
| `comment` `[PROPOSED]` | comment tool (feature-gated) | pin + tip glyph |
| `orbit-3d` `[PROPOSED]` | 3D scene navigation (vision Solution 3, future) | orbit glyph |
| `not-allowed-action` | reserved distinct from `forbidden` for chrome-level blocks | OS-equivalent visual, custom art |

Modifier variants (from canvas §27): ⌘ deep-select = default arrow with `⌘` badge while over a
group/parent context `[PROPOSED]`; ⌘ snap-off = no cursor change (guide dims instead — `08` §6);
⇧ constraints = no cursor change; Space = `hand-open` (§3).

## 4. Hit-zone pairing law

Each cursor is the visible confirmation of a HIT ZONE; the two are specified together:
handles (16px hit areas, `08` §2), rotate arcs (r≈14), splitters (8px band), text ranges
(native), panel rows (whole-row hit). A cursor may never appear where the click does nothing,
and a hit zone may never be cursorless on the canvas.

## 5. Implementation notes (for the implementation phase)

- Cursor art ships as inline SVG data-URI cursors with `hotspot-x/y`; a single module owns the
  mapping cursor-ID → art (`rules/code-quality.md`: no per-feature cursor choices).
- Rotation-aware cursors are computed at pointer time, not pre-baked per angle — the same
  rotated SVG, transformed (one art asset per family).
- The cursor set is versioned with the DS; new cursors are added here first (`07` §6 process).
- Accessibility: cursor changes are never the ONLY feedback for state (hover outlines and
  labels carry meaning — `11` §7); custom cursors keep a visible fallback if art fails to load
  (map to `crosshair`/`default` rather than invisible).
