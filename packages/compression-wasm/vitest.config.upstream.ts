import { readFileSync } from 'node:fs';
import path from 'node:path';

import { defineConfig, type Plugin } from 'vitest/config';

// Runs UPSTREAM'S OWN compression suites against the Rust/wasm implementation, by redirecting one import
// inside each of upstream's test files to this facade. Only inside its test files, and only the one module
// each lane names: every other import keeps resolving to upstream, so the framing constants, the fixtures and
// every helper stay upstream's and the only substituted thing is the code under test.
//
// This is the lane bitmap-wasm uses to run 376 unchanged upstream tests, and it is why a mirror is worth more
// than a reimplementation: these suites were not written for us and cannot have been fitted to us.
//
// Each lane substitutes exactly ONE module, which is a deliberate limit rather than an omission. The two
// encoder suites verify their output by decoding it, and they import the decoder separately; leaving that
// decoder upstream's means the wasm encoder is checked against an independent implementation. Substituting
// both ends would check this crate against itself, where one shared misreading of the format cancels out.
//
// THE LANE IS VACUOUS IF THE SUBSTITUTION SILENTLY STOPS MATCHING. Measured: with the specifier changed to
// one that never matches, the deflate suite still reported 25 of 25 passing, because every import fell
// through to upstream and the suite tested upstream against itself. Nothing downstream notices — a green
// conformance run is exactly what a working facade looks like. So every lane is verified to exist and to
// refer to its test file BEFORE any test runs, and each lane counts its own substitutions and fails if it
// made none. `tests/generator/conformance-lane.test.ts` asserts the same property for every facade.
//
// Upstream renames things: these specifiers carried no `.ts` extension and the deflate export was
// `inflateDeflate` before the pin moved to `develop`. Both assertions below caught that.

const upstreamPackages = path.resolve(import.meta.dirname, '../../upstream/packages');
const upstreamSource = path.join(upstreamPackages, 'compression/src');

interface Lane {
  /** Upstream's test file, run verbatim. */
  readonly testFile: string;
  /** The import specifier inside that file which this lane redirects. */
  readonly specifier: string;
  /** The facade module that answers it, exporting exactly what the substituted module exports. */
  readonly facade: string;
}

const LANES: readonly Lane[] = [
  { testFile: 'deflate.test.ts', specifier: './deflate.ts', facade: 'src/upstreamDeflate.ts' },
  { testFile: 'compress.test.ts', specifier: './compress.ts', facade: 'src/upstreamCompress.ts' },
  { testFile: 'lzma.test.ts', specifier: './lzma.ts', facade: 'src/upstreamLzma.ts' },
  { testFile: 'lzmaCompress.test.ts', specifier: './lzmaCompress.ts', facade: 'src/upstreamLzmaCompress.ts' },
];

const resolved = LANES.map((lane) => ({
  ...lane,
  testPath: path.join(upstreamSource, lane.testFile),
  facadePath: path.resolve(import.meta.dirname, lane.facade),
}));

/** Fails the run when a file this lane substitutes into no longer imports what the lane substitutes. */
function assertSubstitutionsApply(): void {
  for (const lane of resolved) {
    const source = readFileSync(lane.testPath, 'utf8');
    if (!source.includes(`from '${lane.specifier}'`)) {
      throw new Error(
        `${path.relative(process.cwd(), lane.testPath)} no longer imports '${lane.specifier}', so that ` +
          'conformance lane would silently test upstream against itself. Re-point the substitution.',
      );
    }
  }
}

function substituteWasmImplementation(): Plugin {
  assertSubstitutionsApply();
  const substitutions = new Map(resolved.map((lane) => [lane.specifier, 0]));
  return {
    name: 'compression-wasm-upstream-conformance',
    enforce: 'pre',
    resolveId(source, importer) {
      if (importer === undefined) return null;
      // Matched against the importing TEST FILE, not just the directory, so `./deflate.ts` is replaced inside
      // `deflate.test.ts` and left alone inside `compress.test.ts`.
      const lane = resolved.find((candidate) => candidate.specifier === source && importer === candidate.testPath);
      if (lane === undefined) return null;
      substitutions.set(lane.specifier, (substitutions.get(lane.specifier) ?? 0) + 1);
      return lane.facadePath;
    },
    buildEnd() {
      const dead = [...substitutions.entries()].filter(([, count]) => count === 0).map(([specifier]) => specifier);
      if (dead.length > 0) {
        throw new Error(
          `never substituted ${dead.join(', ')}, so nothing under test in those lanes was ours`,
        );
      }
    },
  };
}

export default defineConfig({
  root: import.meta.dirname,
  plugins: [substituteWasmImplementation()],
  resolve: {
    alias: [
      { find: /^@flighthq\/([^/]+)$/u, replacement: `${upstreamPackages}/$1/src/index.ts` },
      { find: /^@flighthq\/([^/]+)\/(.+)$/u, replacement: `${upstreamPackages}/$1/src/$2` },
    ],
  },
  test: {
    environment: 'node',
    globals: true,
    include: resolved.map((lane) => lane.testPath),
  },
});
