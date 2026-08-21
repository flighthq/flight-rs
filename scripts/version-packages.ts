// Stamps every publishable package to one version — run before tagging a release, and by CI before
// publishing a snapshot.
//
// `--flight <version>` additionally rewrites every `@flighthq/*` dependency range to `^<version>`.
// That is a claim of compatibility, so the release lane makes it only after running the parity suite
// against those exact published packages. It is separate from the package version because the two
// legitimately differ: a port-only fix ships 0.4.1 while still depending on `^0.4.0`.
//
// Usage:
//   tsx scripts/version-packages.ts <version>
//   tsx scripts/version-packages.ts <version> --flight <flight-version>

import { readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';

import { publishablePackages } from './publishable-packages.ts';

const scriptPath = fileURLToPath(import.meta.url);

// A bare release, or a snapshot from edge-version.ts. Rejecting anything else here is what stops a
// shell mishap ("--tag", an empty string) from being written into a manifest and published.
const VERSION_PATTERN = /^\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?$/u;

export function stampVersion(version: string, workspace?: string, flightVersion?: string): number {
  if (!VERSION_PATTERN.test(version)) throw new Error(`not a valid version: ${version}`);
  if (flightVersion !== undefined && !VERSION_PATTERN.test(flightVersion)) {
    throw new Error(`not a valid version: ${flightVersion}`);
  }

  let changed = 0;
  for (const { manifestPath } of publishablePackages(workspace)) {
    const text = readFileSync(manifestPath, 'utf8');
    // Replace only the top-level "version" line, so the diff is one line rather than a reserialize
    // that would reorder keys and churn the manifest.
    let updated = text.replace(/^(\s*"version":\s*")[^"]*(")/mu, `$1${version}$2`);
    if (flightVersion !== undefined) updated = stampFlightRanges(updated, flightVersion);
    if (updated !== text) {
      writeFileSync(manifestPath, updated);
      changed += 1;
    }
  }
  return changed;
}

/**
 * Rewrites every `"@flighthq/x": "<range>"` to `^<flightVersion>`. Line-oriented for the same reason
 * as the version stamp: a JSON round-trip would reorder keys and bury the change.
 */
function stampFlightRanges(text: string, flightVersion: string): string {
  return text.replace(/^(\s*"@flighthq\/[^"]+":\s*")[^"]*(")/gmu, `$1^${flightVersion}$2`);
}

if (process.argv[1] !== undefined && join(process.argv[1]) === scriptPath) {
  const version = process.argv[2];
  const flightIndex = process.argv.indexOf('--flight');
  const flightVersion = flightIndex === -1 ? undefined : process.argv[flightIndex + 1];

  if (
    version === undefined ||
    !VERSION_PATTERN.test(version) ||
    (flightIndex !== -1 && (flightVersion === undefined || !VERSION_PATTERN.test(flightVersion)))
  ) {
    process.stderr.write('Usage: tsx scripts/version-packages.ts <version> [--flight <flight-version>]\n');
    process.exit(1);
  }

  const total = publishablePackages().length;
  const changed = stampVersion(version, undefined, flightVersion);
  const suffix = flightVersion === undefined ? '' : `, @flighthq/* -> ^${flightVersion}`;
  process.stdout.write(`[version:packages] set ${changed}/${total} packages to ${version}${suffix}\n`);
}
