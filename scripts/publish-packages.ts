// Publishes this repository's publishable packages to npm — today that is `@flighthq/bitmap-wasm`
// alone (see publishable-packages.ts).
//
// Two guards make a re-run safe, which matters because a publish that fails halfway is otherwise
// resolved by guesswork:
//
//   Already published. A version already on the registry is skipped, not retried, so re-running
//                      after a partial failure completes the set instead of erroring on the part
//                      that worked.
//   Tag would move backwards. `npm publish --tag edge` moves that tag to whatever it publishes, and
//                      npm offers no publish-without-a-tag. Two builds finishing out of commit order
//                      would otherwise leave `edge` pointing at the older snapshot. The comparison
//                      is against the REGISTRY rather than the branch tip on purpose: a tip
//                      comparison starves under burst commits, since every run sees a newer tip and
//                      skips, so nothing ever publishes.
//
// `prepack` in each package does the real build (clean, wasm rebuild, tsc), so this deliberately
// does NOT pass --ignore-scripts: the wasm module is what makes the tarball worth publishing, and
// skipping the hook would ship a stale or empty dist. That is the opposite of Flight's own
// publish-packages.ts, which builds its whole graph once up front and then suppresses hooks; with a
// single package there is nothing to amortize.
//
// Flight's script also carries a bounded worker pool, retry/backoff, and an error classifier. None
// of that is ported: those exist because 141 concurrent publishes make npm fail for reasons
// unrelated to any package. One package publishes serially, so a failure here is a real failure and
// should surface rather than be retried into ambiguity.
//
// Usage:
//   tsx scripts/publish-packages.ts                 publish to the default `latest` dist-tag
//   tsx scripts/publish-packages.ts --dry-run       pack and report, upload nothing
//   tsx scripts/publish-packages.ts --tag <tag>     publish under a dist-tag (edge/next)

import { execFileSync } from 'node:child_process';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';

import { publishablePackages, type PublishablePackage } from './publishable-packages.ts';
import { isSnapshotVersionSuperseded } from './snapshot-version-order.ts';

const scriptPath = fileURLToPath(import.meta.url);

interface Options {
  dryRun: boolean;
  tag: string;
}

function parseOptions(argv: readonly string[]): Options {
  let dryRun = false;
  let tag = 'latest';
  for (let index = 0; index < argv.length; index += 1) {
    const argument = argv[index];
    if (argument === '--dry-run') dryRun = true;
    else if (argument === '--tag') {
      const value = argv[index + 1];
      if (value === undefined) throw new Error('--tag requires a value');
      tag = value;
      index += 1;
    } else throw new Error(`unknown argument: ${argument}`);
  }
  return { dryRun, tag };
}

/** Versions already on the registry, and where the target dist-tag currently points. */
function registryState(name: string, tag: string): { tagVersion: string | undefined; versions: Set<string> } {
  try {
    const raw = execFileSync('npm', ['view', name, 'versions', 'dist-tags', '--json'], {
      encoding: 'utf8',
      stdio: ['ignore', 'pipe', 'pipe'],
    });
    const parsed = JSON.parse(raw) as {
      'dist-tags'?: Record<string, string>;
      versions?: string[] | string;
    };
    const versions = parsed.versions ?? [];
    return {
      tagVersion: parsed['dist-tags']?.[tag],
      versions: new Set(Array.isArray(versions) ? versions : [versions]),
    };
  } catch {
    // A package that has never been published has no versions and no tags. Any other failure —
    // network, auth — surfaces at the publish step, where the error is unambiguous.
    return { tagVersion: undefined, versions: new Set<string>() };
  }
}

function publish(target: PublishablePackage, options: Options): 'published' | 'skipped' {
  const { name, version } = target.manifest;
  const state = registryState(name, options.tag);

  if (state.versions.has(version)) {
    process.stdout.write(`[publish] ${name}@${version} already on the registry, skipping\n`);
    return 'skipped';
  }
  if (state.tagVersion !== undefined && isSnapshotVersionSuperseded(version, state.tagVersion)) {
    process.stdout.write(
      `[publish] ${name}@${version} skipped: ${options.tag} already points at the newer ${state.tagVersion}\n`,
    );
    return 'skipped';
  }

  // A scoped package defaults to restricted, which fails the publish outright. The manifest also
  // declares publishConfig.access, so this is belt and braces for a manifest that loses the field.
  const arguments_ = ['publish', '--access', 'public', '--tag', options.tag];
  if (options.dryRun) arguments_.push('--dry-run');

  execFileSync('npm', arguments_, { cwd: target.directory, env: process.env, stdio: 'inherit' });
  process.stdout.write(`[publish] ${name}@${version} -> ${options.tag}${options.dryRun ? ' (dry run)' : ''}\n`);
  return 'published';
}

function main(): void {
  const options = parseOptions(process.argv.slice(2));
  const targets = publishablePackages();
  if (targets.length === 0) {
    process.stderr.write('[publish] no publishable packages found\n');
    process.exit(1);
  }

  let published = 0;
  let skipped = 0;
  for (const target of targets) {
    if (publish(target, options) === 'published') published += 1;
    else skipped += 1;
  }
  process.stdout.write(`[publish] ${published} published, ${skipped} skipped\n`);
}

if (process.argv[1] !== undefined && join(process.argv[1]) === scriptPath) {
  try {
    main();
  } catch (error) {
    process.stderr.write(`[publish] ${error instanceof Error ? error.message : String(error)}\n`);
    process.exit(1);
  }
}
