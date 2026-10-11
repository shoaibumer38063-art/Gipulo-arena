import { expect, test } from '@playwright/test';

// Failure path: if the Wasm core cannot load, the shell shows a clear error state instead of
// a blank or broken page (engine-core.md §10: clear startup error, never silence).
test('shell shows an error state when the engine cannot load', async ({ page }) => {
  await page.route('**/*.wasm', (route) => route.abort());

  await page.goto('/');

  const status = page.locator('[data-engine-status]');
  await expect(status).toHaveAttribute('data-engine-status', 'error', { timeout: 20_000 });
  await expect(status).toHaveText(/Engine error:/);
  // The gallery still renders: the shell degrades, it does not die.
  await expect(page.locator('body')).not.toBeEmpty();
});
