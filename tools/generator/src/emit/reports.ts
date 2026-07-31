import { existsSync, readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';

import type { ApiReport, UpstreamInventory } from '../model/inventory.ts';
import type { LoweringAudit } from '../analyze/lowering.ts';
import type { RustGenerationReport } from './core.ts';

export function createApiReport(inventory: UpstreamInventory): ApiReport {
  return {
    packages: inventory.packages.map((item) => ({
      exports: item.exports,
      rustCrate: item.rustCrate,
      name: item.name,
      sdkIncluded: item.sdkIncluded,
    })),
    schemaVersion: 1,
    upstreamCommit: inventory.upstreamCommit,
  };
}

export function inventorySummary(inventory: UpstreamInventory): string {
  const lines = [
    '# Upstream Inventory',
    '',
    `Upstream commit: \`${inventory.upstreamCommit}\``,
    '',
    '| Metric | Count |',
    '| --- | ---: |',
    `| Packages | ${inventory.summary.packages} |`,
    `| Source files | ${inventory.summary.sourceFiles} |`,
    `| Test files | ${inventory.summary.testFiles} |`,
    `| Public exports | ${inventory.summary.exports} |`,
    `| Export conflicts | ${inventory.summary.exportConflicts} |`,
    '',
    '| Upstream package | Rust crate | Sources | Tests | Exports | SDK | Conflicts |',
    '| --- | --- | ---: | ---: | ---: | :---: | ---: |',
  ];
  for (const item of inventory.packages) {
    lines.push(
      `| \`${item.name}\` | \`${item.rustCrate}\` | ${item.sourceFiles} | ${item.testFiles} | ${item.exports.length} | ${item.sdkIncluded ? 'yes' : 'no'} | ${item.exportConflicts.length} |`,
    );
  }
  lines.push('');
  return lines.join('\n');
}

export function loweringSummary(audit: LoweringAudit): string {
  const lines = [
    '# Lowering Audit',
    '',
    '| Metric | Count |',
    '| --- | ---: |',
    `| Packages | ${audit.summary.packages} |`,
    `| Source files | ${audit.summary.files} |`,
    `| Candidate declarations | ${audit.summary.declarations} |`,
    `| Lowered declarations | ${audit.summary.lowered} |`,
    `| Current diagnostics | ${audit.summary.diagnostics} |`,
    '',
    '| Package | Declarations | Lowered | Diagnostics |',
    '| --- | ---: | ---: | ---: |',
  ];
  for (const item of audit.packages) {
    lines.push(`| \`${item.packageName}\` | ${item.declarations} | ${item.lowered} | ${item.diagnostics.length} |`);
  }
  lines.push('');
  return lines.join('\n');
}

export function generationSummary(report: RustGenerationReport): string {
  const lines = [
    '# Automatic Rust Generation',
    '',
    `Upstream commit: \`${report.upstreamCommit}\``,
    '',
    '| Metric | Count |',
    '| --- | ---: |',
    `| Inventoried packages | ${report.summary.packages} |`,
    `| Default-generated packages | ${report.summary.eligible} |`,
    `| Emittable packages | ${report.summary.emittable} |`,
    `| Blocked packages | ${report.summary.blocked} |`,
    `| Compiled candidates | ${report.summary.candidateCompiled} |`,
    `| Compile-blocked candidates | ${report.summary.candidateCompileBlocked} |`,
    `| Dependency-blocked candidates | ${report.summary.candidateDependencyBlocked} |`,
    `| Cultivated packages | ${report.summary.cultivated} |`,
    `| Host-bound packages | ${report.summary.hostBound} |`,
    `| Excluded packages | ${report.summary.excluded} |`,
    `| Source/package blockers | ${report.summary.sourceBlockers} |`,
    `| Eligible async scopes | ${report.asyncTasks.summary.eligibleScopes} |`,
    `| Portable executable async scopes | ${report.asyncTasks.summary.portableExecutableScopes} |`,
    `| Host-placeholder async scopes | ${report.asyncTasks.summary.hostPlaceholderScopes} |`,
    `| Unsupported async scopes | ${report.asyncTasks.summary.unsupportedScopes} |`,
    `| Async scopes matching the legacy body-erasure path | ${report.asyncTasks.summary.legacyErasurePathScopes} |`,
    `| Upstream conformance files translated and passing | ${report.conformance.summary.passingTestFiles}/${report.conformance.summary.totalUpstreamTestFiles} |`,
    `| Generated conformance cases passing | ${report.conformance.summary.passingCases}/${report.conformance.summary.translatedCases} |`,
    '',
    '| Package | Disposition | Status | Candidate | Sources emitted/attempted | API generated/expected | Missing | Dependents direct/transitive | Opaque sources | Blockers | Target |',
    '| --- | --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |',
  ];
  for (const item of report.automaticPackages) {
    const opaqueSources = item.emittedSources.filter((source) => source.usesOpaqueHostValues).length;
    lines.push(
      `| \`${item.package}\` | ${item.disposition} | ${item.status} | ${item.candidate.status} | ${item.emittedSources.length}/${item.attemptedSources} | ${item.generatedExports.length}/${item.apiExports} | ${item.missingExports.length} | ${item.directDependents}/${item.transitiveDependents} | ${opaqueSources} | ${item.blockers.length} | ${item.fullyPromotedTarget ? 'full' : item.promotedTarget ? 'partial' : 'no'} |`,
    );
  }
  lines.push(
    '',
    '## Async tasks',
    '',
    `Disposition partition: ${String(report.asyncTasks.summary.eligibleScopes)} eligible = ${String(report.asyncTasks.summary.portableExecutableScopes)} portable executable + ${String(report.asyncTasks.summary.hostPlaceholderScopes)} host placeholder + ${String(report.asyncTasks.summary.unsupportedScopes)} unsupported.`,
    '',
    '| Operation | Count |',
    '| --- | ---: |',
    `| Await expressions | ${report.asyncTasks.summary.operations.awaits} |`,
    `| Async iterations | ${report.asyncTasks.summary.operations.asyncIterations} |`,
    `| Promise.all | ${report.asyncTasks.summary.operations.promiseAll} |`,
    `| Promise.allSettled | ${report.asyncTasks.summary.operations.promiseAllSettled} |`,
    `| Promise.resolve | ${report.asyncTasks.summary.operations.promiseResolve} |`,
    `| Promise.reject | ${report.asyncTasks.summary.operations.promiseReject} |`,
    `| Promise.then | ${report.asyncTasks.summary.operations.promiseThen} |`,
    `| Promise.catch | ${report.asyncTasks.summary.operations.promiseCatch} |`,
    `| Promise.finally | ${report.asyncTasks.summary.operations.promiseFinally} |`,
    `| Void expressions | ${report.asyncTasks.summary.operations.voidExpressions} |`,
    '',
    '| Package | Eligible | Portable executable | Host placeholder | Unsupported | Legacy erasure path |',
    '| --- | ---: | ---: | ---: | ---: | ---: |',
  );
  for (const item of report.asyncTasks.packages) {
    lines.push(
      `| \`${item.package}\` | ${item.summary.eligibleScopes} | ${item.summary.portableExecutableScopes} | ${item.summary.hostPlaceholderScopes} | ${item.summary.unsupportedScopes} | ${item.summary.legacyErasurePathScopes} |`,
    );
  }
  lines.push('', '### Unsupported async scopes', '');
  for (const item of report.asyncTasks.packages) {
    for (const scope of item.scopes.filter((candidate) => candidate.disposition === 'unsupported')) {
      lines.push(
        `- \`${scope.package}\` \`${scope.source}:${String(scope.line)}:${String(scope.column)}\` \`${scope.lexicalPath}\` (${scope.fingerprint}): ${scope.reason ?? 'Unspecified async task blocker.'}${scope.matchesLegacyErasurePath ? ' Matched the legacy body-erasure path.' : ''}`,
      );
    }
  }
  lines.push(
    '',
    '## Generated upstream conformance',
    '',
    '| Package | Files translated/passing/in scope | Cases translated/passing | Unsupported files |',
    '| --- | ---: | ---: | ---: |',
  );
  for (const item of report.conformance.packages) {
    lines.push(
      `| \`${item.package}\` | ${item.translatedTestFiles}/${item.passingTestFiles}/${item.testFiles.length} | ${item.translatedCases}/${item.passingCases} | ${item.unsupportedTestFiles} |`,
    );
  }
  lines.push('', '### Unsupported in-scope upstream test files', '');
  for (const item of report.conformance.packages) {
    for (const file of item.testFiles.filter((candidate) => candidate.status !== 'translated')) {
      const reasons = file.unsupported
        .map((unsupported) =>
          `${unsupported.test ? `${unsupported.test}: ` : ''}${unsupported.reason}`.replace(/\s+/gu, ' '),
        )
        .join('; ');
      lines.push(`- \`${file.source}\` (${String(file.translatedCases)}/${String(file.testCases)} cases): ${reasons}`);
    }
  }
  lines.push('', '## Blockers', '');
  for (const item of report.automaticPackages.filter((candidate) => candidate.blockers.length > 0)) {
    lines.push(`### \`${item.package}\``, '');
    for (const blocker of item.blockers) {
      lines.push(`- **${blocker.stage}** \`${blocker.source}\`: ${blocker.reason.replace(/\s+/gu, ' ')}`);
    }
    lines.push('');
  }
  lines.push('## Candidate compile blockers', '');
  for (const item of report.automaticPackages.filter(
    (candidate) => candidate.candidate.compileDiagnostics.length > 0,
  )) {
    lines.push(`### \`${item.package}\``, '');
    for (const diagnostic of item.candidate.compileDiagnostics) {
      const location = diagnostic.source ? ` \`${diagnostic.source}\`` : '';
      const code = diagnostic.code ? ` **${diagnostic.code}**` : '';
      lines.push(`-${code}${location}: ${diagnostic.message}`);
    }
    lines.push('');
  }
  return lines.join('\n');
}

export function stableJson(value: unknown): string {
  return `${JSON.stringify(value, undefined, 2)}\n`;
}

export function writeOrCheck(file: string, content: string, check: boolean): void {
  const normalized = content.replace(/\r\n/gu, '\n');
  if (check) {
    if (!existsSync(file)) throw new Error(`Generated report is missing: ${path.relative(process.cwd(), file)}`);
    const current = readFileSync(file, 'utf8').replace(/\r\n/gu, '\n');
    if (current !== normalized) throw new Error(`Generated report is stale: ${path.relative(process.cwd(), file)}`);
    return;
  }
  writeFileSync(file, normalized);
}
