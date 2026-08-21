import { execFileSync } from 'node:child_process';
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';

import { compareChannel, packagesAt } from '../../scripts/upstream-latest.ts';

const upstream = path.resolve('upstream');

function git(repository: string, ...arguments_: readonly string[]): string {
  return execFileSync('git', arguments_, { cwd: repository, encoding: 'utf8' }).trim();
}

function createAheadFixture(): string {
  const repository = mkdtempSync(path.join(tmpdir(), 'flight-rs-upstream-latest-'));
  git(repository, 'init', '--quiet');
  git(repository, 'config', 'user.email', 'fixture@flighthq.dev');
  git(repository, 'config', 'user.name', 'Flight Fixture');

  const bitmap = path.join(repository, 'packages', 'bitmap');
  mkdirSync(bitmap, { recursive: true });
  writeFileSync(path.join(bitmap, 'package.json'), '{}\n');
  git(repository, 'add', '.');
  git(repository, 'commit', '--quiet', '-m', 'pinned');
  const pinned = git(repository, 'rev-parse', 'HEAD');

  const physics3d = path.join(repository, 'packages', 'physics3d');
  mkdirSync(physics3d, { recursive: true });
  writeFileSync(path.join(physics3d, 'package.json'), '{}\n');
  git(repository, 'add', '.');
  git(repository, 'commit', '--quiet', '-m', 'add physics3d');
  git(repository, 'update-ref', 'refs/remotes/origin/main', 'HEAD');
  git(repository, 'reset', '--quiet', '--hard', pinned);

  return repository;
}

describe('latest upstream reference', () => {
  it('reads a ref without checking it out or moving the pin', () => {
    const pinBefore = git(upstream, 'rev-parse', 'HEAD');
    const statusBefore = git(upstream, 'status', '--porcelain');

    const packages = packagesAt('HEAD', upstream);
    expect(packages).toContain('bitmap');
    expect(packages).toContain('physics2d');

    // The whole point of the tool: planning against newer upstream must not disturb the tree that
    // every generated file and report describes.
    expect(git(upstream, 'rev-parse', 'HEAD')).toBe(pinBefore);
    expect(git(upstream, 'status', '--porcelain')).toBe(statusBefore);
  });

  it('reports how far a channel is ahead, and which packages it adds', () => {
    const fixture = createAheadFixture();
    try {
      const pinBefore = git(fixture, 'rev-parse', 'HEAD');
      const report = compareChannel('origin/main', fixture);

      expect(report.ahead).toBe(1);
      expect(report.behind).toBe(0);
      expect(report.packagesAdded).toEqual(['physics3d']);
      expect(report.packagesRemoved).toEqual([]);
      expect(report.sha).toMatch(/^[0-9a-f]{7,}$/u);
      expect(git(fixture, 'rev-parse', 'HEAD')).toBe(pinBefore);
    } finally {
      rmSync(fixture, { force: true, recursive: true });
    }
  });

  it('reads packages added by the current pinned epoch', () => {
    // physics3d landed upstream in b554b6de2 and is now part of the pinned input. Remote-tracking
    // refs remain deliberately unasserted: they are an offline cache and can legitimately be ahead,
    // stale, or equal to the pin.
    expect(packagesAt('HEAD', upstream)).toContain('physics3d');
  });
});
