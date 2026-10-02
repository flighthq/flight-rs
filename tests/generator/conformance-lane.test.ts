import { readFileSync, readdirSync, statSync } from 'node:fs';
import path from 'node:path';

import { publishablePackages } from '../../scripts/publishable-packages.ts';

// A conformance lane substitutes our implementation into an upstream test file and runs upstream's own
// suite against it. That makes a green run strong evidence — and makes a lane that quietly stops
// substituting indistinguishable from a working one.
//
// Measured on compression-wasm before this was guarded: with the substituted specifier changed to one that
// never matches, the lane still reported 25 of 25 passing, because every import fell through to upstream
// and the suite tested upstream against itself. The failure mode is silence, and the symptom is a green
// conformance run, which is exactly what success looks like.
//
// It has since happened for real, which is the reason this file is not the only guard. Upstream started
// writing explicit `.ts` extensions on its relative imports, every lane's specifier stopped matching, and
// the physics lanes — which had no guard of their own — went on reporting green. Each lane config now also
// asserts its own specifiers before any test runs and counts its substitutions, failing if it made none.
// This file is the repo-wide second line: it checks the property no single lane can check about the others.

const workspace = path.resolve('.');
const packagesDirectory = path.join(workspace, 'packages');

interface Lane {
  facade: string;
  /** Specifiers the config redirects. A config may declare several. */
  specifiers: string[];
  /** Facade modules the config redirects them to. */
  facadeModules: string[];
  upstreamDirectory: string;
}

function matchAll(source: string, pattern: RegExp): string[] {
  return [...source.matchAll(pattern)].flatMap((match) => (match[1] === undefined ? [] : [match[1]]));
}

/**
 * Every facade that declares a conformance lane, with the specifiers it substitutes.
 *
 * Two config shapes are recognised deliberately rather than one being normalised away: a single-substitution
 * lane states its specifier inline in `resolveId`, while a multi-substitution lane declares a `LANES` table.
 * Specifiers and facade modules are collected independently and asserted independently — pairing them by
 * position would be a guess, and a wrong guess here produces a confident assertion about the wrong pair.
 */
function lanes(): Lane[] {
  const found: Lane[] = [];
  for (const facade of readdirSync(packagesDirectory)) {
    const config = path.join(packagesDirectory, facade, 'vitest.config.upstream.ts');
    let source: string;
    try {
      source = readFileSync(config, 'utf8');
    } catch {
      continue;
    }
    const upstreamPackage = /upstreamSource = path\.join\(upstreamPackages, '([^'/]+)\/src'\)/u.exec(source)?.[1];
    const specifiers = [
      ...matchAll(source, /source (?:!==|===) '([^']+)'/gu),
      ...matchAll(source, /specifier: '([^']+)'/gu),
    ];
    const facadeModules = [
      ...matchAll(source, /(?:facade|referenceFacade) = path\.resolve\(import\.meta\.dirname, '([^']+)'\)/gu),
      ...matchAll(source, /facade: '([^']+)'/gu),
    ];
    // A lane that substitutes nothing is a lane that only re-runs upstream against upstream. That is a
    // legitimate thing to have — bitmap's suite exercises overrides through aliases instead — so a config
    // with no specifier comparison is skipped rather than failed.
    if (specifiers.length === 0 || upstreamPackage === undefined) continue;
    found.push({
      facade,
      specifiers,
      facadeModules,
      upstreamDirectory: path.join(workspace, 'upstream/packages', upstreamPackage, 'src'),
    });
  }
  return found;
}

function upstreamTestFiles(directory: string): string[] {
  return readdirSync(directory)
    .filter((entry) => entry.endsWith('.test.ts'))
    .map((entry) => path.join(directory, entry))
    .filter((entry) => statSync(entry).isFile());
}

describe('upstream conformance lanes', () => {
  it('substitutes a specifier the upstream tests actually import', () => {
    const found = lanes();
    expect(found.length, 'at least one facade declares a substituting conformance lane').toBeGreaterThan(0);

    for (const lane of found) {
      const testFiles = upstreamTestFiles(lane.upstreamDirectory);
      expect(testFiles.length, `${lane.facade}: upstream test files exist to substitute into`).toBeGreaterThan(0);
      for (const specifier of lane.specifiers) {
        const importers = testFiles.filter((file) => readFileSync(file, 'utf8').includes(`from '${specifier}'`));
        expect(
          importers.map((file) => path.relative(workspace, file)),
          `${lane.facade} substitutes '${specifier}', which no upstream test file imports — the lane ` +
            'would pass while testing upstream against itself',
        ).not.toEqual([]);
      }
    }
  });

  it('points every lane at a facade module that exists', () => {
    // The other half of the same silence: a substitution that resolves to a missing file fails loudly, but
    // one that resolves to a stale module left behind by a rename does not.
    const found = lanes();
    for (const lane of found) {
      expect(
        lane.facadeModules.length,
        `${lane.facade}: a lane that substitutes something names what it substitutes in`,
      ).toBeGreaterThan(0);
      for (const target of lane.facadeModules) {
        const resolved = path.join(packagesDirectory, lane.facade, target);
        expect(() => statSync(resolved), `${lane.facade}: ${target} exists`).not.toThrow();
      }
    }
  });

  it('keeps every publishable facade covered by a lane', () => {
    // A published package is the one that must not lose its conformance evidence quietly.
    for (const facade of publishablePackages(workspace)) {
      const config = path.join(facade.directory, 'vitest.config.upstream.ts');
      expect(() => statSync(config), `${facade.manifest.name} declares a conformance lane`).not.toThrow();
    }
  });
});
