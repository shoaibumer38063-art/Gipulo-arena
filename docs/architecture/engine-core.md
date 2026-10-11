# Engine Core & Shell Boundary (Rust/Wasm ⇄ TypeScript) — Architecture Contract

> Status: DESIGN v1 (2026-10-11). Binding ADR: `ADR-001-technology-stack.md` (ACCEPTED — Option B:
> Rust/Wasm core + TS/React shell). Hub: `canvas.md`. This document resolves WHERE each subsystem
> lives and HOW the two halves communicate. It is the placement law of record — the other twelve
> contracts describe behavior; this one describes location and transport.

## 1. The founding law — Boundary Duplication Law

**No editor logic may exist in both halves.** Every subsystem has exactly one implementation
side; the other side may only *transport* (DOM events in, intents down, state reads/notifications
up). If logic feels "needed on both sides," the design is wrong — decide the owner, delete the
copy (extends CQ1.1 duplication prohibition across the FFI boundary).

## 2. Ownership table (the placement law)

**CORE — Rust crates, compiled to Wasm (and natively for tests/server):**
document model · geometry engine (paths/bounds/ops/spatial index) · layout engine (Auto
Layout/constraints/RVS) · coordinate conversions & matrices · camera · scene + renderer (wgpu;
document layer + overlay layer) · text shaping/layout · selection store · hover/edit-target
stores · gesture & tool state machine (CAN-MODE) · snapping · history/transactions · cursor-token
computation · `.gip` serialization/migrations · asset decode scheduling · a11y summary tree
(source for the mirror) · undo stack.

**SHELL — TypeScript/React (DOM-side):**
DOM event capture/normalization → canonical events · focus scopes + shortcut registry dispatch
(interactions I1/I2) · cursor CSS application from core-reported tokens · panels/menus/dialogs/
tooltips/toasts (all DS components) · accessibility mirror DOM (canvas §24 — renders core's a11y
summaries) · hidden textarea / IME composition bridge · OS integration (system clipboard bytes,
file open/save dialogs, File System Access, drag-drop of files, window title/dirty state) ·
Code Workspace UI + preview sandbox hosting (code-generation.md §5) · theming flow (DS token
values pushed INTO core for canvas-side chrome drawing) · devtools/debug overlays.

**Placement rationale:** everything the renderer must read per-frame (camera, selection, hover,
mode, document) is core-side — one truth, zero sync. Everything whose reason to exist is the DOM
(accessibility, IME, panels, OS dialogs) is shell-side. The state machine lives core-side because
hit-test/selection/snapping determine output — making "exactly one input owner" structural.

## 3. Boundary protocol

- **Transport:** wasm-bindgen; ONE generated client package (`engine-client`); **generated code is
  never hand-edited**; the generated surface is wrapped by a small hand-written typed facade.
- **Downward calls = intents/commands only** (`feed_event`, `select`, `set_tool`, `transact_intent`,
  `undo`, `open_file_bytes`, `resize_surface`, `set_theme_tokens`, …) returning typed values,
  IDs, or handles. The shell CANNOT write document fields directly — see §6.
- **Upward signals:** (a) discrete events via callbacks — selection changed · history changed ·
  dirty changed · cursor changed · a11y subtree updates · document replaced; (b) **per-frame reads
  via snapshot calls at rAF** for continuously-changing display state (dirty flag, zoom label,
  ruler data) — no per-field callback firehose.
- **Render path:** shell hands over the canvas element (or transfers an `OffscreenCanvas` to the
  worker) exactly once at init; the core owns the surface and draws. **No pixels cross the
  boundary** — the single exception is explicit user-facing pixel intents (eyedropper) which are
  core-side reads anyway.
- **Coordinates:** cross the boundary as `f64` world/viewport values only; all canvas math and
  space conversions stay core-side (CAN-COORD-2). The shell performs zero geometry math.
- **Hot-path data (bulk):** typed-array **views** over wasm memory (`Float64Array`/`Uint32Array`)
  with explicit length + version stamp; the shell copies only if it retains beyond the call.
- **Identity:** interned `u64` handles for nodes/sessions across the boundary — never string
  paths, never JS object graphs retained by the core.
- **Call-cost rule:** boundary calls are coin-flip cheap individually but are still an FFI edge:
  per-event calls fine (pointer/key are rare vs frames); per-node loops across the boundary are
  forbidden — bulk queries return arrays/views, never N calls.

## 4. The event → frame loop (concrete)

1. Shell captures DOM event, resolves focus scope + shortcut registry (interactions I1/I2).
2. Canvas-scope events → `engine.feed_event(canonical)`; panel actions → direct intent calls.
3. Core: state machine runs → hit-test/snap/preview-update (transient state) or transaction.
4. Core marks dirty regions; its rAF renders document + overlay passes via wgpu to its surface.
5. Discrete callbacks fire → shell updates panels/a11y (batched per frame; a11y announcements
   coalesced per CAN-A11Y-2).
6. Commits emit history events → dirty indicator; save: shell requests `.gip` bytes → OS write.
7. Cancel path mirrored: Escape/window-blur/pointer-cancel → core intent → exact restore + redraw
   (input-system.md §2 funnels retained in core).

## 5. Threading & hosting

- **v1:** module may start on the main thread — the boundary API is designed async-safe so the
  move is re-plumbing, not redesign.
- **Target:** core in a dedicated Web Worker owning `OffscreenCanvas` + rAF; main thread only
  forwards events (Comlink-class pattern; verified viable with wgpu `SurfaceTarget::OffscreenCanvas`
  [PUBLIC: matthewjberger/webgpu-worker, 2026]). UI jank under engine load becomes impossible.
- **Threads (rayon in wasm) later, only if measured need:** requires `SharedArrayBuffer` ⇒
  **COOP/COEP response headers** on every host (dev server, preview, production) — hosting
  requirement recorded now; if a host cannot set them, the single-threaded profile ships instead.
- Workers also host: export renders, codegen builds, asset decode — dispatched by the shell
  (which owns the worker pool as DOM-adjacent infrastructure) calling core entry points.

## 6. Laws this boundary makes physical

- **SM3 (sole mutation path):** the shell has *no* document write API at all — writes are
  impossible except as core transactions. Undo atomicity (CAN-HIST-1) is enforced by construction.
- **CAN-ARCH-2/3:** dependency direction and "pixels never flow back" are enforced by what the
  boundary exposes (geometry in, no readback out).
- **CAN-SS chrome / CAN-OVL:** overlay descriptors and rendering exist only core-side; the shell
  cannot draw editor chrome (it only draws DOM panels).
- **G7 determinism:** all deterministic logic is core-side; the shell is allowed to be stateful DOM.

## 7. Memory discipline

- wasm32: 4 GB ceiling, grow-only [PUBLIC] — canvas §26 budget ≤1.5 GB keeps ≥2× headroom; budget
  suite tracks live memory + growth over scripted sessions.
- Core owns caches (tessellation/booleans/spatial index); shell holds only DOM/React state.
- Large transient payloads (imports, exports, pasted `.gip` fragments) have explicit
  dispose/one-shot paths (bytes in → IDs out; or IDs in → bytes out, never both retained).

## 8. Error & crash policy

- **Boundary paths never `unwrap`** — typed `Result` → JS exception with a stable error code; the
  shell maps codes to DS error UI patterns (never raw messages as copy).
- **Wasm traps kill the module instance** (platform reality): the shell detects the trap, presents
  the recovery surface, and the core resumes from the last autosave snapshot. **Panic-recovery &
  document-integrity policy is a P0-class ADR at implementation start** (autosave cadence,
  recovery UX, telemetry-free diagnostics). **Realized as ADR-008 (PROPOSED, 2026-10-11).** No risky subsystem (booleans, huge imports, codegen
  builds) ships before that ADR exists.

## 9. Testing pyramid (placement-aware)

1. **`cargo test` (native, headless)** — the majority: geometry/layout/coords/history/file-format/
   machine trajectory tests. Fast, deterministic, no browser, no wasm.
2. **`wasm-bindgen-test` (headless browser)** — the boundary contract itself: codecs, surface
   init, callback wiring, memory-view lifetimes.
3. **Vitest** — shell-only logic: reducers, shortcut registry, panel formatting (no editor math).
4. **Playwright** — E2E interaction scripts, golden-image canvas snapshots, a11y mirror checks,
   code-preview boot (code-generation.md §9).
5. **Budget suite** — native + wasm on the five reference documents (canvas §26), now including
   wasm module load time + memory ceiling checks.
CI runs all five; (1)+(2) gate merge; (4)+(5) gate task completion per testing rules T5.

## 10. Build & artifact discipline (realized in the scaffold, 2026-10-11)

- Targets: `wasm32-unknown-unknown` (pinned in `rust-toolchain.toml`); dev profile keeps
  `debug = 0` for build speed; **module size is a tracked budget** (first-load target measured
  against canvas §26 open-time ≤3s incl. instantiation).
- Bindings build: `scripts/build-wasm.sh` runs `wasm-pack build crates/gip-session --target web
  --dev --out-dir "$ROOT/bindings/pkg" --out-name gip_engine` (the out-dir is **absolute** —
  wasm-pack resolves relative out-dirs against the given crate path, a real trap hit during the
  scaffold). Output is generated: gitignored,
  never hand-edited; CI runs the same script and asserts `bindings/pkg/gip_engine_bg.wasm` exists.
- Version handshake realized: `PROTOCOL_VERSION` in `gip-session` (Rust) mirrored in
  `app/src/engine/client.ts`; `assertProtocolVersion` throws a descriptive startup error on
  mismatch (protects against stale bindings after agent edits).
- The shell does NOT import the wasm package yet — it will, starting with the first real
  intent/snapshot surface (renderer/boundary phase). Until then the handshake contract is
  exercised by shell unit tests.

## 11. Open questions (tracked in ADR-001 §"Open questions")

1. Worker rollout timing + COOP/COEP hosting plan. 2. Panic/trap recovery ADR (P0-class) — realized as ADR-008.
3. Snapshot-read granularity tuning (which states are per-frame reads vs events) — tuned at impl
with budget measurements. 4. Dev-profile build speed strategy (cargo profile + watch tooling).
5. Whether scene/selection debug overlays expose any debug-only boundary reads (devtools) —
decide when devtools land (must not become a second truth).

## 12. Test focus (for this contract itself)

Boundary round-trips (every intent: shell fixture → core → expected events) · no-copy correctness
(typed-array views invalidated correctly after growth) · protocol version mismatch → clean error ·
trap injection → recovery surface + snapshot resume · memory growth scripted sessions (no
unbounded growth) · callback batching (no per-event panel storms) · determinism: same intent
sequence ⇒ identical core outputs (native vs wasm parity checks on golden docs).
