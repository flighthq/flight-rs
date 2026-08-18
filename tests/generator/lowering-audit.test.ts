import { assertCompleteLoweringAudit, type LoweringAudit } from '../../tools/generator/src/analyze/lowering.ts';

describe('lowering audit policy', () => {
  it('accepts complete zero-diagnostic coverage', () => {
    expect(() => assertCompleteLoweringAudit(audit())).not.toThrow();
  });

  it('reports incomplete packages without rerunning the exhaustive audit', () => {
    const incomplete = audit();
    incomplete.packages[0]!.lowered = 2;
    incomplete.packages[0]!.diagnostics.push({
      column: 4,
      line: 3,
      message: 'unsupported construct',
      source: 'upstream/packages/example/src/example.ts',
    });
    incomplete.summary.lowered = 2;
    incomplete.summary.diagnostics = 1;

    expect(() => assertCompleteLoweringAudit(incomplete)).toThrow(
      'Lowering coverage regression: 2/3 declarations lowered with 1 diagnostic across 1 package: @flighthq/example (2/3 lowered, 1 diagnostic)',
    );
  });
});

function audit(): LoweringAudit {
  return {
    packages: [
      {
        declarations: 3,
        diagnostics: [],
        files: 1,
        lowered: 3,
        packageName: '@flighthq/example',
      },
    ],
    schemaVersion: 1,
    summary: {
      declarations: 3,
      diagnostics: 0,
      files: 1,
      lowered: 3,
      packages: 1,
    },
  };
}
