// Which packages under `packages/` are published to npm.
//
// The set is derived rather than listed: a package is publishable unless it marks itself
// `"private": true`. Today that resolves only to the bitmap wasm facade. The incomplete Physics ABI
// prototypes opt out in their own manifests, and `generated/` crates are all `publish = false` and
// are consumed through this repository, not from a registry.
//
// Deriving it means adding a second facade needs no edit here, and — more importantly — a package
// cannot be published by accident: opting out is a field in its own manifest, which is also where a
// reader looks to find out.

import { existsSync, readFileSync, readdirSync } from 'node:fs';
import { dirname, join, relative, resolve } from 'node:path';
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

if (process.argv[1] !== undefined && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const arguments_ = process.argv.slice(2);
  const namesOnly = arguments_.length === 1 && arguments_[0] === '--names';
  if (arguments_.length > 0 && !namesOnly) {
    throw new Error(`Usage: ${relative(repositoryRoot, process.argv[1])} [--names]`);
  }

  for (const { directory, manifest } of publishablePackages()) {
    process.stdout.write(`${namesOnly ? manifest.name : relative(repositoryRoot, directory)}\n`);
  }
}
