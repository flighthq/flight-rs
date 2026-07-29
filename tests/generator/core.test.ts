import { readFileSync } from 'node:fs';
import path from 'node:path';

import { formatRust, normalizeDiagnosticSource } from '../../tools/generator/src/emit/core.ts';

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
});

function collectReportSources(value: unknown): string[] {
  if (Array.isArray(value)) return value.flatMap((item) => collectReportSources(item));
  if (!value || typeof value !== 'object') return [];
  return Object.entries(value).flatMap(([key, item]) =>
    key === 'source' && typeof item === 'string' ? [item] : collectReportSources(item),
  );
}
