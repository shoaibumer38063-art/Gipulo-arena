import { expect, test, type Page } from '@playwright/test';

// Slice 0b (task 020b): rectangles are created, selected, moved, cancelled, deleted, and undone
// through the core. The shell only forwards raw pointer and key input and paints the result.

async function openReady(page: Page) {
  await page.goto('/');
  await expect(page.locator('[data-engine-status]')).toHaveAttribute('data-engine-status', 'ready', {
    timeout: 20_000,
  });
}

/** Canvas-relative point → page coordinates. */
async function at(page: Page, x: number, y: number) {
  const box = await page.getByTestId('editor-canvas').boundingBox();
  if (!box) throw new Error('canvas has no box');
  return { x: box.x + x, y: box.y + y };
}

async function drag(page: Page, from: { x: number; y: number }, to: { x: number; y: number }) {
  await page.mouse.move(from.x, from.y);
  await page.mouse.down();
  await page.mouse.move(to.x, to.y, { steps: 6 });
  await page.mouse.up();
}

const objects = (page: Page) => page.getByTestId('object-readout');

test.describe('editor canvas edits (Slice 0b)', () => {
  test('R then drag creates a rectangle, which is selected', async ({ page }) => {
    await openReady(page);
    await expect(objects(page)).toHaveText('0 objects · 0 selected');
    await page.keyboard.press('r');
    await expect(page.getByTestId('tool-rectangle')).toHaveAttribute('aria-pressed', 'true');
    await drag(page, await at(page, 200, 200), await at(page, 320, 260));
    await expect(objects(page)).toHaveText('1 object · 1 selected');
  });

  test('Escape during a drag cancels it and leaves no object', async ({ page }) => {
    await openReady(page);
    await page.keyboard.press('r');
    await page.mouse.move((await at(page, 200, 200)).x, (await at(page, 200, 200)).y);
    await page.mouse.down();
    await page.mouse.move((await at(page, 300, 260)).x, (await at(page, 300, 260)).y, { steps: 4 });
    await page.keyboard.press('Escape');
    await page.mouse.up();
    await expect(objects(page)).toHaveText('0 objects · 0 selected');
    await page.keyboard.press('Control+z');
    await expect(objects(page)).toHaveText('0 objects · 0 selected');
  });

  test('moving is one undo step: undo reverts the move, then the create', async ({ page }) => {
    await openReady(page);
    await page.keyboard.press('r');
    await drag(page, await at(page, 200, 200), await at(page, 320, 260));
    await page.keyboard.press('v');
    await drag(page, await at(page, 260, 230), await at(page, 340, 240));
    await expect(objects(page)).toHaveText('1 object · 1 selected');

    await page.keyboard.press('Control+z'); // reverts the move: the object still exists
    await expect(objects(page)).toHaveText('1 object · 1 selected');
    await page.keyboard.press('Control+z'); // reverts the create
    await expect(objects(page)).toHaveText('0 objects · 0 selected');
    // Selection is UI state, not history: redo brings the object back but does not reselect it.
    await page.keyboard.press('Control+Shift+z');
    await expect(objects(page)).toHaveText('1 object · 0 selected');
  });

  test('a cancelled move leaves no history entry', async ({ page }) => {
    await openReady(page);
    await page.keyboard.press('r');
    await drag(page, await at(page, 200, 200), await at(page, 320, 260));
    await page.keyboard.press('v');
    await page.mouse.move((await at(page, 260, 230)).x, (await at(page, 260, 230)).y);
    await page.mouse.down();
    await page.mouse.move((await at(page, 340, 240)).x, (await at(page, 340, 240)).y, { steps: 4 });
    await page.keyboard.press('Escape');
    await page.mouse.up();
    // Only the create is in history, so one undo removes the object.
    await page.keyboard.press('Control+z');
    await expect(objects(page)).toHaveText('0 objects · 0 selected');
  });

  test('Delete removes the selection and undo restores it', async ({ page }) => {
    await openReady(page);
    await page.keyboard.press('r');
    await drag(page, await at(page, 200, 200), await at(page, 320, 260));
    await page.keyboard.press('Delete');
    await expect(objects(page)).toHaveText('0 objects · 0 selected');
    await page.keyboard.press('Control+z');
    await expect(objects(page)).toHaveText('1 object · 0 selected');
  });

  test('panning with the middle button does not create an undo step', async ({ page }) => {
    await openReady(page);
    const start = await at(page, 100, 100);
    await page.mouse.move(start.x, start.y);
    await page.mouse.down({ button: 'middle' });
    await page.mouse.move(start.x + 30, start.y + 10, { steps: 4 });
    await page.mouse.up({ button: 'middle' });
    await expect(page.getByTestId('view-readout')).toHaveText('Zoom 100% · origin 30, 10');
    await page.keyboard.press('Control+z');
    await expect(page.getByTestId('view-readout')).toHaveText('Zoom 100% · origin 30, 10');
  });
});

test.describe('resize handles (Slice 1)', () => {
  const size = (page: Page) => page.getByTestId('size-readout');

  test('dragging the SE handle resizes the selected rectangle', async ({ page }) => {
    await openReady(page);
    await page.keyboard.press('r');
    await drag(page, await at(page, 200, 200), await at(page, 320, 260));
    await page.keyboard.press('v'); // the Rectangle tool is sticky; resize needs Select
    await expect(size(page)).toHaveText('120 × 60');
    await drag(page, await at(page, 320, 260), await at(page, 360, 290));
    await expect(size(page)).toHaveText('160 × 90');
  });

  test('Escape cancels a resize and undo reverts it', async ({ page }) => {
    await openReady(page);
    await page.keyboard.press('r');
    await drag(page, await at(page, 200, 200), await at(page, 320, 260));
    await page.keyboard.press('v'); // the Rectangle tool is sticky; resize needs Select
    const se = await at(page, 320, 260);
    await page.mouse.move(se.x, se.y);
    await page.mouse.down();
    await page.mouse.move((await at(page, 360, 290)).x, (await at(page, 360, 290)).y, { steps: 4 });
    await page.keyboard.press('Escape');
    await page.mouse.up();
    await expect(size(page)).toHaveText('120 × 60');

    await drag(page, se, await at(page, 360, 290));
    await expect(size(page)).toHaveText('160 × 90');
    await page.keyboard.press('Control+z');
    await expect(size(page)).toHaveText('120 × 60');
  });

  test('Shift keeps the aspect ratio on a corner handle', async ({ page }) => {
    await openReady(page);
    await page.keyboard.press('r');
    await drag(page, await at(page, 200, 200), await at(page, 320, 260));
    await page.keyboard.press('v'); // the Rectangle tool is sticky; resize needs Select
    const se = await at(page, 320, 260);
    await page.keyboard.down('Shift');
    await drag(page, se, await at(page, 380, 260));
    await page.keyboard.up('Shift');
    // 120×60 is 2:1; the pointer moved +60 in x only, so the larger scale (1.5) sets 180×90.
    await expect(size(page)).toHaveText('180 × 90');
  });
});

