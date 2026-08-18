import { mkdirSync, mkdtempSync, readFileSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';

import { portConfig } from '../../tools/generator/port.config.ts';
import { harvestConformance } from '../../tools/generator/src/conformance/harvest.ts';
import type { UpstreamInventory } from '../../tools/generator/src/model/inventory.ts';

describe('generated upstream conformance harvest', () => {
  it('translates complete pure math and color files and reports the remaining scope', () => {
    const workspace = process.cwd();
    const inventory = JSON.parse(
      readFileSync(path.join(workspace, 'reports/inventory.json'), 'utf8'),
    ) as UpstreamInventory;
    const harvest = harvestConformance(
      workspace,
      inventory.packages,
      portConfig.conformanceHarvest,
      inventory.summary.testFiles,
    );

    expect(harvest.report.summary).toMatchObject({
      inScopeTestFiles: 24,
      outOfScopeTestFiles: 1142,
      totalUpstreamTestFiles: 1166,
      translatedCases: 45,
      translatedTestFiles: 4,
      unsupportedTestFiles: 20,
    });
    expect(harvest.report.packages.find((item) => item.package === '@flighthq/math')).toMatchObject({
      translatedCases: 39,
      translatedTestFiles: 3,
      unsupportedTestFiles: 12,
    });
    expect(harvest.report.packages.find((item) => item.package === '@flighthq/color')).toMatchObject({
      translatedCases: 6,
      translatedTestFiles: 1,
      unsupportedTestFiles: 8,
    });

    const math = harvest.rustModules.get('@flighthq/math');
    const color = harvest.rustModules.get('@flighthq/color');
    expect(math?.match(/#\[test\]/gu)).toHaveLength(39);
    expect(math).toContain('assert!((crate::clamp(f64::NAN, 0.0_f64, 10.0_f64)).is_nan());');
    expect(math).toContain('crate::approx_equal(1.0_f64, 1.0_f64, None)');
    expect(math).toContain('flight_close((180.0_f64 * crate::DEG_TO_RAD), std::f64::consts::PI, 10_i32);');
    expect(color?.match(/#\[test\]/gu)).toHaveLength(6);
    expect(color).toContain('flight_close(crate::linear_channel_to_srgb(1.0_f64), 1.0_f64, 8_i32);');
  });

  it('reports a selected test case when its matcher is unsupported', () => {
    const workspace = mkdtempSync(path.join(tmpdir(), 'flight-rs-conformance-'));
    const sourceDirectory = path.join(workspace, 'upstream/packages/example/src');
    mkdirSync(sourceDirectory, { recursive: true });
    writeFileSync(path.join(sourceDirectory, 'value.ts'), 'export function value(): number { return 1; }\n');
    writeFileSync(
      path.join(sourceDirectory, 'value.test.ts'),
      [
        "import { value } from './value';",
        "it('requires an unsupported matcher', () => {",
        '  expect(value()).toThrow();',
        '});',
      ].join('\n'),
    );

    const harvest = harvestConformance(
      workspace,
      [
        {
          dependencies: [],
          directory: 'upstream/packages/example',
          exportConflicts: [],
          exportLanes: [],
          exports: [],
          name: '@flighthq/example',
          rustCrate: 'flighthq-example',
          sdkExposures: [],
          sdkIncluded: false,
          sourceFiles: 1,
          testFiles: 1,
          version: '0.0.0',
        },
      ],
      [
        {
          package: '@flighthq/example',
          sources: ['value.test.ts'],
          unsupportedReason: 'not selected',
        },
      ],
      1,
    );

    expect(harvest.report.summary).toMatchObject({
      translatedCases: 0,
      translatedTestFiles: 0,
      unsupportedTestFiles: 1,
    });
    expect(harvest.report.packages[0]?.testFiles[0]).toMatchObject({
      status: 'partial',
      unsupported: [
        {
          line: 2,
          reason: 'unsupported expect matcher: toThrow',
          test: 'requires an unsupported matcher',
        },
      ],
    });
  });
});
