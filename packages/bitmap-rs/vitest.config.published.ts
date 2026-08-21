// Runs the same parity suite against the PUBLISHED `@flighthq/bitmap`, instead of the pinned
// submodule sources that `vitest.config.ts` aliases to.
//
// The default config answers "do the Rust kernels match the source this port was generated from".
// This one answers the question that actually matters to a consumer: "do they match the package this
// release claims to be a drop-in for". Those differ whenever the pin is not exactly the released
// commit — which is the normal case, because Flight publishes every package at the family version
// whether or not it changed, so a release routinely carries a `bitmap` identical to the one we
// generated from and a version number that has moved on.
//
// Deliberately no aliases: resolution falls through to `node_modules`, so whatever
// `@flighthq/bitmap` was installed is what the suite compares against. Install the version under
// test first:
//
//   npm install --no-save @flighthq/bitmap@<version> @flighthq/types@<version>
//   npx vitest run --config packages/bitmap-rs/vitest.config.published.ts

import { defineConfig } from 'vitest/config';

export default defineConfig({
  root: import.meta.dirname,
  test: {
    environment: 'node',
    globals: true,
    include: ['src/**/*.test.ts'],
    server: {
      deps: {
        // Every `@flighthq/*` package publishes extensionless relative imports (`from './contract'`),
        // which Node's ESM resolver rejects — the packages are bundler-only by convention. Left
        // externalized, Vitest hands them to Node and the suite dies on resolution before running a
        // single assertion. Inlining routes them through Vite's resolver, which is the same treatment
        // a consumer's bundler gives them, so this measures the package as it is actually consumed.
        inline: [/@flighthq\//u],
      },
    },
  },
});
