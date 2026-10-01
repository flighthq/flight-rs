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
// So every lane must name a specifier that the upstream files it runs actually import. This asserts that
// for all of them at once, which is the part no single lane can check about the others.

const workspace = path.resolve('.');
const packagesDirectory = path.join(workspace, 'packages');

interface Lane {
  facade: string;
  specifier: string;
  upstreamDirectory: string;
}

/** Every facade that declares a conformance lane, with the specifier it substitutes. */
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
    // A lane that substitutes nothing is a lane that only re-runs upstream against upstream. That is a
    // legitimate thing to have — bitmap's suite exercises overrides through aliases instead — so a config
    // with no `resolveId` comparison is skipped rather than failed.
    const specifier = /source !== '([^']+)'/u.exec(source)?.[1];
    const upstreamPackage = /upstreamSource = path\.join\(upstreamPackages, '([^'/]+)\/src'\)/u.exec(source)?.[1];
    if (specifier === undefined || upstreamPackage === undefined) continue;
    found.push({
      facade,
      specifier,
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
      const importers = upstreamTestFiles(lane.upstreamDirectory).filter((file) =>
        readFileSync(file, 'utf8').includes(`from '${lane.specifier}'`),
      );
      expect(
        importers.map((file) => path.relative(workspace, file)),
        `${lane.facade} substitutes '${lane.specifier}', which no upstream test file imports — the lane ` +
          'would pass while testing upstream against itself',
      ).not.toEqual([]);
    }
  });

  it('points every lane at a facade module that exists', () => {
    // The other half of the same silence: a substitution that resolves to a missing file fails loudly, but
    // one that resolves to a stale module left behind by a rename does not.
    for (const facade of readdirSync(packagesDirectory)) {
      const config = path.join(packagesDirectory, facade, 'vitest.config.upstream.ts');
      let source: string;
      try {
        source = readFileSync(config, 'utf8');
      } catch {
        continue;
      }
      const target = /facade = path\.resolve\(import\.meta\.dirname, '([^']+)'\)/u.exec(source)?.[1];
      if (target === undefined) continue;
      const resolved = path.join(packagesDirectory, facade, target);
      expect(() => statSync(resolved), `${facade}: ${target} exists`).not.toThrow();
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
