import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { existsSync, mkdirSync, readFileSync, readdirSync, rmSync } from 'node:fs';
import path from 'node:path';
import ts from 'typescript';

import { portConfig, type RustTarget, type WasmFacadeTarget } from '../../port.config.ts';
import { sourcePathToRustModule } from '../analyze/inventory.ts';
import { lowerTypeScriptSource } from '../lower/typescript.ts';
import type { IrFunctionDeclaration, IrType, LoweringDiagnostic } from '../model/ir.ts';
import { RustEmissionError, emitRustModule, type RustImport } from './rust.ts';
import { stableJson, writeOrCheck } from './reports.ts';

export interface RustGenerationReport {
  blessedFacades: typeof portConfig.blessedFacades;
  schemaVersion: 2;
  targets: RustTargetReport[];
  upstreamCommit: string;
  wasmFacades: WasmFacadeReport[];
}

export interface WasmFacadeReport {
  coreCrate: string;
  crate: string;
  exports: string[];
  output: string;
  outputSha256: string;
  template: string;
  templateSha256: string;
}

export interface RustTargetReport {
  crate: string;
  deferredDeclarations: Array<{
    fingerprint: string;
    name: string;
    reason: string;
    source: string;
  }>;
  deferredSources: Array<{
    fingerprint: string;
    reason: string;
    source: string;
  }>;
  emittedSources: Array<{
    declarationNames: string[];
    declarations: number;
    output: string;
    outputSha256: string;
    source: string;
  }>;
  inlineDependencies: RustTarget['inlineDependencies'];
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
  const wasmFacades = portConfig.wasmFacades.map((facade) =>
    generateWasmFacade(workspaceDirectory, facade, targets, check),
  );
  const report: RustGenerationReport = {
    blessedFacades: portConfig.blessedFacades,
    schemaVersion: 2,
    targets,
    upstreamCommit,
    wasmFacades,
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
  const selectedSources = target.sourceSelection ? new Set(target.sourceSelection.sources) : undefined;
  const usedSelections = new Set<string>();
  const deferredSources: RustTargetReport['deferredSources'] = [];
  const deferredDeclarations: RustTargetReport['deferredDeclarations'] = [];
  const sourceExclusions: RustTargetReport['sourceExclusions'] = [];
  const unsupportedSources: RustTargetReport['unsupportedSources'] = [];
  const emittedSources: RustTargetReport['emittedSources'] = [];
  const modules: string[] = [];
  const outputs: PendingOutput[] = [];
  const semanticTypes = collectSemanticTypes(workspaceDirectory, target);
  const inlineFunctions = collectInlineFunctions(workspaceDirectory, target);

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
    if (selectedSources && !selectedSources.has(sourceName)) {
      deferredSources.push({
        fingerprint: sha256(readFileSync(file, 'utf8')),
        reason: target.sourceSelection!.reason,
        source: relative(workspaceDirectory, file),
      });
      continue;
    }
    usedSelections.add(sourceName);
    const moduleName = sourcePathToRustModule(file);
    if (!moduleName) continue;
    const sourceText = readFileSync(file, 'utf8');
    const sourceFile = ts.createSourceFile(file, sourceText, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
    const lowered = lowerTypeScriptSource(sourceFile, target.package, workspaceDirectory);
    const declarationSelection = target.declarationSelection?.[sourceName];
    const selectedDeclarations = declarationSelection ? new Set(declarationSelection.names) : undefined;
    const declarations = selectedDeclarations
      ? lowered.declarations.filter((declaration) => selectedDeclarations.has(declaration.name))
      : lowered.declarations;
    if (selectedDeclarations) {
      for (const declaration of lowered.declarations) {
        if (selectedDeclarations.has(declaration.name)) continue;
        deferredDeclarations.push({
          fingerprint: declaration.origin.fingerprint,
          name: declaration.name,
          reason: declarationSelection!.reason,
          source: declaration.origin.source,
        });
      }
      const missing = [...selectedDeclarations].filter(
        (name) => !declarations.some((declaration) => declaration.name === name),
      );
      if (missing.length > 0) {
        throw new Error(`Stale ${target.package} declaration selections in ${sourceName}: ${missing.join(', ')}`);
      }
    }
    const importedSemanticTypes = collectImportedSemanticTypes(sourceFile, workspaceDirectory);
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
          declarations,
          imports: collectRustImports(
            sourceFile,
            target,
            target.package === '@flighthq/types' ? Object.keys(importedSemanticTypes) : [],
          ),
          inlineFunctions,
          semanticTypes: {
            ...semanticTypes,
            ...importedSemanticTypes,
          },
          source: relative(workspaceDirectory, file),
          typeImports: [],
        }),
        file,
      );
      const outputFile = path.join(crateSourceDirectory, `${moduleName}.rs`);
      outputs.push({ content: emitted, file: outputFile });
      modules.push(moduleName);
      emittedSources.push({
        declarationNames: declarations.map((declaration) => declaration.name).sort(),
        declarations: declarations.length,
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
  const staleSelections = selectedSources ? [...selectedSources].filter((source) => !usedSelections.has(source)) : [];
  if (staleSelections.length > 0) {
    throw new Error(`Stale ${target.package} source selections: ${staleSelections.join(', ')}`);
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
  deferredDeclarations.sort((left, right) =>
    left.source === right.source ? left.name.localeCompare(right.name) : left.source.localeCompare(right.source),
  );
  deferredSources.sort((left, right) => left.source.localeCompare(right.source));
  sourceExclusions.sort((left, right) => left.source.localeCompare(right.source));
  unsupportedSources.sort((left, right) => left.source.localeCompare(right.source));
  return {
    crate: target.crate,
    deferredDeclarations,
    deferredSources,
    emittedSources,
    inlineDependencies: target.inlineDependencies,
    package: target.package,
    sourceExclusions,
    unsupportedSources,
    typeMappings: target.typeMappings,
  };
}

function generateWasmFacade(
  workspaceDirectory: string,
  facade: WasmFacadeTarget,
  targets: RustTargetReport[],
  check: boolean,
): WasmFacadeReport {
  const core = targets.find((target) => target.crate === facade.coreCrate);
  if (!core) throw new Error(`Wasm facade ${facade.crate} references missing core crate ${facade.coreCrate}`);
  const generatedDeclarations = new Set(core.emittedSources.flatMap((source) => source.declarationNames));
  const missing = facade.exports.filter((name) => !generatedDeclarations.has(name));
  if (missing.length > 0) {
    throw new Error(`Wasm facade ${facade.crate} references deferred core exports: ${missing.join(', ')}`);
  }

  const crateDirectory = path.join(workspaceDirectory, portConfig.generatedDirectory, 'crates', facade.crate);
  if (!check) rmSync(crateDirectory, { force: true, recursive: true });
  const sourceFile = path.join(crateDirectory, 'src', 'lib.rs');
  const manifestFile = path.join(crateDirectory, 'Cargo.toml');
  const templateFile = path.join(workspaceDirectory, facade.rustTemplate);
  const template = readFileSync(templateFile, 'utf8');
  const source = formatRust(template, templateFile);
  const manifest = [
    '[package]',
    `name = "${facade.crate}"`,
    'version = "0.1.0"',
    'edition = "2024"',
    'license = "MIT"',
    'publish = false',
    '',
    '[lib]',
    'crate-type = ["cdylib", "rlib"]',
    'path = "src/lib.rs"',
    '',
    '[dependencies]',
    `${facade.coreCrate} = { path = "../${facade.coreCrate}" }`,
    'flighthq-types = { path = "../flighthq-types" }',
    'wasm-bindgen = "0.2"',
    '',
  ].join('\n');
  const outputs = [
    { content: manifest, file: manifestFile },
    { content: source, file: sourceFile },
  ];
  for (const output of outputs) {
    mkdirSync(path.dirname(output.file), { recursive: true });
    writeOrCheck(output.file, output.content, check);
  }
  verifyNoStaleOutputs(crateDirectory, new Set(outputs.map((output) => output.file)), check);
  return {
    coreCrate: facade.coreCrate,
    crate: facade.crate,
    exports: [...facade.exports].sort(),
    output: relative(workspaceDirectory, sourceFile),
    outputSha256: sha256(source),
    template: facade.rustTemplate,
    templateSha256: sha256(template),
  };
}

function collectInlineFunctions(workspaceDirectory: string, target: RustTarget): IrFunctionDeclaration[] {
  return Object.entries(target.inlineDependencies ?? {}).map(([name, mapping]) => {
    const source = path.join(workspaceDirectory, mapping.source);
    const sourceFile = ts.createSourceFile(
      source,
      readFileSync(source, 'utf8'),
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(sourceFile, mapping.package, workspaceDirectory);
    const declaration = lowered.declarations.find(
      (item): item is IrFunctionDeclaration => item.kind === 'function' && item.name === name,
    );
    if (!declaration) {
      throw new Error(`Inline dependency ${name} did not resolve from ${mapping.source}`);
    }
    return declaration;
  });
}

function collectSemanticTypes(workspaceDirectory: string, target: RustTarget): Readonly<Record<string, IrType>> {
  return Object.fromEntries(
    Object.entries(target.typeMappings).map(([name, mapping]) => {
      const source = path.join(workspaceDirectory, mapping.source);
      const sourceFile = ts.createSourceFile(
        source,
        readFileSync(source, 'utf8'),
        ts.ScriptTarget.Latest,
        true,
        ts.ScriptKind.TS,
      );
      const lowered = lowerTypeScriptSource(sourceFile, '@flighthq/types', workspaceDirectory);
      const declaration = lowered.declarations.find((item) => item.kind === 'type' && item.name === name);
      if (!declaration || declaration.kind !== 'type') {
        throw new Error(`Semantic type mapping ${name} did not resolve from ${mapping.source}`);
      }
      return [name, declaration.type];
    }),
  );
}

function emitCargoManifest(target: RustTarget): string {
  const dependencies = Object.values(target.dependencies)
    .sort((left, right) => left.crate.localeCompare(right.crate))
    .map(({ crate }) => `${crate} = { path = "../${crate}" }`);
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
    ...(dependencies.length > 0 ? ['[dependencies]', ...dependencies, ''] : []),
  ].join('\n');
}

function emitLibrary(target: RustTarget, modules: string[]): string {
  const aliases = Object.entries(target.typeMappings)
    .sort(([left], [right]) => left.localeCompare(right))
    .map(
      ([name, mapping]) =>
        `/// Generated semantic mapping for ${mapping.source}.\n/// ${mapping.reason}\n${mapping.rustDefinition ?? `pub type ${name} = ${mapping.rust};`}`,
    );
  const declarations = modules.sort().map((moduleName) => `mod ${moduleName};\npub use ${moduleName}::*;`);
  return [
    '// @generated by tools/generator; do not edit.',
    '#![forbid(unsafe_code)]',
    '',
    '/// Opaque ownership token for host-only TypeScript values that cannot cross the generated Rust boundary.',
    '#[derive(Clone, Default)]',
    'pub struct OpaqueHostValue;',
    '',
    ...aliases.flatMap((alias) => [alias, '']),
    ...declarations,
    '',
  ].join('\n');
}

function collectRustImports(
  sourceFile: ts.SourceFile,
  target: RustTarget,
  additionalCrateTypes: string[],
): RustImport[] {
  const groups = new Map<string, RustImport['names']>();
  for (const statement of sourceFile.statements) {
    if (!ts.isImportDeclaration(statement) || !ts.isStringLiteral(statement.moduleSpecifier)) continue;
    const specifier = statement.moduleSpecifier.text;
    const dependency = target.dependencies[specifier];
    const module = specifier.startsWith('.') ? 'crate' : dependency?.crate.replaceAll('-', '_');
    if (!module) continue;
    const bindings = statement.importClause?.namedBindings;
    if (!bindings || !ts.isNamedImports(bindings)) continue;
    const names = groups.get(module) ?? [];
    names.push(
      ...bindings.elements.map((element) => ({
        imported: element.propertyName?.text ?? element.name.text,
        local: element.name.text,
      })),
    );
    groups.set(module, names);
  }
  if (additionalCrateTypes.length > 0) {
    const names = groups.get('crate') ?? [];
    names.push(...additionalCrateTypes.map((name) => ({ imported: name, local: name })));
    groups.set('crate', names);
  }
  return [...groups].map(([module, names]) => ({ module, names }));
}

function collectImportedSemanticTypes(
  sourceFile: ts.SourceFile,
  workspaceDirectory: string,
): Readonly<Record<string, IrType>> {
  const types = new Map<string, IrType>();
  const visited = new Set<string>();
  const visit = (file: ts.SourceFile): void => {
    if (visited.has(file.fileName)) return;
    visited.add(file.fileName);
    for (const statement of file.statements) {
      if (!ts.isImportDeclaration(statement) || !ts.isStringLiteral(statement.moduleSpecifier)) continue;
      const bindings = statement.importClause?.namedBindings;
      if (!bindings || !ts.isNamedImports(bindings)) continue;
      const specifier = statement.moduleSpecifier.text;
      for (const element of bindings.elements) {
        const name = element.propertyName?.text ?? element.name.text;
        const source =
          specifier === '@flighthq/types'
            ? path.join(workspaceDirectory, portConfig.upstreamDirectory, 'packages', 'types', 'src', `${name}.ts`)
            : specifier.startsWith('.')
              ? path.resolve(path.dirname(file.fileName), `${specifier}.ts`)
              : undefined;
        if (!source || !existsSync(source)) continue;
        const semanticSource = ts.createSourceFile(
          source,
          readFileSync(source, 'utf8'),
          ts.ScriptTarget.Latest,
          true,
          ts.ScriptKind.TS,
        );
        const lowered = lowerTypeScriptSource(semanticSource, '@flighthq/types', workspaceDirectory);
        const declaration = lowered.declarations.find((item) => item.kind === 'type' && item.name === name);
        if (declaration?.kind === 'type') types.set(name, declaration.type);
        visit(semanticSource);
      }
    }
  };
  visit(sourceFile);
  return Object.fromEntries(types);
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
