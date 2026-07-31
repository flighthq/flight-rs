import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';

import { portConfig } from '../../tools/generator/port.config.ts';
import {
  formatRust,
  normalizeDiagnosticSource,
  validateAsyncTaskDispositionPartition,
  validateCandidateCrateGraph,
  validateTaskConstructionDispositionPartition,
  type CandidateCrateNode,
  type RustGenerationReport,
} from '../../tools/generator/src/emit/core.ts';

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

describe('candidate crate resolution', () => {
  const candidateTypes = {
    crate: 'flighthq-types',
    dependencies: [],
    fullyPromotedTarget: false,
    package: '@flighthq/types',
  } satisfies CandidateCrateNode;

  it('rejects a fully promoted package whose dependency closure is not fully promoted', () => {
    expect(() =>
      validateCandidateCrateGraph([
        candidateTypes,
        {
          crate: 'flighthq-easing',
          dependencies: [{ crate: 'flighthq-types', package: '@flighthq/types' }],
          fullyPromotedTarget: true,
          package: '@flighthq/easing',
        },
      ]),
    ).toThrow('Fully promoted package @flighthq/easing depends on non-fully-promoted package @flighthq/types');
  });

  it('rejects duplicate Cargo identities and dependency edges that disagree with the resolution map', () => {
    expect(() =>
      validateCandidateCrateGraph([
        candidateTypes,
        {
          crate: 'flighthq-types',
          dependencies: [],
          fullyPromotedTarget: false,
          package: '@flighthq/other-types',
        },
      ]),
    ).toThrow('Duplicate candidate Cargo package identity flighthq-types: @flighthq/types and @flighthq/other-types');

    expect(() =>
      validateCandidateCrateGraph([
        candidateTypes,
        {
          crate: 'flighthq-tween',
          dependencies: [{ crate: 'flighthq-renamed-types', package: '@flighthq/types' }],
          fullyPromotedTarget: false,
          package: '@flighthq/tween',
        },
      ]),
    ).toThrow(
      'Candidate dependency edge @flighthq/tween -> @flighthq/types names flighthq-renamed-types, but the resolution map selects flighthq-types',
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
    expect(report.summary.candidateCompiled).toBe(23);
    expect(report.asyncTasks.summary).toMatchObject({
      eligibleConstructions: 204,
      eligibleScopes: 162,
      hostPlaceholderScopes: 0,
      operations: {
        asyncIterations: 3,
        awaits: 190,
      },
      portableExecutableConstructions: 9,
      portableExecutableScopes: 3,
      unsupportedConstructions: 195,
      unsupportedScopes: 159,
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
    expect(asyncScopes).toHaveLength(162);
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
    ).toBe(85);
    expect(
      report.asyncTasks.summary.unsupportedReasons.find((item) =>
        item.reason.startsWith('Portable task source still requires'),
      )?.scopes,
    ).toBe(20);
    const taskConstructions = report.asyncTasks.packages.flatMap((item) => item.constructions);
    expect(taskConstructions).toHaveLength(204);
    expect(taskConstructions.filter((item) => item.kind === 'ready')).toHaveLength(19);
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
    expect(portableSources).toHaveLength(1226);
    expect(portableOpaqueSources).toHaveLength(166);
    expect(portableOpaqueSources.length / portableSources.length).toBeLessThanOrEqual(167 / 1227);
    const screen = report.automaticPackages.find((item) => item.package === '@flighthq/screen');
    expect(screen?.candidate.status).toBe('source-blocked');
    expect(screen?.asyncTasks).toHaveLength(2);
    expect(
      screen?.asyncTasks.every(
        (scope) =>
          scope.disposition === 'unsupported' && scope.reason?.startsWith('Portable task source still requires'),
      ),
    ).toBe(true);
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
      passingTestFiles: 4,
      totalUpstreamTestFiles: 1166,
      translatedCases: 45,
      translatedTestFiles: 4,
    });
    expect(existsSync(path.join(process.cwd(), 'generated/candidates/flighthq-types'))).toBe(false);
    expect(
      readFileSync(path.join(process.cwd(), 'generated/candidates/flighthq-signals/Cargo.toml'), 'utf8'),
    ).toContain('flighthq-types = { path = "../../crates/flighthq-types" }');
    expect(
      readFileSync(path.join(process.cwd(), 'generated/candidates/flighthq-signals/Cargo.toml'), 'utf8'),
    ).toContain('flighthq-runtime = { path = "../../crates/flighthq-runtime" }');
    expect(readFileSync(path.join(process.cwd(), 'generated/crates/flighthq-types/Cargo.toml'), 'utf8')).toContain(
      'flighthq-runtime = { path = "../flighthq-runtime" }',
    );
    expect(
      readFileSync(path.join(process.cwd(), 'generated/crates/flighthq-surface-wasm/Cargo.toml'), 'utf8'),
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
