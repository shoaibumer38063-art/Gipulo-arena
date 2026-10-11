# 13 — Research Notes ("research what I missed" — the brief's mandate, executed)

> The Design System Brief explicitly requires deep research of professional editor interface
> systems (Figma · Framer · Sketch · Illustrator · Affinity · Spline · modern editors) BEFORE
> finalizing the DS — not to copy, but to find missing categories. This file records what was
> studied, what was adopted, what was deferred, and the sources. `[UI3]` tags throughout the DS
> docs point here.

## 1. Sources studied (2026-10-11)

| Source | Key observations used |
|---|---|
| Figma — "Our approach to designing UI3" (figma.com/blog/our-approach-to-designing-ui3) | Panels went FLOATING then back to **docked + resizable** because floating "slowed people down — speed is a feature"; **Minimize UI** for distraction-free/small screens; properties panel restructure (component controls first, unified layout section, constraints expanded by default); **visible labels match aria-labels** and tooltips carry direction ("vertical padding") |
| Figma — "Behind our redesigned interface" (figma.com/blog/behind-our-redesign-ui3) | Radical minimalism (reveal-on-hover UI) tested and rejected — "too unstable"; the winning shape = roomier canvas + slim toolbar + collapsible panels; layout properties merged into one panel (aligns with code mental model) |
| Fast Company — Figma UI overhaul (fastcompany.com/91145720/) | Resizable/collapsible properties panel; moving away from hover-to-discover; labels visible at a glance |
| Cursor Design Mode coverage (builder.io/blog/cursor-design-mode-visual-editing; superdesign.dev/blog/cursor-for-design) | Design-to-code visual editing needs explicit apply/review; known failure = **design-system drift (agents emitting raw hex/pixels instead of tokens)** — validates this DS's tokens-only law and the Code Workspace's parity duty (`code-generation.md`) |
| Earlier session research (recorded in `docs/architecture/*`) | Figma Mirror-DOM accessibility architecture (selection/focus sync, coalesced announcements) → adopted as CAN-A11Y contract; tldraw camera/state-machine patterns → canvas architecture, not UI chrome |

## 2. Findings → decisions (adopted into the DS)

| Category found in research | Decision |
|---|---|
| Minimize UI mode (both panels collapse; quick restore) | **Adopted** — `12` §2; ties into the ≤900px single-column reference behavior |
| Docked-resizable panels over floating | **Adopted** (matches reference grid too) — `12` §2; splitter spec `06` §7 |
| Visible labels must equal aria-labels | **Adopted as law** — `03` §6, `11` §5 |
| Tooltips add direction/context, not repetition | **Adopted** — `06` §4 |
| Layout properties unified into one panel (code mental model) | **Noted for the properties-panel feature**: the panel's section ORDER is a feature-doc concern (canvas §07 memory); DS fixes row anatomy only (`06` §8) |
| Command palette as an editor-wide surface | **Adopted as specified-but-future** — `06` §14 (shortcut registry integration via interactions I1) |
| Design-tool ↔ code drift (tokens bypassed) | **Adopted as the reason** for tokens-only + registry + codegen parity suite; no direct UI surface |
| Comment pins / presence cursors / avatar stack | **Specified at requirement level, deferred** with the collaboration feature — `06` §14 |
| Version history UI | **Specified at requirement level** — row/timeline anatomy reuses layer-row class; feature phase owns it |
| Onboarding / empty-canvas education | **Adopted** into empty-state spec (`06` §13) — no separate system v1 |

## 3. Categories deliberately NOT adopted (with reasons)

- **Bottom toolbar** (Figma UI3) — Gipulo's own reference defines a 40px top toolbar `[DS]`;
  identity beats mimicry. Revisit only via owner decision.
- **Reveal-on-hover minimal UI** — rejected in the same public research ("too unstable").
- **Spring-physics motion** — no evidence of need; one easing keeps the feel fast (see `10`).
- **Floating-by-default panels** — rejected by Figma's own public reversal + our speed law.

## 4. Open questions this research surfaced (tracked, owner-visible)

1. Toolbar evolution (top ribbon per reference vs bottom float per UI3) — recorded; keep
   reference.
2. AA contrast corrections for very small text (`11` §2) — one consolidated owner DS review
   proposed.
3. RVS affordance colors (`08` §8) — small ADR (ADR-005) before that feature ships.
4. Touch/tablet editing depth — the brief lists tablet-sized windows; full touch editing is
   out of v1 scope (noted, not designed).

## 5. Method note (for future DS research tasks)

Research must produce DECISIONS, not dumps: each finding either becomes a spec change (with a
DS doc reference), a tracked open question, or an explicit rejection with a reason. "We studied
Figma" is not a deliverable — "we changed X because Figma's public reversal showed Y" is.
