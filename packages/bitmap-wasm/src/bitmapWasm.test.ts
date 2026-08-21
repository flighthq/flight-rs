import { createBitmap, createBitmapRegion } from '@flighthq/bitmap';
import * as reference from '@flighthq/bitmap';
import type { Bitmap, BitmapRegion } from '@flighthq/types';

import * as rs from './bitmapWasm';

function paintBitmap(width: number, height: number): Bitmap {
  const surface = createBitmap(width, height, 0);
  for (let index = 0; index < width * height; index += 1) {
    surface.data[index * 4] = (index * 37 + 11) & 0xff;
    surface.data[index * 4 + 1] = (index * 53 + 7) & 0xff;
    surface.data[index * 4 + 2] = (index * 97 + 3) & 0xff;
    surface.data[index * 4 + 3] = (index * 17 + 1) & 0xff;
  }
  return surface;
}

function fullRegion(bitmap: Bitmap): BitmapRegion {
  return createBitmapRegion(bitmap, 0, 0, bitmap.width, bitmap.height);
}

function cloneBitmap(source: Bitmap): Bitmap {
  const clone = createBitmap(source.width, source.height, 0);
  clone.data.set(source.data);
  return clone;
}

describe('generated wasm boundary', () => {
  it('initializes synchronously and idempotently', () => {
    rs.initBitmapWasm();
    rs.initBitmapWasm();
  });

  it('matches all selected color-matrix builders and composition', () => {
    const actual = Array.from({ length: 20 }, () => 0);
    const expected = Array.from({ length: 20 }, () => 0);
    const cases: Array<[(out: number[]) => void, (out: number[]) => void]> = [
      [
        (out) => rs.buildBitmapBrightnessColorMatrix(out, 1.25),
        (out) => reference.buildBitmapBrightnessColorMatrix(out, 1.25),
      ],
      [
        (out) => rs.buildBitmapContrastColorMatrix(out, 0.75),
        (out) => reference.buildBitmapContrastColorMatrix(out, 0.75),
      ],
      [rs.buildBitmapGrayscaleColorMatrix, reference.buildBitmapGrayscaleColorMatrix],
      [
        (out) => rs.buildBitmapHueRotationColorMatrix(out, 42),
        (out) => reference.buildBitmapHueRotationColorMatrix(out, 42),
      ],
      [rs.buildBitmapInvertColorMatrix, reference.buildBitmapInvertColorMatrix],
      [
        (out) => rs.buildBitmapSaturationColorMatrix(out, 1.5),
        (out) => reference.buildBitmapSaturationColorMatrix(out, 1.5),
      ],
      [rs.buildBitmapSepiaColorMatrix, reference.buildBitmapSepiaColorMatrix],
      [rs.setBitmapColorMatrixIdentity, reference.setBitmapColorMatrixIdentity],
    ];
    for (const [generated, cultivated] of cases) {
      generated(actual);
      cultivated(expected);
      expect(actual).toEqual(expected);
    }

    const first = actual.slice();
    reference.buildBitmapContrastColorMatrix(expected, 1.2);
    const second = expected.slice();
    rs.concatBitmapColorMatrix(actual, first, second);
    reference.concatBitmapColorMatrix(expected, first, second);
    expect(actual).toEqual(expected);
  });

  it('matches selected output-buffer kernels', () => {
    const source = paintBitmap(4, 3);
    const region = fullRegion(source);
    const actual = new Uint8ClampedArray(source.data.length);
    const expected = new Uint8ClampedArray(source.data.length);
    const matrix = Array.from({ length: 20 }, () => 0);
    reference.buildBitmapSaturationColorMatrix(matrix, 0.4);

    rs.colorMatrixBitmap(actual, region, matrix);
    reference.colorMatrixBitmap(expected, region, matrix);
    expect(actual).toEqual(expected);

    const options = {
      edge: 'mirror' as const,
      matrix: [1, 2, 1, 2, 4, 2, 1, 2, 1],
      matrixX: 3,
      matrixY: 3,
      preserveAlpha: false,
    };
    rs.convolveBitmap(actual, region, options);
    reference.convolveBitmap(expected, region, options);
    expect(actual).toEqual(expected);

    rs.dilateBitmap(actual, region, 1);
    reference.dilateBitmap(expected, region, 1);
    expect(actual).toEqual(expected);

    rs.erodeBitmap(actual, region, 1);
    reference.erodeBitmap(expected, region, 1);
    expect(actual).toEqual(expected);

    rs.pixelateBitmap(actual, region, 2);
    reference.pixelateBitmap(expected, region, 2);
    expect(actual).toEqual(expected);

    rs.premultiplyBitmapPixels(actual, source.data, source.data.length);
    reference.premultiplyBitmapPixels(expected, source.data, source.data.length);
    expect(actual).toEqual(expected);

    rs.unpremultiplyBitmapPixels(actual, source.data, source.data.length);
    reference.unpremultiplyBitmapPixels(expected, source.data, source.data.length);
    expect(actual).toEqual(expected);
  });

  it('matches mutating region operations, clipping, and invalidation', () => {
    const source = paintBitmap(3, 2);
    const actualBitmap = paintBitmap(3, 2);
    const expectedBitmap = cloneBitmap(actualBitmap);
    const sourceRegion = createBitmapRegion(source, 0, 0, 3, 2);
    const actual = createBitmapRegion(actualBitmap, -1, 0, 3, 2);
    const expected = createBitmapRegion(expectedBitmap, -1, 0, 3, 2);

    rs.copyBitmapPixels(actual, sourceRegion);
    reference.copyBitmapPixels(expected, sourceRegion);
    expect(actualBitmap.data).toEqual(expectedBitmap.data);
    expect(actualBitmap.version).toBe(expectedBitmap.version);

    rs.fillBitmapRectangle(actual, 0xaabbccdd);
    reference.fillBitmapRectangle(expected, 0xaabbccdd);
    expect(actualBitmap.data).toEqual(expectedBitmap.data);

    rs.multiplyBitmapAlpha(actual, 0.5);
    reference.multiplyBitmapAlpha(expected, 0.5);
    expect(actualBitmap.data).toEqual(expectedBitmap.data);

    rs.setBitmapAlpha(actual, 123);
    reference.setBitmapAlpha(expected, 123);
    expect(actualBitmap.data).toEqual(expectedBitmap.data);

    rs.copyBitmapAlpha(actual, sourceRegion);
    reference.copyBitmapAlpha(expected, sourceRegion);
    expect(actualBitmap.data).toEqual(expectedBitmap.data);
    expect(actualBitmap.version).toBe(expectedBitmap.version);

    const inverted = Array.from({ length: 256 }, (_, value) => 255 - value);
    rs.applyBitmapPaletteMap(actual, sourceRegion, inverted, null, null, null);
    reference.applyBitmapPaletteMap(expected, sourceRegion, inverted, null, null, null);
    expect(actualBitmap.data).toEqual(expectedBitmap.data);
    expect(actualBitmap.version).toBe(expectedBitmap.version);

    const byteInverted = Uint8Array.from(inverted);
    rs.applyBitmapCurve(actual, sourceRegion, byteInverted, null, null, null);
    reference.applyBitmapCurve(expected, sourceRegion, byteInverted, null, null, null);
    expect(actualBitmap.data).toEqual(expectedBitmap.data);
    expect(actualBitmap.version).toBe(expectedBitmap.version);

    rs.applyBitmapLevels(actual, sourceRegion, 12, 240, 0.75);
    reference.applyBitmapLevels(expected, sourceRegion, 12, 240, 0.75);
    expect(actualBitmap.data).toEqual(expectedBitmap.data);
    expect(actualBitmap.version).toBe(expectedBitmap.version);

    const comparison = cloneBitmap(actualBitmap);
    comparison.data[5] = (comparison.data[5]! + 128) & 0xff;
    expect(rs.getBitmapMismatch(actualBitmap, comparison, 10)).toEqual(
      reference.getBitmapMismatch(expectedBitmap, comparison, 10),
    );

    const actualFingerprint = rs.createBitmapFingerprint(actualBitmap, 2);
    const expectedFingerprint = reference.createBitmapFingerprint(expectedBitmap, 2);
    expect(actualFingerprint.gridSize).toBe(expectedFingerprint.gridSize);
    expect(actualFingerprint.cells).toEqual(expectedFingerprint.cells);
    const comparisonFingerprint = reference.createBitmapFingerprint(comparison, 2);
    expect(rs.compareBitmapFingerprints(actualFingerprint, comparisonFingerprint)).toBe(
      reference.compareBitmapFingerprints(expectedFingerprint, comparisonFingerprint),
    );

    rs.mergeBitmapChannels(actual, sourceRegion, sourceRegion, sourceRegion, sourceRegion);
    reference.mergeBitmapChannels(expected, sourceRegion, sourceRegion, sourceRegion, sourceRegion);
    expect(actualBitmap.data).toEqual(expectedBitmap.data);
    expect(actualBitmap.version).toBe(expectedBitmap.version);

    expect(rs.getBitmapColorBoundsRectangle(actual, 0xffffff00, 0xaabbcc00)).toEqual(
      reference.getBitmapColorBoundsRectangle(expected, 0xffffff00, 0xaabbcc00),
    );
    expect(rs.getBitmapHistogram(actual)).toEqual(reference.getBitmapHistogram(expected));
  });

  it('matches deterministic noise variants and coverage', () => {
    const actualBitmap = createBitmap(4, 3, 0);
    const expectedBitmap = createBitmap(4, 3, 0);
    const actual = fullRegion(actualBitmap);
    const expected = fullRegion(expectedBitmap);

    rs.fillBitmapNoise(actual, 123, 10, 240, true);
    reference.fillBitmapNoise(expected, 123, 10, 240, true);
    expect(actualBitmap.data).toEqual(expectedBitmap.data);

    rs.fillBitmapPerlinNoise(actual, 3, 4, 2, 99, false, true, 0xf);
    reference.fillBitmapPerlinNoise(expected, 3, 4, 2, 99, false, true, 0xf);
    expect(actualBitmap.data).toEqual(expectedBitmap.data);

    rs.fillBitmapTurbulence(actual, 3, 4, 2, 101, true, false, 0x7);
    reference.fillBitmapTurbulence(expected, 3, 4, 2, 101, true, false, 0x7);
    expect(actualBitmap.data).toEqual(expectedBitmap.data);

    expect(rs.getBitmapCoverage(actualBitmap, 0, 2)).toBe(reference.getBitmapCoverage(expectedBitmap, 0, 2));
  });
});
