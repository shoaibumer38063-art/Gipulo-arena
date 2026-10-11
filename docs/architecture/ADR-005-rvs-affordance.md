# ADR-005 — Responsive Vector Segments: Affordance Language

```text
Decision:   RVS affordances use EXISTING guide tokens, not red. Eligible straight segments =
            magenta (guide/spacing #ec2fd0): 2px overlay on the path + a small dot per eligible
            segment. Activated segments (will absorb resize/reflow) = success green (#14ae5c):
            2px overlay + count pill ("3 stretching"). Non-stretching curve/cutout context =
            35% blue dashed reminder (guide/geometry). Red stays reserved for ruler guides.
Date:       2026-10-11 (authored)
Status:     ACCEPTED under the owner's standing delegation of design decisions (2026-10-11).
            Reversible by a superseding ADR.
Closes:     design-system/08_canvas_ui.md §8 "pending ADR-005" · research notes item 3 · the
            README owner-review queue item (b) · rules/layout.md RVS-2 colour TODO.
```

## Context

- The vision document proposed red = eligible, green = activated. Red is reserved on the canvas for
  ruler guides; a second red meaning breaks the closed colour language (CAN-GUIDE-1, rules/canvas.md
  L3-2 flags this exact conflict).
- Accessibility (11_accessibility): colour is never the only signal, so RVS states also carry
  shape/pill/dash cues.
- Colour-vision check: magenta and green differ in lightness and hue family, and each carries a
  non-colour cue (dot, pill), so deuteranopia/protanopia are covered by the shape channel.

## Decision (sub-decisions)

| # | Sub-decision | Choice |
|---|---|---|
| 1 | Eligible segments | magenta `guide/spacing`, 2px overlay, dot at each eligible segment |
| 2 | Activated segments | success `#14ae5c`, 2px overlay, count pill with the number of stretching segments |
| 3 | Non-stretching context | 35% blue dashed reminder on curves and boundaries |
| 4 | Red | unchanged: ruler guides only |
| 5 | Colour-only signal | forbidden: each state has a shape or text cue as well |
| 6 | Tokens | all colours come from `02_tokens`; no inline hex (token-drift test covers it) |
| 7 | Toggle behaviour | unchanged from rules/layout RVS-2: toggle on selection → eligible shown; user activates → green |

## Consequences

1. The RVS feature can be built against these tokens; no new colour tokens are needed.
2. `guide/spacing` now also carries RVS eligibility (documented in 08 §9 "means").
3. The closed colour language is unchanged in size; the RVS entry is an application of existing meanings.

## Owner review (non-blocking)

Recorded for transparency: this was decided under delegation. You may override any row by a superseding ADR.
