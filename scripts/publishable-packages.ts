// Which packages under `packages/` are published to npm.
//
// The set is derived rather than listed: a package is publishable unless it marks itself
// `"private": true`. Today that resolves to exactly `@flighthq/bitmap-rs`, the only blessed facade
// and the only artifact here intended for consumers — `generated/` crates are all `publish = false`
// and are consumed through this repository, not from a registry.
//
// Deriving it means adding a second facade needs no edit here, and — more importantly — a package
// cannot be published by accident: opting out is a field in its own manifest, which is also where a
// reader looks to find out.

import { existsSync, readFileSync, readdirSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const repositoryRoot = join(dirname(fileURLToPath(import.meta.url)), '..');

export interface PackageManifest {
  name: string;
  private?: boolean;
  publishConfig?: { access?: string };
  version: string;
  [key: string]: unknown;
}

export interface PublishablePackage {
  directory: string;
  manifest: PackageManifest;
  manifestPath: string;
}

export function publishablePackages(workspace: string = repositoryRoot): PublishablePackage[] {
  const packagesDirectory = join(workspace, 'packages');
  if (!existsSync(packagesDirectory)) return [];

  const packages: PublishablePackage[] = [];
  for (const entry of readdirSync(packagesDirectory, { withFileTypes: true }).sort((left, right) =>
    left.name.localeCompare(right.name),
  )) {
    if (!entry.isDirectory()) continue;
    const directory = join(packagesDirectory, entry.name);
    const manifestPath = join(directory, 'package.json');
    if (!existsSync(manifestPath)) continue;

    const manifest = JSON.parse(readFileSync(manifestPath, 'utf8')) as PackageManifest;
    if (manifest.private === true) continue;
    packages.push({ directory, manifest, manifestPath });
  }
  return packages;
}
