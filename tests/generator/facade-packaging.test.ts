import { existsSync, readFileSync, readdirSync } from 'node:fs';
import path from 'node:path';
import ts from 'typescript';

import { wasmGlueFiles as bitmapWasmGlueFiles } from '../../packages/bitmap-wasm/scripts/copy-wasm-glue.ts';
import { bitmapWasmExports } from '../../packages/bitmap-wasm/vitest.config.upstream.ts';
import { wasmGlueFiles as physics2DWasmGlueFiles } from '../../packages/physics2d-abi-wasm/scripts/copy-wasm-glue.ts';
import { wasmGlueFiles as physics3DWasmGlueFiles } from '../../packages/physics3d-abi-wasm/scripts/copy-wasm-glue.ts';
import { publishablePackages } from '../../scripts/publishable-packages.ts';
import { portConfig } from '../../tools/generator/port.config.ts';

// `config.test.ts` proves the generated-core facade exports exist in Rust. These tests cover the
// other half of the boundary: TypeScript shadowing, upstream fallback, wasm glue, and publishable
// manifests for every blessed facade.

const workspace = path.resolve('.');
const facadeDirectory = path.join(workspace, 'packages/bitmap-wasm');
const packagedGlue = [
  ['bitmap-wasm', bitmapWasmGlueFiles, 'src/bitmapWasm.ts'],
  ['physics2d-abi-wasm', physics2DWasmGlueFiles, 'src/physics2DAbiWasm.ts'],
  ['physics3d-abi-wasm', physics3DWasmGlueFiles, 'src/physics3DAbiWasm.ts'],
] as const;

const manifest = JSON.parse(readFileSync(path.join(facadeDirectory, 'package.json'), 'utf8')) as {
  dependencies?: Record<string, string>;
  flightWasmSubstitute?: { authoritativePackage?: string; crate?: string };
  scripts?: Record<string, string>;
} & Record<string, unknown>;

const substitute = manifest.flightWasmSubstitute ?? {};
const authoritativePackage = substitute.authoritativePackage ?? '@flighthq/bitmap';

// The facade names the crate it is built from; that is the row of `wasmFacades` whose export list
// this package must mirror. Looking it up by name rather than position keeps the two in step if a
// second wasm facade is ever added.
const bitmapFacade = portConfig.wasmFacades.find((facade) => facade.authoritativePackage === authoritativePackage);

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

/**
 * Every value name a package's `index.ts` actually makes public, following its re-export graph.
 *
 * Upstream's barrel shape is not stable and must not be assumed: `@flighthq/bitmap` moved from re-exporting a
 * single `./contract` to enumerating `export * from './bitmapX.ts'` per module, extension included. A resolver
 * that only understood one of those shapes reported an EMPTY public surface for the other — which turns this
 * guard from "shadows only names upstream exports" into "fails on every name", and in the opposite direction
 * would have passed anything at all.
 */
function publicValueNames(indexFile: string): Set<string> {
  const names = new Set<string>();
  const seen = new Set<string>();
  const visit = (file: string): void => {
    if (seen.has(file)) return;
    seen.add(file);
    let sourceFile: ts.SourceFile;
    try {
      sourceFile = parse(file);
    } catch {
      return;
    }
    for (const name of exportedFunctionNames(sourceFile)) names.add(name);
    for (const name of exportedConstNames(sourceFile)) names.add(name);
    for (const statement of sourceFile.statements) {
      if (!ts.isExportDeclaration(statement)) continue;
      const specifier = statement.moduleSpecifier;
      if (!specifier || !ts.isStringLiteral(specifier) || !specifier.text.startsWith('.')) continue;
      const clause = statement.exportClause;
      if (clause && ts.isNamedExports(clause)) {
        // A named re-export publishes exactly these, whatever else the target module exports.
        for (const element of clause.elements) names.add(element.name.text);
        continue;
      }
      // `export *` republishes the target's whole surface, so it has to be read.
      const base = path.join(path.dirname(file), specifier.text);
      visit(base.endsWith('.ts') ? base : `${base}.ts`);
    }
  };
  visit(indexFile);
  return names;
}

function exportedConstNames(sourceFile: ts.SourceFile): string[] {
  const names: string[] = [];
  for (const statement of sourceFile.statements) {
    if (!ts.isVariableStatement(statement)) continue;
    if (!(ts.getCombinedModifierFlags(statement.declarationList.declarations[0]!) & ts.ModifierFlags.Export)) continue;
    for (const declaration of statement.declarationList.declarations) {
      if (ts.isIdentifier(declaration.name)) names.push(declaration.name.text);
    }
  }
  return names;
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

function namedImports(sourceFile: ts.SourceFile): string[] {
  const names: string[] = [];
  for (const statement of sourceFile.statements) {
    if (!ts.isImportDeclaration(statement)) continue;
    const bindings = statement.importClause?.namedBindings;
    if (bindings === undefined || !ts.isNamedImports(bindings)) continue;
    for (const element of bindings.elements) names.push((element.propertyName ?? element.name).text);
  }
  return names;
}

describe('blessed facade packaging', () => {
  it('resolves the published API contract from npm rather than the local compatibility shim', () => {
    const configPath = path.join(facadeDirectory, 'tsconfig.published.json');
    const result = ts.readConfigFile(configPath, ts.sys.readFile);
    expect(result.error, `${configPath} parses`).toBeUndefined();

    const config = result.config as {
      compilerOptions?: { paths?: unknown };
      exclude?: string[];
      include?: string[];
    };
    expect(config.include).toContain('published-api-contract.ts');
    expect(config.exclude).toContain('src/upstream-contract.d.ts');
    expect(config.compilerOptions?.paths).toBeUndefined();

    const parsed = ts.parseJsonConfigFileContent(result.config, ts.sys, facadeDirectory, undefined, configPath);
    expect(parsed.errors).toEqual([]);
    expect(parsed.options.paths, 'no inherited alias may redirect package imports away from npm').toBeUndefined();
    expect(parsed.fileNames).toContain(path.join(facadeDirectory, 'published-api-contract.ts'));
    expect(parsed.fileNames).not.toContain(path.join(facadeDirectory, 'src/upstream-contract.d.ts'));
  });

  it('is built from a wasm facade the generator declares', () => {
    expect(substitute.crate, 'package.json flightWasmSubstitute.crate').toBeTruthy();
    expect(bitmapFacade, `port.config wasmFacades entry for ${String(substitute.crate)}`).toBeDefined();
  });

  it('is named after the upstream package it substitutes, plus -wasm', () => {
    // The mirror-world convention: append `-wasm` to the WHOLE upstream name, dashes included, so
    // `@flighthq/physics3d-abi` would be mirrored by `@flighthq/physics3d-abi-wasm`. Derived from
    // `authoritativePackage` rather than restated, so a facade cannot be named one thing and claim to
    // substitute another — which is the drift that produced the surface/bitmap confusion.
    for (const { directory, manifest } of publishablePackages(workspace)) {
      const upstream = (manifest.flightWasmSubstitute as { authoritativePackage?: string } | undefined)
        ?.authoritativePackage;
      expect(upstream, `${manifest.name} records the package it substitutes`).toBeTruthy();

      expect(manifest.name, `${manifest.name} mirrors ${String(upstream)}`).toBe(`${String(upstream)}-wasm`);
      // The directory carries the unscoped name, so the tree reads the same as the registry.
      expect(path.basename(directory)).toBe(`${String(upstream).replace(/^@[^/]+\//u, '')}-wasm`);
    }
  });

  it('configures every publishable package as a blessed wasm facade', () => {
    for (const { directory, manifest: packageManifest } of publishablePackages(workspace)) {
      const upstream = (packageManifest.flightWasmSubstitute as { authoritativePackage?: string } | undefined)
        ?.authoritativePackage;
      const facade = portConfig.wasmFacades.find((item) => item.authoritativePackage === upstream);
      expect(facade, `${packageManifest.name} has a wasmFacades entry`).toBeDefined();
      expect(facade?.crate).toBe(
        (packageManifest.flightWasmSubstitute as { facadeCrate?: string } | undefined)?.facadeCrate,
      );
      expect(
        portConfig.blessedFacades.some(
          (item) => item.package === packageManifest.name && path.resolve(item.path) === directory,
        ),
        `${packageManifest.name} is blessed at its publishable path`,
      ).toBe(true);
    }
  });

  it('shadows each independent physics ABI constructor over a complete upstream re-export', () => {
    for (const dimension of ['2', '3'] as const) {
      const directory = path.join(workspace, `packages/physics${dimension}d-abi-wasm`);
      const upstream = `@flighthq/physics${dimension}d-abi`;
      const implementation = `./physics${dimension}DAbiWasm`;
      const index = parse(path.join(directory, 'src/index.ts'));
      const facade = portConfig.wasmFacades.find((item) => item.authoritativePackage === upstream);
      const shadowed = reexportedNames(index, implementation);
      expect(shadowed.filter((name) => !name.startsWith('init'))).toEqual(facade?.exports);
      expect(shadowed).toContain(`initPhysics${dimension}DAbiWasm`);
      expect(hasStarReexport(index, upstream)).toBe(true);
    }
  });

  it('shadows exactly the wasm exports the generator built, over a complete upstream re-export', () => {
    const index = parse(path.join(facadeDirectory, 'src/index.ts'));
    const shadowed = reexportedNames(index, './bitmapWasm');

    // `initBitmapWasm` is the facade's own entry point rather than an upstream name, so it is the
    // one addition the generated export set does not account for.
    expect(shadowed.filter((name) => name !== 'initBitmapWasm').sort()).toEqual(
      [...(bitmapFacade?.exports ?? [])].sort(),
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
    const upstreamExports = publicValueNames(
      path.join(workspace, portConfig.upstreamDirectory, 'packages', packageDirectory, 'src/index.ts'),
    );
    // A resolver that silently resolved nothing would make the loop below vacuous.
    expect(upstreamExports.size, `upstream ${authoritativePackage} exposes a readable public surface`).toBeGreaterThan(
      20,
    );

    for (const name of bitmapFacade?.exports ?? []) {
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

  it('routes every wasm override through the unchanged upstream test corpus', () => {
    const index = parse(path.join(facadeDirectory, 'src/index.ts'));
    const shadowed = reexportedNames(index, './bitmapWasm').filter((name) => name !== 'initBitmapWasm');
    expect([...bitmapWasmExports].sort()).toEqual(shadowed.sort());

    const upstreamTests = path.join(workspace, 'upstream/packages/bitmap/src');
    const imported = new Set(
      readdirSync(upstreamTests)
        .filter((file) => file.endsWith('.test.ts'))
        .flatMap((file) => namedImports(parse(path.join(upstreamTests, file)))),
    );
    for (const name of bitmapWasmExports) {
      expect(imported.has(name), `${name} is imported by an upstream bitmap test`).toBe(true);
    }

    const rootManifest = JSON.parse(readFileSync(path.join(workspace, 'package.json'), 'utf8')) as {
      scripts?: Record<string, string>;
    };
    expect(manifest.scripts?.test).toContain('vitest.config.upstream.ts');
    expect(rootManifest.scripts?.['test:release']).toContain('test:upstream');
  });

  it('copies every non-TypeScript module the facade imports into dist', () => {
    for (const [packageName, glueFiles, implementationPath] of packagedGlue) {
      const directory = path.join(workspace, 'packages', packageName);
      const implementation = parse(path.join(directory, implementationPath));
      const copied = new Set<string>(glueFiles);

      for (const specifier of relativeImportSpecifiers(implementation)) {
        // `tsc -b` emits JavaScript for TypeScript sources; anything imported with an explicit `.js`
        // extension resolves to a checked-in wasm-bindgen artifact that only the copy step delivers.
        if (!specifier.endsWith('.js')) continue;
        const file = path.basename(specifier);
        expect(copied.has(file), `${packageName}: ${specifier} is copied into dist`).toBe(true);
        expect(existsSync(path.join(directory, 'src/wasm', file))).toBe(true);
      }
    }
  });

  it('declares manifests npm can publish', () => {
    for (const { directory, manifest: packageManifest } of publishablePackages(workspace)) {
      for (const field of ['name', 'version', 'description', 'license', 'author', 'repository']) {
        expect(packageManifest[field], `${packageManifest.name} package.json ${field}`).toBeTruthy();
      }
      expect(packageManifest.private, `${packageManifest.name} must not be private`).toBeUndefined();

      // npm defaults a scoped package to restricted, which fails the publish outright on a free
      // account. Keep the public-access intent in each package as well as the release command.
      if (String(packageManifest.name).startsWith('@')) {
        expect((packageManifest.publishConfig as { access?: string } | undefined)?.access).toBe('public');
      }

      expect(existsSync(path.join(directory, 'README.md'))).toBe(true);
      expect(existsSync(path.join(directory, 'LICENSE.md'))).toBe(true);

      // `*` resolves to whatever `latest` happens to be at install time, which silently pairs a
      // wasm backend with an upstream release it was not tested against.
      const dependencies = (packageManifest.dependencies ?? {}) as Record<string, string>;
      expect(Object.keys(dependencies).length).toBeGreaterThan(0);
      for (const [name, range] of Object.entries(dependencies)) {
        expect(range, `${packageManifest.name}: ${name} dependency range`).not.toBe('*');
        expect(range, `${packageManifest.name}: ${name} dependency range`).toMatch(/^[\^~]?\d+\.\d+\.\d+/u);
      }

      // Each `prepack` step runs at publish time, where a missing script fails the release rather
      // than a test. The repository was filtered without `scripts/clean-package-dist.ts` once.
      const scripts = (packageManifest.scripts ?? {}) as Record<string, string>;
      for (const command of Object.values(scripts)) {
        const invoked = /(?:^|\s)tsx\s+(\S+\.ts)/u.exec(command)?.[1];
        if (!invoked) continue;
        expect(existsSync(path.resolve(directory, invoked)), `${packageManifest.name}: ${invoked} exists`).toBe(true);
      }
    }
  });
  // The hybrid is deliberate: `agents/wasm-barrier.md` records which `@flighthq/bitmap` functions belong in
  // TypeScript permanently, and the reasons are structural rather than performance-dependent — platform access,
  // a crossing that dominates a byte of work, allocation whose identity lives in JavaScript, string formatting,
  // and plain data. Nothing about a faster kernel changes any of them.
  //
  // Written down, that judgement decays. Enforced, it cannot: widening the exposed set to include one of these
  // now fails here, with the reason one line away.
  const permanentlyTypeScript = {
    'host and DOM interop': [
      'createBitmapFromImageSource',
      'captureBitmapFromImageResource',
      'encodeBitmap',
      'explainBitmapReadback',
    ],
    'single-pixel accessors, where the crossing dominates the work': [
      'getBitmapPixel',
      'getBitmapPixelLuminance',
      'getBitmapPixelRgb',
      'setBitmapPixel',
      'setBitmapPixelRgb',
      'invalidateBitmap',
    ],
    'allocation and entity construction': ['createBitmap', 'cloneBitmap', 'createBitmapRegion', 'splitBitmapChannels'],
    'string formatting and parsing': ['formatBitmapFingerprint', 'parseBitmapFingerprint'],
    'data rather than code': ['BITMAP_FINGERPRINT_COMPUTATION_ID'],
  } as const;

  it('keeps the permanently-TypeScript functions out of every wasm export set', () => {
    const exposed = new Set(portConfig.wasmFacades.flatMap((facade) => facade.exports));
    for (const [reason, names] of Object.entries(permanentlyTypeScript)) {
      for (const name of names) {
        expect(exposed.has(name), `${name} must stay in TypeScript — ${reason}`).toBe(false);
      }
    }
  });

  it('names only functions upstream still exports, so the list above cannot rot unnoticed', () => {
    // A guard listing names that no longer exist has quietly stopped guarding anything: every assertion above
    // would pass for a function upstream deleted, and pass just as well for one it renamed into the exposed set
    // under a new name. This is the half that fails when upstream moves — it already caught
    // `createBitmapFromCanvas` and `drawBitmap` being removed.
    const upstreamExports = publicValueNames(
      path.join(workspace, portConfig.upstreamDirectory, 'packages', 'bitmap', 'src/index.ts'),
    );
    expect(upstreamExports.size, 'upstream @flighthq/bitmap exposes a readable public surface').toBeGreaterThan(20);
    for (const names of Object.values(permanentlyTypeScript)) {
      for (const name of names) {
        expect(upstreamExports.has(name), `${name} is still exported by upstream @flighthq/bitmap`).toBe(true);
      }
    }
  });
});
