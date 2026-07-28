import path from 'node:path';

import {
  analyzeUpstream,
  packageNameToRustCrate,
  sourcePathToImplementationModule,
  sourcePathToRustModule,
} from '../../tools/generator/src/analyze/inventory.ts';
import { auditLowering } from '../../tools/generator/src/analyze/lowering.ts';

describe('cultivated upstream analysis', () => {
  it('accounts for every package and representative export', () => {
    const inventory = analyzeUpstream(path.resolve('.'));
    const geometry = inventory.packages.find((item) => item.name === '@flighthq/geometry');

    expect(inventory.upstreamCommit).toBe('5d24729f7360475e28a105ae0caeeaa2e1328260');
    expect(inventory.summary.packages).toBe(131);
    expect(inventory.summary.sourceFiles).toBeGreaterThan(1_000);
    expect(inventory.summary.testFiles).toBeGreaterThan(1_000);
    expect(geometry?.exports.some((item) => item.name === 'createVector2')).toBe(true);
    expect(geometry?.rustCrate).toBe('flighthq-geometry');
  });

  it('retains zero-diagnostic lowering coverage from the cultivated generator', () => {
    const audit = auditLowering(path.resolve('.'));

    expect(audit.summary.packages).toBe(131);
    expect(audit.summary.lowered).toBe(audit.summary.declarations);
    expect(audit.summary.diagnostics).toBe(0);
  });
});

describe('Rust identity mapping', () => {
  it('maps package and source identities deterministically', () => {
    expect(packageNameToRustCrate('@flighthq/render-gl')).toBe('flighthq-render-gl');
    expect(sourcePathToRustModule('upstream/packages/geometry/src/vector2.ts')).toBe('vector2');
    expect(sourcePathToRustModule('upstream/packages/render-gl/src/glShader.ts')).toBe('gl_shader');
    expect(sourcePathToRustModule('upstream/packages/menu/src/menu-templates.ts')).toBe('menu_templates');
    expect(sourcePathToRustModule('upstream/packages/signals/src/internal.ts')).toBeUndefined();
    expect(sourcePathToImplementationModule('upstream/packages/signals/src/internal.ts')).toBe('_internal_internal');
  });
});
