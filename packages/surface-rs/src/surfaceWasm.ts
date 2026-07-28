import { invalidateImageResource } from '@flighthq/image';
import type { SurfaceConvolutionOptions } from '@flighthq/surface';
import type { RectangleLike, Surface, SurfaceHistogram, SurfaceRegion } from '@flighthq/types';

import {
  apply_surface_palette_map_wasm,
  build_surface_brightness_color_matrix_wasm,
  build_surface_contrast_color_matrix_wasm,
  build_surface_grayscale_color_matrix_wasm,
  build_surface_hue_rotation_color_matrix_wasm,
  build_surface_invert_color_matrix_wasm,
  build_surface_saturation_color_matrix_wasm,
  build_surface_sepia_color_matrix_wasm,
  color_matrix_surface_wasm,
  concat_surface_color_matrix_wasm,
  convolve_surface_wasm,
  copy_surface_alpha_wasm,
  copy_surface_pixels_wasm,
  dilate_surface_wasm,
  erode_surface_wasm,
  fill_surface_noise_wasm,
  fill_surface_perlin_noise_wasm,
  fill_surface_rectangle_wasm,
  fill_surface_turbulence_wasm,
  get_surface_color_bounds_rectangle_wasm,
  get_surface_coverage_wasm,
  get_surface_histogram_wasm,
  initSync,
  multiply_surface_alpha_wasm,
  pixelate_surface_wasm,
  premultiply_surface_pixels_wasm,
  set_surface_alpha_wasm,
  set_surface_color_matrix_identity_wasm,
  unpremultiply_surface_pixels_wasm,
} from './wasm/surface_wasm.js';
import { surfaceWasmBytes } from './wasm/surfaceWasmBytes';

let initialized = false;
const EMPTY_CHANNEL_MAP = new Float64Array();

/**
 * Eagerly instantiates the mechanically generated surface module. Every
 * overridden operation also initializes it lazily, so calling this is optional.
 */
export function initSurfaceWasm(): void {
  ensureSurfaceWasm();
}

export function applySurfacePaletteMap(
  dest: Readonly<SurfaceRegion>,
  source: Readonly<SurfaceRegion>,
  redMap: ReadonlyArray<number> | null,
  greenMap: ReadonlyArray<number> | null,
  blueMap: ReadonlyArray<number> | null,
  alphaMap: ReadonlyArray<number> | null,
): void {
  ensureSurfaceWasm();
  apply_surface_palette_map_wasm(
    asUint8(dest.surface.data),
    descriptorOf(dest),
    asUint8(source.surface.data),
    descriptorOf(source),
    channelMap(redMap),
    channelMap(greenMap),
    channelMap(blueMap),
    channelMap(alphaMap),
  );
  invalidateImageResource(dest.surface);
}

export function buildSurfaceBrightnessColorMatrix(out: number[], amount: number): void {
  runMatrixWriter(out, (typed) => build_surface_brightness_color_matrix_wasm(typed, amount));
}

export function buildSurfaceContrastColorMatrix(out: number[], amount: number): void {
  runMatrixWriter(out, (typed) => build_surface_contrast_color_matrix_wasm(typed, amount));
}

export function buildSurfaceGrayscaleColorMatrix(out: number[]): void {
  runMatrixWriter(out, build_surface_grayscale_color_matrix_wasm);
}

export function buildSurfaceHueRotationColorMatrix(out: number[], degrees: number): void {
  runMatrixWriter(out, (typed) => build_surface_hue_rotation_color_matrix_wasm(typed, degrees));
}

export function buildSurfaceInvertColorMatrix(out: number[]): void {
  runMatrixWriter(out, build_surface_invert_color_matrix_wasm);
}

export function buildSurfaceSaturationColorMatrix(out: number[], amount: number): void {
  runMatrixWriter(out, (typed) => build_surface_saturation_color_matrix_wasm(typed, amount));
}

export function buildSurfaceSepiaColorMatrix(out: number[]): void {
  runMatrixWriter(out, build_surface_sepia_color_matrix_wasm);
}

export function setSurfaceColorMatrixIdentity(out: number[]): void {
  runMatrixWriter(out, set_surface_color_matrix_identity_wasm);
}

export function concatSurfaceColorMatrix(
  out: number[],
  first: ReadonlyArray<number>,
  second: ReadonlyArray<number>,
): void {
  runMatrixWriter(out, (typed) =>
    concat_surface_color_matrix_wasm(typed, Float64Array.from(first), Float64Array.from(second)),
  );
}

export function colorMatrixSurface(
  out: Uint8ClampedArray,
  source: Readonly<SurfaceRegion>,
  matrix: ReadonlyArray<number>,
): void {
  ensureSurfaceWasm();
  color_matrix_surface_wasm(
    asUint8(out),
    asUint8(source.surface.data),
    descriptorOf(source),
    Float64Array.from(matrix),
  );
}

export function convolveSurface(
  out: Uint8ClampedArray,
  source: Readonly<SurfaceRegion>,
  options: Readonly<SurfaceConvolutionOptions>,
): void {
  ensureSurfaceWasm();
  convolve_surface_wasm(
    asUint8(out),
    asUint8(source.surface.data),
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

export function copySurfacePixels(
  dest: Readonly<SurfaceRegion>,
  source: Readonly<SurfaceRegion>,
  composite: boolean = false,
): void {
  ensureSurfaceWasm();
  copy_surface_pixels_wasm(
    asUint8(dest.surface.data),
    descriptorOf(dest),
    asUint8(source.surface.data),
    descriptorOf(source),
    composite,
  );
  invalidateImageResource(dest.surface);
}

export function copySurfaceAlpha(dest: Readonly<SurfaceRegion>, source: Readonly<SurfaceRegion>): void {
  ensureSurfaceWasm();
  copy_surface_alpha_wasm(
    asUint8(dest.surface.data),
    descriptorOf(dest),
    asUint8(source.surface.data),
    descriptorOf(source),
  );
  invalidateImageResource(dest.surface);
}

export function multiplySurfaceAlpha(out: Readonly<SurfaceRegion>, factor: number): void {
  ensureSurfaceWasm();
  multiply_surface_alpha_wasm(asUint8(out.surface.data), descriptorOf(out), factor);
  invalidateImageResource(out.surface);
}

export function setSurfaceAlpha(out: Readonly<SurfaceRegion>, alpha: number): void {
  ensureSurfaceWasm();
  set_surface_alpha_wasm(asUint8(out.surface.data), descriptorOf(out), alpha);
  invalidateImageResource(out.surface);
}

export function fillSurfaceRectangle(dest: Readonly<SurfaceRegion>, color: number): void {
  ensureSurfaceWasm();
  fill_surface_rectangle_wasm(asUint8(dest.surface.data), descriptorOf(dest), color);
  invalidateImageResource(dest.surface);
}

export function fillSurfaceNoise(
  dest: Readonly<SurfaceRegion>,
  seed: number,
  low: number = 0,
  high: number = 255,
  grayScale: boolean = false,
): void {
  ensureSurfaceWasm();
  fill_surface_noise_wasm(asUint8(dest.surface.data), descriptorOf(dest), seed, low, high, grayScale);
  invalidateImageResource(dest.surface);
}

export function fillSurfacePerlinNoise(
  dest: Readonly<SurfaceRegion>,
  baseX: number,
  baseY: number,
  octaves: number,
  seed: number,
  grayScale: boolean = false,
  stitch: boolean = false,
  channelOptions: number = 0x7,
): void {
  ensureSurfaceWasm();
  fill_surface_perlin_noise_wasm(
    asUint8(dest.surface.data),
    descriptorOf(dest),
    baseX,
    baseY,
    octaves,
    seed,
    grayScale,
    stitch,
    channelOptions,
  );
  invalidateImageResource(dest.surface);
}

export function fillSurfaceTurbulence(
  dest: Readonly<SurfaceRegion>,
  baseX: number,
  baseY: number,
  octaves: number,
  seed: number,
  grayScale: boolean = false,
  stitch: boolean = false,
  channelOptions: number = 0x7,
): void {
  ensureSurfaceWasm();
  fill_surface_turbulence_wasm(
    asUint8(dest.surface.data),
    descriptorOf(dest),
    baseX,
    baseY,
    octaves,
    seed,
    grayScale,
    stitch,
    channelOptions,
  );
  invalidateImageResource(dest.surface);
}

export function dilateSurface(out: Uint8ClampedArray, source: Readonly<SurfaceRegion>, radius: number): void {
  ensureSurfaceWasm();
  dilate_surface_wasm(asUint8(out), asUint8(source.surface.data), descriptorOf(source), radius);
}

export function erodeSurface(out: Uint8ClampedArray, source: Readonly<SurfaceRegion>, radius: number): void {
  ensureSurfaceWasm();
  erode_surface_wasm(asUint8(out), asUint8(source.surface.data), descriptorOf(source), radius);
}

export function pixelateSurface(out: Uint8ClampedArray, source: Readonly<SurfaceRegion>, blockSize: number): void {
  ensureSurfaceWasm();
  pixelate_surface_wasm(asUint8(out), asUint8(source.surface.data), descriptorOf(source), blockSize);
}

export function premultiplySurfacePixels(
  out: Uint8ClampedArray,
  source: Readonly<Uint8ClampedArray>,
  length: number,
): void {
  ensureSurfaceWasm();
  premultiply_surface_pixels_wasm(asUint8(out), asUint8(source), length);
}

export function unpremultiplySurfacePixels(
  out: Uint8ClampedArray,
  source: Readonly<Uint8ClampedArray>,
  length: number,
): void {
  ensureSurfaceWasm();
  unpremultiply_surface_pixels_wasm(asUint8(out), asUint8(source), length);
}

export function getSurfaceCoverage(
  source: Readonly<Surface>,
  backgroundColor: number,
  channelTolerance: number = 0,
): number {
  ensureSurfaceWasm();
  return get_surface_coverage_wasm(
    asUint8(source.data),
    source.width,
    source.height,
    backgroundColor,
    channelTolerance,
  );
}

export function getSurfaceColorBoundsRectangle(
  source: Readonly<SurfaceRegion>,
  mask: number,
  color: number,
  findColor: boolean = true,
): RectangleLike | null {
  ensureSurfaceWasm();
  const rectangle = new Float64Array(4);
  const found = get_surface_color_bounds_rectangle_wasm(
    rectangle,
    asUint8(source.surface.data),
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

export function getSurfaceHistogram(source: Readonly<SurfaceRegion>): SurfaceHistogram {
  ensureSurfaceWasm();
  const histogram = new Float64Array(1024);
  get_surface_histogram_wasm(histogram, asUint8(source.surface.data), descriptorOf(source));
  return {
    red: Array.from(histogram.subarray(0, 256)),
    green: Array.from(histogram.subarray(256, 512)),
    blue: Array.from(histogram.subarray(512, 768)),
    alpha: Array.from(histogram.subarray(768, 1024)),
  };
}

function ensureSurfaceWasm(): void {
  if (initialized) return;
  initSync({ module: surfaceWasmBytes });
  initialized = true;
}

function runMatrixWriter(out: number[], operation: (typed: Float64Array) => void): void {
  ensureSurfaceWasm();
  const typed = Float64Array.from(out);
  operation(typed);
  for (let index = 0; index < typed.length; index += 1) out[index] = typed[index]!;
}

function asUint8(data: Readonly<Uint8ClampedArray>): Uint8Array {
  const view = data as Uint8ClampedArray;
  return new Uint8Array(view.buffer, view.byteOffset, view.byteLength);
}

function descriptorOf(region: Readonly<SurfaceRegion>): Float64Array {
  return Float64Array.of(region.surface.width, region.surface.height, region.x, region.y, region.width, region.height);
}

function channelMap(values: ReadonlyArray<number> | null): Float64Array {
  return values ? Float64Array.from(values) : EMPTY_CHANNEL_MAP;
}
