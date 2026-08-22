// Runs the consumer smoke test against INSTALLED packages, with no aliases at all.
//
// Every other config in this repository redirects `@flighthq/*` at `upstream/packages/*/src` so the
// suites test the pinned sources. This one deliberately does not: resolution falls through to
// `node_modules`, so whatever tarball was installed is what gets imported. Aliasing anything here
// would quietly re-test the sources and report success for a tarball nobody loaded.

import { defineConfig } from 'vitest/config';

export default defineConfig({
  test: {
    environment: 'node',
    globals: true,
    include: ['tests/consumer/**/*.test.ts'],
    server: {
      deps: {
        // Every `@flighthq/*` package publishes extensionless relative imports (`from './contract'`),
        // which Node's ESM resolver rejects — they are bundler-only by convention. Left externalized,
        // Vitest hands them to Node and the run dies on resolution before any assertion. Inlining
        // routes them through Vite's resolver, which is the treatment a consumer's bundler gives
        // them, so this measures the package as it is actually consumed.
        inline: [/@flighthq\//u],
      },
    },
  },
});
