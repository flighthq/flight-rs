// Stamps every publishable package to one version — run before tagging a release, and by CI before
// publishing a snapshot.
//
// Usage: tsx scripts/version-packages.ts <version>   (e.g. 0.3.0, or 0.3.0-edge.99.f399a5d)

import { readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';

import { publishablePackages } from './publishable-packages.ts';

const scriptPath = fileURLToPath(import.meta.url);

// A bare release, or a snapshot from edge-version.ts. Rejecting anything else here is what stops a
// shell mishap ("--tag", an empty string) from being written into a manifest and published.
const VERSION_PATTERN = /^\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?$/u;

export function stampVersion(version: string, workspace?: string): number {
  if (!VERSION_PATTERN.test(version)) throw new Error(`not a valid version: ${version}`);

  let changed = 0;
  for (const { manifestPath } of publishablePackages(workspace)) {
    const text = readFileSync(manifestPath, 'utf8');
    // Replace only the top-level "version" line, so the diff is one line rather than a reserialize
    // that would reorder keys and churn the manifest.
    const updated = text.replace(/^(\s*"version":\s*")[^"]*(")/mu, `$1${version}$2`);
    if (updated !== text) {
      writeFileSync(manifestPath, updated);
      changed += 1;
    }
  }
  return changed;
}

if (process.argv[1] !== undefined && join(process.argv[1]) === scriptPath) {
  const version = process.argv[2];
  if (version === undefined || !VERSION_PATTERN.test(version)) {
    process.stderr.write('Usage: tsx scripts/version-packages.ts <version>   (e.g. 0.3.0)\n');
    process.exit(1);
  }
  const total = publishablePackages().length;
  process.stdout.write(`[version:packages] set ${stampVersion(version)}/${total} packages to ${version}\n`);
}
