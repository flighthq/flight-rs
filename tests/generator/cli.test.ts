const mocks = vi.hoisted(() => ({
  auditLowering: vi.fn(),
  generateRust: vi.fn(),
  writeOrCheck: vi.fn(),
}));

vi.mock('../../tools/generator/src/analyze/inventory.ts', () => ({
  analyzeUpstream: () => ({
    packages: [],
    schemaVersion: 2,
    summary: {
      exportConflicts: 0,
      exportLanes: 0,
      exports: 0,
      packages: 0,
      rootExports: 0,
      sourceFiles: 0,
      testFiles: 0,
    },
    upstreamCommit: 'recorded-upstream',
  }),
}));

vi.mock('../../tools/generator/src/analyze/lowering.ts', () => ({
  auditLowering: mocks.auditLowering,
}));

vi.mock('../../tools/generator/src/emit/core.ts', () => ({
  generateRust: mocks.generateRust,
}));

vi.mock('../../tools/generator/src/emit/reports.ts', () => ({
  createApiReport: () => ({}),
  generationSummary: () => '',
  inventorySummary: () => '',
  loweringSummary: () => '',
  stableJson: () => '',
  writeOrCheck: mocks.writeOrCheck,
}));

describe('generator CLI report writes', () => {
  it('does not update inventory reports when later generation fails', async () => {
    const arguments_ = process.argv;
    const exitCode = process.exitCode;
    const stderr = vi.spyOn(process.stderr, 'write').mockImplementation(() => true);
    try {
      process.argv = ['node', 'tools/generator/src/cli.ts'];
      process.exitCode = undefined;
      mocks.auditLowering.mockReturnValue({
        packages: [],
        schemaVersion: 1,
        summary: { declarations: 0, diagnostics: 0, files: 0, lowered: 0, packages: 0 },
      });
      mocks.generateRust.mockImplementation(() => {
        throw new Error('generation failed');
      });

      await import('../../tools/generator/src/cli.ts');

      expect(mocks.writeOrCheck).not.toHaveBeenCalled();
      expect(process.exitCode).toBe(1);
      expect(stderr).toHaveBeenCalledWith('generation failed\n');
    } finally {
      process.argv = arguments_;
      process.exitCode = exitCode;
      stderr.mockRestore();
    }
  });
});
