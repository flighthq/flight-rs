import { writeFileSync } from 'node:fs';

import * as reference from '@flighthq/bitmap';
import { createBitmap, createBitmapRegion } from '@flighthq/bitmap';

import { convolveBitmap, dilateBitmap, multiplyBitmapAlpha, pixelateBitmap, setBitmapAlpha } from './bitmapWasm';

// Where does the wasm barrier pay for itself? Measured rather than assumed, because the answer decides which
// functions belong behind it at all — and the answer turns out not to be "the big ones".
function time(iterations: number, fn: () => void): number {
  fn();
  const start = process.hrtime.bigint();
  for (let i = 0; i < iterations; i++) fn();
  return Number(process.hrtime.bigint() - start) / iterations / 1e6;
}

const rows: string[] = [];
function compare(label: string, iterations: number, wasmFn: () => void, tsFn: () => void): void {
  const wasm = time(iterations, wasmFn);
  const ts = time(iterations, tsFn);
  rows.push(
    `${label.padEnd(34)} wasm ${wasm.toFixed(4).padStart(9)} ms   ts ${ts.toFixed(4).padStart(9)} ms   ${(ts / wasm).toFixed(2).padStart(5)}x`,
  );
}

it('measures the barrier against arithmetic per byte', { timeout: 300_000 }, () => {
  // Bandwidth-bound: one byte written per pixel, trivial arithmetic.
  for (const side of [16, 256, 1024]) {
    const a = createBitmapRegion(createBitmap(side, side, 0x804030ff));
    const b = createBitmapRegion(createBitmap(side, side, 0x804030ff));
    const n = side > 256 ? 100 : 5_000;
    compare(
      `setBitmapAlpha ${side}x${side}`,
      n,
      () => setBitmapAlpha(a, 128),
      () => reference.setBitmapAlpha(b, 128),
    );
    compare(
      `multiplyBitmapAlpha ${side}x${side}`,
      n,
      () => multiplyBitmapAlpha(a, 0.75),
      () => reference.multiplyBitmapAlpha(b, 0.75),
    );
  }

  // Compute-bound: a 5x5 convolution is 25 multiply-adds per channel per pixel.
  const options = {
    divisor: 36,
    edge: 'clamp' as const,
    matrix: [1, 1, 1, 1, 1, 1, 2, 2, 2, 1, 1, 2, 4, 2, 1, 1, 2, 2, 2, 1, 1, 1, 1, 1, 1],
    matrixX: 5,
    matrixY: 5,
  };
  for (const side of [64, 256, 512]) {
    const source = createBitmapRegion(createBitmap(side, side, 0x804030ff));
    const outA = new Uint8ClampedArray(side * side * 4);
    const outB = new Uint8ClampedArray(side * side * 4);
    const n = side > 256 ? 20 : 200;
    compare(
      `convolveBitmap 5x5 ${side}x${side}`,
      n,
      () => convolveBitmap(outA, source, options),
      () => reference.convolveBitmap(outB, source, options),
    );
  }

  // In between: pixelate averages a block, so arithmetic per byte sits between the two extremes.
  for (const side of [256, 1024]) {
    const source = createBitmapRegion(createBitmap(side, side, 0x804030ff));
    const outA = new Uint8ClampedArray(side * side * 4);
    const outB = new Uint8ClampedArray(side * side * 4);
    compare(
      `pixelateBitmap 8 ${side}x${side}`,
      side > 256 ? 50 : 500,
      () => pixelateBitmap(outA, source, 8),
      () => reference.pixelateBitmap(outB, source, 8),
    );
  }

  // Morphological ops: a window min/max per pixel, shadowed by the facade.
  for (const side of [256, 512]) {
    const source = createBitmapRegion(createBitmap(side, side, 0x804030ff));
    const outA = new Uint8ClampedArray(side * side * 4);
    const outB = new Uint8ClampedArray(side * side * 4);
    compare(
      `dilateBitmap r3 ${side}x${side}`,
      side > 256 ? 20 : 100,
      () => dilateBitmap(outA, source, 3),
      () => reference.dilateBitmap(outB, source, 3),
    );
  }

  writeFileSync('/tmp/barrier.txt', 'wasm vs TypeScript, >1x means wasm is faster\n' + rows.join('\n') + '\n');
  expect(rows.length).toBeGreaterThan(0);
});
