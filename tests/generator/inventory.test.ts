import { execFileSync } from 'node:child_process';
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import os from 'node:os';
import path from 'node:path';

import {
  analyzeUpstream,
  packageNameToRustCrate,
  packageRootExportLane,
  readUpstreamCommit,
  resolvePackageExportLane,
  sourcePathToImplementationModule,
  sourcePathToRustModule,
} from '../../tools/generator/src/analyze/inventory.ts';

function git(directory: string, ...arguments_: string[]): string {
  return execFileSync('git', ['-C', directory, ...arguments_], {
    encoding: 'utf8',
    stdio: ['ignore', 'pipe', 'pipe'],
  }).trim();
}

function commitFile(directory: string, file: string, content: string): string {
  writeFileSync(path.join(directory, file), content);
  git(directory, 'add', file);
  git(directory, 'commit', '-m', `fixture ${file}`);
  return git(directory, 'rev-parse', 'HEAD');
}

describe('cultivated upstream analysis', () => {
  it('accounts for every package and representative export', () => {
    const inventory = analyzeUpstream(path.resolve('.'));
    const inventoryByName = new Map(inventory.packages.map((item) => [item.name, item]));
    const geometry = inventory.packages.find((item) => item.name === '@flighthq/geometry');
    if (!geometry) throw new Error('Expected @flighthq/geometry');
    const geometryRoot = resolvePackageExportLane(inventoryByName, '@flighthq/geometry');
    const geometryContract = resolvePackageExportLane(inventoryByName, '@flighthq/geometry/contract');

    expect(inventory.upstreamCommit).toBe('181dea5e323d8b0845ede2fade7f67b2ce8d2554');
    expect(inventory.summary.packages).toBe(143);
    expect(inventory.summary.exportLanes).toBe(299);
    expect(inventory.summary.exports).toBe(32_998);
    expect(inventory.summary.rootExports).toBe(12_782);
    expect(inventory.summary.sourceFiles).toBe(2_544);
    expect(inventory.summary.testFiles).toBe(1_419);
    expect(geometry.exports.some((item) => item.name === 'createVector2')).toBe(true);
    expect(geometry.exportLanes.map((lane) => lane.specifier)).toEqual([
      '@flighthq/geometry',
      '@flighthq/geometry/contract',
    ]);
    expect(packageRootExportLane(geometry)).toBe(geometryRoot);
    expect(geometryContract.exports.find((item) => item.name === 'createVector2')).toEqual(
      geometryRoot.exports.find((item) => item.name === 'createVector2'),
    );
    expect(() => resolvePackageExportLane(inventoryByName, '@flighthq/geometry/private')).toThrow(
      'Package import uses an unaccounted export lane: @flighthq/geometry/private',
    );
    expect(geometry.sdkExposures.map((exposure) => exposure.sdkLane)).toEqual([
      '@flighthq/sdk',
      '@flighthq/sdk/contract',
      '@flighthq/sdk/core',
    ]);
    expect(geometry.rustCrate).toBe('flighthq-geometry');
  });
});

describe('upstream provenance', () => {
  it('requires the initialized upstream HEAD to match the recorded gitlink', () => {
    const workspace = mkdtempSync(path.join(os.tmpdir(), 'flight-generator-provenance-'));
    const upstream = path.join(workspace, 'upstream');
    try {
      mkdirSync(upstream);
      git(workspace, 'init');
      git(workspace, 'config', 'user.email', 'generator@example.invalid');
      git(workspace, 'config', 'user.name', 'Generator Test');
      git(upstream, 'init');
      git(upstream, 'config', 'user.email', 'generator@example.invalid');
      git(upstream, 'config', 'user.name', 'Generator Test');
      const recorded = commitFile(upstream, 'source.ts', 'export const version = 1;\n');
      writeFileSync(
        path.join(workspace, '.gitmodules'),
        '[submodule "upstream"]\n\tpath = upstream\n\turl = https://example.invalid/upstream.git\n',
      );
      git(workspace, 'add', '.gitmodules');
      git(workspace, 'update-index', '--add', '--cacheinfo', `160000,${recorded},upstream`);
      git(workspace, 'commit', '-m', 'record upstream');

      expect(readUpstreamCommit(upstream)).toBe(recorded);

      const moved = commitFile(upstream, 'source.ts', 'export const version = 2;\n');
      expect(moved).not.toBe(recorded);
      expect(() => readUpstreamCommit(upstream)).toThrow(
        `Upstream submodule HEAD ${moved} does not match the recorded commit ${recorded}`,
      );

      rmSync(path.join(upstream, '.git'), { force: true, recursive: true });
      expect(() => readUpstreamCommit(upstream)).toThrow('Upstream submodule is not initialized');
    } finally {
      rmSync(workspace, { force: true, recursive: true });
    }
  });
});

describe('Rust identity mapping', () => {
  it('maps package and source identities deterministically', () => {
    expect(packageNameToRustCrate('@flighthq/render-gl')).toBe('flighthq-render-gl');
    expect(sourcePathToRustModule('upstream/packages/geometry/src/vector2.ts')).toBe('vector2');
    expect(sourcePathToRustModule('upstream/packages/render-gl/src/glShader.ts')).toBe('gl_shader');
    expect(sourcePathToRustModule('upstream/packages/menu/src/menu-templates.ts')).toBe('menu_templates');
    expect(sourcePathToRustModule('upstream/packages/signals/src/internal.ts')).toBeUndefined();
    expect(sourcePathToImplementationModule('upstream/packages/signals/src/internal.ts')).toBe('_internal_internal');
  });
});
