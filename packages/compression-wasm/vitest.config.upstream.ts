import { readFileSync } from 'node:fs';
import path from 'node:path';

import { defineConfig, type Plugin } from 'vitest/config';

// Runs UPSTREAM'S OWN deflate suite against the wasm decoder, by redirecting the `./deflate` import inside
// upstream's test files to this facade. Only inside its test files: the implementation modules keep
// resolving to upstream, so the registry, the framing constants and every helper stay upstream's and the
// only substituted thing is the decoder under test.
//
// This is the lane bitmap-wasm uses to run 376 unchanged upstream tests, and it is why a mirror is worth
// more than a reimplementation: the suite was not written for us and cannot have been fitted to us.
//
// THE LANE IS VACUOUS IF THE SUBSTITUTION SILENTLY STOPS MATCHING. Measured: with the specifier changed to
// one that never matches, this suite still reports 25 of 25 passing, because every import falls through to
// upstream and the suite tests upstream against itself. Nothing downstream notices — a green conformance
// run is exactly what a working facade looks like. So the specifier and the test file are verified to
// exist and to refer to each other BEFORE any test runs, and the config throws rather than degrading.
// `tests/generator/conformance-lane.test.ts` asserts the same property for every facade.

const upstreamPackages = path.resolve(import.meta.dirname, '../../upstream/packages');
const upstreamSource = path.join(upstreamPackages, 'compression/src');
const facade = path.resolve(import.meta.dirname, 'src/upstreamDeflate.ts');
const SUBSTITUTED_SPECIFIER = './deflate';
const upstreamTestFile = path.join(upstreamSource, 'deflate.test.ts');

/**
 * Fails the run when the file this lane substitutes into no longer imports what it substitutes.
 *
 * Upstream renames things: on `develop` this module's export is already `decompressDeflate` rather than
 * `inflateDeflate`. When the pin moves, this must fail loudly instead of quietly conforming to nothing.
 */
function assertSubstitutionApplies(): void {
  const source = readFileSync(upstreamTestFile, 'utf8');
  if (!source.includes(`from '${SUBSTITUTED_SPECIFIER}'`)) {
    throw new Error(
      `${path.relative(process.cwd(), upstreamTestFile)} no longer imports '${SUBSTITUTED_SPECIFIER}', ` +
        'so this conformance lane would silently test upstream against itself. Re-point the substitution.',
    );
  }
}

function substituteWasmDecoder(): Plugin {
  assertSubstitutionApplies();
  let substitutions = 0;
  return {
    name: 'compression-wasm-upstream-conformance',
    enforce: 'pre',
    resolveId(source, importer) {
      if (source !== SUBSTITUTED_SPECIFIER || importer === undefined) return null;
      if (!importer.startsWith(upstreamSource) || !importer.endsWith('.test.ts')) return null;
      substitutions++;
      return facade;
    },
    buildEnd() {
      if (substitutions === 0) {
        throw new Error('the wasm decoder was never substituted, so nothing under test was ours');
      }
    },
  };
}

export default defineConfig({
  root: import.meta.dirname,
  plugins: [substituteWasmDecoder()],
  resolve: {
    alias: [
      { find: /^@flighthq\/([^/]+)$/u, replacement: `${upstreamPackages}/$1/src/index.ts` },
      { find: /^@flighthq\/([^/]+)\/(.+)$/u, replacement: `${upstreamPackages}/$1/src/$2` },
    ],
  },
  test: {
    environment: 'node',
    globals: true,
    include: [upstreamTestFile],
  },
});
