import { writeFileSync } from 'node:fs';

import * as reference from '@flighthq/bitmap';
import { createBitmap, createBitmapRegion } from '@flighthq/bitmap';

import * as rs from './bitmapWasm';

// Measures the twenty-seven exports still backed by GENERATED kernels, so the decision about each one is made
// from a number rather than from its shape. Three outcomes are possible and all three are useful:
//
//  - real per-pixel arithmetic, currently losing  -> worth mirroring, like the five already done
//  - already winning                              -> leave it alone
//  - trivial work per call                        -> should not cross the barrier AT ALL, and mirroring it would
//                                                    not help, because the cost is the crossing
//
// That last group is the one worth catching here. A colour-matrix builder returns twenty floats; no kernel work
// makes a wasm call cheaper than the handful of nanoseconds TypeScript needs for it.

function time(iterations: number, fn: () => void): number {
  for (let warmup = 0; warmup < 5; warmup++) fn();
  const start = process.hrtime.bigint();
  for (let i = 0; i < iterations; i++) fn();
  return Number(process.hrtime.bigint() - start) / iterations / 1e6;
}

const rows: string[] = [];
function compare(label: string, iterations: number, wasmFn: () => void, tsFn: () => void): void {
  const wasm = time(iterations, wasmFn);
  const ts = time(iterations, tsFn);
  rows.push(
    `${label.padEnd(36)} wasm ${wasm.toFixed(5).padStart(10)} ms   ts ${ts.toFixed(5).padStart(10)} ms   ${(ts / wasm).toFixed(2).padStart(6)}x`,
  );
}

it('measures every remaining generated export', { timeout: 600_000 }, () => {
  rs.initBitmapWasm();

  // Tiny per-call work: the colour-matrix family and the identity/concat helpers.
  const matrixA = Array.from({ length: 20 }, (_, i) => i / 20);
  const matrixB = Array.from({ length: 20 }, (_, i) => (20 - i) / 20);
  const out20 = new Array<number>(20).fill(0);
  compare(
    'buildBrightnessColorMatrix',
    20_000,
    () => rs.buildBitmapBrightnessColorMatrix(out20, 0.4),
    () => reference.buildBitmapBrightnessColorMatrix(out20, 0.4),
  );
  compare(
    'buildHueRotationColorMatrix',
    20_000,
    () => rs.buildBitmapHueRotationColorMatrix(out20, 35),
    () => reference.buildBitmapHueRotationColorMatrix(out20, 35),
  );
  compare(
    'buildSaturationColorMatrix',
    20_000,
    () => rs.buildBitmapSaturationColorMatrix(out20, 1.4),
    () => reference.buildBitmapSaturationColorMatrix(out20, 1.4),
  );
  compare(
    'setColorMatrixIdentity',
    20_000,
    () => rs.setBitmapColorMatrixIdentity(out20),
    () => reference.setBitmapColorMatrixIdentity(out20),
  );
  compare(
    'concatColorMatrix',
    20_000,
    () => rs.concatBitmapColorMatrix(out20, matrixA, matrixB),
    () => reference.concatBitmapColorMatrix(out20, matrixA, matrixB),
  );

  // Per-pixel kernels, at a realistic size.
  for (const side of [256, 1024]) {
    const n = side > 256 ? 60 : 400;
    const src = createBitmap(side, side, 0x8040c0ff);
    for (let i = 0; i < src.data.length; i++) src.data[i] = (i * 31) & 0xff;
    const srcRegion = createBitmapRegion(src);
    const dstA = createBitmapRegion(createBitmap(side, side, 0));
    const dstB = createBitmapRegion(createBitmap(side, side, 0));
    const outA = new Uint8ClampedArray(side * side * 4);
    const outB = new Uint8ClampedArray(side * side * 4);
    const lut = new Uint8Array(256);
    for (let i = 0; i < 256; i++) lut[i] = 255 - i;
    const map = Array.from({ length: 256 }, (_, i) => (i * 7) & 0xff);
    const s = `${side}²`;

    compare(
      `colorMatrixBitmap ${s}`,
      n,
      // Writes a raw buffer rather than a region, unlike its neighbours.
      () => rs.colorMatrixBitmap(outA, srcRegion, matrixA),
      () => reference.colorMatrixBitmap(outB, srcRegion, matrixA),
    );
    compare(
      `applyBitmapCurve ${s}`,
      n,
      () => rs.applyBitmapCurve(dstA, srcRegion, lut, lut, lut, lut),
      () => reference.applyBitmapCurve(dstB, srcRegion, lut, lut, lut, lut),
    );
    compare(
      `applyBitmapLevels ${s}`,
      n,
      () => rs.applyBitmapLevels(dstA, srcRegion, 16, 240, 1.2),
      () => reference.applyBitmapLevels(dstB, srcRegion, 16, 240, 1.2),
    );
    compare(
      `applyBitmapPaletteMap ${s}`,
      n,
      () => rs.applyBitmapPaletteMap(dstA, srcRegion, map, map, map, map),
      () => reference.applyBitmapPaletteMap(dstB, srcRegion, map, map, map, map),
    );
    compare(
      `copyBitmapPixels ${s}`,
      n,
      () => rs.copyBitmapPixels(dstA, srcRegion),
      () => reference.copyBitmapPixels(dstB, srcRegion),
    );
    compare(
      `copyBitmapAlpha ${s}`,
      n,
      () => rs.copyBitmapAlpha(dstA, srcRegion),
      () => reference.copyBitmapAlpha(dstB, srcRegion),
    );
    compare(
      `mergeBitmapChannels ${s}`,
      n,
      () => rs.mergeBitmapChannels(dstA, srcRegion, srcRegion, srcRegion, srcRegion),
      () => reference.mergeBitmapChannels(dstB, srcRegion, srcRegion, srcRegion, srcRegion),
    );
    compare(
      `premultiplyBitmapPixels ${s}`,
      n,
      () => rs.premultiplyBitmapPixels(outA, src.data, src.data.length),
      () => reference.premultiplyBitmapPixels(outB, src.data, src.data.length),
    );
    compare(
      `unpremultiplyBitmapPixels ${s}`,
      n,
      () => rs.unpremultiplyBitmapPixels(outA, src.data, src.data.length),
      () => reference.unpremultiplyBitmapPixels(outB, src.data, src.data.length),
    );
    compare(
      `fillBitmapRectangle ${s}`,
      n,
      () => rs.fillBitmapRectangle(dstA, 0x204080ff),
      () => reference.fillBitmapRectangle(dstB, 0x204080ff),
    );
    compare(
      `fillBitmapNoise ${s}`,
      n,
      () => rs.fillBitmapNoise(dstA, 12345),
      () => reference.fillBitmapNoise(dstB, 12345),
    );
    compare(
      `fillBitmapPerlinNoise ${s}`,
      Math.max(10, n / 4),
      () => rs.fillBitmapPerlinNoise(dstA, 8, 8, 2, 7),
      () => reference.fillBitmapPerlinNoise(dstB, 8, 8, 2, 7),
    );
    compare(
      `fillBitmapTurbulence ${s}`,
      Math.max(10, n / 4),
      () => rs.fillBitmapTurbulence(dstA, 8, 8, 3, 7),
      () => reference.fillBitmapTurbulence(dstB, 8, 8, 3, 7),
    );

    // Reducers: a lot of reading, a few numbers out.
    compare(
      `getBitmapHistogram ${s}`,
      n,
      () => void rs.getBitmapHistogram(srcRegion),
      () => void reference.getBitmapHistogram(srcRegion),
    );
    compare(
      `getBitmapCoverage ${s}`,
      n,
      () => void rs.getBitmapCoverage(src, 8),
      () => void reference.getBitmapCoverage(src, 8),
    );
    compare(
      `getBitmapColorBounds ${s}`,
      n,
      () => void rs.getBitmapColorBoundsRectangle(srcRegion, 0xffffffff, 0x000000ff, true),
      () => void reference.getBitmapColorBoundsRectangle(srcRegion, 0xffffffff, 0x000000ff, true),
    );
    compare(
      `getBitmapMismatch ${s}`,
      n,
      () => void rs.getBitmapMismatch(src, src),
      () => void reference.getBitmapMismatch(src, src),
    );
  }

  writeFileSync('/tmp/remaining.txt', 'wasm vs TypeScript, >1x means wasm is faster\n' + rows.join('\n') + '\n');
  expect(rows.length).toBe(39);
});
