import path from 'node:path';

import { defineConfig } from 'vitest/config';

// The barrier benchmark, kept out of the default facade run because it takes ~30 seconds and measures rather
// than asserts. `npm run bench:barrier` writes its table to /tmp/barrier.txt.
//
// It exists because "is this function worth putting behind the wasm barrier?" is an empirical question, and
// the answer for this package is currently uncomfortable — see agents/wasm-barrier.md.
const upstream = path.resolve(import.meta.dirname, '../../upstream/packages');

export default defineConfig({
  root: import.meta.dirname,
  resolve: {
    alias: [
      { find: /^@flighthq\/([^/]+)$/u, replacement: `${upstream}/$1/src/index.ts` },
      { find: /^@flighthq\/([^/]+)\/(.+)$/u, replacement: `${upstream}/$1/src/$2` },
    ],
  },
  test: { environment: 'node', globals: true, include: ['src/**/*.bench.test.ts'] },
});
