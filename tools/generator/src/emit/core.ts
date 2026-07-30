import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { existsSync, mkdirSync, readFileSync, readdirSync, rmSync } from 'node:fs';
import path from 'node:path';
import ts from 'typescript';

import {
  portConfig,
  type PackageDisposition,
  type PackagePolicyRule,
  type RustTarget,
  type WasmFacadeTarget,
} from '../../port.config.ts';
import { sourcePathToImplementationModule, sourcePathToRustModule } from '../analyze/inventory.ts';
import { lowerTypeScriptSource } from '../lower/typescript.ts';
import type { PackageInventory, UpstreamInventory } from '../model/inventory.ts';
import type { IrFunctionDeclaration, IrType, LoweringDiagnostic } from '../model/ir.ts';
import { emitNativeHostCapabilityRuntime, nativeHostCapabilityExports } from './native-host.ts';
import { RustEmissionError, emitRustModule, isNumericNamespaceInitializer, type RustImport } from './rust.ts';
import { stableJson, writeOrCheck } from './reports.ts';

const CARGO_DIAGNOSTIC_BUFFER_BYTES = 64 * 1024 * 1024;

export interface RustGenerationReport {
  automaticPackages: AutomaticPackageReport[];
  blessedFacades: typeof portConfig.blessedFacades;
  schemaVersion: 3;
  summary: AutomaticPackageSummary;
  targets: RustTargetReport[];
  upstreamCommit: string;
  wasmFacades: WasmFacadeReport[];
}

export interface AutomaticPackageBlocker {
  diagnostics: LoweringDiagnostic[];
  fingerprint: string;
  reason: string;
  source: string;
  stage: 'emission' | 'lowering' | 'package' | 'syntax';
}

export interface AutomaticCandidateReport {
  compileDiagnostics: Array<{ code?: string; message: string; source?: string }>;
  path?: string;
  status:
    | 'compile-blocked'
    | 'compiled'
    | 'dependency-blocked'
    | 'materialized'
    | 'not-applicable'
    | 'promoted'
    | 'source-blocked';
  syntaxCheckedSources: number;
  unresolvedDependencies: Array<{
    candidateStatus: AutomaticCandidateReport['status'];
    package: string;
    status: AutomaticPackageReport['status'];
  }>;
}

export interface AutomaticPackageReport {
  apiExports: number;
  attemptedSources: number;
  blockers: AutomaticPackageBlocker[];
  candidate: AutomaticCandidateReport;
  crate: string;
  dependencies: Array<{ crate: string; package: string }>;
  directDependents: number;
  disposition: 'generated' | PackageDisposition;
  emittedSources: Array<{
    declarationNames: string[];
    declarations: number;
    module: string;
    outputSha256: string;
    source: string;
    usesOpaqueHostValues: boolean;
  }>;
  generatedExports: string[];
  missingExports: string[];
  package: string;
  policyReason?: string;
  fullyPromotedTarget: boolean;
  promotedTarget: boolean;
  requiredDependencies: Array<{ crate: string; package: string }>;
  status: 'blocked' | 'cultivated' | 'emittable' | 'excluded' | 'host-bound';
  transitiveDependents: number;
}

export interface AutomaticPackageSummary {
  blocked: number;
  candidateCompileBlocked: number;
  candidateCompiled: number;
  candidateDependencyBlocked: number;
  cultivated: number;
  eligible: number;
  emittable: number;
  excluded: number;
  hostBound: number;
  packages: number;
  sourceBlockers: number;
}

export interface CandidateCrateNode {
  crate: string;
  dependencies: Array<{ crate: string; package: string }>;
  fullyPromotedTarget: boolean;
  package: string;
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

interface AutomaticPackageAttempt {
  modules: Array<{ content: string; module: string; source: string }>;
  report: AutomaticPackageReport;
}

interface ImportedSemanticTypes {
  enumNames: readonly string[];
  functions: readonly IrFunctionDeclaration[];
  typeParameters: Readonly<Record<string, readonly string[]>>;
  types: Readonly<Record<string, IrType>>;
}

const parsedSourceCache = new Map<string, ts.SourceFile>();
const loweredSourceCache = new Map<string, ReturnType<typeof lowerTypeScriptSource>>();
const importedSemanticTypesCache = new Map<string, ImportedSemanticTypes>();
const packageSemanticTypesCache = new Map<string, Pick<ImportedSemanticTypes, 'typeParameters' | 'types'>>();
const packageDeclarationIndexCache = new Map<string, ReadonlyMap<string, string>>();
const typeDeclarationIndexCache = new Map<string, ReadonlyMap<string, string>>();
const typeEnumNamesCache = new Map<string, readonly string[]>();

export function generateRust(
  workspaceDirectory: string,
  check: boolean,
  inventory: UpstreamInventory,
): RustGenerationReport {
  validatePackagePolicy(inventory.packages);
  const attempts = inventory.packages.map((item) =>
    attemptAutomaticPackage(workspaceDirectory, item, inventory.packages),
  );
  const automaticPackages = materializeAutomaticCandidates(
    workspaceDirectory,
    check,
    attempts,
    annotateDependencyImpact(attempts.map((item) => item.report)),
  );
  const targets = portConfig.targets.map((target) => generateTarget(workspaceDirectory, target, check));
  const wasmFacades = portConfig.wasmFacades.map((facade) =>
    generateWasmFacade(workspaceDirectory, facade, targets, check),
  );
  const report: RustGenerationReport = {
    automaticPackages,
    blessedFacades: portConfig.blessedFacades,
    schemaVersion: 3,
    summary: summarizeAutomaticPackages(automaticPackages),
    targets,
    upstreamCommit: inventory.upstreamCommit,
    wasmFacades,
  };
  const generatedDirectory = path.join(workspaceDirectory, portConfig.generatedDirectory);
  mkdirSync(generatedDirectory, { recursive: true });
  writeOrCheck(path.join(generatedDirectory, 'manifest.json'), stableJson(report), check);
  return report;
}

function attemptAutomaticPackage(
  workspaceDirectory: string,
  packageInventory: PackageInventory,
  packages: PackageInventory[],
): AutomaticPackageAttempt {
  const policy = resolvePackagePolicy(packageInventory.name);
  const hostBackendPolicy = policy?.disposition === 'host-backend' ? policy : undefined;
  const dependencies = packageInventory.dependencies
    .flatMap((packageName) => {
      const dependency = packages.find((item) => item.name === packageName);
      return dependency ? [{ crate: dependency.rustCrate, package: dependency.name }] : [];
    })
    .sort((left, right) => left.package.localeCompare(right.package));
  const promoted = portConfig.targets.find((target) => target.package === packageInventory.name);
  const promotedTarget = Boolean(promoted);
  const fullyPromotedTarget = Boolean(
    promoted && !promoted.sourceSelection && !promoted.declarationSelection && promoted.sourceExclusions.length === 0,
  );
  if (policy && policy.disposition !== 'host-backend') {
    return {
      modules: [],
      report: {
        apiExports: packageInventory.exports.length,
        attemptedSources: 0,
        blockers: [],
        candidate: {
          compileDiagnostics: [],
          status: 'not-applicable',
          syntaxCheckedSources: 0,
          unresolvedDependencies: [],
        },
        crate: packageInventory.rustCrate,
        dependencies,
        directDependents: 0,
        disposition: policy.disposition,
        emittedSources: [],
        fullyPromotedTarget,
        generatedExports: [],
        missingExports: packageInventory.exports.map((item) => item.name),
        package: packageInventory.name,
        policyReason: policy.reason,
        promotedTarget,
        requiredDependencies: [],
        status: policy.disposition,
        transitiveDependents: 0,
      },
    };
  }

  const sourceDirectory = path.join(workspaceDirectory, packageInventory.directory, 'src');
  const dependencyMap = Object.fromEntries(
    dependencies.map((dependency) => [dependency.package, { crate: dependency.crate }]),
  );
  const override = portConfig.targets.find((target) => target.package === packageInventory.name);
  const target: RustTarget = {
    crate: packageInventory.rustCrate,
    dependencies: { ...dependencyMap, ...override?.dependencies },
    package: packageInventory.name,
    sourceExclusions: [],
    typeMappings: override?.typeMappings ?? {},
  };
  const packageSemanticTypes = collectPackageSemanticTypes(sourceDirectory, packageInventory.name, workspaceDirectory);
  const entityRuntimeSemanticTypes = collectEntityRuntimeSemanticTypes(workspaceDirectory);
  const semanticTypes = {
    ...entityRuntimeSemanticTypes.types,
    ...packageSemanticTypes.types,
    ...collectSemanticTypes(workspaceDirectory, target),
  };
  const blockers: AutomaticPackageBlocker[] = [];
  const emittedSources: AutomaticPackageReport['emittedSources'] = [];
  const generatedExports = new Set<string>();
  const moduleOutputs: AutomaticPackageAttempt['modules'] = [];
  const modules = new Map<string, string>();
  const requiredDependencies = new Set<string>();
  let attemptedSources = 0;

  for (const file of walkTypeScriptSources(sourceDirectory)) {
    const moduleName = sourcePathToImplementationModule(file);
    attemptedSources++;
    const sourceText = readFileSync(file, 'utf8');
    const source = relative(workspaceDirectory, file);
    const fingerprint = sha256(sourceText);
    const previous = modules.get(moduleName);
    if (previous) {
      blockers.push({
        diagnostics: [],
        fingerprint,
        reason: `Rust module collision: ${source} and ${previous} both map to ${moduleName}.rs`,
        source,
        stage: 'package',
      });
      continue;
    }
    modules.set(moduleName, source);

    try {
      const sourceFile = parseTypeScriptFile(file);
      const lowered = lowerTypeScriptFile(file, packageInventory.name, workspaceDirectory);
      if (lowered.diagnostics.length > 0) {
        blockers.push({
          diagnostics: lowered.diagnostics,
          fingerprint,
          reason: 'TypeScript lowering produced diagnostics.',
          source,
          stage: 'lowering',
        });
        continue;
      }
      const importedSemanticTypes = collectImportedSemanticTypes(sourceFile, workspaceDirectory);
      const localDeclarations = new Set(lowered.declarations.map((declaration) => declaration.name));
      const emitted = emitRustModule({
        declarations: lowered.declarations,
        entityRuntimeAggregateAvailable: packageInventory.name === portConfig.typeLowering.entityRuntimeFamily.package,
        enumNames: [...collectTypeEnumNames(workspaceDirectory), ...importedSemanticTypes.enumNames],
        imports: collectRustImports(
          sourceFile,
          target,
          workspaceDirectory,
          [...Object.keys(importedSemanticTypes.types), ...importedSemanticTypes.enumNames].filter(
            (name) =>
              !localDeclarations.has(name) &&
              Boolean(findPackageDeclarationSource(workspaceDirectory, packageInventory.name, name)),
          ),
          packageInventory.name === '@flighthq/types'
            ? []
            : [
                ...collectInferredTopLevelTypeImports(
                  lowered.declarations,
                  importedSemanticTypes.functions,
                  workspaceDirectory,
                ),
                ...Object.keys(importedSemanticTypes.types).filter(
                  (name) =>
                    !localDeclarations.has(name) && Boolean(findTypeDeclarationSource(workspaceDirectory, name)),
                ),
              ],
        ),
        inlineFunctions: [],
        semanticFunctions: importedSemanticTypes.functions,
        semanticTypes: {
          ...semanticTypes,
          ...importedSemanticTypes.types,
        },
        semanticTypeParameters: {
          ...entityRuntimeSemanticTypes.typeParameters,
          ...packageSemanticTypes.typeParameters,
          ...importedSemanticTypes.typeParameters,
        },
        source,
        typeImports: [],
      });
      emittedSources.push({
        declarationNames: lowered.declarations.map((declaration) => declaration.name).sort(),
        declarations: lowered.declarations.length,
        module: moduleName,
        outputSha256: sha256(emitted),
        source,
        usesOpaqueHostValues: emitted.includes('OpaqueHostValue'),
      });
      moduleOutputs.push({ content: emitted, module: moduleName, source });
      for (const dependency of dependencies) {
        if (emitted.includes(`use ${dependency.crate.replaceAll('-', '_')}::`)) {
          requiredDependencies.add(dependency.package);
        }
      }
      for (const declaration of lowered.declarations) {
        if (declaration.exported) generatedExports.add(declaration.name);
      }
      for (const statement of sourceFile.statements) {
        if (ts.isExportDeclaration(statement) && statement.exportClause && ts.isNamedExports(statement.exportClause)) {
          for (const element of statement.exportClause.elements) generatedExports.add(element.name.text);
        }
      }
    } catch (error) {
      blockers.push({
        diagnostics: [],
        fingerprint,
        reason: error instanceof Error ? error.message : String(error),
        source,
        stage: 'emission',
      });
    }
  }

  if (!hostBackendPolicy) {
    const opaqueSources = emittedSources.filter((source) => source.usesOpaqueHostValues);
    const baseline =
      (portConfig.opaqueHostValueBaseline as Readonly<Record<string, number>>)[packageInventory.name] ?? 0;
    if (opaqueSources.length > baseline) {
      for (const opaqueSource of opaqueSources) {
        blockers.push({
          diagnostics: [],
          fingerprint: sha256(readFileSync(path.join(workspaceDirectory, opaqueSource.source), 'utf8')),
          reason: `Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (${String(opaqueSources.length)} opaque sources exceeds the approved baseline of ${String(baseline)}); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.`,
          source: opaqueSource.source,
          stage: 'emission',
        });
      }
      const rejectedSources = new Set(opaqueSources.map((source) => source.source));
      emittedSources.splice(
        0,
        emittedSources.length,
        ...emittedSources.filter((source) => !rejectedSources.has(source.source)),
      );
      moduleOutputs.splice(
        0,
        moduleOutputs.length,
        ...moduleOutputs.filter((output) => !rejectedSources.has(output.source)),
      );
    }
  }

  const missingExports = packageInventory.exports
    .map((item) => item.name)
    .filter((name) => !generatedExports.has(name))
    .sort();
  if (missingExports.length > 0) {
    blockers.push({
      diagnostics: [],
      fingerprint: sha256(missingExports.join('\0')),
      reason: `Generated crate is missing ${String(missingExports.length)} of ${String(packageInventory.exports.length)} upstream exports; re-export or declaration synthesis is required.`,
      source: `${packageInventory.directory}/src`,
      stage: 'package',
    });
  }
  blockers.sort((left, right) => left.source.localeCompare(right.source) || left.reason.localeCompare(right.reason));
  emittedSources.sort((left, right) => left.source.localeCompare(right.source));
  if (blockers.length === 0) {
    for (const output of moduleOutputs) {
      try {
        output.content = formatRust(output.content, output.source);
        const emittedSource = emittedSources.find((item) => item.source === output.source);
        if (emittedSource) emittedSource.outputSha256 = sha256(output.content);
      } catch (error) {
        if (!(error instanceof RustEmissionError)) throw error;
        blockers.push({
          diagnostics: [],
          fingerprint: sha256(output.content),
          reason: error instanceof Error ? error.message : String(error),
          source: output.source,
          stage: 'syntax',
        });
      }
    }
  }
  const status = blockers.length === 0 ? 'emittable' : 'blocked';
  return {
    modules: status === 'emittable' ? moduleOutputs : [],
    report: {
      apiExports: packageInventory.exports.length,
      attemptedSources,
      blockers,
      candidate: {
        compileDiagnostics: [],
        status: status === 'emittable' ? (fullyPromotedTarget ? 'promoted' : 'materialized') : 'source-blocked',
        syntaxCheckedSources: status === 'emittable' ? moduleOutputs.length : 0,
        unresolvedDependencies: [],
      },
      crate: packageInventory.rustCrate,
      dependencies,
      directDependents: 0,
      disposition: hostBackendPolicy?.disposition ?? 'generated',
      emittedSources,
      fullyPromotedTarget,
      generatedExports: [...generatedExports].sort(),
      missingExports,
      package: packageInventory.name,
      ...(hostBackendPolicy ? { policyReason: hostBackendPolicy.reason } : {}),
      promotedTarget,
      requiredDependencies: dependencies.filter((item) => requiredDependencies.has(item.package)),
      status,
      transitiveDependents: 0,
    },
  };
}

function annotateDependencyImpact(packages: AutomaticPackageReport[]): AutomaticPackageReport[] {
  const directDependents = new Map<string, Set<string>>();
  for (const item of packages) {
    for (const dependency of item.dependencies) {
      const dependents = directDependents.get(dependency.package) ?? new Set<string>();
      dependents.add(item.package);
      directDependents.set(dependency.package, dependents);
    }
  }
  return packages.map((item) => {
    const visited = new Set<string>();
    const pending = [...(directDependents.get(item.package) ?? [])];
    while (pending.length > 0) {
      const dependent = pending.pop()!;
      if (dependent === item.package || visited.has(dependent)) continue;
      visited.add(dependent);
      pending.push(...(directDependents.get(dependent) ?? []));
    }
    return {
      ...item,
      directDependents: directDependents.get(item.package)?.size ?? 0,
      transitiveDependents: visited.size,
    };
  });
}

function materializeAutomaticCandidates(
  workspaceDirectory: string,
  check: boolean,
  attempts: AutomaticPackageAttempt[],
  packages: AutomaticPackageReport[],
): AutomaticPackageReport[] {
  validateCandidateCrateGraph(packages);
  const candidateRoot = path.join(workspaceDirectory, portConfig.generatedDirectory, 'candidates');
  if (!check) rmSync(candidateRoot, { force: true, recursive: true });
  const expected = new Set<string>();
  const packageByName = new Map(packages.map((item) => [item.package, item]));
  const attemptByPackage = new Map(attempts.map((item) => [item.report.package, item]));
  const dependencyReady = (item: AutomaticPackageReport, visiting: ReadonlySet<string> = new Set()): boolean => {
    if (item.status !== 'emittable') return false;
    if (visiting.has(item.package)) return true;
    const next = new Set([...visiting, item.package]);
    return item.requiredDependencies.every((dependency) => {
      const resolved = packageByName.get(dependency.package);
      return !resolved || dependencyReady(resolved, next);
    });
  };
  const materialized = packages.map((item): AutomaticPackageReport => {
    if (item.status !== 'emittable' || item.fullyPromotedTarget) return item;
    const attempt = attemptByPackage.get(item.package);
    if (!attempt) throw new Error(`Missing automatic generation attempt for ${item.package}`);
    const unresolvedDependencies = item.requiredDependencies.flatMap((dependency) => {
      const resolved = packageByName.get(dependency.package);
      if (!resolved) return [];
      return dependencyReady(resolved)
        ? []
        : [
            {
              candidateStatus: resolved.fullyPromotedTarget
                ? resolved.candidate.status
                : resolved.status === 'emittable'
                  ? ('dependency-blocked' as const)
                  : resolved.candidate.status,
              package: resolved.package,
              status: resolved.status,
            },
          ];
    });
    const crateDirectory = path.join(candidateRoot, item.crate);
    const crateSourceDirectory = path.join(crateDirectory, 'src');
    const target: RustTarget = {
      crate: item.crate,
      dependencies: Object.fromEntries(
        item.requiredDependencies.map((dependency) => [dependency.package, { crate: dependency.crate }]),
      ),
      package: item.package,
      sourceExclusions: [],
      typeMappings: portConfig.targets.find((candidate) => candidate.package === item.package)?.typeMappings ?? {},
    };
    const outputs: PendingOutput[] = attempt.modules.map((module) => ({
      content: module.content,
      file: path.join(crateSourceDirectory, `${module.module}.rs`),
    }));
    outputs.push(
      {
        content: emitCandidateCargoManifest(
          item,
          packageByName,
          attempt.modules.some((module) => module.content.includes('regex::')),
        ),
        file: path.join(crateDirectory, 'Cargo.toml'),
      },
      {
        content: formatRust(
          emitLibrary(
            target,
            attempt.modules.map((module) => module.module),
          ),
          path.join(crateSourceDirectory, 'lib.rs'),
        ),
        file: path.join(crateSourceDirectory, 'lib.rs'),
      },
    );
    for (const output of outputs) {
      mkdirSync(path.dirname(output.file), { recursive: true });
      writeOrCheck(output.file, output.content, check);
      expected.add(output.file);
    }
    return {
      ...item,
      candidate: {
        compileDiagnostics: [],
        path: relative(workspaceDirectory, crateDirectory),
        status: unresolvedDependencies.length > 0 ? 'dependency-blocked' : 'materialized',
        syntaxCheckedSources: attempt.modules.length,
        unresolvedDependencies,
      },
    };
  });
  const compileReady = materialized.filter((item) => item.candidate.status === 'materialized');
  const workspaceManifest = emitCandidateWorkspaceManifest(compileReady);
  const workspaceManifestFile = path.join(candidateRoot, 'Cargo.toml');
  mkdirSync(candidateRoot, { recursive: true });
  writeOrCheck(workspaceManifestFile, workspaceManifest, check);
  expected.add(workspaceManifestFile);
  const compiled = compileAutomaticCandidates(workspaceDirectory, candidateRoot, materialized);
  verifyNoStaleOutputs(candidateRoot, expected, check);
  return compiled;
}

export function validateCandidateCrateGraph(packages: readonly CandidateCrateNode[]): void {
  const packageByName = new Map<string, CandidateCrateNode>();
  const packageByCrate = new Map<string, CandidateCrateNode>();
  for (const item of packages) {
    const duplicatePackage = packageByName.get(item.package);
    if (duplicatePackage) {
      throw new Error(`Duplicate candidate package resolution node: ${item.package}`);
    }
    const duplicateCrate = packageByCrate.get(item.crate);
    if (duplicateCrate && duplicateCrate.package !== item.package) {
      throw new Error(
        `Duplicate candidate Cargo package identity ${item.crate}: ${duplicateCrate.package} and ${item.package}`,
      );
    }
    packageByName.set(item.package, item);
    packageByCrate.set(item.crate, item);
  }

  for (const item of packages) {
    for (const dependency of item.dependencies) {
      const resolved = packageByName.get(dependency.package);
      if (!resolved) continue;
      if (dependency.crate !== resolved.crate) {
        throw new Error(
          `Candidate dependency edge ${item.package} -> ${dependency.package} names ${dependency.crate}, but the resolution map selects ${resolved.crate}`,
        );
      }
      if (item.fullyPromotedTarget && !resolved.fullyPromotedTarget) {
        throw new Error(
          `Fully promoted package ${item.package} depends on non-fully-promoted package ${dependency.package}`,
        );
      }
    }
  }
}

function emitCandidateCargoManifest(
  candidate: AutomaticPackageReport,
  packageByName: ReadonlyMap<string, AutomaticPackageReport>,
  usesRegex: boolean,
): string {
  const dependencies = candidate.requiredDependencies.map((dependency) => {
    const resolved = packageByName.get(dependency.package);
    const dependencyPath = resolved?.fullyPromotedTarget
      ? `../../crates/${dependency.crate}`
      : `../${dependency.crate}`;
    return `${dependency.crate} = { path = "${dependencyPath}" }`;
  });
  if (usesRegex) dependencies.push('regex = "1"');
  return [
    '# @generated candidate crate; do not edit.',
    '[package]',
    `name = "${candidate.crate}"`,
    'version = "0.1.0"',
    'edition = "2024"',
    'license = "MIT"',
    'publish = false',
    '',
    '[lib]',
    'path = "src/lib.rs"',
    '',
    ...(dependencies.length > 0 ? ['[dependencies]', ...dependencies.sort(), ''] : []),
  ].join('\n');
}

function emitCandidateWorkspaceManifest(candidates: AutomaticPackageReport[]): string {
  return [
    '# @generated candidate workspace; do not edit.',
    '[workspace]',
    'members = [',
    ...candidates.map((item) => `  "${item.crate}",`),
    ']',
    'resolver = "3"',
    '',
  ].join('\n');
}

function compileAutomaticCandidates(
  workspaceDirectory: string,
  candidateRoot: string,
  packages: AutomaticPackageReport[],
): AutomaticPackageReport[] {
  const compileReady = packages.filter((item) => item.candidate.status === 'materialized');
  if (compileReady.length === 0) return packages;
  const manifest = path.join(candidateRoot, 'Cargo.toml');
  let output = '';
  let cargoSucceeded = true;
  try {
    output = execFileSync(
      'cargo',
      ['check', '--workspace', '--keep-going', '--manifest-path', manifest, '--message-format=json'],
      {
        cwd: workspaceDirectory,
        encoding: 'utf8',
        env: {
          ...process.env,
          CARGO_TARGET_DIR: path.join(workspaceDirectory, 'target', 'generator-candidates'),
        },
        maxBuffer: CARGO_DIAGNOSTIC_BUFFER_BYTES,
        stdio: ['ignore', 'pipe', 'pipe'],
      },
    );
  } catch (error) {
    if (error && typeof error === 'object' && 'code' in error && error.code === 'ENOBUFS') {
      throw new Error(`Candidate Cargo diagnostics exceeded the ${CARGO_DIAGNOSTIC_BUFFER_BYTES}-byte capture limit.`);
    }
    cargoSucceeded = false;
    output =
      error && typeof error === 'object' && 'stdout' in error && typeof error.stdout === 'string' ? error.stdout : '';
    if (output.length === 0) {
      const stderr =
        error && typeof error === 'object' && 'stderr' in error && typeof error.stderr === 'string'
          ? error.stderr.trim()
          : String(error);
      throw new Error(`Candidate Cargo workspace failed before emitting compiler diagnostics: ${stderr}`);
    }
  } finally {
    rmSync(path.join(candidateRoot, 'Cargo.lock'), { force: true });
  }
  const diagnosticsByCrate = new Map<string, AutomaticCandidateReport['compileDiagnostics']>();
  const compiledCrates = new Set<string>();
  for (const line of output.split('\n')) {
    if (!line.startsWith('{')) continue;
    let message: {
      message?: {
        code?: { code?: string };
        level?: string;
        message?: string;
        spans?: Array<{ file_name?: string; is_primary?: boolean }>;
      };
      package_id?: string;
      reason?: string;
      target?: { name?: string };
    };
    try {
      message = JSON.parse(line) as typeof message;
    } catch {
      continue;
    }
    if (message.reason === 'compiler-artifact') {
      const candidate = compileReady.find((item) => message.target?.name === item.crate.replaceAll('-', '_'));
      if (candidate) compiledCrates.add(candidate.crate);
      continue;
    }
    if (message.reason !== 'compiler-message' || message.message?.level !== 'error') continue;
    const candidate = compileReady.find(
      (item) =>
        message.target?.name === item.crate.replaceAll('-', '_') || message.package_id?.includes(`/${item.crate}#`),
    );
    if (!candidate) continue;
    const diagnostics = diagnosticsByCrate.get(candidate.crate) ?? [];
    const source = message.message.spans?.find((span) => span.is_primary)?.file_name;
    diagnostics.push({
      ...(message.message.code?.code ? { code: message.message.code.code } : {}),
      message: message.message.message ?? 'Rust compilation failed.',
      ...(source ? { source: normalizeDiagnosticSource(workspaceDirectory, candidateRoot, source) } : {}),
    });
    diagnosticsByCrate.set(candidate.crate, diagnostics);
  }
  const firstPass = packages.map((item): AutomaticPackageReport => {
    if (item.candidate.status !== 'materialized') return item;
    const diagnostics = diagnosticsByCrate.get(item.crate) ?? [];
    return {
      ...item,
      candidate: {
        ...item.candidate,
        compileDiagnostics: diagnostics,
        status:
          diagnostics.length > 0 ? 'compile-blocked' : compiledCrates.has(item.crate) ? 'compiled' : 'materialized',
      },
    };
  });
  const firstPassByPackage = new Map(firstPass.map((item) => [item.package, item]));
  return firstPass.map((item): AutomaticPackageReport => {
    if (item.candidate.status !== 'materialized') return item;
    const unresolvedDependencies = item.requiredDependencies.flatMap((dependency) => {
      const resolved = firstPassByPackage.get(dependency.package);
      if (!resolved || resolved.fullyPromotedTarget || resolved.candidate.status === 'compiled') return [];
      return [
        {
          candidateStatus: resolved.candidate.status,
          package: resolved.package,
          status: resolved.status,
        },
      ];
    });
    if (unresolvedDependencies.length > 0) {
      return {
        ...item,
        candidate: {
          ...item.candidate,
          status: 'dependency-blocked',
          unresolvedDependencies,
        },
      };
    }
    if (!cargoSucceeded) {
      return {
        ...item,
        candidate: {
          ...item.candidate,
          compileDiagnostics: [
            {
              message: 'Cargo did not emit a compiler artifact for this candidate before the workspace build failed.',
            },
          ],
          status: 'compile-blocked',
        },
      };
    }
    return { ...item, candidate: { ...item.candidate, status: 'compiled' } };
  });
}

function resolvePackagePolicy(packageName: string): PackagePolicyRule | undefined {
  const matching = portConfig.packagePolicy.filter((rule) => matchesPackagePolicy(packageName, rule.match));
  if (matching.length > 1) {
    throw new Error(
      `Package policy rules overlap for ${packageName}: ${matching.map((item) => item.match).join(', ')}`,
    );
  }
  return matching[0];
}

function matchesPackagePolicy(packageName: string, pattern: string): boolean {
  const expression = pattern
    .split('*')
    .map((part) => part.replace(/[\\^$.*+?()[\]{}|]/gu, '\\$&'))
    .join('.*');
  return new RegExp(`^${expression}$`, 'u').test(packageName);
}

function validatePackagePolicy(packages: PackageInventory[]): void {
  const stale = portConfig.packagePolicy.filter(
    (rule) => !packages.some((item) => matchesPackagePolicy(item.name, rule.match)),
  );
  if (stale.length > 0) throw new Error(`Stale package policy rules: ${stale.map((item) => item.match).join(', ')}`);
  for (const item of packages) resolvePackagePolicy(item.name);
}

function summarizeAutomaticPackages(packages: AutomaticPackageReport[]): AutomaticPackageSummary {
  return {
    blocked: packages.filter((item) => item.status === 'blocked').length,
    candidateCompileBlocked: packages.filter((item) => item.candidate.status === 'compile-blocked').length,
    candidateCompiled: packages.filter((item) => item.candidate.status === 'compiled').length,
    candidateDependencyBlocked: packages.filter((item) => item.candidate.status === 'dependency-blocked').length,
    cultivated: packages.filter((item) => item.status === 'cultivated').length,
    eligible: packages.filter((item) => item.disposition === 'generated' || item.disposition === 'host-backend').length,
    emittable: packages.filter((item) => item.status === 'emittable').length,
    excluded: packages.filter((item) => item.status === 'excluded').length,
    hostBound: packages.filter((item) => item.status === 'host-bound').length,
    packages: packages.length,
    sourceBlockers: packages.reduce((total, item) => total + item.blockers.length, 0),
  };
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
  const entityRuntimeSemanticTypes = collectEntityRuntimeSemanticTypes(workspaceDirectory);
  const semanticTypes = {
    ...entityRuntimeSemanticTypes.types,
    ...collectSemanticTypes(workspaceDirectory, target),
  };
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
    const sourceFile = parseTypeScriptFile(file);
    const lowered = lowerTypeScriptFile(file, target.package, workspaceDirectory);
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
      const localEntityRuntimeRoot =
        target.package === portConfig.typeLowering.entityRuntimeFamily.package &&
        declarations.some(
          (declaration) =>
            declaration.kind === 'type' &&
            (declaration.name === portConfig.typeLowering.entityRuntimeFamily.entityType ||
              declaration.name === portConfig.typeLowering.entityRuntimeFamily.runtimeType),
        );
      const moduleSemanticTypes = {
        ...semanticTypes,
        ...importedSemanticTypes.types,
      };
      const moduleSemanticTypeParameters = {
        ...entityRuntimeSemanticTypes.typeParameters,
        ...importedSemanticTypes.typeParameters,
      };
      if (localEntityRuntimeRoot) {
        const localNames = new Set(
          declarations.flatMap((declaration) => (declaration.kind === 'type' ? [declaration.name] : [])),
        );
        for (const name of Object.keys(entityRuntimeSemanticTypes.types)) {
          if (!localNames.has(name)) delete moduleSemanticTypes[name];
        }
        for (const name of Object.keys(entityRuntimeSemanticTypes.typeParameters)) {
          if (!localNames.has(name)) delete moduleSemanticTypeParameters[name];
        }
      }
      const emitted = formatRust(
        emitRustModule({
          declarations,
          entityRuntimeAggregateAvailable: target.package === portConfig.typeLowering.entityRuntimeFamily.package,
          enumNames: [...collectTypeEnumNames(workspaceDirectory), ...importedSemanticTypes.enumNames],
          imports: collectRustImports(
            sourceFile,
            target,
            workspaceDirectory,
            [...Object.keys(importedSemanticTypes.types), ...importedSemanticTypes.enumNames].filter(
              (name) =>
                !declarations.some((declaration) => declaration.name === name) &&
                Boolean(findPackageDeclarationSource(workspaceDirectory, target.package, name)),
            ),
            target.package === '@flighthq/types'
              ? []
              : [
                  ...collectInferredTopLevelTypeImports(
                    lowered.declarations,
                    importedSemanticTypes.functions,
                    workspaceDirectory,
                  ),
                  ...Object.keys(importedSemanticTypes.types).filter(
                    (name) =>
                      !declarations.some((declaration) => declaration.name === name) &&
                      Boolean(findTypeDeclarationSource(workspaceDirectory, name)),
                  ),
                ],
          ),
          inlineFunctions,
          semanticFunctions: importedSemanticTypes.functions,
          semanticTypes: moduleSemanticTypes,
          semanticTypeParameters: moduleSemanticTypeParameters,
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

  verifyEntityRuntimeRootInvariant(target, outputs);
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

function verifyEntityRuntimeRootInvariant(target: RustTarget, outputs: readonly PendingOutput[]): void {
  const modules = outputs.map((output) => output.content);
  const requiresLocalRoot = modules.some(
    (content) => content.includes('crate::EntityRuntime') || content.includes('crate::FlightEntity'),
  );
  if (!requiresLocalRoot) return;
  const requiredDefinitions = [
    ['EntityRuntime', 'pub struct EntityRuntime {'],
    ['EntityRuntimeStorage', 'pub struct EntityRuntimeStorage {'],
    ['FlightEntity', 'pub trait FlightEntity {'],
  ] as const;
  const missing = requiredDefinitions.flatMap(([name, definition]) =>
    modules.some((content) => content.includes(definition)) ? [] : [name],
  );
  if (missing.length > 0) {
    throw new Error(
      `Generated target ${target.crate} emits native entity fields or impls without crate-root support: ${missing.join(', ')}`,
    );
  }
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
      const lowered = lowerTypeScriptFile(source, '@flighthq/types', workspaceDirectory);
      const declaration = lowered.declarations.find((item) => item.kind === 'type' && item.name === name);
      if (!declaration || declaration.kind !== 'type') {
        throw new Error(`Semantic type mapping ${name} did not resolve from ${mapping.source}`);
      }
      return [name, declaration.type];
    }),
  );
}

function collectPackageSemanticTypes(
  sourceDirectory: string,
  packageName: string,
  workspaceDirectory: string,
): Pick<ImportedSemanticTypes, 'typeParameters' | 'types'> {
  const cacheKey = `${workspaceDirectory}\0${packageName}\0${sourceDirectory}`;
  const cached = packageSemanticTypesCache.get(cacheKey);
  if (cached) return cached;
  const types = new Map<string, IrType>();
  const typeParameters = new Map<string, readonly string[]>();
  for (const file of walkTypeScriptSources(sourceDirectory)) {
    const lowered = lowerTypeScriptFile(file, packageName, workspaceDirectory);
    for (const declaration of lowered.declarations) {
      if (declaration.kind !== 'type' || !declaration.exported || types.has(declaration.name)) continue;
      types.set(declaration.name, declaration.type);
      typeParameters.set(declaration.name, declaration.typeParameters);
    }
  }
  const result = {
    typeParameters: Object.fromEntries(typeParameters),
    types: Object.fromEntries(types),
  };
  packageSemanticTypesCache.set(cacheKey, result);
  return result;
}

function collectEntityRuntimeSemanticTypes(
  workspaceDirectory: string,
): Pick<ImportedSemanticTypes, 'typeParameters' | 'types'> {
  const family = portConfig.typeLowering.entityRuntimeFamily;
  const source = findPackageDeclarationSource(workspaceDirectory, family.package, family.runtimeType);
  if (!source) return { typeParameters: {}, types: {} };
  const packageTypes = collectPackageSemanticTypes(path.dirname(source), family.package, workspaceDirectory);
  const allTypes = new Map(Object.entries(packageTypes.types));
  const reachesRoot = (name: string, root: string, visited: ReadonlySet<string> = new Set()): boolean => {
    if (name === root) return true;
    if (visited.has(name)) return false;
    const declaration = allTypes.get(name);
    if (!declaration) return false;
    const nextVisited = new Set([...visited, name]);
    if (declaration.kind === 'named') return reachesRoot(declaration.name, root, nextVisited);
    return (
      declaration.kind === 'anonymous' &&
      declaration.extends.some((base) => base.kind === 'named' && reachesRoot(base.name, root, nextVisited))
    );
  };
  const included = new Set(
    [...allTypes.keys()].filter(
      (name) => reachesRoot(name, family.entityType) || reachesRoot(name, family.runtimeType),
    ),
  );
  const pending = [...included];
  while (pending.length > 0) {
    const name = pending.pop()!;
    const declaration = allTypes.get(name);
    if (!declaration) continue;
    for (const referenced of collectNamedTypeReferences(declaration)) {
      if (!allTypes.has(referenced) || included.has(referenced)) continue;
      included.add(referenced);
      pending.push(referenced);
    }
  }
  return {
    typeParameters: Object.fromEntries(
      Object.entries(packageTypes.typeParameters).filter(([name]) => included.has(name)),
    ),
    types: Object.fromEntries(Object.entries(packageTypes.types).filter(([name]) => included.has(name))),
  };
}

function collectNamedTypeReferences(type: IrType): ReadonlySet<string> {
  const names = new Set<string>();
  const visit = (candidate: IrType): void => {
    switch (candidate.kind) {
      case 'anonymous':
        candidate.extends.forEach(visit);
        candidate.fields.forEach((field) => visit(field.type));
        break;
      case 'array':
        visit(candidate.element);
        break;
      case 'function':
        candidate.parameters.forEach(visit);
        visit(candidate.returns);
        break;
      case 'named':
        names.add(candidate.name);
        candidate.arguments.forEach(visit);
        break;
      case 'nullable':
        visit(candidate.inner);
        break;
      case 'union':
        candidate.variants.forEach(visit);
        break;
      case 'dynamic':
      case 'primitive':
        break;
    }
  };
  visit(type);
  return names;
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
  const sharedOpaqueHostValue =
    target.package !== '@flighthq/types' && Object.hasOwn(target.dependencies, '@flighthq/types');
  return [
    '// @generated by tools/generator; do not edit.',
    '#![forbid(unsafe_code)]',
    '',
    '/// Tagged storage for TypeScript values whose static type is unknown at the generated Rust boundary.',
    ...(sharedOpaqueHostValue
      ? [
          `pub use flighthq_types::{clear_interval, clear_timeout, flight_now_millis, host_set, host_value, ${nativeHostCapabilityExports.join(', ')}, set_interval, set_timeout, FlightCallback, FlightSymbol, FlightTimeout, OpaqueHostValue};`,
        ]
      : [
          '#[derive(Clone, Debug, PartialEq)]',
          'pub enum OpaqueHostValue {',
          '  Undefined,',
          '  Null,',
          '  Bool(bool),',
          '  Number(f64),',
          '  String(String),',
          '  Object,',
          '}',
          'impl Default for OpaqueHostValue {',
          '  fn default() -> Self { Self::Undefined }',
          '}',
          '',
          '/// Native fallback for dynamically typed host reads and calls.',
          'pub fn host_value<T: Default>(_operation: &str) -> T {',
          '  T::default()',
          '}',
          '/// Native fallback for dynamically typed host writes.',
          'pub fn host_set<T>(_operation: &str, value: T) -> T {',
          '  value',
          '}',
          '',
          ...emitNativeHostCapabilityRuntime(),
          '',
          '/// Native identity for TypeScript `symbol` values.',
          '#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]',
          'pub struct FlightSymbol(u64);',
          'impl FlightSymbol {',
          '  pub fn new() -> Self {',
          '    static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);',
          '    Self(NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed))',
          '  }',
          '  pub fn for_name(name: &str) -> Self {',
          '    static NAMES: std::sync::LazyLock<std::sync::Mutex<Vec<String>>> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));',
          '    let mut names = NAMES.lock().unwrap();',
          '    let index = match names.iter().position(|value| value == name) {',
          '      Some(index) => index,',
          '      None => { names.push(name.to_owned()); names.len() - 1 },',
          '    };',
          '    Self((1_u64 << 63) | index as u64)',
          '  }',
          '}',
          'impl Default for FlightSymbol {',
          '  fn default() -> Self { Self::new() }',
          '}',
          '',
          '/// Mechanical contract for TypeScript callback type parameters.',
          "pub trait FlightCallback: Clone + Send + 'static {",
          "  type Args: Clone + Send + 'static;",
          '  fn flight_call(&self, args: Self::Args);',
          '  fn flight_same(&self, other: &Self) -> bool;',
          '  fn flight_noop() -> Self;',
          '  fn flight_from_tuple_callback<Factory>(callback: Factory) -> Self',
          '  where',
          "    Factory: FnMut(Self::Args) + Send + 'static;",
          '}',
          '',
          'macro_rules! impl_flight_callback {',
          '  (() => ()) => {',
          "    impl FlightCallback for std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>> {",
          '      type Args = ();',
          '      fn flight_call(&self, (): Self::Args) { self.lock().unwrap()(); }',
          '      fn flight_same(&self, other: &Self) -> bool { std::sync::Arc::ptr_eq(self, other) }',
          '      fn flight_noop() -> Self { std::sync::Arc::new(std::sync::Mutex::new(Box::new(|| ()))) }',
          '      fn flight_from_tuple_callback<Factory>(mut callback: Factory) -> Self',
          "      where Factory: FnMut(Self::Args) + Send + 'static {",
          '        std::sync::Arc::new(std::sync::Mutex::new(Box::new(move || callback(()))))',
          '      }',
          '    }',
          '  };',
          '  (($($type:ident:$value:ident),+) => ($($argument:ident),+)) => {',
          "    impl<$($type),+> FlightCallback for std::sync::Arc<std::sync::Mutex<Box<dyn FnMut($($type),+) -> () + Send + 'static>>>",
          "    where $($type: Clone + Send + 'static),+ {",
          '      type Args = ($($type,)+);',
          '      fn flight_call(&self, args: Self::Args) {',
          '        let ($($value,)+) = args;',
          '        self.lock().unwrap()($($value),+);',
          '      }',
          '      fn flight_same(&self, other: &Self) -> bool { std::sync::Arc::ptr_eq(self, other) }',
          '      fn flight_noop() -> Self { std::sync::Arc::new(std::sync::Mutex::new(Box::new(|$($value: $type),+| { let _ = ($($value),+); () }))) }',
          '      fn flight_from_tuple_callback<Factory>(mut callback: Factory) -> Self',
          "      where Factory: FnMut(Self::Args) + Send + 'static {",
          '        std::sync::Arc::new(std::sync::Mutex::new(Box::new(move |$($value),+| callback(($($value,)+)))))',
          '      }',
          '    }',
          '  };',
          '}',
          'impl_flight_callback!(() => ());',
          'impl_flight_callback!((A:a) => (a));',
          'impl_flight_callback!((A:a, B:b) => (a, b));',
          'impl_flight_callback!((A:a, B:b, C:c) => (a, b, c));',
          'impl_flight_callback!((A:a, B:b, C:c, D:d) => (a, b, c, d));',
          'impl_flight_callback!((A:a, B:b, C:c, D:d, E:e) => (a, b, c, d, e));',
          'impl_flight_callback!((A:a, B:b, C:c, D:d, E:e, F:f) => (a, b, c, d, e, f));',
          '',
          '/// Cancellable native timer handle for generated `setTimeout` calls.',
          '#[derive(Clone)]',
          'pub struct FlightTimeout {',
          '  cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>,',
          '}',
          'pub fn set_timeout<F>(callback: F, delay_ms: f64) -> FlightTimeout',
          "where F: FnOnce() + Send + 'static {",
          '  let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));',
          '  let worker_cancelled = cancelled.clone();',
          '  std::thread::spawn(move || {',
          '    std::thread::sleep(std::time::Duration::from_secs_f64((delay_ms.max(0.0)) / 1000.0));',
          '    if !worker_cancelled.load(std::sync::atomic::Ordering::Relaxed) { callback(); }',
          '  });',
          '  FlightTimeout { cancelled }',
          '}',
          'pub fn clear_timeout(timer: FlightTimeout) {',
          '  timer.cancelled.store(true, std::sync::atomic::Ordering::Relaxed);',
          '}',
          'pub fn set_interval<F>(mut callback: F, delay_ms: f64) -> FlightTimeout',
          "where F: FnMut() + Send + 'static {",
          '  let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));',
          '  let worker_cancelled = cancelled.clone();',
          '  std::thread::spawn(move || {',
          '    let delay = std::time::Duration::from_secs_f64((delay_ms.max(0.0)) / 1000.0);',
          '    while !worker_cancelled.load(std::sync::atomic::Ordering::Relaxed) {',
          '      std::thread::sleep(delay);',
          '      if !worker_cancelled.load(std::sync::atomic::Ordering::Relaxed) { callback(); }',
          '    }',
          '  });',
          '  FlightTimeout { cancelled }',
          '}',
          'pub fn clear_interval(timer: FlightTimeout) {',
          '  clear_timeout(timer);',
          '}',
          'pub fn flight_now_millis() -> f64 {',
          '  std::time::SystemTime::now()',
          '    .duration_since(std::time::UNIX_EPOCH)',
          '    .unwrap_or_default()',
          '    .as_secs_f64() * 1000.0',
          '}',
        ]),
    '',
    '/// Mechanical representation for TypeScript unions whose variants need distinct native storage.',
    '#[derive(Clone)]',
    'pub enum FlightUnion2<A, B> {',
    '  A(A),',
    '  B(B),',
    '}',
    '',
    '/// Opaque placeholder for a TypeScript Promise until async lowering supplies a native Future.',
    'pub struct Promise<T> {',
    '  marker: std::marker::PhantomData<fn() -> T>,',
    '  value: OpaqueHostValue,',
    '}',
    'impl<T> Clone for Promise<T> {',
    '  fn clone(&self) -> Self { Self { marker: std::marker::PhantomData, value: self.value.clone() } }',
    '}',
    'impl<T> Default for Promise<T> {',
    '  fn default() -> Self { Self { marker: std::marker::PhantomData, value: OpaqueHostValue::Undefined } }',
    '}',
    '',
    ...aliases.flatMap((alias) => [alias, '']),
    ...declarations,
    '',
  ].join('\n');
}

function collectRustImports(
  sourceFile: ts.SourceFile,
  target: RustTarget,
  workspaceDirectory: string,
  additionalCrateTypes: string[],
  additionalFlightTypes: string[] = [],
): RustImport[] {
  const groups = new Map<string, RustImport['names']>();
  const localReExports = new Map<string, string>();
  for (const statement of sourceFile.statements) {
    if (
      ts.isExportDeclaration(statement) &&
      !statement.moduleSpecifier &&
      statement.exportClause &&
      ts.isNamedExports(statement.exportClause)
    ) {
      for (const element of statement.exportClause.elements) {
        localReExports.set(element.propertyName?.text ?? element.name.text, element.name.text);
      }
    }
  }
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
      ...bindings.elements.map((element) => {
        const local = element.name.text;
        const exported = localReExports.get(local);
        return {
          imported: element.propertyName?.text ?? local,
          kind:
            statement.importClause?.isTypeOnly || element.isTypeOnly
              ? ('type' as const)
              : classifyImportedRustBinding(
                  sourceFile.fileName,
                  specifier,
                  element.propertyName?.text ?? local,
                  workspaceDirectory,
                ),
          local: exported ?? local,
          ...(exported ? { public: true } : {}),
        };
      }),
    );
    groups.set(module, names);
  }
  for (const statement of sourceFile.statements) {
    if (
      !ts.isExportDeclaration(statement) ||
      !statement.moduleSpecifier ||
      !ts.isStringLiteral(statement.moduleSpecifier) ||
      !statement.exportClause ||
      !ts.isNamedExports(statement.exportClause)
    ) {
      continue;
    }
    const specifier = statement.moduleSpecifier.text;
    const dependency = target.dependencies[specifier];
    const module = specifier.startsWith('.') ? 'crate' : dependency?.crate.replaceAll('-', '_');
    if (!module) continue;
    const names = groups.get(module) ?? [];
    names.push(
      ...statement.exportClause.elements.map((element) => {
        const imported = element.propertyName?.text ?? element.name.text;
        return {
          imported,
          kind:
            statement.isTypeOnly || element.isTypeOnly
              ? ('type' as const)
              : classifyImportedRustBinding(sourceFile.fileName, specifier, imported, workspaceDirectory),
          local: element.name.text,
          public: true,
        };
      }),
    );
    groups.set(module, names);
  }
  if (additionalCrateTypes.length > 0) {
    const names = groups.get('crate') ?? [];
    names.push(...additionalCrateTypes.map((name) => ({ imported: name, kind: 'type' as const, local: name })));
    groups.set('crate', names);
  }
  if (additionalFlightTypes.length > 0) {
    const names = groups.get('flighthq_types') ?? [];
    names.push(
      ...[...new Set(additionalFlightTypes)].flatMap((name) =>
        names.some((item) => item.local === name)
          ? []
          : [
              {
                imported: name,
                kind: 'type' as const,
                local: name,
              },
            ],
      ),
    );
    groups.set('flighthq_types', names);
  }
  return [...groups].map(([module, names]) => ({ module, names }));
}

function collectInferredTopLevelTypeImports(
  declarations: ReturnType<typeof lowerTypeScriptSource>['declarations'],
  semanticFunctions: readonly IrFunctionDeclaration[],
  workspaceDirectory: string,
): string[] {
  const functions = new Map(semanticFunctions.map((declaration) => [declaration.name, declaration]));
  const names = declarations.flatMap((declaration) => {
    if (declaration.kind !== 'variable' || declaration.type || !declaration.initializer) return [];
    let initializer = declaration.initializer;
    while (initializer.kind === 'cast') initializer = initializer.expression;
    if (initializer.kind !== 'call' || initializer.callee.kind !== 'identifier') return [];
    const returns = functions.get(initializer.callee.name)?.returns;
    return returns?.kind === 'named' && findTypeDeclarationSource(workspaceDirectory, returns.name)
      ? [returns.name]
      : [];
  });
  return [...new Set(names)].sort();
}

function classifyImportedRustBinding(
  importer: string,
  specifier: string,
  name: string,
  workspaceDirectory: string,
): NonNullable<RustImport['names'][number]['kind']> {
  const source = specifier.startsWith('.')
    ? resolveRelativeTypeScriptSource(importer, specifier)
    : specifier.startsWith('@flighthq/')
      ? findPackageDeclarationSource(workspaceDirectory, specifier, name)
      : undefined;
  if (!source) return 'value';
  const statement = parseTypeScriptFile(source).statements.find((candidate) => {
    if (
      (ts.isFunctionDeclaration(candidate) ||
        ts.isClassDeclaration(candidate) ||
        ts.isInterfaceDeclaration(candidate) ||
        ts.isTypeAliasDeclaration(candidate) ||
        ts.isEnumDeclaration(candidate)) &&
      candidate.name?.text === name
    ) {
      return true;
    }
    return (
      ts.isVariableStatement(candidate) &&
      candidate.declarationList.declarations.some(
        (declaration) => ts.isIdentifier(declaration.name) && declaration.name.text === name,
      )
    );
  });
  if (statement && ts.isFunctionDeclaration(statement)) return 'function';
  if (statement && ts.isVariableStatement(statement)) {
    const declaration = statement.declarationList.declarations.find(
      (candidate) => ts.isIdentifier(candidate.name) && candidate.name.text === name,
    );
    if (
      declaration?.initializer &&
      (ts.isArrowFunction(declaration.initializer) || ts.isFunctionExpression(declaration.initializer))
    ) {
      return 'function';
    }
    const loweredDeclaration = declaration
      ? lowerTypeScriptFile(
          source,
          specifier.startsWith('@flighthq/') ? specifier : '@flighthq/internal',
          workspaceDirectory,
        ).declarations.find((item) => item.kind === 'variable' && item.name === name)
      : undefined;
    if (loweredDeclaration?.kind === 'variable' && isNumericNamespaceInitializer(loweredDeclaration.initializer)) {
      return 'type';
    }
    return 'constant';
  }
  if (
    statement &&
    (ts.isClassDeclaration(statement) ||
      ts.isInterfaceDeclaration(statement) ||
      ts.isTypeAliasDeclaration(statement) ||
      ts.isEnumDeclaration(statement))
  ) {
    return 'type';
  }
  return 'value';
}

function collectImportedSemanticTypes(sourceFile: ts.SourceFile, workspaceDirectory: string): ImportedSemanticTypes {
  const cacheKey = `${workspaceDirectory}\0${sourceFile.fileName}`;
  const cached = importedSemanticTypesCache.get(cacheKey);
  if (cached) return cached;
  const types = new Map<string, IrType>();
  const enumNames = new Set<string>();
  const functions = new Map<string, IrFunctionDeclaration>();
  const typeParameters = new Map<string, readonly string[]>();
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
        const localName = element.name.text;
        const source = specifier.startsWith('.')
          ? resolveRelativeTypeScriptSource(file.fileName, specifier)
          : specifier.startsWith('@flighthq/')
            ? findPackageDeclarationSource(workspaceDirectory, specifier, name)
            : undefined;
        if (!source || !existsSync(source)) continue;
        const semanticSource = parseTypeScriptFile(source);
        const lowered = lowerTypeScriptFile(
          source,
          specifier.startsWith('@flighthq/') ? specifier : '@flighthq/internal',
          workspaceDirectory,
        );
        const declaration = lowered.declarations.find(
          (item) =>
            (item.kind === 'type' ||
              item.kind === 'enum' ||
              item.kind === 'function' ||
              (item.kind === 'variable' && item.initializer?.kind === 'function')) &&
            item.name === name,
        );
        if (declaration) {
          for (const sibling of lowered.declarations) {
            if (sibling.kind === 'type' && !types.has(sibling.name)) {
              types.set(sibling.name, sibling.type);
              typeParameters.set(sibling.name, sibling.typeParameters);
            }
            if (
              sibling.kind === 'enum' ||
              (sibling.kind === 'variable' && isNumericNamespaceInitializer(sibling.initializer))
            ) {
              enumNames.add(sibling.name);
            }
            const siblingFunction = asSemanticFunction(sibling);
            if (siblingFunction && !functions.has(siblingFunction.name)) {
              functions.set(siblingFunction.name, siblingFunction);
            }
          }
        }
        const semanticFunction = declaration ? asSemanticFunction(declaration) : undefined;
        if (semanticFunction) functions.set(localName, { ...semanticFunction, name: localName });
        visit(semanticSource);
      }
    }
  };
  visit(sourceFile);
  const result = {
    enumNames: [...enumNames].sort(),
    functions: [...functions.values()].sort((left, right) => left.name.localeCompare(right.name)),
    typeParameters: Object.fromEntries(typeParameters),
    types: Object.fromEntries(types),
  };
  importedSemanticTypesCache.set(cacheKey, result);
  return result;
}

function asSemanticFunction(
  declaration: ReturnType<typeof lowerTypeScriptSource>['declarations'][number],
): IrFunctionDeclaration | undefined {
  if (declaration.kind === 'function') return declaration;
  if (declaration.kind !== 'variable' || declaration.initializer?.kind !== 'function') return undefined;
  const returns =
    declaration.initializer.returns ?? (declaration.type?.kind === 'function' ? declaration.type.returns : undefined);
  if (!returns) return undefined;
  return {
    async: declaration.initializer.async,
    body: declaration.initializer.body,
    exported: declaration.exported,
    kind: 'function',
    name: declaration.name,
    origin: declaration.origin,
    parameters: declaration.initializer.parameters,
    returns,
    typeParameters: [],
  };
}

function resolveRelativeTypeScriptSource(sourceFile: string, specifier: string): string | undefined {
  const base = path.resolve(path.dirname(sourceFile), specifier);
  const candidates = [`${base}.ts`, `${base}.tsx`, path.join(base, 'index.ts'), path.join(base, 'index.tsx')];
  return candidates.find((candidate) => existsSync(candidate));
}

function findPackageDeclarationSource(
  workspaceDirectory: string,
  packageName: string,
  name: string,
): string | undefined {
  if (packageName === '@flighthq/types') return findTypeDeclarationSource(workspaceDirectory, name);
  const cacheKey = `${workspaceDirectory}\0${packageName}`;
  let index = packageDeclarationIndexCache.get(cacheKey);
  if (!index) {
    const directory = path.join(
      workspaceDirectory,
      portConfig.upstreamDirectory,
      'packages',
      packageName.replace(/^@flighthq\//u, ''),
      'src',
    );
    const declarations = new Map<string, string>();
    for (const file of walkTypeScriptSources(directory)) {
      const source = parseTypeScriptFile(file);
      for (const statement of source.statements) {
        if (
          (ts.isInterfaceDeclaration(statement) ||
            ts.isTypeAliasDeclaration(statement) ||
            ts.isClassDeclaration(statement) ||
            ts.isEnumDeclaration(statement) ||
            ts.isFunctionDeclaration(statement)) &&
          statement.name
        ) {
          declarations.set(statement.name.text, file);
        } else if (ts.isVariableStatement(statement)) {
          for (const declaration of statement.declarationList.declarations) {
            if (ts.isIdentifier(declaration.name)) declarations.set(declaration.name.text, file);
          }
        }
      }
    }
    index = declarations;
    packageDeclarationIndexCache.set(cacheKey, index);
  }
  return index.get(name);
}

function findTypeDeclarationSource(workspaceDirectory: string, name: string): string | undefined {
  const directory = path.join(workspaceDirectory, portConfig.upstreamDirectory, 'packages', 'types', 'src');
  const conventional = path.join(directory, `${name}.ts`);
  if (existsSync(conventional)) return conventional;
  let index = typeDeclarationIndexCache.get(workspaceDirectory);
  if (!index) {
    const declarations = new Map<string, string>();
    for (const file of walkTypeScriptSources(directory)) {
      const source = parseTypeScriptFile(file);
      for (const statement of source.statements) {
        if (
          (ts.isInterfaceDeclaration(statement) ||
            ts.isTypeAliasDeclaration(statement) ||
            ts.isClassDeclaration(statement) ||
            ts.isEnumDeclaration(statement) ||
            ts.isFunctionDeclaration(statement)) &&
          statement.name
        ) {
          declarations.set(statement.name.text, file);
        } else if (ts.isVariableStatement(statement)) {
          for (const declaration of statement.declarationList.declarations) {
            if (ts.isIdentifier(declaration.name)) declarations.set(declaration.name.text, file);
          }
        }
      }
    }
    index = declarations;
    typeDeclarationIndexCache.set(workspaceDirectory, index);
  }
  return index.get(name);
}

function collectTypeEnumNames(workspaceDirectory: string): readonly string[] {
  const cached = typeEnumNamesCache.get(workspaceDirectory);
  if (cached) return cached;
  const directory = path.join(workspaceDirectory, portConfig.upstreamDirectory, 'packages', 'types', 'src');
  const names = walkTypeScriptSources(directory)
    .flatMap((file) =>
      parseTypeScriptFile(file).statements.flatMap((statement) =>
        ts.isEnumDeclaration(statement) ? [statement.name.text] : [],
      ),
    )
    .sort();
  typeEnumNamesCache.set(workspaceDirectory, names);
  return names;
}

function parseTypeScriptFile(file: string): ts.SourceFile {
  const cached = parsedSourceCache.get(file);
  if (cached) return cached;
  const source = ts.createSourceFile(file, readFileSync(file, 'utf8'), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  parsedSourceCache.set(file, source);
  return source;
}

function lowerTypeScriptFile(
  file: string,
  packageName: string,
  workspaceDirectory: string,
): ReturnType<typeof lowerTypeScriptSource> {
  const cacheKey = `${workspaceDirectory}\0${packageName}\0${file}`;
  const cached = loweredSourceCache.get(cacheKey);
  if (cached) return cached;
  const lowered = lowerTypeScriptSource(parseTypeScriptFile(file), packageName, workspaceDirectory);
  loweredSourceCache.set(cacheKey, lowered);
  return lowered;
}

export function formatRust(content: string, source: string): string {
  try {
    return execFileSync('rustfmt', ['--emit', 'stdout', '--edition', '2024'], {
      encoding: 'utf8',
      input: content,
      stdio: ['pipe', 'pipe', 'pipe'],
    });
  } catch (error) {
    if (error && typeof error === 'object' && 'code' in error && error.code === 'ENOENT') {
      throw new Error('Required generator tool rustfmt was not found in PATH.');
    }
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

export function normalizeDiagnosticSource(workspaceDirectory: string, candidateRoot: string, source: string): string {
  const resolved = path.isAbsolute(source) ? path.normalize(source) : path.resolve(candidateRoot, source);
  const workspaceRelative = path.relative(workspaceDirectory, resolved);
  const outsideWorkspace =
    workspaceRelative === '..' || workspaceRelative.startsWith(`..${path.sep}`) || path.isAbsolute(workspaceRelative);
  if (!outsideWorkspace) return workspaceRelative.split(path.sep).join('/');

  const external = resolved.split(path.sep).join('/');
  const rustcLibrary = /(?:^|\/)rustc\/[^/]+\/(library\/.*)$/u.exec(external);
  return rustcLibrary ? `<rustc>/${rustcLibrary[1]}` : external;
}
