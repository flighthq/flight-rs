import { createSurface, createSurfaceRegion } from '@flighthq/surface';
import * as reference from '@flighthq/surface';
import type { Surface, SurfaceRegion } from '@flighthq/types';

import * as rs from './surfaceWasm';

function paintSurface(width: number, height: number): Surface {
  const surface = createSurface(width, height, 0);
  for (let index = 0; index < width * height; index += 1) {
    surface.data[index * 4] = (index * 37 + 11) & 0xff;
    surface.data[index * 4 + 1] = (index * 53 + 7) & 0xff;
    surface.data[index * 4 + 2] = (index * 97 + 3) & 0xff;
    surface.data[index * 4 + 3] = (index * 17 + 1) & 0xff;
  }
  return surface;
}

function fullRegion(surface: Surface): SurfaceRegion {
  return createSurfaceRegion(surface, 0, 0, surface.width, surface.height);
}

function cloneSurface(source: Surface): Surface {
  const clone = createSurface(source.width, source.height, 0);
  clone.data.set(source.data);
  return clone;
}

describe('generated wasm boundary', () => {
  it('initializes synchronously and idempotently', () => {
    rs.initSurfaceWasm();
    rs.initSurfaceWasm();
  });

  it('matches all selected color-matrix builders and composition', () => {
    const actual = Array.from({ length: 20 }, () => 0);
    const expected = Array.from({ length: 20 }, () => 0);
    const cases: Array<[(out: number[]) => void, (out: number[]) => void]> = [
      [
        (out) => rs.buildSurfaceBrightnessColorMatrix(out, 1.25),
        (out) => reference.buildSurfaceBrightnessColorMatrix(out, 1.25),
      ],
      [
        (out) => rs.buildSurfaceContrastColorMatrix(out, 0.75),
        (out) => reference.buildSurfaceContrastColorMatrix(out, 0.75),
      ],
      [rs.buildSurfaceGrayscaleColorMatrix, reference.buildSurfaceGrayscaleColorMatrix],
      [
        (out) => rs.buildSurfaceHueRotationColorMatrix(out, 42),
        (out) => reference.buildSurfaceHueRotationColorMatrix(out, 42),
      ],
      [rs.buildSurfaceInvertColorMatrix, reference.buildSurfaceInvertColorMatrix],
      [
        (out) => rs.buildSurfaceSaturationColorMatrix(out, 1.5),
        (out) => reference.buildSurfaceSaturationColorMatrix(out, 1.5),
      ],
      [rs.buildSurfaceSepiaColorMatrix, reference.buildSurfaceSepiaColorMatrix],
      [rs.setSurfaceColorMatrixIdentity, reference.setSurfaceColorMatrixIdentity],
    ];
    for (const [generated, cultivated] of cases) {
      generated(actual);
      cultivated(expected);
      expect(actual).toEqual(expected);
    }

    const first = actual.slice();
    reference.buildSurfaceContrastColorMatrix(expected, 1.2);
    const second = expected.slice();
    rs.concatSurfaceColorMatrix(actual, first, second);
    reference.concatSurfaceColorMatrix(expected, first, second);
    expect(actual).toEqual(expected);
  });

  it('matches selected output-buffer kernels', () => {
    const source = paintSurface(4, 3);
    const region = fullRegion(source);
    const actual = new Uint8ClampedArray(source.data.length);
    const expected = new Uint8ClampedArray(source.data.length);
    const matrix = Array.from({ length: 20 }, () => 0);
    reference.buildSurfaceSaturationColorMatrix(matrix, 0.4);

    rs.colorMatrixSurface(actual, region, matrix);
    reference.colorMatrixSurface(expected, region, matrix);
    expect(actual).toEqual(expected);

    const options = {
      edge: 'mirror' as const,
      matrix: [1, 2, 1, 2, 4, 2, 1, 2, 1],
      matrixX: 3,
      matrixY: 3,
      preserveAlpha: false,
    };
    rs.convolveSurface(actual, region, options);
    reference.convolveSurface(expected, region, options);
    expect(actual).toEqual(expected);

    rs.dilateSurface(actual, region, 1);
    reference.dilateSurface(expected, region, 1);
    expect(actual).toEqual(expected);

    rs.erodeSurface(actual, region, 1);
    reference.erodeSurface(expected, region, 1);
    expect(actual).toEqual(expected);

    rs.pixelateSurface(actual, region, 2);
    reference.pixelateSurface(expected, region, 2);
    expect(actual).toEqual(expected);

    rs.premultiplySurfacePixels(actual, source.data, source.data.length);
    reference.premultiplySurfacePixels(expected, source.data, source.data.length);
    expect(actual).toEqual(expected);

    rs.unpremultiplySurfacePixels(actual, source.data, source.data.length);
    reference.unpremultiplySurfacePixels(expected, source.data, source.data.length);
    expect(actual).toEqual(expected);
  });

  it('matches mutating region operations, clipping, and invalidation', () => {
    const source = paintSurface(3, 2);
    const actualSurface = paintSurface(3, 2);
    const expectedSurface = cloneSurface(actualSurface);
    const sourceRegion = createSurfaceRegion(source, 0, 0, 3, 2);
    const actual = createSurfaceRegion(actualSurface, -1, 0, 3, 2);
    const expected = createSurfaceRegion(expectedSurface, -1, 0, 3, 2);

    rs.copySurfacePixels(actual, sourceRegion);
    reference.copySurfacePixels(expected, sourceRegion);
    expect(actualSurface.data).toEqual(expectedSurface.data);
    expect(actualSurface.version).toBe(expectedSurface.version);

    rs.fillSurfaceRectangle(actual, 0xaabbccdd);
    reference.fillSurfaceRectangle(expected, 0xaabbccdd);
    expect(actualSurface.data).toEqual(expectedSurface.data);

    rs.multiplySurfaceAlpha(actual, 0.5);
    reference.multiplySurfaceAlpha(expected, 0.5);
    expect(actualSurface.data).toEqual(expectedSurface.data);

    rs.setSurfaceAlpha(actual, 123);
    reference.setSurfaceAlpha(expected, 123);
    expect(actualSurface.data).toEqual(expectedSurface.data);

    rs.copySurfaceAlpha(actual, sourceRegion);
    reference.copySurfaceAlpha(expected, sourceRegion);
    expect(actualSurface.data).toEqual(expectedSurface.data);
    expect(actualSurface.version).toBe(expectedSurface.version);

    const inverted = Array.from({ length: 256 }, (_, value) => 255 - value);
    rs.applySurfacePaletteMap(actual, sourceRegion, inverted, null, null, null);
    reference.applySurfacePaletteMap(expected, sourceRegion, inverted, null, null, null);
    expect(actualSurface.data).toEqual(expectedSurface.data);
    expect(actualSurface.version).toBe(expectedSurface.version);

    const byteInverted = Uint8Array.from(inverted);
    rs.applySurfaceCurve(actual, sourceRegion, byteInverted, null, null, null);
    reference.applySurfaceCurve(expected, sourceRegion, byteInverted, null, null, null);
    expect(actualSurface.data).toEqual(expectedSurface.data);
    expect(actualSurface.version).toBe(expectedSurface.version);

    rs.applySurfaceLevels(actual, sourceRegion, 12, 240, 0.75);
    reference.applySurfaceLevels(expected, sourceRegion, 12, 240, 0.75);
    expect(actualSurface.data).toEqual(expectedSurface.data);
    expect(actualSurface.version).toBe(expectedSurface.version);

    const comparison = cloneSurface(actualSurface);
    comparison.data[5] = (comparison.data[5]! + 128) & 0xff;
    expect(rs.getSurfaceMismatch(actualSurface, comparison, 10)).toEqual(
      reference.getSurfaceMismatch(expectedSurface, comparison, 10),
    );

    rs.mergeSurfaceChannels(actual, sourceRegion, sourceRegion, sourceRegion, sourceRegion);
    reference.mergeSurfaceChannels(expected, sourceRegion, sourceRegion, sourceRegion, sourceRegion);
    expect(actualSurface.data).toEqual(expectedSurface.data);
    expect(actualSurface.version).toBe(expectedSurface.version);

    expect(rs.getSurfaceColorBoundsRectangle(actual, 0xffffff00, 0xaabbcc00)).toEqual(
      reference.getSurfaceColorBoundsRectangle(expected, 0xffffff00, 0xaabbcc00),
    );
    expect(rs.getSurfaceHistogram(actual)).toEqual(reference.getSurfaceHistogram(expected));
  });

  it('matches deterministic noise variants and coverage', () => {
    const actualSurface = createSurface(4, 3, 0);
    const expectedSurface = createSurface(4, 3, 0);
    const actual = fullRegion(actualSurface);
    const expected = fullRegion(expectedSurface);

    rs.fillSurfaceNoise(actual, 123, 10, 240, true);
    reference.fillSurfaceNoise(expected, 123, 10, 240, true);
    expect(actualSurface.data).toEqual(expectedSurface.data);

    rs.fillSurfacePerlinNoise(actual, 3, 4, 2, 99, false, true, 0xf);
    reference.fillSurfacePerlinNoise(expected, 3, 4, 2, 99, false, true, 0xf);
    expect(actualSurface.data).toEqual(expectedSurface.data);

    rs.fillSurfaceTurbulence(actual, 3, 4, 2, 101, true, false, 0x7);
    reference.fillSurfaceTurbulence(expected, 3, 4, 2, 101, true, false, 0x7);
    expect(actualSurface.data).toEqual(expectedSurface.data);

    expect(rs.getSurfaceCoverage(actualSurface, 0, 2)).toBe(reference.getSurfaceCoverage(expectedSurface, 0, 2));
  });
});
