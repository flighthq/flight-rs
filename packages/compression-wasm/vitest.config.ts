import path from 'node:path';
import { defineConfig } from 'vitest/config';

const upstream = path.resolve(import.meta.dirname, '../../upstream/packages');

export default defineConfig({
  root: import.meta.dirname,
  resolve: {
    alias: [
      { find: /^@flighthq\/([^/]+)$/u, replacement: `${upstream}/$1/src/index.ts` },
      { find: /^@flighthq\/([^/]+)\/(.+)$/u, replacement: `${upstream}/$1/src/$2` },
    ],
  },
  test: { environment: 'node', globals: true, include: ['src/**/*.test.ts'] },
});
