import { afterEach } from 'vitest';
import '@testing-library/jest-dom/vitest';

import { cleanup } from '@testing-library/react';

// Design System component suites share one cleanup contract (engine-core.md §9 layer 3);
// jest-dom adds the attribute/state matchers the state-matrix suites assert with.
afterEach(() => {
  cleanup();
});
