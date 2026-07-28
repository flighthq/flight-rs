import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { existsSync, mkdirSync, readFileSync, readdirSync, rmSync } from 'node:fs';
import path from 'node:path';
import ts from 'typescript';

import { portConfig, type RustTarget } from '../../port.config.ts';
import { sourcePathToRustModule } from '../analyze/inventory.ts';
import { lowerTypeScriptSource } from '../lower/typescript.ts';
import type { LoweringDiagnostic } from '../model/ir.ts';
import { RustEmissionError, emitRustModule } from './rust.ts';
import { stableJson, writeOrCheck } from './reports.ts';

export interface RustGenerationReport {
  blessedFacades: typeof portConfig.blessedFacades;
  schemaVersion: 1;
  targets: RustTargetReport[];
  upstreamCommit: string;
}

export interface RustTargetReport {
  crate: string;
  emittedSources: Array<{
    declarations: number;
    output: string;
    outputSha256: string;
    source: string;
  }>;
  package: string;
  sourceExclusions: Array<{
    fingerprint: string;
    reason: string;
    source: string;
  }>;
  unsupportedSources: Array<{
    diagnostics: LoweringDiagnostic[];
    reason: string;
    source: string;
  }>;
  typeMappings: RustTarget['typeMappings'];
}

interface PendingOutput {
  content: string;
  file: string;
}

export function generateRust(workspaceDirectory: string, check: boolean, upstreamCommit: string): RustGenerationReport {
  const targets = portConfig.targets.map((target) => generateTarget(workspaceDirectory, target, check));
  const report: RustGenerationReport = {
    blessedFacades: portConfig.blessedFacades,
    schemaVersion: 1,
    targets,
    upstreamCommit,
  };
  const generatedDirectory = path.join(workspaceDirectory, portConfig.generatedDirectory);
  mkdirSync(generatedDirectory, { recursive: true });
  writeOrCheck(path.join(generatedDirectory, 'manifest.json'), stableJson(report), check);
  return report;
}

function generateTarget(workspaceDirectory: string, target: RustTarget, check: boolean): RustTargetReport {
  const packageDirectoryName = target.package.replace(/^@flighthq\//u, '');
  const sourceDirectory = path.join(
    workspaceDirectory,
    portConfig.upstreamDirectory,
    'packages',
    packageDirectoryName,
    'src',
  );
  const crateDirectory = path.join(workspaceDirectory, portConfig.generatedDirectory, 'crates', target.crate);
  const crateSourceDirectory = path.join(crateDirectory, 'src');
  if (!check) rmSync(crateDirectory, { force: true, recursive: true });
  mkdirSync(crateSourceDirectory, { recursive: true });

  const exclusions = new Map(target.sourceExclusions.map((item) => [item.source, item]));
  const usedExclusions = new Set<string>();
  const sourceExclusions: RustTargetReport['sourceExclusions'] = [];
  const unsupportedSources: RustTargetReport['unsupportedSources'] = [];
  const emittedSources: RustTargetReport['emittedSources'] = [];
  const modules: string[] = [];
  const outputs: PendingOutput[] = [];

  for (const file of walkTypeScriptSources(sourceDirectory)) {
    const sourceName = path.relative(sourceDirectory, file).split(path.sep).join('/');
    const exclusion = exclusions.get(sourceName);
    if (exclusion) {
      usedExclusions.add(sourceName);
      const content = readFileSync(file, 'utf8');
      sourceExclusions.push({
        fingerprint: sha256(content),
        reason: exclusion.reason,
        source: relative(workspaceDirectory, file),
      });
      continue;
    }
    const moduleName = sourcePathToRustModule(file);
    if (!moduleName) continue;
    const sourceText = readFileSync(file, 'utf8');
    const sourceFile = ts.createSourceFile(file, sourceText, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
    const lowered = lowerTypeScriptSource(sourceFile, target.package, workspaceDirectory);
    if (lowered.diagnostics.length > 0) {
      unsupportedSources.push({
        diagnostics: lowered.diagnostics,
        reason: 'TypeScript lowering produced diagnostics.',
        source: relative(workspaceDirectory, file),
      });
      continue;
    }
    try {
      const emitted = formatRust(
        emitRustModule({
          declarations: lowered.declarations,
          source: relative(workspaceDirectory, file),
          typeImports: collectFlightTypeImports(sourceFile),
        }),
        file,
      );
      const outputFile = path.join(crateSourceDirectory, `${moduleName}.rs`);
      outputs.push({ content: emitted, file: outputFile });
      modules.push(moduleName);
      emittedSources.push({
        declarations: lowered.declarations.length,
        output: relative(workspaceDirectory, outputFile),
        outputSha256: sha256(emitted),
        source: relative(workspaceDirectory, file),
      });
    } catch (error) {
      if (!(error instanceof RustEmissionError)) throw error;
      unsupportedSources.push({
        diagnostics: [],
        reason: error.message,
        source: relative(workspaceDirectory, file),
      });
    }
  }

  const staleExclusions = [...exclusions.keys()].filter((source) => !usedExclusions.has(source));
  if (staleExclusions.length > 0) {
    throw new Error(`Stale ${target.package} source exclusions: ${staleExclusions.join(', ')}`);
  }

  const cargoManifest = emitCargoManifest(target);
  const library = formatRust(emitLibrary(target, modules), path.join(crateSourceDirectory, 'lib.rs'));
  outputs.push(
    { content: cargoManifest, file: path.join(crateDirectory, 'Cargo.toml') },
    { content: library, file: path.join(crateSourceDirectory, 'lib.rs') },
  );
  if (target.conformanceTemplate) {
    const template = path.join(workspaceDirectory, target.conformanceTemplate);
    const output = path.join(crateDirectory, 'tests', 'conformance.rs');
    outputs.push({ content: formatRust(readFileSync(template, 'utf8'), template), file: output });
  }
  for (const output of outputs) {
    mkdirSync(path.dirname(output.file), { recursive: true });
    writeOrCheck(output.file, output.content, check);
  }
  verifyNoStaleOutputs(crateDirectory, new Set(outputs.map((output) => output.file)), check);

  emittedSources.sort((left, right) => left.source.localeCompare(right.source));
  sourceExclusions.sort((left, right) => left.source.localeCompare(right.source));
  unsupportedSources.sort((left, right) => left.source.localeCompare(right.source));
  return {
    crate: target.crate,
    emittedSources,
    package: target.package,
    sourceExclusions,
    unsupportedSources,
    typeMappings: target.typeMappings,
  };
}

function emitCargoManifest(target: RustTarget): string {
  return [
    '[package]',
    `name = "${target.crate}"`,
    'version = "0.1.0"',
    'edition = "2024"',
    'license = "MIT"',
    'publish = false',
    '',
    '[lib]',
    'path = "src/lib.rs"',
    '',
  ].join('\n');
}

function emitLibrary(target: RustTarget, modules: string[]): string {
  const aliases = Object.entries(target.typeMappings)
    .sort(([left], [right]) => left.localeCompare(right))
    .map(
      ([name, mapping]) =>
        `/// Generated semantic mapping for ${mapping.source}.\n/// ${mapping.reason}\npub type ${name} = ${mapping.rust};`,
    );
  const declarations = modules.sort().map((moduleName) => `mod ${moduleName};\npub use ${moduleName}::*;`);
  return [
    '// @generated by tools/generator; do not edit.',
    '#![forbid(unsafe_code)]',
    '',
    ...aliases.flatMap((alias) => [alias, '']),
    ...declarations,
    '',
  ].join('\n');
}

function collectFlightTypeImports(sourceFile: ts.SourceFile): string[] {
  const imports: string[] = [];
  for (const statement of sourceFile.statements) {
    if (
      !ts.isImportDeclaration(statement) ||
      !ts.isStringLiteral(statement.moduleSpecifier) ||
      statement.moduleSpecifier.text !== '@flighthq/types'
    ) {
      continue;
    }
    const bindings = statement.importClause?.namedBindings;
    if (!bindings || !ts.isNamedImports(bindings)) continue;
    imports.push(...bindings.elements.map((element) => element.propertyName?.text ?? element.name.text));
  }
  return imports;
}

function formatRust(content: string, source: string): string {
  try {
    return execFileSync('rustfmt', ['--emit', 'stdout', '--edition', '2024'], {
      encoding: 'utf8',
      input: content,
      stdio: ['pipe', 'pipe', 'pipe'],
    });
  } catch (error) {
    const stderr =
      error && typeof error === 'object' && 'stderr' in error && typeof error.stderr === 'string'
        ? error.stderr.trim()
        : String(error);
    throw new RustEmissionError(`${relative(process.cwd(), source)}: rustfmt rejected generated Rust: ${stderr}`);
  }
}

function verifyNoStaleOutputs(crateDirectory: string, expected: ReadonlySet<string>, check: boolean): void {
  if (!check) return;
  const actual = walkFiles(crateDirectory);
  const stale = actual.filter((file) => !expected.has(file));
  if (stale.length > 0) {
    throw new Error(`Stale generated Rust output: ${stale.map((file) => relative(process.cwd(), file)).join(', ')}`);
  }
}

function walkTypeScriptSources(directory: string): string[] {
  return walkFiles(directory).filter(
    (file) => /\.tsx?$/u.test(file) && !/\.(?:test|spec)\.tsx?$/u.test(file) && !file.endsWith('.d.ts'),
  );
}

function walkFiles(directory: string): string[] {
  if (!existsSync(directory)) return [];
  const files: string[] = [];
  for (const entry of readdirSync(directory, { withFileTypes: true })) {
    const file = path.join(directory, entry.name);
    if (entry.isDirectory()) files.push(...walkFiles(file));
    else if (entry.isFile()) files.push(file);
  }
  return files.sort();
}

function sha256(content: string): string {
  return `sha256:${createHash('sha256').update(content).digest('hex')}`;
}

function relative(workspaceDirectory: string, file: string): string {
  return path.relative(workspaceDirectory, file).split(path.sep).join('/');
}
