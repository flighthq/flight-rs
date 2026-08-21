// Computes the snapshot ("edge") version CI publishes on every push to a release branch, separate
// from the tag-triggered stable release. Prints `version=<v>` and `tag=<channel>` in GitHub Actions
// `$GITHUB_OUTPUT` key=value form, so a workflow captures both with `>> "$GITHUB_OUTPUT"`.
//
// Scheme:
//   flight   = the version of the pinned upstream submodule, derived by scripts/flight-version.ts.
//              This port does not choose a version of its own; it borrows the Flight release it is
//              a port of, so a reader can tell which upstream a published tarball corresponds to.
//   target   = flight, unless this repository has ALREADY released that version — then the patch
//              above the last release. Without that step every snapshot after a 0.3.0 release would
//              be `0.3.0-edge.N`, which semver sorts BELOW the released 0.3.0, leaving the `edge`
//              dist-tag pointing at something older than `latest`.
//   version  = <target>-<channel>.<count>.<sha>
//              channel  main -> edge, develop -> next (this is also the dist-tag). Only `main`
//                       exists today; `develop` is wired so a second lane needs no code change.
//              count    commits since this repository's last version tag, so the number stays small
//                       and resets each release. It is a numeric prerelease identifier and the real
//                       sort key, which is why the sha needs no ordering of its own.
//              <sha>    short commit sha, disambiguating builds that share a count.
//
// Note the divergence from Flight's own edge-version.ts, which bumps its base by the conventional
// commits in ITS history: this repository's commits describe porting work, not upstream API changes,
// so bumping on them would claim an upstream version that was never generated. The base moves only
// when the submodule pin moves. `target` handles the one case that still needs a local bump.
//
// Usage:
//   tsx scripts/edge-version.ts            branch from GITHUB_REF_NAME, else the current branch
//   tsx scripts/edge-version.ts <branch>   compute for an explicit branch

import { execFileSync } from 'node:child_process';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

import { readFlightVersion } from './flight-version.ts';

const scriptPath = fileURLToPath(import.meta.url);
const repositoryRoot = join(dirname(scriptPath), '..');

export interface SnapshotVersion {
  channel: 'edge' | 'next';
  version: string;
}

/** main publishes the `edge` channel, develop the `next` channel; any other branch is not a target. */
export function channelForBranch(name: string): 'edge' | 'next' | undefined {
  if (name === 'main') return 'edge';
  if (name === 'develop') return 'next';
  return undefined;
}

/**
 * The version to publish for `branch`, and the dist-tag to publish it under.
 *
 * `releasedVersions` is every stable version this repository has already tagged; it decides whether
 * the snapshot names the current upstream version or the patch above the last release.
 */
export function snapshotVersion(
  branch: string,
  flight: string,
  releasedVersions: readonly string[],
  count: number,
  sha: string,
): SnapshotVersion {
  const channel = channelForBranch(branch);
  if (channel === undefined) {
    throw new Error(`branch "${branch}" is not a release channel (expected main or develop)`);
  }
  return { channel, version: `${nextTarget(flight, releasedVersions)}-${channel}.${count}.${sha}` };
}

/**
 * The version a snapshot is heading toward: the upstream version, or the patch above the newest
 * release at or beyond it. A snapshot must sort ABOVE every stable release it follows, or the
 * channel tag ends up behind `latest`.
 */
export function nextTarget(flight: string, releasedVersions: readonly string[]): string {
  const flightParts = parse(flight);
  let target = flightParts;
  for (const released of releasedVersions) {
    const parts = parse(released);
    if (parts === null) continue;
    if (target === null || compare(parts, target) >= 0) target = bumpPatch(parts);
  }
  return (target ?? flightParts ?? [0, 0, 0]).join('.');
}

function bumpPatch(parts: readonly [number, number, number]): [number, number, number] {
  return [parts[0], parts[1], parts[2] + 1];
}

function compare(left: readonly [number, number, number], right: readonly [number, number, number]): number {
  for (let index = 0; index < 3; index += 1) {
    const difference = (left[index] ?? 0) - (right[index] ?? 0);
    if (difference !== 0) return difference;
  }
  return 0;
}

function parse(version: string): [number, number, number] | null {
  const match = /^(\d+)\.(\d+)\.(\d+)$/u.exec(version.trim());
  if (match === null) return null;
  return [Number(match[1]), Number(match[2]), Number(match[3])];
}

/** Every bare numeric tag in this repository, so non-version tags cannot be mistaken for releases. */
function releasedVersions(): string[] {
  try {
    return git('tag', '--list', '[0-9]*.[0-9]*.[0-9]*')
      .split('\n')
      .map((tag) => tag.trim())
      .filter((tag) => /^\d+\.\d+\.\d+$/u.test(tag));
  } catch {
    return [];
  }
}

/**
 * Commits since the last release, so the count stays small and resets each version. Falls back to
 * the total commit count when no version tag is reachable, which is the state before a first release.
 */
function commitCount(): number {
  const tags = releasedVersions();
  const range = tags.length === 0 ? 'HEAD' : `${lastReachableTag() ?? 'HEAD'}..HEAD`;
  return Number(git('rev-list', '--count', range));
}

function lastReachableTag(): string | undefined {
  try {
    return git('describe', '--tags', '--abbrev=0', '--match', '[0-9]*.[0-9]*.[0-9]*');
  } catch {
    return undefined;
  }
}

function git(...arguments_: readonly string[]): string {
  return execFileSync('git', arguments_, { cwd: repositoryRoot, encoding: 'utf8' }).trim();
}

if (process.argv[1] !== undefined && join(process.argv[1]) === scriptPath) {
  const branch = process.argv[2] ?? process.env.GITHUB_REF_NAME ?? git('rev-parse', '--abbrev-ref', 'HEAD');
  try {
    const snapshot = snapshotVersion(
      branch,
      readFlightVersion(repositoryRoot),
      releasedVersions(),
      commitCount(),
      git('rev-parse', '--short=7', 'HEAD'),
    );
    process.stdout.write(`version=${snapshot.version}\ntag=${snapshot.channel}\n`);
  } catch (error) {
    process.stderr.write(`[edge-version] ${error instanceof Error ? error.message : String(error)}\n`);
    process.exit(1);
  }
}
