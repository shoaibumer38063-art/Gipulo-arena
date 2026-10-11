# 08 — Canvas UI System (overlays, guides, handles — and the closed color language)

> The canvas is the product. Its visual UI system is part of the Design System (Brief §Canvas UI;
> `rules/canvas.md` CAN-GUIDE/CAN-SS/CAN-OVL are the behavior law — this doc is the visual law).
> Values `[DS]` from the HTML reference. **Roadmap step 4 is satisfied here: the guide-color
> language is formally closed in §9.**

## 1. Universal zoom laws `[DS]` (binding for every overlay)

- **All guide strokes stay 1px on screen at EVERY zoom.** Handles and labels keep constant
  screen size; only positions follow geometry.
- **Labels hide below 25% zoom; handles hide below 12% zoom.**
- Pixel-crisp: 1px lines drawn at `x.5` device offsets; never blurry 2px-at-40% lines.
- **Contrast casing:** on dark/strongly colored content, selected objects get a 3px white halo
  under the outline (`stroke/halo`); handles stay white-filled. Dark theme uses the same guide
  colors + white handles (guide identity colors are theme-invariant — `02` §1.4).
- Pills flip above/below near viewport edges; label collisions stagger (`08` §7).
- Chrome is never exported, serialized, or selectable (CAN-OVL-1) — a visual system, not a
  document system.

## 2. Blue geometry guides — `guide/geometry` #0d99ff `[DS]`

Spec: 1px stroke · selection handles **7px white squares with 1px blue stroke** · pill radius 3,
`type/pill` · modes: object / frame-group / vector edit / text edit / text-on-path · opacity
law: hover 60% · selected 100% · parent context 40% dashed.
*(Reference discrepancy resolved: the frame-chrome example showed 8×8 handles; canon = 7px
everywhere `[PROPOSED]` — one number, one meaning; owner may confirm.)*

| State | Treatment |
|---|---|
| Hover | 1px outline, 100%, no handles |
| Selected | box + **8 handles** (corners + edges) + size pill (e.g. `60 × 36`) |
| Multi-select | children outlines 60% + group box 100% with handles |
| Resizing | original ghost dashed `3 2` at 50% + live box + live size pill |
| Rotating | box follows angle + angle pill (`20°`) + **angle snap tick marks every 15°** |
| Corner radius | 4 inner circle handles r3 + live pill (`R 10`) |
| Locked | dashed `2 2` at 60% + lock badge (9×7px) |
| Inactive | everything 35%, no hit areas |
| Text editing | box + 1px caret + `selection/fill` selection |
| Component | purple `component` box/handles + purple name tab pill |
| Frame hover | outline + blue name label above (label bg: 15% accent chip on name hover) |
| Frame renaming | white input box, 1px stroke + caret (`05` §3 editing state) |
| Group selected | box only — NO name |
| Child hover inside group | child outline 60% |
| Deep-select (⌘) | parent dashed `3 2` at 40% + child solid with handles |
| Clipped child | outline continues past clip at 50%, dashed `3 2` |
| Mask | contour dashed `4 2` |
| Component set | dashed `4 2` + purple name tab |
| Dragging object | ghost at origin dashed `3 2` + moving copy at 50% + delta pill (`Δ 6, 8`) |

**Precision tiers** `[DS]`: object **<24px** → only 4 corner handles · object **<8px** →
outline + center dot r1.5. Handles are 7px visual with **16px hit areas** (shown while relevant
as 16px dashed `2 2` at 12% zones). **Rotate zone** = arc outside each corner (r≈14, dashed
`2 2`, ↻ glyph) — cursor communicates rotation (`09`).

## 3. Vector edit mode (blue) `[DS]`

- Enter: anchors as hollow 7px squares (white fill, blue stroke), **no bounding box**.
- Point hover: 9px square, light-blue fill `#b3dfff`. Point selected: solid blue fill.
- Bézier handles: r3 circles white/blue + **0.75px** handle lines; mirroring keeps equal length,
  independently draggable (asymmetric mode honored).
- Marquee: dashed `3 2` + 8% fill; touched points become selected. Lasso: dashed loop `3 2`,
  6% fill.
- Segment hover: segment thickens to 3px preview + add-point ghost (circle r4 white + plus).
- Pen preview: rubber-band dashed `3 2` at 50% + preview anchor at 60%.
- Close path: **ring r7 around the first point**. Bend tool: pulls handles out of a segment
  (creates two r3 circles).
- Region hover (vector networks): closed region filled 12% (future-gated with the feature).
- Path itself: 1.5px stroke (`stroke/canvas-path`); handle lines 0.75px.

## 4. Text edit & text-on-path (blue) `[DS]`

- Auto-width: box hugs text, 8 handles · Fixed-width: wraps, side handles emphasized ·
  Auto-height: width handles only.
- Hover: 1px outline. Range select: `selection/fill` + 1px box.
- IME composition: dashed underline `2 1` at 1.5px.
- Overflow (fixed height): `…` pill tab (min 18px).
- Text on path `[PROPOSED]`: start/end square handles on the path; offset drag with % pill
  (`12%`); flip = small side toggle chip (`⇅`) on the path.

## 5. Magenta spacing guides — `guide/spacing` #ec2fd0 `[DS]`

Spec: 1px line · region fill 14% · end ticks 6px · pill radius 3, min-width 18px · smart
selection: **ring r4.5 at each object center** (hollow = available, solid = marked) · **gap
handle 4×16px radius 2** · requires 2+ layers with equal spacing; "Tidy up" creates the
relationship.

| State | Treatment |
|---|---|
| Gap | fill + line + value pill |
| Padding | 4 sides, one pill per side |
| Equal distribution | bracket line over equal gaps + pills |
| Margin-to-parent | 4 lines to parent edges + pills |
| Label hover | ring r6, 3px stroke, 40% |
| Editing value | white input box, 1px magenta stroke + caret |
| Fixed gap | hollow pill (panel bg + magenta stroke/text) |
| Auto gap | dashed `3 2` + "Auto" pill |
| Preview | 50% + dashed, no label hit |
| Available (smart) | rings + pink line 60% |
| Hover (smart) | gap handles appear |
| Marked (smart) | solid ring |
| Drag gap | ALL gaps change together + live pills; handle stretches to 6px wide |
| Reorder | ghost 60% + 2px insertion line |
| Resize marked | neighbors keep equal gap |
| Duplicate marked | ghost 50% dashed `3 2` |
| Delete marked | gap closes; ghost dashed 50% |
| 2D grid | row AND column gap handles |
| Tidy up | messy → dashed target positions |
| Not available | unequal gaps → NO pink shown (plain blue outlines) |

## 6. Orange snapping & measurement — `guide/snap` #ff8a00 `[DS]`

Spec: **snap = 1px dashed `4 3` + × markers at snap points, active within 4px while dragging**
· **measure = 1px solid + 6px end ticks + pixel pill (`type/pill`, r3, min 18px)** · measure on
hold **⌥** · nested measure **⌘⌥** · pinned measure **⇧M** (hollow pill, persists until Esc) ·
**measures bounds, not strokes** · multi-selection uses the group bounding box · values 0–2
decimals.

Single-object drag snapping: edge (line spans both objects + × at ends) · center (through both
centers) · equal gap (measure lines + pills on both gaps) · size match (value pill) · parent
edge (line on frame edge) · pixel grid (**cell highlight 35% fill + crosshair ×**) · text
baseline (dashed line under baselines + ×) · ruler guide (the ruler guide stays RED; the snap
line is orange).

Multiple-object dragging: group box dashed `3 2` at 60% (blue); snap attaches to group edge /
group center / group gap-to-neighbor / child edge (line to the child, not the group).

Measure (⌥): beside · above/below · diagonal (x pill + y pill) · inside hovered (4 outward
distances) · hovered-inside (4 inward) · overlapping edge pairs · decimals (pill grows, max
2dp) · **tight gap (<16px): label lifts above with a vertical leader** · **zero distance:
dot r2.5 + lifted "0" pill** · group→object (from group bounds) · vector anchors (anchor→anchor,
dx/dy pills) · nudge (⌥⇧↑ live value + step arrow).

Lifecycle: approach = 40% dotted `2 3` → snapped = 100% dashed `4 3` → released = fade out
120ms · snap disabled (⌘ held) = guide at 20% + "⌘ held" chip · **zoom <25%: lines only, no
labels** · label collision: stagger above/below · fractional: pixel snap off, 2dp (`28.37`).

## 7. Pills & labels (shared system across §2–6) `[DS]` + `[PROPOSED]`

- Geometry: radius 3, padding `1px 6px`, `type/pill` white text, tabular nums.
- Colors: pill background = the owning guide color (blue size pill, magenta gap pill, orange
  measure pill, `accent` for generic). Hollow variant: panel bg + colored 1px stroke + colored
  text (used for fixed/pinned states).
- Placement: 6px offset below (or above when flipped); near edges flips; collisions stagger
  (never overlap two pills); below 25% zoom labels disappear entirely (§1).
- Never interactive except where specified (editing value = white input variant, magenta-stroked).

## 8. RVS affordance (signature feature — ACCEPTED by ADR-005, 2026-10-11)

The vision doc proposed red = eligible segments, green = activated. Canvas law reserves red for
ruler guides and forbids red-means-error on canvas — a second red meaning risks the closed
language (`rules/canvas.md` L3-2 flags this exact conflict). DS recommendation:

| Element | Recommended treatment | Rationale |
|---|---|---|
| Eligible straight segments | **magenta** (`guide/spacing` #ec2fd0), emphasized 2px overlay over the path + small dot at each eligible segment | Magenta already means "layout/stretch relationships" — segments that will absorb a layout change ARE that meaning; no new color introduced |
| Activated (will stretch) segments | **success green** (`#14ae5c`) 2px overlay + count pill ("3 stretching") | Green = "active/on" reads instantly and matches the success token family; distinct from every guide color |
| Inactive/cutout emphasis (never stretches) | 35% blue dashed reminder on curves/boundaries | Blue = geometry context, low emphasis |
| Alternative (vision-original) | red eligible / green activated | ONLY via ADR-005, because it re-uses ruler red on canvas |

**Requirement:** this treatment must be ADR-005 (RVS affordance language) before RVS
implementation — it changes the closed language or formalizes the recommendation above.

## 9. THE CLOSED COLOR LANGUAGE (formalized — roadmap step 4) `[DS]`

| Color | Token | Means (complete, final) | Never means |
|---|---|---|---|
| Blue #0d99ff | `guide/geometry` | selection, hover, bounds, handles, vector paths/anchors, text edit, geometry context | spacing, snap targets |
| Magenta #ec2fd0 | `guide/spacing` | spacing, gaps, padding, distribution, margins, layout relationships (+ RVS eligibility recommended, §8) | selection, errors |
| Orange #ff8a00 | `guide/snap` | snapping targets, alignment/measurement lines & values | persistent state, layout |
| Red #f24822 | `guide/ruler` | ruler guides, layout grids — ONLY | errors on canvas (error states live in UI chrome, never as canvas guides) |
| Purple #9747ff | `component` | component/instance identity (boxes, name tabs, layer rows) | generic selection |

**The language is CLOSED.** Adding a new guide color or repurposing one is an ADR-level change
that must update this file + `rules/canvas.md` CAN-GUIDE-1 in the same task. No feature may
invent a canvas overlay color outside this table (the ADR-005 recommendation above is the one
pending change).
