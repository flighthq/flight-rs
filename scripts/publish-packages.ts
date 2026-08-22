// Publishes this repository's blessed wasm facades to npm (see publishable-packages.ts).
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
// Standalone `npm pack` remains safe because each package's prepack performs a full rebuild. This
// multi-package release path is deliberately different: it builds every wasm target once, assembles
// each package, and then passes --ignore-scripts to publish. Otherwise three facades each rerun the
// same ten-minute repository generator before compiling their one adapter.
//
// Flight's script also carries a bounded worker pool, retry/backoff, and an error classifier. None
// of that is ported: those exist because 141 concurrent publishes make npm fail for reasons
// unrelated to any package. This much smaller facade set publishes serially, so a failure here is a
// real failure and should surface rather than be retried into ambiguity.
//
// Usage:
//   tsx scripts/publish-packages.ts                 publish to the default `latest` dist-tag
//   tsx scripts/publish-packages.ts --dry-run       pack and report, upload nothing
//   tsx scripts/publish-packages.ts --tag <tag>     publish under a dist-tag (edge/next)

import { execFileSync } from 'node:child_process';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

import { publishablePackages, type PublishablePackage } from './publishable-packages.ts';
import { isSnapshotVersionSuperseded } from './snapshot-version-order.ts';

const scriptPath = fileURLToPath(import.meta.url);
const workspace = join(dirname(scriptPath), '..');

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

function shouldPublish(target: PublishablePackage, options: Options): boolean {
  const { name, version } = target.manifest;
  const state = registryState(name, options.tag);

  if (state.versions.has(version)) {
    process.stdout.write(`[publish] ${name}@${version} already on the registry, skipping\n`);
    return false;
  }
  if (state.tagVersion !== undefined && isSnapshotVersionSuperseded(version, state.tagVersion)) {
    process.stdout.write(
      `[publish] ${name}@${version} skipped: ${options.tag} already points at the newer ${state.tagVersion}\n`,
    );
    return false;
  }
  return true;
}

function build(targets: readonly PublishablePackage[]): void {
  execFileSync('npm', ['run', 'wasm'], { cwd: workspace, env: process.env, stdio: 'inherit' });
  for (const target of targets) {
    for (const script of ['clean', 'clean:dist', 'build:package']) {
      execFileSync('npm', ['run', script], { cwd: target.directory, env: process.env, stdio: 'inherit' });
    }
  }
}

function publish(target: PublishablePackage, options: Options): void {
  const { name, version } = target.manifest;

  // A scoped package defaults to restricted, which fails the publish outright. The manifest also
  // declares publishConfig.access, so this is belt and braces for a manifest that loses the field.
  const arguments_ = ['publish', '--ignore-scripts', '--access', 'public', '--tag', options.tag];
  if (options.dryRun) arguments_.push('--dry-run');

  execFileSync('npm', arguments_, { cwd: target.directory, env: process.env, stdio: 'inherit' });
  process.stdout.write(`[publish] ${name}@${version} -> ${options.tag}${options.dryRun ? ' (dry run)' : ''}\n`);
}

function main(): void {
  const options = parseOptions(process.argv.slice(2));
  const targets = publishablePackages();
  if (targets.length === 0) {
    process.stderr.write('[publish] no publishable packages found\n');
    process.exit(1);
  }

  const eligible = targets.filter((target) => shouldPublish(target, options));
  if (eligible.length > 0) build(eligible);
  for (const target of eligible) publish(target, options);
  process.stdout.write(`[publish] ${eligible.length} published, ${targets.length - eligible.length} skipped\n`);
}

if (process.argv[1] !== undefined && join(process.argv[1]) === scriptPath) {
  try {
    main();
  } catch (error) {
    process.stderr.write(`[publish] ${error instanceof Error ? error.message : String(error)}\n`);
    process.exit(1);
  }
}
