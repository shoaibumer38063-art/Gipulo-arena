import { expect, test } from '@playwright/test';

// Smoke: the shell loads the Wasm core, performs the protocol handshake, and reports ready.
// Covers engine-core.md §10 (handshake) end to end in a real browser.
test('shell loads the engine and completes the protocol handshake', async ({ page }) => {
  const consoleErrors: string[] = [];
  page.on('console', (msg) => {
    if (msg.type() === 'error') consoleErrors.push(msg.text());
  });

  await page.goto('/');

  const status = page.locator('[data-engine-status]');
  await expect(status).toHaveAttribute('data-engine-status', 'ready', { timeout: 20_000 });
  await expect(status).toHaveText(/Engine ready · protocol 0/);
  expect(consoleErrors, consoleErrors.join('\n')).toEqual([]);
});
