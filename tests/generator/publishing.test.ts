import { execFileSync } from 'node:child_process';
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';

import { applyBump, readFlightVersion } from '../../scripts/flight-version.ts';
import { channelForBranch, nextTarget, snapshotVersion } from '../../scripts/edge-version.ts';
import { publishablePackages } from '../../scripts/publishable-packages.ts';
import { isSnapshotVersionSuperseded } from '../../scripts/snapshot-version-order.ts';
import { stampVersion } from '../../scripts/version-packages.ts';

const workspace = path.resolve('.');

describe('published version borrows the upstream release', () => {
  it('derives the Flight version the pin belongs to, not the version stamped in its manifest', () => {
    // Flight stamps `version` only at release time, so between releases the submodule's manifest names
    // the PREVIOUS release while its commits are already working toward the next one. Publishing on
    // the stamped value would claim a version whose API this port does not have.
    //
    // These are golden values for the CURRENT pin, and updating them is meant to be a deliberate step
    // when the pin moves — the failure is the prompt to confirm which Flight release the new pin
    // belongs to, rather than letting the published version drift silently.
    const stamped = JSON.parse(readFileSync(path.join(workspace, 'upstream/packages/sdk/package.json'), 'utf8')) as {
      version: string;
    };
    expect(stamped.version).toBe('0.3.0');
    expect(readFlightVersion(workspace)).toBe('0.4.0');
  });

  it('bumps by conventional-commit level in the lane the base major selects', () => {
    // Pre-1.0 (ZeroVer): every level shifts down one, and the major never moves.
    expect(applyBump('0.2.0', 'breaking')).toBe('0.3.0');
    expect(applyBump('0.2.0', 'feature')).toBe('0.2.1');
    expect(applyBump('0.2.3', 'fix')).toBe('0.2.4');

    // At 1.0 and beyond the normal lane applies, with no code change — the lane is keyed on the base.
    expect(applyBump('1.4.2', 'breaking')).toBe('2.0.0');
    expect(applyBump('1.4.2', 'feature')).toBe('1.5.0');
    expect(applyBump('1.4.2', 'fix')).toBe('1.4.3');
  });

  it('keeps the published dependency range on the upstream version it was generated from', () => {
    // bitmap-rs substitutes @flighthq/bitmap, so a range that drifts from the derived upstream
    // version would pair the wasm kernels with an upstream they were never differentially tested
    // against. Both sides move together or this fails.
    const [surface] = publishablePackages(workspace).filter((item) => item.manifest.name === '@flighthq/bitmap-rs');
    expect(surface, '@flighthq/bitmap-rs is publishable').toBeDefined();

    const flight = readFlightVersion(workspace);
    const [major, minor] = flight.split('.');
    const dependencies = (surface?.manifest.dependencies ?? {}) as Record<string, string>;
    for (const [name, range] of Object.entries(dependencies)) {
      if (!name.startsWith('@flighthq/')) continue;
      expect(range, `${name} tracks upstream ${flight}`).toBe(`^${major}.${minor}.0`);
    }
  });
});

describe('snapshot versions', () => {
  it('names the upstream version before this repository has released it', () => {
    const { version, channel } = snapshotVersion('main', '0.3.0', [], 99, 'f399a5d');
    expect(version).toBe('0.3.0-edge.99.f399a5d');
    expect(channel).toBe('edge');
  });

  it('sorts above a release of the same upstream version', () => {
    // Without this the `edge` tag ends up behind `latest`: semver puts 0.3.0-edge.N below 0.3.0.
    expect(nextTarget('0.3.0', [])).toBe('0.3.0');
    expect(nextTarget('0.3.0', ['0.3.0'])).toBe('0.3.1');
    expect(nextTarget('0.3.0', ['0.3.0', '0.3.1'])).toBe('0.3.2');

    // A release older than the current upstream does not hold the target back.
    expect(nextTarget('0.4.0', ['0.3.0'])).toBe('0.4.0');
  });

  it('maps only release branches to a channel', () => {
    expect(channelForBranch('main')).toBe('edge');
    expect(channelForBranch('develop')).toBe('next');
    expect(channelForBranch('feature/x')).toBeUndefined();
    expect(() => snapshotVersion('feature/x', '0.3.0', [], 1, 'abc1234')).toThrow(/not a release channel/u);
  });
});

describe('dist-tag ordering guard', () => {
  it('refuses to drag a tag backwards, and allows everything else', () => {
    // Older count against the same base: publishing would move the tag back.
    expect(isSnapshotVersionSuperseded('0.3.0-edge.5.aaaaaaa', '0.3.0-edge.9.bbbbbbb')).toBe(true);
    expect(isSnapshotVersionSuperseded('0.3.0-edge.9.bbbbbbb', '0.3.0-edge.5.aaaaaaa')).toBe(false);

    // A stable release outranks any prerelease of the same base.
    expect(isSnapshotVersionSuperseded('0.3.0-edge.9.bbbbbbb', '0.3.0')).toBe(true);
    expect(isSnapshotVersionSuperseded('0.3.0', '0.3.0-edge.9.bbbbbbb')).toBe(false);

    expect(isSnapshotVersionSuperseded('0.3.0', '0.4.0')).toBe(true);
    expect(isSnapshotVersionSuperseded('0.4.0', '0.3.0')).toBe(false);

    // Anything unparseable is treated as "not superseded": publishing is the safe direction, since
    // a wrong `true` silently drops a build AND leaves the tag stale.
    expect(isSnapshotVersionSuperseded('0.3.0', 'garbage')).toBe(false);
    expect(isSnapshotVersionSuperseded('garbage', '0.3.0')).toBe(false);
  });
});

describe('publishable set', () => {
  it('is exactly the blessed facade', () => {
    expect(publishablePackages(workspace).map((item) => item.manifest.name)).toEqual(['@flighthq/bitmap-rs']);
  });

  it('excludes a package that marks itself private, and stamps only what it includes', () => {
    const sandbox = mkdtempSync(path.join(tmpdir(), 'flight-rs-publishable-'));
    try {
      write(sandbox, 'packages/public-one/package.json', { name: '@scope/public-one', version: '0.0.0' });
      write(sandbox, 'packages/private-one/package.json', {
        name: '@scope/private-one',
        private: true,
        version: '0.0.0',
      });

      expect(publishablePackages(sandbox).map((item) => item.manifest.name)).toEqual(['@scope/public-one']);

      expect(stampVersion('1.2.3-edge.4.abc1234', sandbox)).toBe(1);
      expect(readManifest(sandbox, 'packages/public-one/package.json').version).toBe('1.2.3-edge.4.abc1234');
      expect(readManifest(sandbox, 'packages/private-one/package.json').version).toBe('0.0.0');
    } finally {
      rmSync(sandbox, { force: true, recursive: true });
    }
  });

  it('stamps the Flight dependency range independently of the package version', () => {
    const sandbox = mkdtempSync(path.join(tmpdir(), 'flight-rs-flightrange-'));
    try {
      write(sandbox, 'packages/facade/package.json', {
        dependencies: { '@flighthq/bitmap': '^0.3.0', '@flighthq/types': '^0.3.0', 'left-pad': '^1.0.0' },
        name: '@scope/facade',
        version: '0.0.0',
      });

      // The release bridge stamps both from the Flight version it just proved parity against.
      stampVersion('0.4.0', sandbox, '0.4.0');
      let manifest = readFullManifest(sandbox, 'packages/facade/package.json');
      expect(manifest.version).toBe('0.4.0');
      expect(manifest.dependencies['@flighthq/bitmap']).toBe('^0.4.0');
      // A non-Flight dependency is left alone; this stamps a family, not every range.
      expect(manifest.dependencies['left-pad']).toBe('^1.0.0');

      // A port-only fix moves the package version while the Flight it targets stays put, so the two
      // must not be wired together.
      stampVersion('0.4.1', sandbox, '0.4.0');
      manifest = readFullManifest(sandbox, 'packages/facade/package.json');
      expect(manifest.version).toBe('0.4.1');
      expect(manifest.dependencies['@flighthq/bitmap']).toBe('^0.4.0');

      // Omitting the flag leaves ranges untouched, which is the snapshot and tag path.
      stampVersion('0.5.0', sandbox);
      manifest = readFullManifest(sandbox, 'packages/facade/package.json');
      expect(manifest.version).toBe('0.5.0');
      expect(manifest.dependencies['@flighthq/bitmap']).toBe('^0.4.0');
    } finally {
      rmSync(sandbox, { force: true, recursive: true });
    }
  });

  it('rejects a version that is not a version', () => {
    // The stamp is what a workflow writes into a manifest from a shell variable, so an empty or
    // flag-shaped value must fail here rather than reach the registry.
    for (const invalid of ['', '--tag', 'latest', '1.2', 'v1.2.3']) {
      expect(() => stampVersion(invalid, workspace), invalid).toThrow(/not a valid version/u);
    }
  });
});

describe('the release entry points run', () => {
  it('prints a GitHub Actions key=value pair for the current branch', () => {
    const output = execFileSync('node_modules/.bin/tsx', ['scripts/edge-version.ts', 'main'], {
      cwd: workspace,
      encoding: 'utf8',
    });
    expect(output).toMatch(/^version=\d+\.\d+\.\d+-edge\.\d+\.[0-9a-f]{7}\ntag=edge\n$/u);
  });
});

function write(root: string, relative: string, manifest: Record<string, unknown>): void {
  const file = path.join(root, relative);
  mkdirSync(path.dirname(file), { recursive: true });
  writeFileSync(file, `${JSON.stringify(manifest, null, 2)}\n`);
}

function readFullManifest(root: string, relative: string): { dependencies: Record<string, string>; version: string } {
  return JSON.parse(readFileSync(path.join(root, relative), 'utf8')) as {
    dependencies: Record<string, string>;
    version: string;
  };
}

function readManifest(root: string, relative: string): { version: string } {
  return JSON.parse(readFileSync(path.join(root, relative), 'utf8')) as { version: string };
}
