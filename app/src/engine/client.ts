/**
 * Gipulo engine facade — the ONLY module the shell may use to talk to the core.
 *
 * Contract: docs/architecture/engine-core.md §3, §10.
 *   - Shell intents go DOWN (typed calls); the view state comes UP as plain data.
 *   - Generated bindings (bindings/pkg, built by scripts/build-wasm.sh) are consumed here and
 *     NOWHERE else — and never hand-edited.
 *   - Boundary Duplication Law: no editor logic in this file. Transport only. Every coordinate
 *     conversion happens in the core (CAN-COORD-2); the shell sends raw client-pixel positions.
 */
import init, { Engine, protocol_version } from '../../../bindings/pkg/gip_engine.js';

/** Shell-side mirror of `PROTOCOL_VERSION` in core/crates/gip-session. Keep in sync — the
 * handshake exists precisely to catch drift (a mismatch is a clear error, never silence). */
export const PROTOCOL_VERSION = 1;

/**
 * Verifies the core's reported protocol version against the shell's expectation.
 * Throws a descriptive error on mismatch (engine-core.md §10: clear startup error).
 */
export function assertProtocolVersion(coreVersion: number, expected: number = PROTOCOL_VERSION): void {
  if (coreVersion !== expected) {
    throw new Error(
      `engine protocol mismatch: shell=${expected} core=${coreVersion} — ` +
        'rebuild the wasm bindings (scripts/build-wasm.sh) and reload',
    );
  }
}

/** The view state the core reports for painting. Positions are viewport pixels. */
export interface EngineView {
  readonly zoom: number;
  readonly zoomPercent: number;
  readonly cameraX: number;
  readonly cameraY: number;
  readonly originX: number;
  readonly originY: number;
  readonly width: number;
  readonly height: number;
}

/** The shell's handle to one engine instance. Methods forward to the core. */
export interface EngineHandle {
  readonly protocolVersion: number;
  /** Records the canvas client rect. Returns false for invalid sizes. */
  setViewport(x: number, y: number, width: number, height: number): boolean;
  /** Zooms by `factor` around a client-pixel point, keeping that point fixed. */
  zoomAt(clientX: number, clientY: number, factor: number): boolean;
  /** Pans by a pixel delta (content follows the pointer). */
  panBy(dx: number, dy: number): boolean;
  /** The current view for painting. */
  view(): EngineView;
}

let started: Promise<EngineHandle> | undefined;

/**
 * Instantiates the Wasm core once and performs the protocol handshake.
 * Repeated calls share the same startup promise.
 *
 * @throws if the Wasm module cannot load or the protocol version mismatches.
 */
export function loadEngine(): Promise<EngineHandle> {
  started ??= (async () => {
    await init();
    assertProtocolVersion(protocol_version());
    const engine = new Engine();
    return wrap(engine, protocol_version());
  })();
  return started;
}

function wrap(engine: Engine, protocolVersion: number): EngineHandle {
  return {
    protocolVersion,
    setViewport: (x, y, width, height) => engine.set_viewport(x, y, width, height),
    zoomAt: (clientX, clientY, factor) => engine.zoom_at(clientX, clientY, factor),
    panBy: (dx, dy) => engine.pan_by(dx, dy),
    view: () => {
      const v = engine.view();
      // Copy out of the Wasm-owned object so the shell holds plain data only.
      const plain: EngineView = {
        zoom: v.zoom,
        zoomPercent: v.zoom_percent,
        cameraX: v.camera_x,
        cameraY: v.camera_y,
        originX: v.origin_x,
        originY: v.origin_y,
        width: v.width,
        height: v.height,
      };
      v.free();
      return plain;
    },
  };
}
