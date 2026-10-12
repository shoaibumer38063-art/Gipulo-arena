import { describe, expect, it } from 'vitest';

import { PROTOCOL_VERSION, assertProtocolVersion } from './client';

describe('engine boundary handshake (engine-core.md §10)', () => {
  it('accepts a matching protocol version', () => {
    expect(() => assertProtocolVersion(PROTOCOL_VERSION)).not.toThrow();
  });

  it('rejects a mismatched protocol version with a descriptive error', () => {
    expect(() => assertProtocolVersion(PROTOCOL_VERSION + 1)).toThrow(/protocol mismatch/);
    expect(() => assertProtocolVersion(PROTOCOL_VERSION + 1)).toThrow(/rebuild the wasm bindings/);
  });

  it('keeps the shell mirror at the view-boundary protocol version', () => {
    expect(PROTOCOL_VERSION).toBe(3);
  });
});
