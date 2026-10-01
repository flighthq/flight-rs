import path from 'node:path';

import { defineConfig } from 'vitest/config';

// Opt-in barrier benchmark; `npm run bench:barrier` runs it alongside bitmap's and writes
// /tmp/compression-barrier.txt. Kept out of the default facade run because it measures rather than asserts.
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
