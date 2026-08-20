// Mirrors upstream's scripts/clean-package-dist.ts, which every blessed facade's
// `prepack` invokes as `../../scripts/clean-package-dist.ts`. The filtered
// repository kept the package scripts but not the script they call, so `npm pack`
// and `npm publish` failed on the missing path before a tarball was ever built.

import { existsSync, readdirSync, rmSync } from 'node:fs';
import { dirname, isAbsolute, join, relative, resolve, sep } from 'node:path';
import { fileURLToPath } from 'node:url';

const scriptPath = fileURLToPath(import.meta.url);
const root = resolve(dirname(scriptPath), '..');
const packagesDirectory = resolve(root, 'packages');

/**
 * Removes generated distributions and build metadata together so the next build cannot trust one
 * without the other. A stale `.tsbuildinfo` next to a deleted `dist` makes `tsc -b` believe the
 * output is current and emit nothing, which is how an incomplete tarball gets published.
 */
export function cleanPackageBuildOutputs(directories: readonly string[]): void {
  for (const directory of directories) {
    const packageDirectory = resolve(directory);
    rmSync(join(packageDirectory, 'dist'), { force: true, recursive: true });
    for (const entry of readdirSync(packageDirectory, { withFileTypes: true })) {
      if (entry.isFile() && entry.name.endsWith('.tsbuildinfo')) {
        rmSync(join(packageDirectory, entry.name), { force: true });
      }
    }
  }
}

function isPackageDirectory(directory: string): boolean {
  const pathFromPackages = relative(packagesDirectory, directory);
  return (
    pathFromPackages !== '' &&
    pathFromPackages !== '..' &&
    !pathFromPackages.startsWith(`..${sep}`) &&
    !isAbsolute(pathFromPackages)
  );
}

function main(): void {
  const cwd = resolve(process.cwd());
  if (cwd === root) {
    const directories = readdirSync(packagesDirectory, { withFileTypes: true })
      .filter((entry) => entry.isDirectory())
      .map((entry) => join(packagesDirectory, entry.name));
    cleanPackageBuildOutputs(directories);
    return;
  }
  if (!isPackageDirectory(cwd)) {
    throw new Error(`clean-package-dist must be run from the repository root or a package under ${packagesDirectory}`);
  }
  if (!existsSync(join(cwd, 'package.json'))) {
    throw new Error('clean-package-dist must be run from a package directory with package.json');
  }
  cleanPackageBuildOutputs([cwd]);
}

if (resolve(process.argv[1] ?? '') === resolve(scriptPath)) main();
