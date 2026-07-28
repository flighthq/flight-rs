// The full upstream API remains available. Only functions present in the
// generator's selected, compiled surface slice are shadowed by wasm-backed
// implementations; every deferred function remains the cultivated TypeScript
// implementation until its source is admitted to generation.
export {
  buildSurfaceBrightnessColorMatrix,
  buildSurfaceContrastColorMatrix,
  buildSurfaceGrayscaleColorMatrix,
  buildSurfaceHueRotationColorMatrix,
  buildSurfaceInvertColorMatrix,
  buildSurfaceSaturationColorMatrix,
  buildSurfaceSepiaColorMatrix,
  colorMatrixSurface,
  concatSurfaceColorMatrix,
  convolveSurface,
  copySurfaceAlpha,
  copySurfacePixels,
  dilateSurface,
  erodeSurface,
  fillSurfaceNoise,
  fillSurfacePerlinNoise,
  fillSurfaceRectangle,
  fillSurfaceTurbulence,
  getSurfaceCoverage,
  initSurfaceWasm,
  multiplySurfaceAlpha,
  pixelateSurface,
  premultiplySurfacePixels,
  setSurfaceAlpha,
  setSurfaceColorMatrixIdentity,
  unpremultiplySurfacePixels,
} from './surfaceWasm';
export * from '@flighthq/surface';
