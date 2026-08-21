// Orders the versions edge-version.ts produces, so a publish can refuse to drag a dist-tag backwards.
//
// A dist-tag is a mutable pointer, and `npm publish --tag edge` moves it to whatever it just
// published — npm offers no publish-without-setting-a-tag. So when two builds for different commits
// reach the publish step out of commit order, the older one would point `edge` at an older snapshot.
// Skipping it loses nothing: main is linear and `count` is monotonic, so the newer snapshot already
// on the tag contains the older commit's work.
//
// Deliberately narrow rather than a general semver implementation. It understands exactly the two
// shapes this repository publishes — `<major>.<minor>.<patch>` for a stable release and
// `<major>.<minor>.<patch>-<channel>.<count>.<sha>` for a snapshot — and treats anything it cannot
// parse as "not superseded". That direction is the safe one: a wrong `false` merely publishes, which
// is the behavior without the guard, whereas a wrong `true` silently drops a snapshot AND leaves the
// tag stale.

interface ParsedVersion {
  base: readonly [number, number, number];
  // null for a stable release; the monotonic build count for a snapshot. The sha is parsed but
  // discarded — edge-version.ts documents it as disambiguation only, never the sort key.
  count: number | null;
}

/**
 * True when `tagVersion` (what the dist-tag currently points at) is strictly newer than `version`
 * (what this run is about to publish), meaning publishing would move the tag backwards.
 */
export function isSnapshotVersionSuperseded(version: string, tagVersion: string): boolean {
  const ours = parseVersion(version);
  const theirs = parseVersion(tagVersion);
  if (ours === null || theirs === null) return false;

  for (let index = 0; index < 3; index += 1) {
    const mine = ours.base[index] ?? 0;
    const other = theirs.base[index] ?? 0;
    if (other !== mine) return other > mine;
  }

  // Same base version. A stable release outranks any prerelease of that base, per semver.
  if (theirs.count === null) return ours.count !== null;
  if (ours.count === null) return false;
  return theirs.count > ours.count;
}

function parseVersion(version: string): ParsedVersion | null {
  const match = /^(\d+)\.(\d+)\.(\d+)(?:-[0-9a-z-]+\.(\d+)\.[0-9a-z-]+)?$/iu.exec(version.trim());
  if (match === null) return null;
  const [, major, minor, patch, count] = match;
  return {
    base: [Number(major), Number(minor), Number(patch)],
    count: count === undefined ? null : Number(count),
  };
}
