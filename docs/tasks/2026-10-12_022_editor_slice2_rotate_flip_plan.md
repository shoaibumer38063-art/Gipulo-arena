# Task 022 — Editor Slice 2: rotate, flip, and resize on rotated objects (PLAN)

- **Date:** 2026-10-12 · **Status:** PLAN, awaiting decisions D1–D3 before code.
- **Predecessor:** Slice 1 (resize with handles, task 021), committed locally as `0f52a9d`; not yet pushed.
- **Rules read:** `rules/canvas.md` CAN-TR-1..7, CAN-HOVER-2, CAN-CUR-1..3, CAN-SEL-6.

## 1. Scope — "an object you can rotate, flip, and resize at any angle"

In scope:
- **Rotate** by a rotate zone just outside each corner (arc zone r≈14 px, [DS]). Pivot = object center (CAN-TR-4).
  Shift constrains to **15° increments** (CAN-TR-4).
- **Flip** horizontal and vertical about the object center (CAN-TR-5), from keyboard (Shift+H / Shift+V).
- **Resize of rotated objects** (deferred from Slice 1): the eight handles follow the object's axes;
  the start-value math works in the object's local frame (CAN-TR-2, CAN-TR-3).
- **Cursor hints** (CAN-CUR-1/2): the cursor shows the action that will happen before pointer-down.
  Resize cursors follow the object's axes, quantized to the eight-direction set; rotate zones show a rotate cursor.
- **Priority of activation zones** (CAN-HOVER-2): corner handles > edge handles > rotate zones > body.
- One history entry per rotate, flip, or resize gesture; Escape restores exactly (CAN-MODE-4).

Out of scope (deferred):
- Resize of **flipped** objects: handles hidden while flipped (see Status).
- Rotating or resizing **several** objects at once (needs multi-bounds; a later slice).
- Snapping and guides (CAN-SNAP): their own slice (snap budget then measured).
- Text, vector edit, layers, properties, groups, file UI.

## 2. Model decisions

- **Transform model (CAN-TR-5):** flips are retained as flags (`flip_h`, `flip_v`), not decomposed into
  rotation. This is already the model in `gip-document::Transform`. CAN-TR-5 marks it "ADR needed", so a short ADR is required.
- **Angle convention (CAN-TR-4):** one convention, used everywhere including saved display values.
- **Gesture math:** rotation = angle(pointer − center) − angle(press − center) + start rotation.
  Resize in the local frame = inverse-rotate the pointer delta, then apply `resize_box`. All from gesture-start values.

## 3. Acceptance criteria (proposed, numbered AC-41..)

- **AC-41** Rotate zone outside each corner; dragging rotates about the object center by exactly the pointer's angular change.
- **AC-42** Shift during rotate snaps to multiples of 15°, measured from the start angle; exact within 1e-9 rad.
- **AC-43** Flip H / Flip V (Shift+H, Shift+V) mirror about the object center; flipping twice restores the original exactly.
- **AC-44** Resize of a rotated object keeps the opposite edge fixed in the object's local frame; Shift and Alt behave as in Slice 1.
- **AC-45** Rotation and resize are one history entry each; Escape restores the exact start transform (bitwise equal draw list).
- **AC-46** 1,000 back-and-forth rotate gestures produce no drift (start-value math, CAN-TR-6).
- **AC-47** Activation priority: corner handle beats a rotate zone where they overlap; rotate zone beats body.
- **AC-48** Cursor hint reflects the action before pointer-down (rotate zone → rotate cursor; corner → diagonal resize quantized to the object's axes).
- **AC-49** Protocol bump and boundary additions (flip, rotate commit) with the shell mirror updated in the same change.
- **AC-50** Pointer→visual ≤ 2 frames p95 during rotate on the `rects-1000` fixture (headless, as in 020c).

## 4. Decisions needed (recommended option first)

- **D1 Flip model:** A) retain flip flags, ADR-recorded (recommended; matches the current `Transform` and the CAN-TR-5 proposal). B) decompose negative scale into rotation + positive scale (simpler math, but loses the flip state users see).
- **D2 Angle convention:** A) −180…180 degrees (recommended, common in design tools). B) 0…360 degrees.
- **D3 Cursor hints:** A) include in this slice (recommended: rotated-resize cursors are part of AC-48). B) defer to a cursor slice (handles still work, cursors do not change).

## 5. Verification plan

- Rust: rotation about center; Shift 15° snapping; flip twice is identity; rotated resize local-frame tests; priority and cancel tests; 1,000× drift.
- App: typecheck; Vitest (cursor mapping table, protocol mirror); production build.
- Playwright: rotate by zone; Shift-rotate; Shift+H flip; resize a rotated object; Escape cancel; undo; frame pacing on the fixture.
- Gates: clippy `-D warnings`, fmt, wasm check, boundary check.

## 6. Estimated size

About 600 lines of core and tests, and 300 lines of shell and E2E. One task summary (`022`) at completion.

## 7. Status

APPROVED (D1 keep flip flags, D2 −180…180°, D3 cursor hints included). Implemented in task 022
(`2026-10-12_022_editor_slice2_rotate_flip.md`).

**Recorded deferral (lifted in task 023):** resize of a **flipped** object was not offered in this slice. Handles are hidden
while an object is flipped (`resize_target` excludes flipped nodes). Flip itself works. Lifting this
needs the resize math to handle mirrored local axes; it is a later slice.
