/**
 * Gipulo engine facade — the ONLY module the shell may use to talk to the core.
 *
 * Contract: docs/architecture/engine-core.md §3, §10.
 *   - Shell intents go DOWN; discrete events and per-frame snapshots come UP.
 *   - Generated bindings (bindings/pkg, built by scripts/build-wasm.sh) are consumed here and
 *     NOWHERE else — and never hand-edited.
 *   - Boundary Duplication Law: no editor logic in this file. Transport only.
 *
 * Scaffold status: the bindings package is not built yet, so this module currently provides
 * the protocol handshake contract only. When the renderer/boundary phases land, the handshake
 * will call the core's `protocol_version()` through the generated package.
 */

/** Shell-side mirror of `PROTOCOL_VERSION` in core/crates/gip-session. Keep in sync — the
 * handshake exists precisely to catch drift (a mismatch is a clear error, never silence). */
export const PROTOCOL_VERSION = 0;

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
