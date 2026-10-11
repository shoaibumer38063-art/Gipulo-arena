# Task Summary 007 — Design System Implementation in Code (tokens + component library + gallery)

```text
Task: Implement the formalized Design System in the shell: token layer in code (drift-tested),
      17 registry entries (15 component modules + token layer + global scrollbar) with their state matrices, the gallery story page, and the test
      suites that make "state matrix = test matrix" real.
Date: 2026-10-11
```

**Problem being solved:** The DS was authored as docs (`docs/design-system/`), but the vision's
promise — "an AI agent should almost never have to guess what this should look like" — only
holds when the system exists as reusable, tested code. Without it, every future feature would
re-derive buttons/fields/menus, violating the composition law (07 §1) and the tokens-only law
(ui.md UI1).

**What was inspected:** All 14 DS docs (esp. `02_tokens` values, `05`/`06` state matrices,
`07` registry, `08` canvas rules, `13` research), the source HTML reference (ground truth),
`rules/ui.md` + `rules/icons.md` + `rules/code-quality.md` (naming/duplication laws),
`engine-core.md` (shell placement, testing pyramid layer 3 = Vitest for shell-only logic).

**Approach selected:**
1. **Token layer first, drift-guarded:** `tokens.css` (all ~110 tokens, light/dark, `@font-face`)
   mirrored 1:1 in `tokens.ts` (`TOKEN_NAMES` + `cssVar` + `applyTheme`); a test parses the CSS
   and fails on any drift in EITHER direction, and asserts dark-theme keys ⊆ light keys
   (theme-invariance of guide colors is enforced, not just documented).
2. **Self-hosted fonts:** downloaded the real Inter variable + JetBrains Mono variable files
   (48 KB + 31 KB woff2) into `design-system/fonts/` — offline-safe, no CDN dependency,
   `font-display: swap`.
3. **Components** (17 registry entries → 15 component modules + tokens + scrollbar): Button (all 5 variants + loading), Segmented, Field
   (scrubbable prefix with preview/commit phases, Mixed placeholder, invalid/disabled), SearchField,
   Toggle, Checkbox/Radio (native inputs rebuilt with appearance:none — a11y kept, defaults
   replaced), Slider (custom, keyboard + pointer), Chip, Menu (full keyboard nav, accent hover,
   check gutter, destructive, separators, one-level submenus with 150ms hover intent), Tooltip
   (400ms delay, instant on focus), Tabs (roving tabindex), Panel + Section, PropertyRow,
   LayerRow (indent law incl. 8px beyond level 8, guide lines, all 13 states), Icon sprite
   (reference set + node types, 16 grid / 1.25 stroke), plus scrollbar/selection/focus/reduced-
   motion laws in `globals.css`. Gallery = story page: every component in states + an
   illustrative editor frame (toolbar/panels/canvas mock).
4. **Tests = the state matrix** (engine-core.md §9 layer 3): 47 tests across token integrity,
   controls, surfaces (incl. fake-timer hover intent + tooltip delay), rows, gallery smoke.
5. **Environment:** vitest jsdom + testing-library + jest-dom; `vite.config.ts` carries the test
   config; `test-setup.ts` handles cleanup.

**Bugs found & fixed during the task (recorded per honesty law):**
1. **Field committed discarded edits:** Escape → `setDraft(null)` + `blur()` raced — React still
   had the old draft in the synchronous blur handler, so the aborted value committed. Fixed with
   an explicit `cancelNextBlurCommit` flag (test: Escape must NOT commit — now passes). Real
   user-facing bug, caught by the state-matrix test exactly as the testing rules intend.
2. Pointer capture is not universal (`jsdom`, older browsers) — `setPointerCapture`/
   `releasePointerCapture` calls are now progressive (helpers) so gestures still work.
3. `tsc` strictness items: `exactOptionalPropertyTypes` on Menu's callback props (added
   `| undefined`), missing `@types/node` for test fs access, `import.meta.url` not a file URL
   under Vitest (switched to cwd-relative paths).
4. Reference discrepancy re-confirmed: frame handles 8px in one HTML example vs 7px everywhere
   else — 7px remains canon (`08` §2) and is what the code uses.

**Verification (all run in this sandbox):** `tsc --noEmit` PASS · `vitest run` **47/47 PASS**
(6 files) · `vite build` PASS (49 modules, 252 KB JS / 77 KB gzip, fonts emitted) · dev server
serves the gallery (HTTP 200 incl. tokens.css + woff2). No editor features were built; the
gallery is explicitly a foundation surface.

**Alternatives rejected:** CSS-in-JS (tokens-as-CSS-variables is the multi-renderer contract —
the same tokens get pushed into the Rust core later; engine-core.md §2) · a component library
dependency (contradicts rule 3 of the DS laws — and the reference defines an identity no
library matches) · storybook (heavier than the problem; a typed gallery page + tests proves the
same thing with zero new dependencies) · building all 26 registry entries now (CQ1.2 — the
remaining five ship with their first real consumer).

**Remaining risks:** (1) Menu v1 supports one submenu depth (documented; deeper nesting arrives
with the first feature needing it). (2) Pointer-based interaction paths (scrub, slider drag) are
covered by unit tests for their logic; real pointer behavior gets Playwright coverage in the
E2E phase. (3) AA contrast recommendations from `11` §2 still await the owner's DS review —
code faithfully implements the reference values. (4) LayerRow virtualization is not yet needed
(trees are small until the layers feature) — flagged in the registry.

**Next steps:** ADR-002 (transform/matrix conventions) · ADR-003 (`.gip` format v0 — unblocks
fixture generation + the document crate) · ADR-004 (history storage) · ADR-005 (RVS affordance
colors — small, before that feature) · P0-class panic/trap recovery ADR · then the first engine
subsystems (coords → geometry → document) with golden-fixture suites.
