import { expect, test, type Page } from '@playwright/test';

// Slice 0c (task 020c, AC-24): frame pacing on the committed 1,000-rectangle fixture
// (fixtures/rects-1000.gip, served from public/fixtures). Measured headless with rAF timestamps;
// the numbers are printed so they can be recorded with the fixture name.

async function openFixture(page: Page, name: string) {
  await page.goto(`/?fixture=${name}`);
  await expect(page.locator('[data-engine-status]')).toHaveAttribute('data-engine-status', 'ready', {
    timeout: 20_000,
  });
  await expect(page.getByTestId('object-readout')).toHaveText('1000 objects · 0 selected', { timeout: 20_000 });
}

async function recordFrames(page: Page, interaction: () => Promise<void>): Promise<number[]> {
  await page.evaluate(() => {
    const w = window as unknown as { __frames: number[]; __stop: boolean };
    w.__frames = [];
    w.__stop = false;
    const tick = (t: number) => {
      w.__frames.push(t);
      if (!w.__stop) requestAnimationFrame(tick);
    };
    requestAnimationFrame(tick);
  });
  await interaction();
  await page.evaluate(() => {
    (window as unknown as { __stop: boolean }).__stop = true;
  });
  // One more frame so the last interval is recorded.
  await page.waitForTimeout(50);
  return page.evaluate(() => {
    const frames = (window as unknown as { __frames: number[] }).__frames;
    return frames.slice(1).map((t, i) => t - frames[i]!);
  });
}

function p95(values: number[]): number {
  const sorted = [...values].sort((a, b) => a - b);
  return sorted[Math.min(sorted.length - 1, Math.floor(sorted.length * 0.95))] ?? Number.NaN;
}

test.describe('1,000-rectangle fixture (AC-24)', () => {
  test('pan keeps p95 frame interval within 32 ms', async ({ page }) => {
    await openFixture(page, 'rects-1000');
    const canvas = await page.getByTestId('editor-canvas').boundingBox();
    if (!canvas) throw new Error('canvas has no box');
    const deltas = await recordFrames(page, async () => {
      const x = canvas.x + 300;
      const y = canvas.y + 300;
      await page.mouse.move(x, y);
      await page.mouse.down({ button: 'middle' });
      await page.mouse.move(x + 400, y + 200, { steps: 60 });
      await page.mouse.up({ button: 'middle' });
    });
    const value = p95(deltas);
    console.log(`rects-1000 pan: frames=${deltas.length} p95=${value.toFixed(1)}ms`);
    expect(deltas.length).toBeGreaterThan(20);
    expect(value).toBeLessThanOrEqual(32);
  });

  test('dragging a selected rectangle keeps p95 frame interval within 32 ms', async ({ page }) => {
    await openFixture(page, 'rects-1000');
    const canvas = await page.getByTestId('editor-canvas').boundingBox();
    if (!canvas) throw new Error('canvas has no box');
    // Rect 1 sits at world (40, 40), size 48×36; the origin is the canvas corner at 100 %.
    const grab = { x: canvas.x + 40 + 24, y: canvas.y + 40 + 18 };
    await page.keyboard.press('v');
    await page.mouse.click(grab.x, grab.y);
    await expect(page.getByTestId('object-readout')).toHaveText('1000 objects · 1 selected');
    const deltas = await recordFrames(page, async () => {
      await page.mouse.move(grab.x, grab.y);
      await page.mouse.down();
      await page.mouse.move(grab.x + 300, grab.y + 150, { steps: 60 });
      await page.mouse.up();
    });
    const value = p95(deltas);
    console.log(`rects-1000 drag: frames=${deltas.length} p95=${value.toFixed(1)}ms`);
    expect(deltas.length).toBeGreaterThan(20);
    expect(value).toBeLessThanOrEqual(32);
  });

  test('resizing a selected rectangle keeps p95 frame interval within 32 ms', async ({ page }) => {
    await openFixture(page, 'rects-1000');
    const canvas = await page.getByTestId('editor-canvas').boundingBox();
    if (!canvas) throw new Error('canvas has no box');
    // Rect 1 is at world (40, 40), 48×36. Its SE corner is at (88, 76).
    await page.keyboard.press('v');
    await page.mouse.click(canvas.x + 64, canvas.y + 58);
    await expect(page.getByTestId('size-readout')).toHaveText('48 × 36');
    const deltas = await recordFrames(page, async () => {
      await page.mouse.move(canvas.x + 88, canvas.y + 76);
      await page.mouse.down();
      await page.mouse.move(canvas.x + 288, canvas.y + 176, { steps: 60 });
      await page.mouse.up();
    });
    const value = p95(deltas);
    console.log(`rects-1000 resize: frames=${deltas.length} p95=${value.toFixed(1)}ms`);
    expect(deltas.length).toBeGreaterThan(20);
    expect(value).toBeLessThanOrEqual(32);
  });
});
