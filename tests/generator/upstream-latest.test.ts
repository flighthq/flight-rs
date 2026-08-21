import { execFileSync } from 'node:child_process';
import path from 'node:path';

import { compareChannel, packagesAt } from '../../scripts/upstream-latest.ts';

const upstream = path.resolve('upstream');

function git(...arguments_: readonly string[]): string {
  return execFileSync('git', arguments_, { cwd: upstream, encoding: 'utf8' }).trim();
}

describe('latest upstream reference', () => {
  it('reads a ref without checking it out or moving the pin', () => {
    const pinBefore = git('rev-parse', 'HEAD');
    const statusBefore = git('status', '--porcelain');

    const packages = packagesAt('HEAD', upstream);
    expect(packages).toContain('bitmap');
    expect(packages).toContain('physics2d');

    // The whole point of the tool: planning against newer upstream must not disturb the tree that
    // every generated file and report describes.
    expect(git('rev-parse', 'HEAD')).toBe(pinBefore);
    expect(git('status', '--porcelain')).toBe(statusBefore);
  });

  it('reports how far a channel is ahead, and which packages it adds', () => {
    const report = compareChannel('origin/main', upstream);

    expect(report.ahead).toBeGreaterThan(0);
    expect(report.sha).toMatch(/^[0-9a-f]{7,}$/u);

    // Added packages are the ones absent from the pin — the signal for "what could be generated
    // next" once the pin moves.
    const pinned = new Set(packagesAt('HEAD', upstream));
    for (const name of report.packagesAdded) expect(pinned.has(name)).toBe(false);
  });

  it('does not invent a package upstream has not written', () => {
    // physics3d is chartered in Flight but has no implementation on any ref. Asserting its absence
    // keeps a future reader from taking the charter as evidence the source exists — and turns this
    // into a real signal the day upstream lands it.
    for (const reference of ['HEAD', 'origin/main', 'origin/develop']) {
      expect(packagesAt(reference, upstream), reference).not.toContain('physics3d');
    }
  });
});
