import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import ts from 'typescript';

import { wasmGlueFiles } from '../../packages/bitmap-rs/scripts/copy-wasm-glue.ts';
import { portConfig } from '../../tools/generator/port.config.ts';

// `packages/bitmap-rs` is the only package in this repository published to npm. `config.test.ts`
// already proves each wasm facade export exists as a declaration in its core crate; these cover the
// other half of the boundary — that the TypeScript facade exposes exactly that set, that the set is
// genuinely a drop-in for upstream, and that the manifest can actually be published.

const workspace = path.resolve('.');
const facadeDirectory = path.join(workspace, 'packages/bitmap-rs');

const manifest = JSON.parse(readFileSync(path.join(facadeDirectory, 'package.json'), 'utf8')) as {
  dependencies?: Record<string, string>;
  flightRsSubstitute?: { authoritativePackage?: string; crate?: string };
  scripts?: Record<string, string>;
} & Record<string, unknown>;

const substitute = manifest.flightRsSubstitute ?? {};
const authoritativePackage = substitute.authoritativePackage ?? '@flighthq/bitmap';

// The facade names the crate it is built from; that is the row of `wasmFacades` whose export list
// this package must mirror. Looking it up by name rather than position keeps the two in step if a
// second wasm facade is ever added.
const surfaceFacade = portConfig.wasmFacades.find((facade) => facade.coreCrate === substitute.crate);

function parse(file: string): ts.SourceFile {
  return ts.createSourceFile(file, readFileSync(file, 'utf8'), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
}

/** Named exports re-exported from a module specifier, e.g. `export { a, b } from './x'`. */
function reexportedNames(sourceFile: ts.SourceFile, from: string): string[] {
  const names: string[] = [];
  for (const statement of sourceFile.statements) {
    if (!ts.isExportDeclaration(statement)) continue;
    if (!statement.moduleSpecifier || !ts.isStringLiteral(statement.moduleSpecifier)) continue;
    if (statement.moduleSpecifier.text !== from) continue;
    const clause = statement.exportClause;
    if (clause && ts.isNamedExports(clause)) for (const element of clause.elements) names.push(element.name.text);
  }
  return names;
}

/** Whether `export * from '<module>'` is present. */
function hasStarReexport(sourceFile: ts.SourceFile, from: string): boolean {
  return sourceFile.statements.some(
    (statement) =>
      ts.isExportDeclaration(statement) &&
      !statement.exportClause &&
      statement.moduleSpecifier !== undefined &&
      ts.isStringLiteral(statement.moduleSpecifier) &&
      statement.moduleSpecifier.text === from,
  );
}

function exportedFunctionNames(sourceFile: ts.SourceFile): string[] {
  const names: string[] = [];
  for (const statement of sourceFile.statements) {
    if (!ts.isFunctionDeclaration(statement) || !statement.name) continue;
    const exported = ts.getCombinedModifierFlags(statement) & ts.ModifierFlags.Export;
    if (exported) names.push(statement.name.text);
  }
  return names;
}

function relativeImportSpecifiers(sourceFile: ts.SourceFile): string[] {
  const specifiers: string[] = [];
  for (const statement of sourceFile.statements) {
    if (!ts.isImportDeclaration(statement)) continue;
    if (!ts.isStringLiteral(statement.moduleSpecifier)) continue;
    const specifier = statement.moduleSpecifier.text;
    if (specifier.startsWith('.')) specifiers.push(specifier);
  }
  return specifiers;
}

describe('blessed facade packaging', () => {
  it('is built from a wasm facade the generator declares', () => {
    expect(substitute.crate, 'package.json flightRsSubstitute.crate').toBeTruthy();
    expect(surfaceFacade, `port.config wasmFacades entry for ${String(substitute.crate)}`).toBeDefined();
  });

  it('shadows exactly the wasm exports the generator built, over a complete upstream re-export', () => {
    const index = parse(path.join(facadeDirectory, 'src/index.ts'));
    const shadowed = reexportedNames(index, './bitmapWasm');

    // `initBitmapWasm` is the facade's own entry point rather than an upstream name, so it is the
    // one addition the generated export set does not account for.
    expect(shadowed.filter((name) => name !== 'initBitmapWasm').sort()).toEqual(
      [...(surfaceFacade?.exports ?? [])].sort(),
    );
    expect(shadowed).toContain('initBitmapWasm');

    // Without this the package silently stops being API-complete: dropping a function from the wasm
    // slice would remove it from the facade instead of falling back to the TypeScript original.
    expect(hasStarReexport(index, authoritativePackage)).toBe(true);

    const implementation = parse(path.join(facadeDirectory, 'src/bitmapWasm.ts'));
    expect(exportedFunctionNames(implementation).sort()).toEqual(shadowed.slice().sort());
  });

  it('shadows only names the pinned upstream package actually exports', () => {
    const packageDirectory = authoritativePackage.replace(/^@flighthq\//u, '');
    const upstreamIndex = parse(
      path.join(workspace, portConfig.upstreamDirectory, 'packages', packageDirectory, 'src/index.ts'),
    );
    const upstreamExports = new Set([
      ...reexportedNames(upstreamIndex, './contract'),
      ...exportedFunctionNames(upstreamIndex),
    ]);

    for (const name of surfaceFacade?.exports ?? []) {
      expect(upstreamExports.has(name), `${name} is exported by upstream ${authoritativePackage}`).toBe(true);
    }
  });

  it('differentially tests every function it ships against the upstream implementation', () => {
    const implementation = parse(path.join(facadeDirectory, 'src/bitmapWasm.ts'));
    const parityTest = readFileSync(path.join(facadeDirectory, 'src/bitmapWasm.test.ts'), 'utf8');

    // The whole claim of this package is that the Rust kernels are indistinguishable from the
    // TypeScript ones. An export nobody compares against `reference` ships that claim untested.
    for (const name of exportedFunctionNames(implementation)) {
      expect(new RegExp(`\\brs\\.${name}\\b`, 'u').test(parityTest), `${name} is exercised by the parity suite`).toBe(
        true,
      );
    }
  });

  it('copies every non-TypeScript module the facade imports into dist', () => {
    const implementation = parse(path.join(facadeDirectory, 'src/bitmapWasm.ts'));
    const copied = new Set<string>(wasmGlueFiles);

    for (const specifier of relativeImportSpecifiers(implementation)) {
      // `tsc -b` emits JavaScript for TypeScript sources; anything imported with an explicit `.js`
      // extension resolves to a checked-in wasm-bindgen artifact that only the copy step delivers.
      if (!specifier.endsWith('.js')) continue;
      const file = path.basename(specifier);
      expect(copied.has(file), `${specifier} is copied into dist by copy-wasm-glue`).toBe(true);
      expect(existsSync(path.join(facadeDirectory, 'src/wasm', file))).toBe(true);
    }
  });

  it('declares a manifest npm can publish', () => {
    for (const field of ['name', 'version', 'description', 'license', 'author', 'repository']) {
      expect(manifest[field], `package.json ${field}`).toBeTruthy();
    }
    expect(manifest.private, 'a published package must not be private').toBeUndefined();

    // npm defaults a scoped package to restricted, which fails the publish outright on a free
    // account. Upstream passes `--access public` from its publish script; this repository has no
    // such script, so the manifest is the only place that knowledge can live.
    if (String(manifest.name).startsWith('@')) {
      expect((manifest.publishConfig as { access?: string } | undefined)?.access).toBe('public');
    }

    expect(existsSync(path.join(facadeDirectory, 'README.md'))).toBe(true);
    expect(existsSync(path.join(facadeDirectory, 'LICENSE.md'))).toBe(true);

    // `*` resolves to whatever `latest` happens to be at install time, which silently pairs the
    // generated wasm slice with an upstream release it was never differentially tested against.
    const dependencies = (manifest.dependencies ?? {}) as Record<string, string>;
    expect(Object.keys(dependencies).length).toBeGreaterThan(0);
    for (const [name, range] of Object.entries(dependencies)) {
      expect(range, `${name} dependency range`).not.toBe('*');
      expect(range, `${name} dependency range`).toMatch(/^[\^~]?\d+\.\d+\.\d+/u);
    }

    // Each `prepack` step runs at publish time, where a missing script fails the release rather
    // than a test. The repository was filtered without `scripts/clean-package-dist.ts` once already.
    const scripts = (manifest.scripts ?? {}) as Record<string, string>;
    for (const command of Object.values(scripts)) {
      const invoked = /(?:^|\s)tsx\s+(\S+\.ts)/u.exec(command)?.[1];
      if (!invoked) continue;
      expect(existsSync(path.resolve(facadeDirectory, invoked)), `${invoked} exists`).toBe(true);
    }
  });
});
