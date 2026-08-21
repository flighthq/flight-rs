import { invalidateBitmap } from '@flighthq/bitmap';
import type {
  RectangleLike,
  Bitmap,
  BitmapConvolutionOptions,
  BitmapFingerprint,
  BitmapHistogram,
  BitmapMismatch,
  BitmapRegion,
} from '@flighthq/types';

import {
  apply_bitmap_curve_wasm,
  apply_bitmap_levels_wasm,
  apply_bitmap_palette_map_wasm,
  build_bitmap_brightness_color_matrix_wasm,
  build_bitmap_contrast_color_matrix_wasm,
  build_bitmap_grayscale_color_matrix_wasm,
  build_bitmap_hue_rotation_color_matrix_wasm,
  build_bitmap_invert_color_matrix_wasm,
  build_bitmap_saturation_color_matrix_wasm,
  build_bitmap_sepia_color_matrix_wasm,
  color_matrix_bitmap_wasm,
  compare_bitmap_fingerprints_wasm,
  concat_bitmap_color_matrix_wasm,
  convolve_bitmap_wasm,
  copy_bitmap_alpha_wasm,
  copy_bitmap_pixels_wasm,
  create_bitmap_fingerprint_wasm,
  dilate_bitmap_wasm,
  erode_bitmap_wasm,
  fill_bitmap_noise_wasm,
  fill_bitmap_perlin_noise_wasm,
  fill_bitmap_rectangle_wasm,
  fill_bitmap_turbulence_wasm,
  get_bitmap_color_bounds_rectangle_wasm,
  get_bitmap_coverage_wasm,
  get_bitmap_histogram_wasm,
  get_bitmap_mismatch_wasm,
  initSync,
  merge_bitmap_channels_wasm,
  multiply_bitmap_alpha_wasm,
  pixelate_bitmap_wasm,
  premultiply_bitmap_pixels_wasm,
  set_bitmap_alpha_wasm,
  set_bitmap_color_matrix_identity_wasm,
  unpremultiply_bitmap_pixels_wasm,
} from './wasm/bitmap_wasm.js';
import { bitmapWasmBytes } from './wasm/bitmapWasmBytes';

let initialized = false;
const EMPTY_CHANNEL_MAP = new Float64Array();
const EMPTY_BYTE_CHANNEL_MAP = new Uint8Array();

/**
 * Eagerly instantiates the mechanically generated bitmap module. Every
 * overridden operation also initializes it lazily, so calling this is optional.
 */
export function initBitmapWasm(): void {
  ensureBitmapWasm();
}

export function applyBitmapCurve(
  out: Readonly<BitmapRegion>,
  source: Readonly<BitmapRegion>,
  redLut: Readonly<Uint8Array | Uint8ClampedArray | null>,
  greenLut: Readonly<Uint8Array | Uint8ClampedArray | null>,
  blueLut: Readonly<Uint8Array | Uint8ClampedArray | null>,
  alphaLut: Readonly<Uint8Array | Uint8ClampedArray | null> = null,
): void {
  ensureBitmapWasm();
  apply_bitmap_curve_wasm(
    asUint8(out.bitmap.data),
    descriptorOf(out),
    asUint8(source.bitmap.data),
    descriptorOf(source),
    byteChannelMap(redLut),
    byteChannelMap(greenLut),
    byteChannelMap(blueLut),
    byteChannelMap(alphaLut),
  );
  invalidateBitmap(out.bitmap);
}

export function applyBitmapLevels(
  out: Readonly<BitmapRegion>,
  source: Readonly<BitmapRegion>,
  blackPoint: number = 0,
  whitePoint: number = 255,
  gamma: number = 1,
): void {
  ensureBitmapWasm();
  apply_bitmap_levels_wasm(
    asUint8(out.bitmap.data),
    descriptorOf(out),
    asUint8(source.bitmap.data),
    descriptorOf(source),
    blackPoint,
    whitePoint,
    gamma,
  );
  invalidateBitmap(out.bitmap);
}

export function applyBitmapPaletteMap(
  dest: Readonly<BitmapRegion>,
  source: Readonly<BitmapRegion>,
  redMap: ReadonlyArray<number> | null,
  greenMap: ReadonlyArray<number> | null,
  blueMap: ReadonlyArray<number> | null,
  alphaMap: ReadonlyArray<number> | null,
): void {
  ensureBitmapWasm();
  apply_bitmap_palette_map_wasm(
    asUint8(dest.bitmap.data),
    descriptorOf(dest),
    asUint8(source.bitmap.data),
    descriptorOf(source),
    channelMap(redMap),
    channelMap(greenMap),
    channelMap(blueMap),
    channelMap(alphaMap),
  );
  invalidateBitmap(dest.bitmap);
}

export function buildBitmapBrightnessColorMatrix(out: number[], amount: number): void {
  runMatrixWriter(out, (typed) => build_bitmap_brightness_color_matrix_wasm(typed, amount));
}

export function buildBitmapContrastColorMatrix(out: number[], amount: number): void {
  runMatrixWriter(out, (typed) => build_bitmap_contrast_color_matrix_wasm(typed, amount));
}

export function buildBitmapGrayscaleColorMatrix(out: number[]): void {
  runMatrixWriter(out, build_bitmap_grayscale_color_matrix_wasm);
}

export function buildBitmapHueRotationColorMatrix(out: number[], degrees: number): void {
  runMatrixWriter(out, (typed) => build_bitmap_hue_rotation_color_matrix_wasm(typed, degrees));
}

export function buildBitmapInvertColorMatrix(out: number[]): void {
  runMatrixWriter(out, build_bitmap_invert_color_matrix_wasm);
}

export function buildBitmapSaturationColorMatrix(out: number[], amount: number): void {
  runMatrixWriter(out, (typed) => build_bitmap_saturation_color_matrix_wasm(typed, amount));
}

export function buildBitmapSepiaColorMatrix(out: number[]): void {
  runMatrixWriter(out, build_bitmap_sepia_color_matrix_wasm);
}

export function setBitmapColorMatrixIdentity(out: number[]): void {
  runMatrixWriter(out, set_bitmap_color_matrix_identity_wasm);
}

export function concatBitmapColorMatrix(
  out: number[],
  first: ReadonlyArray<number>,
  second: ReadonlyArray<number>,
): void {
  runMatrixWriter(out, (typed) =>
    concat_bitmap_color_matrix_wasm(typed, Float64Array.from(first), Float64Array.from(second)),
  );
}

export function colorMatrixBitmap(
  out: Uint8ClampedArray,
  source: Readonly<BitmapRegion>,
  matrix: ReadonlyArray<number>,
): void {
  ensureBitmapWasm();
  color_matrix_bitmap_wasm(asUint8(out), asUint8(source.bitmap.data), descriptorOf(source), Float64Array.from(matrix));
}

export function compareBitmapFingerprints(
  first: Readonly<BitmapFingerprint>,
  second: Readonly<BitmapFingerprint>,
): number {
  ensureBitmapWasm();
  return compare_bitmap_fingerprints_wasm(asUint8(first.cells), first.gridSize, asUint8(second.cells), second.gridSize);
}

export function createBitmapFingerprint(source: Readonly<Bitmap>, gridSize: number = 16): BitmapFingerprint {
  ensureBitmapWasm();
  const cells = new Uint8Array(gridSize * gridSize * 3);
  create_bitmap_fingerprint_wasm(cells, asUint8(source.data), source.width, source.height, gridSize);
  return { cells, gridSize };
}

export function convolveBitmap(
  out: Uint8ClampedArray,
  source: Readonly<BitmapRegion>,
  options: Readonly<BitmapConvolutionOptions>,
): void {
  ensureBitmapWasm();
  convolve_bitmap_wasm(
    asUint8(out),
    asUint8(source.bitmap.data),
    descriptorOf(source),
    Float64Array.from(options.matrix),
    options.matrixX,
    options.matrixY,
    options.bias ?? 0,
    options.edge ?? 'clamp',
    options.divisor ?? Number.NaN,
    options.preserveAlpha ?? true,
  );
}

export function copyBitmapPixels(
  dest: Readonly<BitmapRegion>,
  source: Readonly<BitmapRegion>,
  composite: boolean = false,
): void {
  ensureBitmapWasm();
  copy_bitmap_pixels_wasm(
    asUint8(dest.bitmap.data),
    descriptorOf(dest),
    asUint8(source.bitmap.data),
    descriptorOf(source),
    composite,
  );
  invalidateBitmap(dest.bitmap);
}

export function copyBitmapAlpha(dest: Readonly<BitmapRegion>, source: Readonly<BitmapRegion>): void {
  ensureBitmapWasm();
  copy_bitmap_alpha_wasm(
    asUint8(dest.bitmap.data),
    descriptorOf(dest),
    asUint8(source.bitmap.data),
    descriptorOf(source),
  );
  invalidateBitmap(dest.bitmap);
}

export function multiplyBitmapAlpha(out: Readonly<BitmapRegion>, factor: number): void {
  ensureBitmapWasm();
  multiply_bitmap_alpha_wasm(asUint8(out.bitmap.data), descriptorOf(out), factor);
  invalidateBitmap(out.bitmap);
}

export function setBitmapAlpha(out: Readonly<BitmapRegion>, alpha: number): void {
  ensureBitmapWasm();
  set_bitmap_alpha_wasm(asUint8(out.bitmap.data), descriptorOf(out), alpha);
  invalidateBitmap(out.bitmap);
}

export function fillBitmapRectangle(dest: Readonly<BitmapRegion>, color: number): void {
  ensureBitmapWasm();
  fill_bitmap_rectangle_wasm(asUint8(dest.bitmap.data), descriptorOf(dest), color);
  invalidateBitmap(dest.bitmap);
}

export function fillBitmapNoise(
  dest: Readonly<BitmapRegion>,
  seed: number,
  low: number = 0,
  high: number = 255,
  grayScale: boolean = false,
): void {
  ensureBitmapWasm();
  fill_bitmap_noise_wasm(asUint8(dest.bitmap.data), descriptorOf(dest), seed, low, high, grayScale);
  invalidateBitmap(dest.bitmap);
}

export function fillBitmapPerlinNoise(
  dest: Readonly<BitmapRegion>,
  baseX: number,
  baseY: number,
  octaves: number,
  seed: number,
  grayScale: boolean = false,
  stitch: boolean = false,
  channelOptions: number = 0x7,
): void {
  ensureBitmapWasm();
  fill_bitmap_perlin_noise_wasm(
    asUint8(dest.bitmap.data),
    descriptorOf(dest),
    baseX,
    baseY,
    octaves,
    seed,
    grayScale,
    stitch,
    channelOptions,
  );
  invalidateBitmap(dest.bitmap);
}

export function fillBitmapTurbulence(
  dest: Readonly<BitmapRegion>,
  baseX: number,
  baseY: number,
  octaves: number,
  seed: number,
  grayScale: boolean = false,
  stitch: boolean = false,
  channelOptions: number = 0x7,
): void {
  ensureBitmapWasm();
  fill_bitmap_turbulence_wasm(
    asUint8(dest.bitmap.data),
    descriptorOf(dest),
    baseX,
    baseY,
    octaves,
    seed,
    grayScale,
    stitch,
    channelOptions,
  );
  invalidateBitmap(dest.bitmap);
}

export function dilateBitmap(out: Uint8ClampedArray, source: Readonly<BitmapRegion>, radius: number): void {
  ensureBitmapWasm();
  dilate_bitmap_wasm(asUint8(out), asUint8(source.bitmap.data), descriptorOf(source), radius);
}

export function erodeBitmap(out: Uint8ClampedArray, source: Readonly<BitmapRegion>, radius: number): void {
  ensureBitmapWasm();
  erode_bitmap_wasm(asUint8(out), asUint8(source.bitmap.data), descriptorOf(source), radius);
}

export function pixelateBitmap(out: Uint8ClampedArray, source: Readonly<BitmapRegion>, blockSize: number): void {
  ensureBitmapWasm();
  pixelate_bitmap_wasm(asUint8(out), asUint8(source.bitmap.data), descriptorOf(source), blockSize);
}

export function premultiplyBitmapPixels(
  out: Uint8ClampedArray,
  source: Readonly<Uint8ClampedArray>,
  length: number,
): void {
  ensureBitmapWasm();
  premultiply_bitmap_pixels_wasm(asUint8(out), asUint8(source), length);
}

export function unpremultiplyBitmapPixels(
  out: Uint8ClampedArray,
  source: Readonly<Uint8ClampedArray>,
  length: number,
): void {
  ensureBitmapWasm();
  unpremultiply_bitmap_pixels_wasm(asUint8(out), asUint8(source), length);
}

export function getBitmapCoverage(
  source: Readonly<Bitmap>,
  backgroundColor: number,
  channelTolerance: number = 0,
): number {
  ensureBitmapWasm();
  return get_bitmap_coverage_wasm(asUint8(source.data), source.width, source.height, backgroundColor, channelTolerance);
}

export function getBitmapColorBoundsRectangle(
  source: Readonly<BitmapRegion>,
  mask: number,
  color: number,
  findColor: boolean = true,
): RectangleLike | null {
  ensureBitmapWasm();
  const rectangle = new Float64Array(4);
  const found = get_bitmap_color_bounds_rectangle_wasm(
    rectangle,
    asUint8(source.bitmap.data),
    descriptorOf(source),
    mask,
    color,
    findColor,
  );
  return found
    ? {
        x: rectangle[0]!,
        y: rectangle[1]!,
        width: rectangle[2]!,
        height: rectangle[3]!,
      }
    : null;
}

export function getBitmapHistogram(source: Readonly<BitmapRegion>): BitmapHistogram {
  ensureBitmapWasm();
  const histogram = new Float64Array(1024);
  get_bitmap_histogram_wasm(histogram, asUint8(source.bitmap.data), descriptorOf(source));
  return {
    red: Array.from(histogram.subarray(0, 256)),
    green: Array.from(histogram.subarray(256, 512)),
    blue: Array.from(histogram.subarray(512, 768)),
    alpha: Array.from(histogram.subarray(768, 1024)),
  };
}

export function getBitmapMismatch(
  source: Readonly<Bitmap>,
  other: Readonly<Bitmap>,
  channelTolerance: number = 0,
): BitmapMismatch {
  ensureBitmapWasm();
  const mismatch = new Float64Array(4);
  get_bitmap_mismatch_wasm(
    mismatch,
    asUint8(source.data),
    source.width,
    source.height,
    asUint8(other.data),
    other.width,
    other.height,
    channelTolerance,
  );
  return {
    mismatchedPixels: mismatch[0]!,
    totalPixels: mismatch[1]!,
    fraction: mismatch[2]!,
    maxChannelDelta: mismatch[3]!,
  };
}

export function mergeBitmapChannels(
  out: Readonly<BitmapRegion>,
  red: Readonly<BitmapRegion>,
  green: Readonly<BitmapRegion>,
  blue: Readonly<BitmapRegion>,
  alpha: Readonly<BitmapRegion>,
): void {
  ensureBitmapWasm();
  merge_bitmap_channels_wasm(
    asUint8(out.bitmap.data),
    descriptorOf(out),
    asUint8(red.bitmap.data),
    descriptorOf(red),
    asUint8(green.bitmap.data),
    descriptorOf(green),
    asUint8(blue.bitmap.data),
    descriptorOf(blue),
    asUint8(alpha.bitmap.data),
    descriptorOf(alpha),
  );
  invalidateBitmap(out.bitmap);
}

function ensureBitmapWasm(): void {
  if (initialized) return;
  initSync({ module: bitmapWasmBytes });
  initialized = true;
}

function runMatrixWriter(out: number[], operation: (typed: Float64Array) => void): void {
  ensureBitmapWasm();
  const typed = Float64Array.from(out);
  operation(typed);
  for (let index = 0; index < typed.length; index += 1) out[index] = typed[index]!;
}

function asUint8(data: Readonly<Uint8Array | Uint8ClampedArray>): Uint8Array {
  const view = data as Uint8Array | Uint8ClampedArray;
  return new Uint8Array(view.buffer, view.byteOffset, view.byteLength);
}

function descriptorOf(region: Readonly<BitmapRegion>): Float64Array {
  return Float64Array.of(region.bitmap.width, region.bitmap.height, region.x, region.y, region.width, region.height);
}

function channelMap(values: ReadonlyArray<number> | null): Float64Array {
  return values ? Float64Array.from(values) : EMPTY_CHANNEL_MAP;
}

function byteChannelMap(values: Readonly<Uint8Array | Uint8ClampedArray | null>): Uint8Array {
  return values ? asUint8(values) : EMPTY_BYTE_CHANNEL_MAP;
}
