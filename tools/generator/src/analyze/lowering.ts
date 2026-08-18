import { readFileSync, readdirSync } from 'node:fs';
import path from 'node:path';
import ts from 'typescript';

import { portConfig } from '../../port.config.ts';
import { lowerTypeScriptSource } from '../lower/typescript.ts';
import type { LoweringDiagnostic } from '../model/ir.ts';

export interface PackageLoweringAudit {
  declarations: number;
  diagnostics: LoweringDiagnostic[];
  files: number;
  lowered: number;
  packageName: string;
}

export interface LoweringAudit {
  packages: PackageLoweringAudit[];
  schemaVersion: 1;
  summary: {
    declarations: number;
    diagnostics: number;
    files: number;
    lowered: number;
    packages: number;
  };
}

export function auditLowering(workspaceDirectory: string): LoweringAudit {
  const packagesDirectory = path.join(workspaceDirectory, 'upstream', 'packages');
  const packages = readdirSync(packagesDirectory, { withFileTypes: true })
    .filter((entry) => entry.isDirectory())
    .map((entry) => path.join(packagesDirectory, entry.name))
    .map((directory) => ({ directory, metadata: readPackageMetadata(directory) }))
    .filter(({ metadata }) => !isExcludedPackage(metadata.name))
    .sort((left, right) => left.metadata.name.localeCompare(right.metadata.name))
    .map(({ directory, metadata }) => auditPackage(directory, metadata.name, workspaceDirectory));

  return {
    packages,
    schemaVersion: 1,
    summary: {
      declarations: sum(packages, (item) => item.declarations),
      diagnostics: sum(packages, (item) => item.diagnostics.length),
      files: sum(packages, (item) => item.files),
      lowered: sum(packages, (item) => item.lowered),
      packages: packages.length,
    },
  };
}

export function assertCompleteLoweringAudit(audit: LoweringAudit): void {
  if (audit.summary.lowered === audit.summary.declarations && audit.summary.diagnostics === 0) return;

  const incomplete = audit.packages.filter((item) => item.lowered !== item.declarations || item.diagnostics.length > 0);
  const packages = incomplete
    .slice(0, 5)
    .map(
      (item) =>
        `${item.packageName} (${String(item.lowered)}/${String(item.declarations)} lowered, ${quantity(item.diagnostics.length, 'diagnostic')})`,
    )
    .join(', ');
  const remainder = incomplete.length > 5 ? `, and ${String(incomplete.length - 5)} more` : '';
  throw new Error(
    `Lowering coverage regression: ${String(audit.summary.lowered)}/${String(audit.summary.declarations)} declarations lowered with ${quantity(audit.summary.diagnostics, 'diagnostic')} across ${quantity(incomplete.length, 'package')}: ${packages}${remainder}`,
  );
}

function quantity(count: number, singular: string): string {
  return `${String(count)} ${singular}${count === 1 ? '' : 's'}`;
}

function isExcludedPackage(packageName: string): boolean {
  return portConfig.packagePolicy.some((rule) => {
    if (rule.disposition !== 'excluded') return false;
    const expression = rule.match
      .split('*')
      .map((part) => part.replace(/[\\^$.*+?()[\]{}|]/gu, '\\$&'))
      .join('.*');
    return new RegExp(`^${expression}$`, 'u').test(packageName);
  });
}

function auditPackage(directory: string, packageName: string, workspaceDirectory: string): PackageLoweringAudit {
  const sourceDirectory = path.join(directory, 'src');
  const files = walkTypeScriptSources(sourceDirectory);
  let declarations = 0;
  let lowered = 0;
  const diagnostics: LoweringDiagnostic[] = [];
  for (const file of files) {
    const source = ts.createSourceFile(
      file,
      readFileSync(file, 'utf8'),
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    declarations += source.statements.filter(isCandidateDeclaration).length;
    const result = lowerTypeScriptSource(source, packageName, workspaceDirectory);
    lowered += result.accountedDeclarations;
    diagnostics.push(...result.diagnostics);
  }
  diagnostics.sort(
    (left, right) => left.source.localeCompare(right.source) || left.line - right.line || left.column - right.column,
  );
  return { declarations, diagnostics, files: files.length, lowered, packageName };
}

function isCandidateDeclaration(statement: ts.Statement): boolean {
  return (
    ts.isClassDeclaration(statement) ||
    ts.isEnumDeclaration(statement) ||
    ts.isFunctionDeclaration(statement) ||
    ts.isInterfaceDeclaration(statement) ||
    ts.isModuleDeclaration(statement) ||
    ts.isTypeAliasDeclaration(statement) ||
    ts.isVariableStatement(statement)
  );
}

function readPackageMetadata(directory: string): { name: string } {
  return JSON.parse(readFileSync(path.join(directory, 'package.json'), 'utf8')) as { name: string };
}

function walkTypeScriptSources(directory: string): string[] {
  const files: string[] = [];
  for (const entry of readdirSync(directory, { withFileTypes: true })) {
    const file = path.join(directory, entry.name);
    if (entry.isDirectory()) files.push(...walkTypeScriptSources(file));
    else if (
      /\.tsx?$/u.test(entry.name) &&
      !/\.(?:test|spec)\.tsx?$/u.test(entry.name) &&
      !entry.name.endsWith('.d.ts')
    ) {
      files.push(file);
    }
  }
  return files.sort();
}

function sum<T>(items: T[], select: (item: T) => number): number {
  return items.reduce((total, item) => total + select(item), 0);
}
