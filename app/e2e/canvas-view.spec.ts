import { expect, test } from '@playwright/test';

// Slice 0a (task 020): the canvas shows the core's view. Pan and zoom go to the core; the shell
// only forwards raw pointer and wheel input and paints what comes back.
test.describe('editor canvas view', () => {
  test('starts at 100 % with the origin at the canvas corner', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('[data-engine-status]')).toHaveAttribute('data-engine-status', 'ready', {
      timeout: 20_000,
    });
    await expect(page.getByTestId('view-readout')).toHaveText('Zoom 100% · origin 0, 0');
  });

  test('middle-button drag pans the content with the pointer (no document edit)', async ({ page }) => {
    await page.goto('/');
    const canvas = page.getByTestId('editor-canvas');
    await expect(page.locator('[data-engine-status]')).toHaveAttribute('data-engine-status', 'ready', {
      timeout: 20_000,
    });
    const box = await canvas.boundingBox();
    if (!box) throw new Error('canvas has no box');

    await page.mouse.move(box.x + 100, box.y + 100);
    await page.mouse.down({ button: 'middle' });
    await page.mouse.move(box.x + 150, box.y + 120, { steps: 5 });
    await page.mouse.up({ button: 'middle' });

    await expect(page.getByTestId('view-readout')).toHaveText('Zoom 100% · origin 50, 20');
  });

  test('ctrl + wheel zooms around the pointer, and the readout follows the core', async ({ page }) => {
    await page.goto('/');
    const canvas = page.getByTestId('editor-canvas');
    await expect(page.locator('[data-engine-status]')).toHaveAttribute('data-engine-status', 'ready', {
      timeout: 20_000,
    });
    const box = await canvas.boundingBox();
    if (!box) throw new Error('canvas has no box');

    await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
    await page.keyboard.down('Control');
    await page.mouse.wheel(0, -100);
    await page.keyboard.up('Control');

    await expect(page.getByTestId('view-readout')).not.toHaveText(/^Zoom 100%/);
    await expect(page.getByTestId('view-readout')).toHaveText(/^Zoom \d+% · origin/);
  });
});
