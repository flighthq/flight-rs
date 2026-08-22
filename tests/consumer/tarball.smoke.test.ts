// Consumes the PACKED TARBALL the way a user would, rather than the sources it was built from.
//
// Every other suite imports `../src`, which proves the implementation is correct and proves nothing
// about what ships. Two real defects in this package were invisible to all of them and only appeared
// here: a tarball missing its wasm-bindgen glue, which installed fine and then failed at import; and
// an emitted `.d.ts` importing a type from the wrong upstream module, which only broke consumers who
// typechecked it.
//
// Requires the tarball to be installed first — `npm install --no-save <tarball>` — so that
// `@flighthq/bitmap-wasm` resolves out of `node_modules`. Installing it also pulls its declared
// `@flighthq/*` dependency range, which is itself part of what this checks: a range naming a version
// npm cannot serve fails at install, before a single assertion runs.

import { readFileSync } from 'node:fs';
import path from 'node:path';

import { createBitmap, createBitmapRegion } from '@flighthq/bitmap';
import * as reference from '@flighthq/bitmap';
import * as shipped from '@flighthq/bitmap-wasm';

function paint(width: number, height: number, seed = 0): ReturnType<typeof createBitmap> {
  const bitmap = createBitmap(width, height, 0);
  for (let index = 0; index < width * height; index += 1) {
    bitmap.data[index * 4] = (index * 37 + 11 + seed) & 0xff;
    bitmap.data[index * 4 + 1] = (index * 53 + 7 + seed) & 0xff;
    bitmap.data[index * 4 + 2] = (index * 97 + 3 + seed) & 0xff;
    bitmap.data[index * 4 + 3] = (index * 17 + 1 + seed) & 0xff;
  }
  return bitmap;
}

describe('the packed tarball, as a consumer installs it', () => {
  it('imports and re-exports the whole upstream surface', () => {
    // `export * from '@flighthq/bitmap'` is what keeps the package API-complete regardless of how
    // much has been ported, so a consumer never has to know which functions are wasm-backed.
    for (const name of Object.keys(reference)) {
      expect(name in shipped, `${name} is reachable from the installed package`).toBe(true);
    }
  });

  it('initializes its embedded wasm with no asset fetch', () => {
    // The module is baked in as base64 precisely so this needs no file read, no network, and no
    // await. If the glue or the bytes were missing from the tarball, this is where it surfaces.
    expect(() => shipped.initBitmapWasm()).not.toThrow();
    expect(() => shipped.initBitmapWasm()).not.toThrow();
  });

  it('computes the same pixels as the TypeScript it substitutes', () => {
    shipped.initBitmapWasm();

    const region = (bitmap: ReturnType<typeof createBitmap>) => createBitmapRegion(bitmap, 0, 0, 8, 8);

    const wasmFilled = paint(8, 8);
    const referenceFilled = paint(8, 8);
    shipped.fillBitmapRectangle(createBitmapRegion(wasmFilled, 1, 1, 4, 3), 0x80ff0000);
    reference.fillBitmapRectangle(createBitmapRegion(referenceFilled, 1, 1, 4, 3), 0x80ff0000);
    expect(Array.from(wasmFilled.data)).toEqual(Array.from(referenceFilled.data));

    const wasmOut = new Uint8ClampedArray(8 * 8 * 4);
    const referenceOut = new Uint8ClampedArray(8 * 8 * 4);
    shipped.pixelateBitmap(wasmOut, region(paint(8, 8)), 3);
    reference.pixelateBitmap(referenceOut, region(paint(8, 8)), 3);
    expect(Array.from(wasmOut)).toEqual(Array.from(referenceOut));

    // An allocating query, so the comparison covers a returned structure rather than a written buffer.
    expect(shipped.getBitmapHistogram(region(paint(8, 8)))).toEqual(reference.getBitmapHistogram(region(paint(8, 8))));
  });

  it('declares a dependency range the registry actually satisfied', () => {
    // The range is written at release time and is the easiest thing to get wrong — a caret over a
    // stable version does not match a prerelease, so `^0.4.0` against a registry carrying only
    // `0.4.0-next.…` installs nothing. Reading both manifests back out of node_modules proves npm
    // resolved what the package asked for, rather than trusting that it would.
    const read = (specifier: string): { name: string; version: string; dependencies?: Record<string, string> } =>
      JSON.parse(readFileSync(path.join('node_modules', specifier, 'package.json'), 'utf8'));

    const installed = read('@flighthq/bitmap-wasm');
    expect(installed.name).toBe('@flighthq/bitmap-wasm');

    const range = installed.dependencies?.['@flighthq/bitmap'];
    expect(range, 'the installed package declares a bitmap dependency').toBeTruthy();

    // The install itself is the real proof that npm could satisfy the range — it fails outright
    // otherwise. What is left to check is that it resolved within the intended FAMILY rather than
    // some unrelated major.minor, which a hand-edited range could silently widen. Compared by
    // major.minor rather than with a semver library, to keep this fixture dependency-free.
    const family = (version: string): string => version.replace(/^\^?(\d+)\.(\d+)\..*$/u, '$1.$2');
    expect(family(read('@flighthq/bitmap').version)).toBe(family(String(range)));
  });
});
