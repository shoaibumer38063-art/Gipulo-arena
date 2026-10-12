import { readFileSync, readdirSync } from 'node:fs';
import { join } from 'node:path';

import { describe, expect, it } from 'vitest';

import type { EngineView } from '../engine/client';
import { objectReadout, originCrosshair, sizeReadout, viewReadout } from './paint';

function view(overrides: Partial<EngineView> = {}): EngineView {
  return {
    zoom: 1,
    zoomPercent: 100,
    cameraX: 0,
    cameraY: 0,
    originX: 100,
    originY: 50,
    width: 800,
    height: 600,
    ...overrides,
  };
}

describe('origin crosshair (CAN-SS-3: crisp 1 px lines)', () => {
  it('runs full width and height through the origin the core reports', () => {
    const [horizontal, vertical] = originCrosshair(view());
    expect(horizontal).toEqual({ x1: 0, y1: 50.5, x2: 800, y2: 50.5 });
    expect(vertical).toEqual({ x1: 100.5, y1: 0, x2: 100.5, y2: 600 });
  });

  it('half-pixel aligns fractional origins', () => {
    const [horizontal] = originCrosshair(view({ originY: 49.7 }));
    expect(horizontal?.y1).toBe(50.5);
  });

  it('depends only on the reported origin and size, never on zoom (the core owns zoom)', () => {
    const at1 = originCrosshair(view({ zoom: 1, zoomPercent: 100 }));
    const at32 = originCrosshair(view({ zoom: 32, zoomPercent: 3200 }));
    expect(at32).toEqual(at1);
  });
});

describe('view readout', () => {
  it('shows zoom percent and the origin in whole viewport pixels', () => {
    expect(viewReadout(view({ zoom: 1.25, zoomPercent: 125, originX: -12.4, originY: 3.6 }))).toBe(
      'Zoom 125% · origin -12, 4',
    );
  });
});

describe('size readout', () => {
  it('shows the selected object size in whole world units', () => {
    expect(sizeReadout(new Float64Array([10, 20, 160.4, 89.6]))).toBe('160 × 90');
    expect(sizeReadout(new Float64Array())).toBe('');
  });
});

describe('object readout', () => {
  it('counts objects and the selection', () => {
    expect(objectReadout(0, 0)).toBe('0 objects · 0 selected');
    expect(objectReadout(1, 1)).toBe('1 object · 1 selected');
    expect(objectReadout(3, 0)).toBe('3 objects · 0 selected');
  });
});

describe('boundary law: the canvas shell performs no coordinate conversion (CAN-COORD-2)', () => {
  const dir = join(process.cwd(), 'src', 'canvas');
  const sources = readdirSync(dir)
    .filter((f) => f.endsWith('.ts') || f.endsWith('.tsx'))
    .filter((f) => !f.endsWith('.test.ts'))
    .map((f) => ({ name: f, text: readFileSync(join(dir, f), 'utf8') }));

  it('finds the canvas sources', () => {
    expect(sources.map((s) => s.name).sort()).toEqual(['CanvasHost.tsx', 'paint.ts', 'shortcuts.ts']);
  });

  it.each([
    ['zoom multiplication', /\*\s*[\w.]*zoom\b|\bzoom\s*\*/],
    ['zoom division', /\/\s*[\w.]*zoom\b/],
    ['camera arithmetic', /[+-]\s*[\w.]*camera[XY]?\b/i],
  ])('contains no %s', (_label, pattern) => {
    for (const source of sources) {
      // Comments may name the rule; only executable code is checked.
      const code = source.text
        .split('\n')
        .filter((line) => !line.trim().startsWith('//') && !line.trim().startsWith('*'))
        .join('\n');
      expect(code, source.name).not.toMatch(pattern);
    }
  });
});
