import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import ts from 'typescript';

import { portConfig } from '../../tools/generator/port.config.ts';
import {
  collectSelectedDeclarationSupport,
  filterUnusedValueImports,
  formatRust,
  promotePublicSignatureSupport,
  classifyImportedRustBinding,
  emittedPortableTaskUsesOpaqueHostValue,
  normalizeDiagnosticSource,
  rustDependencyForSpecifier,
  validateAsyncTaskDispositionPartition,
  validateCandidateCrateGraph,
  validateCompatibilityCrateTargets,
  validateTaskConstructionDispositionPartition,
  type CandidateCrateNode,
  type RustGenerationReport,
} from '../../tools/generator/src/emit/core.ts';
import type { RustImport } from '../../tools/generator/src/emit/rust.ts';
import { lowerTypeScriptSource } from '../../tools/generator/src/lower/typescript.ts';

describe('generator prerequisites', () => {
  it('fails generation when rustfmt is unavailable', () => {
    const path = process.env.PATH;
    try {
      process.env.PATH = '';
      expect(() => formatRust('pub fn generated() {}\n', 'fixture.rs')).toThrow(
        'Required generator tool rustfmt was not found in PATH.',
      );
    } finally {
      if (path === undefined) delete process.env.PATH;
      else process.env.PATH = path;
    }
  });
});

describe('portable task host-value boundaries', () => {
  it('only attributes opaque host storage to the task declaration that contains it', () => {
    const syncOpaque = [
      '// Source: fixture.ts:1',
      'fn sync_host_value() -> crate::OpaqueHostValue { crate::OpaqueHostValue::Object }',
      '// Source: fixture.ts:2',
      'pub fn portable() -> crate::FlightTask<String> { crate::FlightTask::ready("ok".to_owned()) }',
    ].join('\n');
    const taskOpaque = [
      '// Source: fixture.ts:1',
      'pub fn portable() -> crate::FlightTask<String> {',
      '  let value = crate::OpaqueHostValue::Object;',
      '  crate::FlightTask::ready(value.to_string())',
      '}',
    ].join('\n');

    expect(emittedPortableTaskUsesOpaqueHostValue(syncOpaque)).toBe(false);
    expect(emittedPortableTaskUsesOpaqueHostValue(taskOpaque)).toBe(true);
  });
});

describe('manifest-lane dependency resolution', () => {
  it('maps public subpath imports to the configured package crate', () => {
    const bitmap = portConfig.targets.find((target) => target.package === '@flighthq/bitmap');
    if (!bitmap) throw new Error('Expected cultivated bitmap target');

    expect(rustDependencyForSpecifier(bitmap, '@flighthq/types/contract')).toEqual({ crate: 'flighthq-types' });
    expect(rustDependencyForSpecifier(bitmap, '@flighthq/entity/contract')).toBeUndefined();
  });

  it('classifies relative barrel re-exports from their defining declarations', () => {
    const workspace = process.cwd();
    const importer = path.join(workspace, 'upstream/packages/math/src/index.ts');

    expect(classifyImportedRustBinding(importer, './contract', 'approxEqual', workspace)).toBe('function');
    expect(classifyImportedRustBinding(importer, './contract', 'DEG_TO_RAD', workspace)).toBe('constant');
    expect(classifyImportedRustBinding(importer, './contract', 'RandomSource', workspace)).toBe('type');

    const geometryImporter = path.join(workspace, 'upstream/packages/geometry/src/enableGeometryPoolGuards.ts');
    expect(
      classifyImportedRustBinding(geometryImporter, './geometryPoolGuards', 'geometryPoolReleaseGuard', workspace),
    ).toBe('mutable');
  });
});

describe('selected declaration dependencies', () => {
  it('retains the transitive module-local types used by selected signatures', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/bitmap/src/selection.ts',
      `
        enum PixelFormat { Rgba = 'rgba' }
        interface Pixels { readonly data: ArrayLike<number>; readonly format: PixelFormat; }
        interface Comparison { readonly pixels: Pixels; }
        interface Deferred { readonly value: string; }
        export function selected(source: Comparison): number { return source.pixels.data.length; }
        export function deferred(source: Deferred): string { return source.value; }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/bitmap', '/workspace');

    expect(lowered.diagnostics).toEqual([]);
    expect([...collectSelectedDeclarationSupport(lowered.declarations, new Set(['selected']))].sort()).toEqual([
      'Comparison',
      'PixelFormat',
      'Pixels',
      'selected',
    ]);

    const selectedNames = collectSelectedDeclarationSupport(lowered.declarations, new Set(['selected']));
    const declarations = promotePublicSignatureSupport(
      lowered.declarations.filter((declaration) => selectedNames.has(declaration.name)),
    );
    expect(
      declarations
        .filter((declaration) => declaration.exported)
        .map(({ name }) => name)
        .sort(),
    ).toEqual(['Comparison', 'PixelFormat', 'Pixels', 'selected']);
  });

  it('drops value imports referenced only by declarations outside the selection', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/bitmap/src/import-selection.ts',
      `
        const retained = 1;
        export function selected(): number { return retained; }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/bitmap', '/workspace');
    const selected = lowered.declarations.filter((declaration) => declaration.name === 'selected');
    const imports: RustImport[] = [
      {
        module: 'crate',
        names: [
          { imported: 'retained', kind: 'value', local: 'retained' },
          { imported: 'deferred', kind: 'function', local: 'deferred' },
          { imported: 'Comparison', kind: 'type', local: 'Comparison' },
          { imported: 'public_value', kind: 'value', local: 'public_value', public: true },
        ],
      },
    ];

    expect(lowered.diagnostics).toEqual([]);
    expect(filterUnusedValueImports(imports, selected)).toEqual([
      {
        module: 'crate',
        names: [
          { imported: 'retained', kind: 'value', local: 'retained' },
          { imported: 'Comparison', kind: 'type', local: 'Comparison' },
          { imported: 'public_value', kind: 'value', local: 'public_value', public: true },
        ],
      },
    ]);
  });
});

describe('candidate crate resolution', () => {
  const candidateTypes = {
    crate: 'flighthq-types',
    fullyPromotedTarget: false,
    package: '@flighthq/types',
    requiredDependencies: [],
  } satisfies CandidateCrateNode;

  it('rejects a fully promoted package whose emitted dependency closure is not fully promoted', () => {
    expect(() =>
      validateCandidateCrateGraph([
        candidateTypes,
        {
          crate: 'flighthq-easing',
          fullyPromotedTarget: true,
          package: '@flighthq/easing',
          requiredDependencies: [{ crate: 'flighthq-types', package: '@flighthq/types' }],
        },
      ]),
    ).toThrow('Fully promoted package @flighthq/easing depends on non-fully-promoted package @flighthq/types');
  });

  it('allows a promoted package to declare a dependency that its emitted crate does not reference', () => {
    expect(() =>
      validateCandidateCrateGraph([
        candidateTypes,
        {
          crate: 'flighthq-easing',
          fullyPromotedTarget: true,
          package: '@flighthq/easing',
          requiredDependencies: [],
        },
      ]),
    ).not.toThrow();
  });

  it('rejects duplicate Cargo identities and dependency edges that disagree with the resolution map', () => {
    expect(() =>
      validateCandidateCrateGraph([
        candidateTypes,
        {
          crate: 'flighthq-types',
          fullyPromotedTarget: false,
          package: '@flighthq/other-types',
          requiredDependencies: [],
        },
      ]),
    ).toThrow('Duplicate candidate Cargo package identity flighthq-types: @flighthq/types and @flighthq/other-types');

    expect(() =>
      validateCandidateCrateGraph([
        candidateTypes,
        {
          crate: 'flighthq-tween',
          fullyPromotedTarget: false,
          package: '@flighthq/tween',
          requiredDependencies: [{ crate: 'flighthq-renamed-types', package: '@flighthq/types' }],
        },
      ]),
    ).toThrow(
      'Candidate dependency edge @flighthq/tween -> @flighthq/types names flighthq-renamed-types, but the resolution map selects flighthq-types',
    );
  });

  it('has no compatibility-named target left: bitmap generates under its canonical crate', () => {
    // Upstream renamed `surface` to `bitmap`, and this repository carried `flighthq-surface` for a
    // while so the shipped facade kept working. That migration is finished, so no target should
    // declare the exception any more — a reappearance means someone reintroduced a divergent crate
    // name without the migration path architecture.md requires.
    for (const target of portConfig.targets) {
      // Read through a widening cast on purpose: with no target declaring it, the literal's inferred
      // union no longer carries the optional property at all, and `target.compatibilityForCrate`
      // stops compiling. That absence is exactly what this asserts, so the cast keeps the assertion
      // meaningful — and it starts failing again the moment someone reintroduces the field.
      const declared = (target as { compatibilityForCrate?: string }).compatibilityForCrate;
      expect(declared, `${target.package} declares no naming exception`).toBeUndefined();
    }
  });

  it('still rejects an automatic candidate beside a compatibility crate for the same package', () => {
    // The validator outlives the exception it was written for: it is what any FUTURE compatibility
    // crate would be held to. Exercised with a synthetic target rather than a live one, so removing
    // the last real exception does not delete the guard with it.
    const compatibility = {
      compatibilityForCrate: 'flighthq-widget',
      crate: 'flighthq-widget-compat',
      package: '@flighthq/widget',
    };

    expect(() =>
      validateCompatibilityCrateTargets(
        [{ crate: 'flighthq-widget', disposition: 'generated', package: '@flighthq/widget' }],
        [compatibility],
      ),
    ).toThrow(
      'Compatibility crate flighthq-widget-compat contains @flighthq/widget definitions while automatic candidate flighthq-widget is enabled; only one may materialize',
    );

    expect(() =>
      validateCompatibilityCrateTargets(
        [{ crate: 'flighthq-widget', disposition: 'cultivated', package: '@flighthq/widget' }],
        [compatibility],
      ),
    ).not.toThrow();

    expect(() =>
      validateCompatibilityCrateTargets(
        [{ crate: 'flighthq-widget-v2', disposition: 'cultivated', package: '@flighthq/widget' }],
        [compatibility],
      ),
    ).toThrow(
      'Compatibility crate flighthq-widget-compat declares canonical crate flighthq-widget for @flighthq/widget, but inventory selects flighthq-widget-v2',
    );
  });
});

describe('async task disposition reporting', () => {
  it('rejects a scope without a disposition instead of dropping it from the partition', () => {
    const report = JSON.parse(
      readFileSync(path.join(process.cwd(), 'reports/generation.json'), 'utf8'),
    ) as RustGenerationReport;
    const scope = report.asyncTasks.packages.flatMap((item) => item.scopes)[0]!;

    expect(() => validateAsyncTaskDispositionPartition([{ ...scope, disposition: undefined as never }])).toThrow(
      'Async task disposition partition is incomplete.',
    );
  });

  it('rejects an undisposed task construction instead of hiding a task value', () => {
    const report = JSON.parse(
      readFileSync(path.join(process.cwd(), 'reports/generation.json'), 'utf8'),
    ) as RustGenerationReport;
    const construction = report.asyncTasks.packages.flatMap((item) => item.constructions)[0]!;

    expect(() =>
      validateTaskConstructionDispositionPartition([{ ...construction, disposition: undefined as never }]),
    ).toThrow('Task construction disposition partition is incomplete.');
  });
});

describe('compiler diagnostic source paths', () => {
  it('normalizes external rustc paths independently of checkout depth', () => {
    const source = '/rustc/8bab26f4f68e0e26f0bb7960be334d5b520ea452/library/alloc/src/macros.rs';
    const shallowWorkspace = '/tmp/repo';
    const deepWorkspace = '/tmp/one/two/three/repo';

    expect(
      normalizeDiagnosticSource(shallowWorkspace, path.join(shallowWorkspace, 'generated/candidates'), source),
    ).toBe('<rustc>/library/alloc/src/macros.rs');
    expect(normalizeDiagnosticSource(deepWorkspace, path.join(deepWorkspace, 'generated/candidates'), source)).toBe(
      '<rustc>/library/alloc/src/macros.rs',
    );
  });

  it('keeps candidate sources relative to the workspace without escaping it', () => {
    const workspace = '/tmp/repo';
    const candidateRoot = path.join(workspace, 'generated/candidates');

    expect(normalizeDiagnosticSource(workspace, candidateRoot, 'flighthq-example/src/lib.rs')).toBe(
      'generated/candidates/flighthq-example/src/lib.rs',
    );
    expect(normalizeDiagnosticSource(workspace, candidateRoot, '/opt/rust/library/example.rs')).toBe(
      '/opt/rust/library/example.rs',
    );
  });

  it('keeps generated JSON and Markdown source paths free of parent traversal prefixes', () => {
    for (const file of ['generated/manifest.json', 'reports/generation.json']) {
      const report = JSON.parse(readFileSync(path.join(process.cwd(), file), 'utf8')) as unknown;
      expect(collectReportSources(report).filter((source) => source.startsWith('../'))).toEqual([]);
    }
    expect(readFileSync(path.join(process.cwd(), 'reports/generation.md'), 'utf8')).not.toMatch(/`\.\.\//u);
  });

  it('blocks opaque-source growth outside declared host backends', () => {
    const report = JSON.parse(
      readFileSync(path.join(process.cwd(), 'reports/generation.json'), 'utf8'),
    ) as RustGenerationReport;
    const baseline = portConfig.opaqueHostValueBaseline as Readonly<Record<string, number>>;

    for (const item of report.automaticPackages) {
      if (item.disposition === 'host-backend') continue;
      const opaqueSources = item.emittedSources.filter((source) => source.usesOpaqueHostValues).length;
      expect(opaqueSources, item.package).toBeLessThanOrEqual(baseline[item.package] ?? 0);
    }

    expect(report.automaticPackages.find((item) => item.package === '@flighthq/render-wgpu')?.disposition).toBe(
      'host-backend',
    );
    const tween = report.automaticPackages.find((item) => item.package === '@flighthq/tween');
    expect(tween?.candidate.status).toBe('source-blocked');
    expect(
      tween?.blockers.some(
        (blocker) =>
          blocker.source === 'upstream/packages/tween/src/timer.ts' &&
          blocker.reason.includes('requires OpaqueHostValue after static type recovery'),
      ),
    ).toBe(true);
  });

  it('resolves dependency-closed promotions through one Cargo identity', () => {
    const report = JSON.parse(
      readFileSync(path.join(process.cwd(), 'reports/generation.json'), 'utf8'),
    ) as RustGenerationReport;
    const types = report.automaticPackages.find((item) => item.package === '@flighthq/types');
    const easing = report.automaticPackages.find((item) => item.package === '@flighthq/easing');
    const tween = report.automaticPackages.find((item) => item.package === '@flighthq/tween');

    expect(types?.fullyPromotedTarget).toBe(true);
    expect(types?.candidate.status).toBe('promoted');
    expect(easing?.fullyPromotedTarget).toBe(true);
    expect(easing?.requiredDependencies).toContainEqual({
      crate: 'flighthq-types',
      package: '@flighthq/types',
    });
    expect(tween?.candidate.status).not.toBe('dependency-blocked');
    expect(tween?.requiredDependencies).toContainEqual({
      crate: 'flighthq-types',
      package: '@flighthq/types',
    });
    expect(report.summary.candidateCompiled).toBe(25);
    expect(report.asyncTasks.summary).toMatchObject({
      eligibleConstructions: 229,
      eligibleScopes: 177,
      hostPlaceholderScopes: 0,
      operations: {
        asyncIterations: 3,
        awaits: 207,
      },
      portableExecutableConstructions: 26,
      portableExecutableScopes: 20,
      unsupportedConstructions: 203,
      unsupportedScopes: 157,
    });
    expect(report.asyncTasks.summary.eligibleScopes).toBe(
      report.asyncTasks.summary.portableExecutableScopes +
        report.asyncTasks.summary.hostPlaceholderScopes +
        report.asyncTasks.summary.unsupportedScopes,
    );
    expect(report.asyncTasks.summary.eligibleConstructions).toBe(
      report.asyncTasks.summary.portableExecutableConstructions +
        report.asyncTasks.summary.hostPlaceholderConstructions +
        report.asyncTasks.summary.unsupportedConstructions,
    );
    const asyncScopes = report.asyncTasks.packages.flatMap((item) => item.scopes);
    expect(asyncScopes).toHaveLength(177);
    expect(
      asyncScopes.every(
        (scope) =>
          ['portable-executable', 'unsupported'].includes(scope.disposition) &&
          scope.package.length > 0 &&
          scope.source.length > 0 &&
          scope.lexicalPath.length > 0 &&
          scope.line > 0 &&
          scope.column > 0 &&
          (scope.disposition === 'portable-executable' || Boolean(scope.reason)) &&
          /^sha256:[0-9a-f]{64}$/u.test(scope.fingerprint),
      ),
    ).toBe(true);
    expect(
      report.asyncTasks.summary.unsupportedReasons.find((item) =>
        item.reason.startsWith('Async output type is not recovered'),
      )?.scopes,
    ).toBe(8);
    expect(
      report.asyncTasks.summary.unsupportedReasons.find((item) =>
        item.reason.startsWith('Portable task source still requires'),
      )?.scopes,
    ).toBe(5);
    const taskConstructions = report.asyncTasks.packages.flatMap((item) => item.constructions);
    expect(taskConstructions).toHaveLength(229);
    expect(taskConstructions.filter((item) => item.kind === 'ready')).toHaveLength(21);
    expect(
      report.asyncTasks.summary.unsupportedConstructionReasons.find((item) =>
        item.reason.startsWith('taskAll Rust lowering is implemented'),
      )?.constructions,
    ).toBe(1);
    expect(
      taskConstructions.every(
        (construction) =>
          construction.package.length > 0 &&
          construction.source.length > 0 &&
          construction.lexicalPath.length > 0 &&
          construction.line > 0 &&
          construction.column > 0 &&
          (construction.disposition === 'portable-executable' || Boolean(construction.reason)) &&
          /^sha256:[0-9a-f]{64}$/u.test(construction.fingerprint),
      ),
    ).toBe(true);
    const portableSources = report.automaticPackages
      .filter((item) => item.disposition === 'generated')
      .flatMap((item) => item.emittedSources);
    const portableOpaqueSources = portableSources.filter((source) => source.usesOpaqueHostValues);
    expect(portableSources).toHaveLength(1813);
    expect(portableOpaqueSources).toHaveLength(56);
    expect(portableOpaqueSources.length / portableSources.length).toBeLessThanOrEqual(167 / 1227);
    const screen = report.automaticPackages.find((item) => item.package === '@flighthq/screen');
    expect(screen?.candidate.status).toBe('compiled');
    expect(screen?.asyncTasks).toHaveLength(2);
    expect(screen?.asyncTasks.every((scope) => scope.disposition === 'portable-executable')).toBe(true);
    const screenTarget = report.targets.find((item) => item.package === '@flighthq/screen');
    expect(screenTarget?.emittedSources).toEqual([
      expect.objectContaining({
        declarationNames: ['_backend', 'setScreenBackend'],
        source: 'upstream/packages/screen/src/screen.ts',
      }),
    ]);
    expect(screenTarget?.deferredDeclarations.map((item) => item.name)).toEqual(
      expect.arrayContaining(['getScreenDetailPermission', 'requestScreenDetails']),
    );
    expect(report.conformance.summary).toMatchObject({
      passingCases: 45,
      passingTestFiles: 3,
      totalUpstreamTestFiles: 1561,
      translatedCases: 45,
      translatedTestFiles: 3,
    });
    expect(existsSync(path.join(process.cwd(), 'generated/candidates/flighthq-types'))).toBe(false);
    expect(
      readFileSync(path.join(process.cwd(), 'generated/candidates/flighthq-signals/Cargo.toml'), 'utf8'),
    ).toContain('flighthq-types = { path = "../../crates/flighthq-types" }');
    expect(
      readFileSync(path.join(process.cwd(), 'generated/candidates/flighthq-signals/Cargo.toml'), 'utf8'),
    ).toContain('flighthq-runtime = { path = "../../crates/flighthq-runtime" }');
    expect(readFileSync(path.join(process.cwd(), 'generated/candidates/flighthq-input/Cargo.toml'), 'utf8')).toContain(
      'flighthq-signals = { path = "../flighthq-signals" }',
    );
    expect(
      readFileSync(path.join(process.cwd(), 'generated/candidates/flighthq-input/src/input_manager.rs'), 'utf8'),
    ).toContain('use flighthq_signals::{connect_signal, create_signal, disconnect_signal, emit_signal};');
    expect(readFileSync(path.join(process.cwd(), 'generated/crates/flighthq-types/Cargo.toml'), 'utf8')).toContain(
      'flighthq-runtime = { path = "../flighthq-runtime" }',
    );
    expect(
      readFileSync(path.join(process.cwd(), 'generated/crates/flighthq-bitmap-wasm/Cargo.toml'), 'utf8'),
    ).toContain('flighthq-runtime = { path = "../flighthq-runtime" }');
    expect(
      readFileSync(
        path.join(process.cwd(), 'generated/candidates/flighthq-math/src/__flight_upstream_conformance.rs'),
        'utf8',
      ),
    ).toContain('// @generated from upstream @flighthq/math tests; do not edit.');
    expect(
      readFileSync(
        path.join(process.cwd(), 'generated/candidates/flighthq-math/src/__flight_upstream_conformance.rs'),
        'utf8',
      ),
    ).toContain('install_deterministic_flight_task_scheduler');
    expect(readFileSync(path.join(process.cwd(), 'crates/flighthq-host-winit/Cargo.toml'), 'utf8')).toContain(
      'flighthq-types = { path = "../../generated/crates/flighthq-types" }',
    );
    expect(readFileSync(path.join(process.cwd(), 'crates/flighthq-host-winit/Cargo.toml'), 'utf8')).toContain(
      'flighthq-screen = { path = "../../generated/crates/flighthq-screen" }',
    );
    for (const crate of ['application', 'input', 'power']) {
      expect(readFileSync(path.join(process.cwd(), 'crates/flighthq-host-winit/Cargo.toml'), 'utf8')).toContain(
        `flighthq-${crate} = { path = "../../generated/crates/flighthq-${crate}" }`,
      );
    }
    expect(readFileSync(path.join(process.cwd(), 'generated/crates/flighthq-input/Cargo.toml'), 'utf8')).toContain(
      'flighthq-host-signals = { path = "../flighthq-host-signals" }',
    );
    const runtimeSource = readFileSync(
      path.join(process.cwd(), 'generated/crates/flighthq-runtime/src/lib.rs'),
      'utf8',
    );
    expect(runtimeSource).toContain('#![forbid(unsafe_code)]');
    expect(runtimeSource).not.toMatch(/\bunsafe\s*\{/u);
    expect(readFileSync(path.join(process.cwd(), 'generated/crates/flighthq-types/src/lib.rs'), 'utf8')).not.toContain(
      'pub struct Promise',
    );
  });
});

function collectReportSources(value: unknown): string[] {
  if (Array.isArray(value)) return value.flatMap((item) => collectReportSources(item));
  if (!value || typeof value !== 'object') return [];
  return Object.entries(value).flatMap(([key, item]) =>
    key === 'source' && typeof item === 'string' ? [item] : collectReportSources(item),
  );
}
