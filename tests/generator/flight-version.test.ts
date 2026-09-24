import { execFileSync } from 'node:child_process';
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';

import { readFlightVersion } from '../../scripts/flight-version.ts';

// The workflows fetch the submodule's HISTORY, not just its tags, before anything derives the Flight
// version. `tests/generator/workflows.test.ts` asserts the remedy appears in every such job, but it
// matches the command text — it cannot tell whether that command is the one the failure needs.
//
// This asserts the property instead, on a synthetic repository rather than on the submodule, so it
// states what is true of a shallow checkout anywhere rather than what this workstation happens to
// have fetched.

function git(repository: string, ...arguments_: readonly string[]): string {
  return execFileSync('git', arguments_, { cwd: repository, encoding: 'utf8' }).trim();
}

/**
 * An origin holding a tagged release plus one breaking commit on top, and a workspace whose
 * `upstream/` is a depth-1 clone of it — the shape `actions/checkout` produces for a submodule.
 * The tag is one commit behind HEAD, so it sits outside the shallow boundary.
 */
function createShallowWorkspace(): { workspace: string; upstream: string; cleanup: () => void } {
  const root = mkdtempSync(path.join(tmpdir(), 'flight-rs-flight-version-'));
  const origin = path.join(root, 'origin');
  const workspace = path.join(root, 'workspace');
  const upstream = path.join(workspace, 'upstream');

  mkdirSync(path.join(origin, 'packages', 'sdk'), { recursive: true });
  git(root, 'init', '--quiet', 'origin');
  git(origin, 'config', 'user.email', 'fixture@flighthq.dev');
  git(origin, 'config', 'user.name', 'Flight Fixture');
  writeFileSync(path.join(origin, 'packages', 'sdk', 'package.json'), '{ "version": "0.3.0" }\n');
  git(origin, 'add', '.');
  git(origin, 'commit', '--quiet', '-m', 'chore: update to 0.3.0');
  git(origin, 'tag', '0.3.0');
  writeFileSync(path.join(origin, 'packages', 'sdk', 'index.ts'), 'export {};\n');
  git(origin, 'add', '.');
  git(origin, 'commit', '--quiet', '-m', 'feat!: replace the surface entry point');
  const branch = git(origin, 'rev-parse', '--abbrev-ref', 'HEAD');

  mkdirSync(upstream, { recursive: true });
  git(upstream, 'init', '--quiet');
  git(upstream, 'remote', 'add', 'origin', origin);
  git(upstream, 'fetch', '--quiet', '--depth=1', 'origin', branch);
  git(upstream, 'checkout', '--quiet', 'FETCH_HEAD');

  return { workspace, upstream, cleanup: () => rmSync(root, { force: true, recursive: true }) };
}

describe('deriving the Flight version from a shallow submodule', () => {
  it('needs the history joining the tag to the pin, which fetching tags alone does not supply', () => {
    const { workspace, upstream, cleanup } = createShallowWorkspace();
    try {
      expect(git(upstream, 'rev-parse', '--is-shallow-repository')).toBe('true');
      expect(() => readFlightVersion(workspace)).toThrow(/no reachable version tag/u);

      // The remedy this replaced. The tag object arrives and `git describe` still cannot reach it,
      // which is the CI failure: a lane red for a checkout gap rather than for the change under test.
      git(upstream, 'fetch', '--quiet', '--tags', '--force', 'origin');
      expect(git(upstream, 'tag', '--list')).toContain('0.3.0');
      expect(() => readFlightVersion(workspace)).toThrow(/no reachable version tag/u);

      // The remedy in the workflows. `feat!:` since the tag bumps the minor in the ZeroVer lane.
      git(upstream, 'fetch', '--quiet', '--tags', '--force', '--unshallow', 'origin');
      expect(git(upstream, 'rev-parse', '--is-shallow-repository')).toBe('false');
      expect(readFlightVersion(workspace)).toBe('0.4.0');
    } finally {
      cleanup();
    }
  });
});
