# Task 019 — Shell ↔ engine wiring and browser E2E

- **Date:** 2026-10-11 · **Roadmap:** step 18 · **Status:** DONE

## What
- `app/src/engine/client.ts`: `loadEngine()` instantiates the Wasm core once (from `bindings/pkg`) and verifies the protocol version. It is still the only module that touches the bindings.
- `app/src/App.tsx` + `App.css`: status line (`data-engine-status` = starting | ready | error), tokens only. The Design System gallery is unchanged below it.
- `app/e2e/engine-smoke.spec.ts` + `app/playwright.config.ts`: browser smoke test against the production build (`vite preview`).
- `.github/workflows/ci.yml`: the shell job builds the bindings first, then runs typecheck, unit tests, build, and E2E.
- `app/package.json`: script `test:e2e`; devDependency `@playwright/test`.

## Verification
- `npm run typecheck`: pass · `npm test` (Vitest): 47 passed · `npm run build`: pass · `npm run test:e2e`: 1 passed.
- Dev server (`vite`) also serves the bindings from outside `app/` (checked by HTTP fetch).

## Open
- Error-path E2E (engine load failure shows the error state) is not covered yet; add when the boundary grows.
- Push to GitHub pending a token.
