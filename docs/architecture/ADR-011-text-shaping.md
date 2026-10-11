# ADR-011 — Text Shaping and Font Parsing Stack

```text
Decision:   Shaping uses harfrust (the HarfBuzz project's own Rust port, MIT). Font parsing uses
            skrifa / read-fonts (the Google Fonts "fontations" stack). rustybuzz and ttf-parser are
            NOT used: both are unmaintained upstream.
Date:       2026-10-11 (authored)
Status:     ACCEPTED under the owner's standing delegation of design decisions (2026-10-11).
            Dependencies are NOT added yet; they are registered when the text feature starts
            (dependencies.md §3). Reversible by a superseding ADR.
Closes:     canvas.md §7 open question 7 (shaping strategy) · rendering.md §8 open question 4
            (partly: the rasterization path is still open, see below) · ADR-001 "text shaping crate choice".
Binds:      CAN-TEXT-1 (text engine owns shaping; one layout result) · CAN-TEXT-3 (logical order) ·
            CAN-TEXT-5 (font fallback) · ADR-002 (coordinate spaces: `text` is run-local space).
```

## Context (verified facts, 2026-10-11)

- **rustybuzz** (MIT, a port of HarfBuzz shaping) was **archived by its owner on 2026-07-26**. Its
  README recommends **harfrust**. RUSTSEC-2026-0206 flags it unmaintained.
- **harfrust** is HarfBuzz's own Rust port (MIT), tracks HarfBuzz 14.x, and is already used by
  egui (epaint) in this dependency tree.
- **ttf-parser** is flagged unmaintained (RUSTSEC-2026-0192). Its recommended replacement is
  **skrifa / read-fonts** from Google Fonts' fontations project; harfrust also sits on read-fonts.
- Feature parity for Arabic and complex scripts must be verified by a shape-regression corpus
  before the text feature ships (rustybuzz/harfrust differences are a known risk).

## Options considered

| Option | Verdict |
|---|---|
| **harfrust + skrifa/read-fonts** | ✅ Chosen: maintained, same family as the reference shaper, one font stack shared with shaping |
| rustybuzz + ttf-parser | ❌ Archived and unmaintained |
| Platform shaping (browser `Intl`/Canvas text, CoreText, DirectWrite) | ❌ Four copies of text metrics (CAN-TEXT-1 violation); cannot be identical across platforms |
| Link the C HarfBuzz library | ❌ C dependency in the Wasm core; loses the pure-Rust build (ADR-001) |
| Custom shaper | ❌ Not justified; shaping correctness is a solved, huge problem |

## Decisions

| # | Rule | Choice |
|---|---|---|
| 1 | Shaper | harfrust, pinned by lockfile; version chosen at text-feature start (candidate 0.12, as already resolved in the tree) |
| 2 | Font access | skrifa / read-fonts only. No ttf-parser |
| 3 | Owner | `gip-text` owns shaping, line breaking, caret and selection geometry (CAN-TEXT-1). Rendering and export read its layout result |
| 4 | Determinism | Same font bytes + text + features ⇒ same glyph positions (R4 applies). Font bytes are recorded by content hash in the asset manifest (ADR-003 #6) |
| 5 | Gate before shipping text | (a) shape-regression corpus including Arabic, Devanagari, and mixed-direction text, diffed against the reference HarfBuzz output; (b) wasm size measured against a recorded budget; (c) benches recorded in the task |
| 6 | Fallback fonts | CAN-TEXT-5 applies: missing fonts use a documented fallback with an error state in the layers panel |
| 7 | Dependency scope | harfrust and skrifa/read-fonts may be used ONLY by `gip-text`. dependencies.md §2 is amended when the feature starts |

## Not decided here (explicit)

- **Text rasterization path** (SDF / GPU atlas vs CPU raster): still open, rendering.md §8 item 4.
  It needs its own ADR with the text feature and the WebGL2 tiers (ADR-010): an atlas or SDF path
  must have a declared T3 fallback.
- **Text-on-path**: CAN-TEXT-4 keeps it proposal-stage; no code until its own ADR.

## Owner review (non-blocking)

Decided under delegation. Override any row by a superseding ADR.
