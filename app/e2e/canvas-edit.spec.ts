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


test.describe('rotate, flip, and cursor hints (Slice 2)', () => {
  const size = (page: Page) => page.getByTestId('size-readout');
  const rotation = (page: Page) => page.getByTestId('rotation-readout');

  /** A 120×60 rectangle at (200, 200) → center (260, 230), selected under the Select tool. */
  async function rectangle(page: Page) {
    await openReady(page);
    await page.keyboard.press('r');
    await drag(page, await at(page, 200, 200), await at(page, 320, 260));
    await page.keyboard.press('v'); // the Rectangle tool is sticky; rotate needs Select
    await expect(size(page)).toHaveText('120 × 60');
  }

  /**
   * Canvas-relative point `deg` degrees from the NW rotate zone's start direction, at radius 80 px
   * about the center (260, 230). The NW corner zone sits 14 px outside (200, 200).
   */
  const NW_ZONE = { x: 190, y: 190 };
  function aroundCenter(deg: number) {
    const start = Math.atan2(NW_ZONE.y - 230, NW_ZONE.x - 260);
    const r = Math.hypot(NW_ZONE.x - 260, NW_ZONE.y - 230);
    const a = start + (deg * Math.PI) / 180;
    return { x: 260 + r * Math.cos(a), y: 230 + r * Math.sin(a) };
  }

  async function rotateBy(page: Page, deg: number, shift = false) {
    if (shift) await page.keyboard.down('Shift');
    const to = aroundCenter(deg);
    await drag(page, await at(page, NW_ZONE.x, NW_ZONE.y), await at(page, to.x, to.y));
    if (shift) await page.keyboard.up('Shift');
  }

  test('dragging outside a corner rotates the selection about its center', async ({ page }) => {
    await rectangle(page);
    await rotateBy(page, 30);
    await expect(rotation(page)).toHaveText('30°');
    await expect(size(page)).toHaveText('120 × 60');
  });

  test('Shift snaps the rotation to 15° steps', async ({ page }) => {
    await rectangle(page);
    await rotateBy(page, 22, true);
    await expect(rotation(page)).toHaveText('15°');
  });

  test('a rotation is one undo step', async ({ page }) => {
    await rectangle(page);
    await rotateBy(page, 45);
    await expect(rotation(page)).toHaveText('45°');
    await page.keyboard.press('Control+z');
    await expect(rotation(page)).toHaveText('0°');
  });

  test('Escape during a rotation restores the start angle', async ({ page }) => {
    await rectangle(page);
    const zone = await at(page, NW_ZONE.x, NW_ZONE.y);
    await page.mouse.move(zone.x, zone.y);
    await page.mouse.down();
    const to = aroundCenter(60);
    await page.mouse.move((await at(page, to.x, to.y)).x, (await at(page, to.x, to.y)).y, { steps: 4 });
    await page.keyboard.press('Escape');
    await page.mouse.up();
    await expect(rotation(page)).toHaveText('0°');
  });

  test('the rotate zone shows a grab cursor and a corner shows a diagonal resize cursor', async ({ page }) => {
    await rectangle(page);
    const zone = await at(page, NW_ZONE.x, NW_ZONE.y);
    await page.mouse.move(zone.x, zone.y);
    await expect(page.getByTestId('editor-canvas')).toHaveCSS('cursor', 'grab');
    const corner = await at(page, 200, 200);
    await page.mouse.move(corner.x, corner.y);
    await expect(page.getByTestId('editor-canvas')).toHaveCSS('cursor', 'nwse-resize');
  });

  test('Shift+H flips the selection as one undo step and keeps the box', async ({ page }) => {
    await rectangle(page);
    await page.keyboard.press('Shift+H');
    await expect(size(page)).toHaveText('120 × 60');
    await expect(page.getByTestId('object-readout')).toHaveText('1 object · 1 selected');
    await page.keyboard.press('Control+z'); // reverts the flip only
    await expect(page.getByTestId('object-readout')).toHaveText('1 object · 1 selected');
    await page.keyboard.press('Control+z'); // reverts the create
    await expect(page.getByTestId('object-readout')).toHaveText('0 objects · 0 selected');
  });

  test('a flipped rectangle resizes from the handle under the pointer', async ({ page }) => {
    await rectangle(page);
    await page.keyboard.press('Shift+H');
    // Flipped about the center, the grabbed edge is now the left one at canvas x = 200.
    await drag(page, await at(page, 200, 230), await at(page, 170, 230));
    await expect(size(page)).toHaveText('150 × 60');
  });

  test('Shift+V does not switch to the Select tool', async ({ page }) => {
    await rectangle(page);
    await page.keyboard.press('r');
    await page.keyboard.press('Shift+V');
    await expect(page.getByTestId('tool-rectangle')).toHaveAttribute('aria-pressed', 'true');
  });

  test('two selected objects share a group box that resizes as one', async ({ page }) => {
    await openReady(page);
    await page.keyboard.press('r');
    await drag(page, await at(page, 200, 200), await at(page, 320, 260)); // 120×60
    await drag(page, await at(page, 400, 300), await at(page, 460, 340)); // 60×40
    await page.keyboard.press('v');
    await page.mouse.click((await at(page, 260, 230)).x, (await at(page, 260, 230)).y);
    await page.keyboard.down('Shift');
    await page.mouse.click((await at(page, 430, 320)).x, (await at(page, 430, 320)).y);
    await page.keyboard.up('Shift');
    await expect(page.getByTestId('object-readout')).toHaveText('2 objects · 2 selected');
    await expect(size(page)).toHaveText('260 × 140'); // the group box: (200, 200) to (460, 340)

    const se = await at(page, 460, 340);
    await drag(page, se, await at(page, 500, 360));
    await expect(size(page)).toHaveText('300 × 160');
    await page.keyboard.press('Control+z');
    await expect(size(page)).toHaveText('260 × 140');
  });
});

test.describe('snapping (Slice 4)', () => {
  const guides = (page: Page) => page.getByTestId('editor-canvas');

  /** A at canvas (200, 200) 120×60 (right edge 320); B at (400, 300) 60×40 (left edge 400). */
  async function two(page: Page) {
    await openReady(page);
    await page.keyboard.press('r');
    await drag(page, await at(page, 200, 200), await at(page, 320, 260));
    await drag(page, await at(page, 400, 300), await at(page, 460, 340));
    await page.keyboard.press('v');
    await expect(page.getByTestId('object-readout')).toHaveText('2 objects · 1 selected');
  }

  test('moving an object near another object shows a guide and snaps to its edge', async ({ page }) => {
    await two(page);
    const from = await at(page, 430, 320);
    await page.mouse.click(from.x, from.y); // select B
    await page.mouse.move(from.x, from.y);
    await page.mouse.down();
    // B's left edge lands 1.5 px right of A's right edge (320): it snaps there.
    await page.mouse.move((await at(page, 351.5, 320)).x, (await at(page, 351.5, 320)).y, { steps: 6 });
    await expect(guides(page)).toHaveAttribute('data-guides', '1');
    await page.mouse.up();
    await expect(guides(page)).toHaveAttribute('data-guides', '0');
  });

  test('Ctrl turns snapping off, so no guide appears', async ({ page }) => {
    await two(page);
    const from = await at(page, 430, 320);
    await page.mouse.click(from.x, from.y);
    await page.keyboard.down('Control');
    await page.mouse.move(from.x, from.y);
    await page.mouse.down();
    await page.mouse.move((await at(page, 351.5, 320)).x, (await at(page, 351.5, 320)).y, { steps: 6 });
    await expect(guides(page)).toHaveAttribute('data-guides', '0');
    await page.mouse.up();
    await page.keyboard.up('Control');
  });
});

