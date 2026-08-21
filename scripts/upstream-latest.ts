// Reports what exists in Flight BEYOND the pinned `upstream/` submodule, without moving the pin.
//
// Generation is deliberately anchored to one upstream commit: every report, hash, and generated file
// describes that tree, and moving the pin is a decision with a regeneration attached. But planning
// needs the other question answered too — what has landed upstream since, and what is coming — and
// answering it by moving the pin to look around is how a pinned repository stops being pinned.
//
// So this reads the submodule's *remote-tracking refs* with plumbing only. It never checks anything
// out, never moves HEAD, and never touches the pin recorded in the parent repository. `--fetch`
// updates those refs from the network; without it the command is entirely offline and reports
// whatever was last fetched, which is the right default inside a sandbox.
//
// Usage:
//   tsx scripts/upstream-latest.ts              report against the refs already fetched
//   tsx scripts/upstream-latest.ts --fetch      refresh the remote-tracking refs first
//   tsx scripts/upstream-latest.ts --packages   also list every added/removed package

import { execFileSync } from 'node:child_process';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';

const scriptPath = fileURLToPath(import.meta.url);
const repositoryRoot = join(dirnameOf(scriptPath), '..');
const upstream = join(repositoryRoot, 'upstream');

/** The refs worth comparing against, in the order Flight promotes through them. */
const CHANNELS = ['origin/main', 'origin/develop'] as const;

export interface ChannelReport {
  ahead: number;
  behind: number;
  packagesAdded: string[];
  packagesRemoved: string[];
  ref: string;
  sha: string;
  subject: string;
}

export function packagesAt(reference: string, repository: string = upstream): string[] {
  // `ls-tree` on a ref reads the tree directly — no checkout, no working-tree change.
  return git(repository, 'ls-tree', '--name-only', reference, 'packages/')
    .split('\n')
    .map((line) => line.replace(/^packages\//u, '').trim())
    .filter(Boolean)
    .sort();
}

export function compareChannel(reference: string, repository: string = upstream): ChannelReport {
  const pinned = new Set(packagesAt('HEAD', repository));
  const latest = packagesAt(reference, repository);
  const latestSet = new Set(latest);

  return {
    ahead: Number(git(repository, 'rev-list', '--count', `HEAD..${reference}`)),
    behind: Number(git(repository, 'rev-list', '--count', `${reference}..HEAD`)),
    packagesAdded: latest.filter((name) => !pinned.has(name)),
    packagesRemoved: [...pinned].filter((name) => !latestSet.has(name)).sort(),
    ref: reference,
    sha: git(repository, 'rev-parse', '--short', reference),
    subject: git(repository, 'log', '--format=%s', '-1', reference),
  };
}

function main(): void {
  const argv = process.argv.slice(2);
  const withPackages = argv.includes('--packages');
  if (argv.includes('--fetch')) {
    process.stdout.write('[upstream] fetching remote-tracking refs (the pin is not moved)\n');
    execFileSync('git', ['-C', upstream, 'fetch', '--tags', '--force', 'origin'], { stdio: 'inherit' });
  }

  const pinSha = git(upstream, 'rev-parse', '--short', 'HEAD');
  process.stdout.write(`pinned  ${pinSha}  ${git(upstream, 'log', '--format=%s', '-1', 'HEAD')}\n`);

  for (const reference of CHANNELS) {
    let report: ChannelReport;
    try {
      report = compareChannel(reference);
    } catch {
      process.stdout.write(`${reference.padEnd(16)} not fetched — run with --fetch\n`);
      continue;
    }

    process.stdout.write(
      `${report.ref.padEnd(16)}${report.sha}  +${report.ahead} ahead / -${report.behind} behind  ${report.subject}\n`,
    );
    if (report.packagesAdded.length > 0) {
      process.stdout.write(
        `  added:   ${withPackages ? report.packagesAdded.join(', ') : report.packagesAdded.length}\n`,
      );
    }
    if (report.packagesRemoved.length > 0) {
      process.stdout.write(
        `  removed: ${withPackages ? report.packagesRemoved.join(', ') : report.packagesRemoved.length}\n`,
      );
    }
  }
}

function git(repository: string, ...arguments_: readonly string[]): string {
  return execFileSync('git', arguments_, { cwd: repository, encoding: 'utf8' }).trim();
}

function dirnameOf(file: string): string {
  return file.slice(0, Math.max(0, file.lastIndexOf('/')));
}

if (process.argv[1] !== undefined && join(process.argv[1]) === scriptPath) {
  try {
    main();
  } catch (error) {
    process.stderr.write(`[upstream] ${error instanceof Error ? error.message : String(error)}\n`);
    process.exit(1);
  }
}
