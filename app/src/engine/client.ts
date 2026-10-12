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
import init, { Engine, protocol_version, rect_stride } from '../../../bindings/pkg/gip_engine.js';

/** Shell-side mirror of `PROTOCOL_VERSION` in core/crates/gip-session. Keep in sync — the
 * handshake exists precisely to catch drift (a mismatch is a clear error, never silence). */
export const PROTOCOL_VERSION = 2;

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

/** Tool names the core accepts (CAN-MODE-1). */
export type ToolName = 'select' | 'rectangle';

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
  /** Pointer pressed (client pixels). button 0 = primary, 1 = middle (pan). */
  pointerDown(clientX: number, clientY: number, button: number, shift: boolean): boolean;
  /** Pointer moved (client pixels). */
  pointerMove(clientX: number, clientY: number): boolean;
  /** Pointer released (client pixels). */
  pointerUp(clientX: number, clientY: number): boolean;
  /** Cancels the running gesture with no history entry (pointercancel). */
  cancel(): void;
  /** Escape: cancels a gesture, otherwise clears the selection. */
  escape(): void;
  /** Sets the tool. Returns false during a gesture. */
  setTool(tool: ToolName): boolean;
  /** The active tool. */
  tool(): ToolName;
  /** Deletes the selection as one history entry. */
  deleteSelection(): boolean;
  undo(): boolean;
  redo(): boolean;
  canUndo(): boolean;
  canRedo(): boolean;
  /** Objects on the page. */
  objectCount(): number;
  /** Selected objects. */
  selectionCount(): number;
  /**
   * Flat draw list: `stride` floats per rectangle (see `rectStride`). Each record is four
   * viewport-pixel corners then a flag bitmask (1 selected, 2 hovered, 4 creation preview).
   */
  rects(): Float64Array;
  /** Floats per record in `rects()`. */
  readonly rectStride: number;
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
    rectStride: rect_stride(),
    pointerDown: (x, y, button, shift) => engine.pointer_down(x, y, button, shift),
    pointerMove: (x, y) => engine.pointer_move(x, y),
    pointerUp: (x, y) => engine.pointer_up(x, y),
    cancel: () => engine.cancel(),
    escape: () => engine.escape(),
    setTool: (tool) => engine.set_tool(tool),
    tool: () => engine.tool() as ToolName,
    deleteSelection: () => engine.delete_selection(),
    undo: () => engine.undo(),
    redo: () => engine.redo(),
    canUndo: () => engine.can_undo(),
    canRedo: () => engine.can_redo(),
    objectCount: () => engine.object_count(),
    selectionCount: () => engine.selection_count(),
    rects: () => engine.rects(),
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
