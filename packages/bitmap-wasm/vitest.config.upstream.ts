import path from 'node:path';

import ts from 'typescript';
import { defineConfig, type Plugin } from 'vitest/config';

const upstreamPackages = path.resolve(import.meta.dirname, '../../upstream/packages');
const upstreamSource = path.join(upstreamPackages, 'bitmap/src');
const facade = path.resolve(import.meta.dirname, 'src/bitmapWasm.ts');

export const bitmapWasmExports = new Set([
  'applyBitmapCurve',
  'applyBitmapLevels',
  'applyBitmapPaletteMap',
  'buildBitmapBrightnessColorMatrix',
  'buildBitmapContrastColorMatrix',
  'buildBitmapGrayscaleColorMatrix',
  'buildBitmapHueRotationColorMatrix',
  'buildBitmapInvertColorMatrix',
  'buildBitmapSaturationColorMatrix',
  'buildBitmapSepiaColorMatrix',
  'colorMatrixBitmap',
  'compareBitmapFingerprints',
  'concatBitmapColorMatrix',
  'convolveBitmap',
  'copyBitmapAlpha',
  'copyBitmapPixels',
  'createBitmapFingerprint',
  'dilateBitmap',
  'erodeBitmap',
  'fillBitmapNoise',
  'fillBitmapPerlinNoise',
  'fillBitmapRectangle',
  'fillBitmapTurbulence',
  'getBitmapColorBoundsRectangle',
  'getBitmapCoverage',
  'getBitmapHistogram',
  'getBitmapMismatch',
  'mergeBitmapChannels',
  'multiplyBitmapAlpha',
  'pixelateBitmap',
  'premultiplyBitmapPixels',
  'setBitmapAlpha',
  'setBitmapColorMatrixIdentity',
  'unpremultiplyBitmapPixels',
]);

function importedName(specifier: ts.ImportSpecifier): string {
  return (specifier.propertyName ?? specifier.name).text;
}

function substituteWasmFunctions(): Plugin {
  return {
    name: 'bitmap-wasm-upstream-conformance',
    enforce: 'pre',
    transform(code, id) {
      const filename = id.split('?', 1)[0];
      if (filename === undefined || !filename.startsWith(upstreamSource) || !filename.endsWith('.test.ts')) {
        return null;
      }

      const source = ts.createSourceFile(filename, code, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
      const statements: ts.Statement[] = [];
      let changed = false;

      for (const statement of source.statements) {
        if (
          !ts.isImportDeclaration(statement) ||
          !ts.isStringLiteral(statement.moduleSpecifier) ||
          !statement.moduleSpecifier.text.startsWith('.') ||
          statement.importClause?.namedBindings === undefined ||
          !ts.isNamedImports(statement.importClause.namedBindings)
        ) {
          statements.push(statement);
          continue;
        }

        const wasmSpecifiers = statement.importClause.namedBindings.elements.filter((specifier) =>
          bitmapWasmExports.has(importedName(specifier)),
        );
        if (wasmSpecifiers.length === 0) {
          statements.push(statement);
          continue;
        }

        changed = true;
        const upstreamSpecifiers = statement.importClause.namedBindings.elements.filter(
          (specifier) => !bitmapWasmExports.has(importedName(specifier)),
        );
        if (statement.importClause.name !== undefined || upstreamSpecifiers.length > 0) {
          statements.push(
            ts.factory.updateImportDeclaration(
              statement,
              statement.modifiers,
              ts.factory.updateImportClause(
                statement.importClause,
                statement.importClause.isTypeOnly,
                statement.importClause.name,
                ts.factory.updateNamedImports(statement.importClause.namedBindings, upstreamSpecifiers),
              ),
              statement.moduleSpecifier,
              statement.attributes,
            ),
          );
        }
        statements.push(
          ts.factory.createImportDeclaration(
            undefined,
            ts.factory.createImportClause(
              statement.importClause.isTypeOnly,
              undefined,
              ts.factory.createNamedImports(wasmSpecifiers),
            ),
            ts.factory.createStringLiteral(facade),
            undefined,
          ),
        );
      }

      if (!changed) return null;
      const printer = ts.createPrinter({ newLine: ts.NewLineKind.LineFeed });
      const rewritten = ts.factory.updateSourceFile(source, statements);
      return { code: printer.printFile(rewritten), map: null };
    },
  };
}

export default defineConfig({
  root: import.meta.dirname,
  plugins: [substituteWasmFunctions()],
  resolve: {
    alias: [
      { find: /^@flighthq\/([^/]+)$/u, replacement: `${upstreamPackages}/$1/src/index.ts` },
      { find: /^@flighthq\/([^/]+)\/(.+)$/u, replacement: `${upstreamPackages}/$1/src/$2` },
    ],
  },
  test: {
    environment: 'jsdom',
    globals: true,
    include: ['../../upstream/packages/bitmap/src/**/*.test.ts'],
    setupFiles: [path.resolve(import.meta.dirname, '../../upstream/vitest.setup.ts')],
  },
});
