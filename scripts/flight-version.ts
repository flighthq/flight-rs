// Derives the Flight version the pinned `upstream/` submodule corresponds to. Everything this
// repository publishes is a port of one upstream commit, so its version is not independently
// chosen — it is borrowed from the Flight release that commit belongs to.
//
// Reading `upstream/packages/sdk/package.json` alone is not enough. Flight stamps that field only at
// release time (`version-packages.ts`), so between releases it names the PREVIOUS release: the pin
// here says 0.2.0 while sitting 1887 commits past the 0.2.0 tag, by which point Flight had already
// published 0.3.0. Publishing a port of that tree as 0.2.0 would claim a version whose API it does
// not have.
//
// So this mirrors the lane logic in Flight's own `scripts/edge-version.ts`, applied to the submodule:
// take the stamped version as the base, find the highest conventional-commits level since the
// submodule's last version tag, and bump. Flight uses this to name the release it is heading toward;
// the same computation names the release a given commit belongs to. For the current pin it derives
// 0.3.0, which is what Flight actually published — that agreement is asserted in
// tests/generator/flight-version.test.ts rather than assumed.
//
// Usage:
//   tsx scripts/flight-version.ts     print the derived upstream version

import { execFileSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

export type BumpLevel = 'breaking' | 'feature' | 'fix';

const scriptPath = fileURLToPath(import.meta.url);
const repositoryRoot = join(dirname(scriptPath), '..');

/**
 * The Flight version the pinned submodule belongs to.
 *
 * Throws when the submodule has no reachable version tag. That is deliberately fatal rather than a
 * fallback to the stamped base: a shallow submodule checkout would otherwise yield a plausible but
 * wrong version, and publishing under a wrong number cannot be undone. A failed publish can.
 */
export function readFlightVersion(workspace: string = repositoryRoot): string {
  const upstream = join(workspace, 'upstream');
  const base = readSdkVersion(upstream);
  const tag = lastVersionTag(upstream);
  if (tag === undefined) {
    throw new Error(
      'upstream/ has no reachable version tag, so the Flight version cannot be derived. ' +
        'Fetch the submodule history and tags (git -C upstream fetch --unshallow --tags) and retry.',
    );
  }
  return applyBump(base, detectBumpLevel(upstream, `${tag}..HEAD`));
}

/**
 * Applies a conventional-commits bump, choosing which digit moves by the current lane. Pre-1.0 is
 * the ZeroVer lane where every level shifts down one — breaking moves the minor, feature and fix
 * move the patch, and the major stays 0. At 1.0 and above the normal lane applies. The lane is keyed
 * on the base major, so it switches itself.
 */
export function applyBump(base: string, level: BumpLevel): string {
  const [major = 0, minor = 0, patch = 0] = base.split('.').map((part) => Number.parseInt(part, 10));
  if (major === 0) {
    return level === 'breaking' ? `0.${minor + 1}.0` : `0.${minor}.${patch + 1}`;
  }
  if (level === 'breaking') return `${major + 1}.0.0`;
  if (level === 'feature') return `${major}.${minor + 1}.0`;
  return `${major}.${minor}.${patch + 1}`;
}

/**
 * The highest conventional-commits level in `range` (breaking outranks feature outranks fix). Reads
 * raw bodies NUL-delimited so a multi-line `BREAKING CHANGE:` footer survives intact.
 */
export function detectBumpLevel(repository: string, range: string): BumpLevel {
  const messages = git(repository, 'log', '--format=%B%x00', range)
    .split('\0')
    .map((message) => message.trim())
    .filter(Boolean);

  let level: BumpLevel = 'fix';
  for (const message of messages) {
    if (isBreakingCommit(message)) return 'breaking';
    if (isFeatureCommit(message)) level = 'feature';
  }
  return level;
}

/** A `type!:` subject, or a `BREAKING CHANGE:` footer anywhere in the body. */
function isBreakingCommit(message: string): boolean {
  const subject = message.split('\n', 1)[0] ?? '';
  return /^[a-z]+(\([^)]*\))?!:/u.test(subject) || /^BREAKING[ -]CHANGE:/mu.test(message);
}

/** A `feat:` subject. A breaking `feat!:` is caught earlier by isBreakingCommit. */
function isFeatureCommit(message: string): boolean {
  return /^feat(\([^)]*\))?:/u.test(message.split('\n', 1)[0] ?? '');
}

/**
 * The nearest reachable bare numeric tag. Matched narrowly so the repository's non-version tags
 * (`quimby/seed`, `quimby/base`) cannot stand in for a release.
 */
function lastVersionTag(repository: string): string | undefined {
  try {
    return git(repository, 'describe', '--tags', '--abbrev=0', '--match', '[0-9]*.[0-9]*.[0-9]*');
  } catch {
    return undefined;
  }
}

function readSdkVersion(upstream: string): string {
  const manifest = JSON.parse(readFileSync(join(upstream, 'packages/sdk/package.json'), 'utf8')) as {
    version?: string;
  };
  const version = manifest.version;
  if (version === undefined) throw new Error('upstream/packages/sdk/package.json has no version');
  return version;
}

function git(repository: string, ...arguments_: readonly string[]): string {
  return execFileSync('git', arguments_, { cwd: repository, encoding: 'utf8' }).trim();
}

if (process.argv[1] !== undefined && join(process.argv[1]) === scriptPath) {
  process.stdout.write(`${readFlightVersion()}\n`);
}
