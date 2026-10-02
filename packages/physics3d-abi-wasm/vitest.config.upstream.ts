import { readFileSync } from 'node:fs';
import { globSync } from 'node:fs';
import path from 'node:path';

import { defineConfig, type Plugin } from 'vitest/config';

// Runs UPSTREAM'S OWN physics3d ABI suites against the Rust/wasm backend, by redirecting the import of the
// module under test inside upstream's test files to this facade. Every other import keeps resolving to
// upstream, so the command buffers, the layout constants and every helper stay upstream's.
//
// THE LANE IS VACUOUS IF THE SUBSTITUTION SILENTLY STOPS MATCHING, and it looks exactly like success when it
// does: every import falls through to upstream and the suite tests upstream against itself, reporting green.
// This is not hypothetical here -- the specifiers below carried no `.ts` extension until upstream added one,
// at which point this lane silently stopped substituting anything. So each specifier is checked against the
// test files that import it BEFORE any test runs, and each counts its own substitutions and fails having made
// none. `tests/generator/conformance-lane.test.ts` asserts the same property for every facade.

const upstreamPackages = path.resolve(import.meta.dirname, '../../upstream/packages');
const upstreamSource = path.join(upstreamPackages, 'physics3d-abi/src');

interface Lane {
  /** The import specifier inside upstream's test files which this lane redirects. */
  readonly specifier: string;
  /** The facade module that answers it. */
  readonly facade: string;
}

const LANES: readonly Lane[] = [
  { specifier: './physics3DAbi.ts', facade: 'src/index.ts' },
  { specifier: './referencePhysics3DAbi.ts', facade: 'src/upstreamReferencePhysics3DAbi.ts' },
];

const resolved = LANES.map((lane) => ({
  ...lane,
  facadePath: path.resolve(import.meta.dirname, lane.facade),
}));

/** Fails the run when no upstream test file imports what a lane substitutes. */
function assertSubstitutionsApply(): void {
  const testFiles = globSync(path.join(upstreamSource, '*.test.ts'));
  if (testFiles.length === 0) throw new Error(`no upstream test files found under ${upstreamSource}`);
  for (const lane of resolved) {
    const importers = testFiles.filter((file) => readFileSync(file, 'utf8').includes(`from '${lane.specifier}'`));
    if (importers.length === 0) {
      throw new Error(
        `no upstream test file imports '${lane.specifier}', so this conformance lane would silently test ` +
          'upstream against itself. Re-point the substitution.',
      );
    }
  }
}

function substituteWasmBackend(): Plugin {
  assertSubstitutionsApply();
  const substitutions = new Map(resolved.map((lane) => [lane.specifier, 0]));
  return {
    name: 'physics3d-abi-wasm-upstream-conformance',
    enforce: 'pre',
    resolveId(source, importer) {
      if (importer === undefined || !importer.startsWith(upstreamSource) || !importer.endsWith('.test.ts')) {
        return null;
      }
      const lane = resolved.find((candidate) => candidate.specifier === source);
      if (lane === undefined) return null;
      substitutions.set(lane.specifier, (substitutions.get(lane.specifier) ?? 0) + 1);
      return lane.facadePath;
    },
    buildEnd() {
      const dead = [...substitutions.entries()].filter(([, count]) => count === 0).map(([specifier]) => specifier);
      if (dead.length > 0) {
        throw new Error(`never substituted ${dead.join(', ')}, so nothing under test in those lanes was ours`);
      }
    },
  };
}

export default defineConfig({
  root: import.meta.dirname,
  plugins: [substituteWasmBackend()],
  resolve: {
    alias: [
      { find: /^@flighthq\/([^/]+)$/u, replacement: `${upstreamPackages}/$1/src/index.ts` },
      { find: /^@flighthq\/([^/]+)\/(.+)$/u, replacement: `${upstreamPackages}/$1/src/$2` },
    ],
  },
  test: {
    environment: 'node',
    globals: true,
    include: ['../../upstream/packages/physics3d-abi/src/**/*.test.ts'],
  },
});
