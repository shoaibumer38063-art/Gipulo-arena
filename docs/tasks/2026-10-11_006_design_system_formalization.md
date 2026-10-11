# Task Summary 006 — Design System Formalization (roadmap steps 3 + 4 complete)

```text
Task: Formalize the Gipulo Design System from the HTML reference + brief into the authoritative
      docs/design-system/ set (token layer, components with ALL states, icons, cursors, canvas
      UI, motion, a11y, responsive, research notes) and close the guide-color language.
Date: 2026-10-11
```

**Problem being solved:** The vision mandates the DS as the single visual+interaction source of
truth built BEFORE the editor — so agents "almost never have to guess what this should look
like." Requirements existed (`05` brief), values existed (`07` distilled from the HTML), but no
formalized system: no per-component state matrices beyond examples, no consolidated token layer,
no cursor/icon inventories, and the roadmap's step 4 (closed guide-color language) was still
implicit across files.

**What was inspected:** `docs/memory/05_design_system_brief.md` (273 lines of requirements),
`docs/memory/07_design_tokens_and_visual_specs.md` (333 lines of extracted values incl. §12
canvas overlay specs), the source HTML reference (structure + section map), current rules
(`ui.md`, `icons.md`, `canvas.md` CAN-GUIDE/CAN-SS/CAN-CUR), `03_documentation_system.md`
(canonical doc locations), and fresh research: Figma UI3 design blog posts + Fast Company
coverage, Cursor Design Mode analyses (sources in `13_research_notes.md`).

**Approach selected:** 14 documents in `docs/design-system/` (README + 13 numbered), with:
- **README hub:** mission, the eight DS laws, authority order, file index, tagging convention
  (`[DS]`/`[UI3]`/`[PROPOSED]`), change protocol, honesty section (what does not exist yet).
- **Values sourced, never invented:** all `[DS]` values trace to `07`/the HTML; gaps authored
  beyond the reference are explicitly `[PROPOSED]` (z-layer scale, resizable panel ranges,
  cursor art construction, dialog metrics, contrast-driven token bumps).
- **Decisions made and recorded:** one easing; no springs v1; selection handles canon 7px
  (reconciling 7px vs 8px reference discrepancy, flagged); hot/cold component tiers; guide
  language closed; RVS affordance RECOMMENDED as magenta-eligible/green-activated (vs the
  vision doc's red/green) because red is reserved for ruler guides — flagged for ADR-005 before
  RVS ships (L3-2 satisfied procedurally).
- **Contrast audit performed** against WCAG AA with concrete findings (secondary .5 text ≈
  3.9:1 fails for 11px; primary-button white-on-accent ≈ 3.3:1; etc.) + proposed token bumps
  marked as one consolidated owner DS review — the DS doing its job rather than hiding the
  issue.
- **Research mandate executed** (`13`): findings → adopted/rejected/deferred decisions with
  citations (docked-resizable panels, Minimize UI, labels=aria-labels; bottom toolbar rejected
  in favor of the reference's identity).

**Verification:** value spot-checks against `07` (colors, metric table, guide specs, motion) —
no invented numbers tagged `[DS]`; cross-reference sweep (rules/ui.md now points to the DS set;
rules/canvas.md visual-values pointer updated; `03` gained §6b documenting the DS location);
component registry counts (26 registered entries); CJK-anomaly scan; file/line counts.

**Remaining risks:** (1) the AA contrast findings need the owner's one-time approval to change
reference values (until then reference values stand; recommendations bind new surfaces).
(2) RVS colors await ADR-005. (3) The properties-panel section ORDER and per-selection-type row
sets are feature-doc concerns (DS fixes anatomy/states only — noted in `06` §8). (4) Touch
editing depth deliberately out of v1.

**Next steps:** (a) DS code implementation — tokens.css/.ts + component library + story pages in
`app/src/design-system/` (next phase, with tests); (b) ADR-002/003 bundles (or the owner may
prefer the DS code first); (c) testing system setup (roadmap step 6).
