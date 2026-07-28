export interface SourceExclusion {
  reason: string;
  source: string;
}

export interface RustTarget {
  conformanceTemplate?: string;
  crate: string;
  declarationSelection?: Record<string, { names: string[]; reason: string }>;
  dependencies: Record<string, { crate: string }>;
  inlineDependencies?: Record<string, { package: string; source: string }>;
  package: string;
  sourceSelection?: {
    reason: string;
    sources: string[];
  };
  sourceExclusions: SourceExclusion[];
  typeMappings: Record<string, { reason: string; rust: string; rustDefinition?: string; source: string }>;
}

export interface WasmFacadeTarget {
  coreCrate: string;
  crate: string;
  exports: string[];
  rustTemplate: string;
}

export const portConfig = {
  generatedDirectory: 'generated',
  reportsDirectory: 'reports',
  upstreamDirectory: 'upstream',
  blessedFacades: [
    {
      package: '@flighthq/surface-rs',
      path: 'packages/surface-rs',
      reason: 'Curated JavaScript/wasm boundary for the generated @flighthq/surface implementation.',
    },
  ],
  wasmFacades: [
    {
      coreCrate: 'flighthq-surface',
      crate: 'flighthq-surface-wasm',
      exports: [
        'applySurfaceCurve',
        'applySurfaceLevels',
        'applySurfacePaletteMap',
        'buildSurfaceBrightnessColorMatrix',
        'buildSurfaceContrastColorMatrix',
        'buildSurfaceGrayscaleColorMatrix',
        'buildSurfaceHueRotationColorMatrix',
        'buildSurfaceInvertColorMatrix',
        'buildSurfaceSaturationColorMatrix',
        'buildSurfaceSepiaColorMatrix',
        'colorMatrixSurface',
        'concatSurfaceColorMatrix',
        'convolveSurface',
        'copySurfaceAlpha',
        'copySurfacePixels',
        'dilateSurface',
        'erodeSurface',
        'fillSurfaceNoise',
        'fillSurfacePerlinNoise',
        'fillSurfaceRectangle',
        'fillSurfaceTurbulence',
        'getSurfaceColorBoundsRectangle',
        'getSurfaceCoverage',
        'getSurfaceHistogram',
        'getSurfaceMismatch',
        'mergeSurfaceChannels',
        'multiplySurfaceAlpha',
        'pixelateSurface',
        'premultiplySurfacePixels',
        'setSurfaceAlpha',
        'setSurfaceColorMatrixIdentity',
        'unpremultiplySurfacePixels',
      ],
      rustTemplate: 'tools/generator/templates/surface_wasm.rs',
    },
  ] satisfies WasmFacadeTarget[],
  targets: [
    {
      crate: 'flighthq-types',
      dependencies: {},
      package: '@flighthq/types',
      sourceSelection: {
        sources: [
          'AlphaType.ts',
          'ColorTransform.ts',
          'EasingFunction.ts',
          'EasingSegment.ts',
          'GradientSpread.ts',
          'ImageFormat.ts',
          'ImageResource.ts',
          'ImageResourceCompressed.ts',
          'PixelFormat.ts',
          'PixelOrder.ts',
          'Rectangle.ts',
          'ScalarRemap.ts',
          'StepPosition.ts',
          'Surface.ts',
          'SurfaceCompositeMode.ts',
          'SurfaceEdgeMode.ts',
          'SurfaceFingerprint.ts',
          'SurfaceHistogram.ts',
          'SurfaceMismatch.ts',
          'SurfaceRegion.ts',
          'SurfaceResizeMode.ts',
          'TextureContainer.ts',
          'TextureContainerFormat.ts',
          'TextureContainerLevel.ts',
          'TextureContainerSupercompression.ts',
          'ThresholdOperation.ts',
        ],
        reason:
          'Generated type closure required by the easing canary and surface/image ABI; additional records are added as their consuming packages enter the compiled target set.',
      },
      sourceExclusions: [],
      typeMappings: {},
    },
    {
      conformanceTemplate: 'tools/generator/templates/easing_conformance.rs',
      crate: 'flighthq-easing',
      dependencies: {
        '@flighthq/types': { crate: 'flighthq-types' },
      },
      package: '@flighthq/easing',
      sourceExclusions: [],
      typeMappings: {},
    },
    {
      conformanceTemplate: 'tools/generator/templates/image_conformance.rs',
      crate: 'flighthq-image',
      declarationSelection: {
        'imageResource.ts': {
          names: [
            'disposeImageResource',
            'hasImageResourceData',
            'hasImageResourcePixels',
            'hasImageResourceSource',
            'invalidateImageResource',
            'isImageResourceEmpty',
          ],
          reason:
            'Host-element construction and cloning remain on the TypeScript side; the selected portable resource-state operations are generated for the surface dependency path.',
        },
      },
      dependencies: {
        '@flighthq/types': { crate: 'flighthq-types' },
      },
      package: '@flighthq/image',
      sourceSelection: {
        sources: ['imageResource.ts'],
        reason:
          'Browser element/load/decode operations in imageResourceFrom.ts are host-bound and remain in the blessed TypeScript boundary.',
      },
      sourceExclusions: [],
      typeMappings: {},
    },
    {
      conformanceTemplate: 'tools/generator/templates/surface_conformance.rs',
      crate: 'flighthq-surface',
      declarationSelection: {
        'surfaceCompare.ts': {
          names: ['getSurfaceMismatch'],
          reason:
            'Diff-surface construction remains at the TypeScript boundary; the allocation-free mismatch summary is independently portable.',
        },
        'surfaceChannel.ts': {
          names: ['mergeSurfaceChannels'],
          reason:
            'Surface allocation through the entity package remains at the TypeScript boundary; the region-based merge kernel is independently portable.',
        },
        'surfaceCopy.ts': {
          names: ['copySurfacePixels'],
          reason:
            'Channel-specific copying joins with the generated ImageChannel type and relative re-export resolution.',
        },
        'surfaceFill.ts': {
          names: ['fillSurfaceRectangle'],
          reason:
            'Flood fill joins after reusable module-level scratch buffers lower to generated Rust synchronization primitives.',
        },
        'surfaceFormat.ts': {
          names: ['premultiplySurfacePixels', 'unpremultiplySurfacePixels'],
          reason:
            'Pixel-order conversion joins after tuple destructuring and typed-array subarray/set methods lower without losing alias safety.',
        },
        'surfaceHistogram.ts': {
          names: ['getSurfaceHistogram'],
          reason:
            'Histogram equalization joins after its palette-map dependency enters the generated surface slice; the allocation-only histogram query is independently portable.',
        },
        'surfacePixel.ts': {
          names: [
            'LUMA_B',
            'LUMA_G',
            'LUMA_R',
            'getSurfacePixel',
            'getSurfacePixelLuminance',
            'getSurfacePixelRgb',
            'setSurfacePixel',
            'setSurfacePixelRgb',
          ],
          reason:
            'The ImageChannel-parameterized reader joins when relative TypeScript re-export imports resolve directly to their generated dependency crate.',
        },
      },
      dependencies: {
        '@flighthq/image': { crate: 'flighthq-image' },
        '@flighthq/types': { crate: 'flighthq-types' },
      },
      inlineDependencies: {
        invalidateImageResource: {
          package: '@flighthq/image',
          source: 'upstream/packages/image/src/imageResource.ts',
        },
      },
      package: '@flighthq/surface',
      sourceSelection: {
        sources: [
          'surfaceAlpha.ts',
          'surfaceChannel.ts',
          'surfaceColorMatrix.ts',
          'surfaceCompare.ts',
          'surfaceConvolution.ts',
          'surfaceCopy.ts',
          'surfaceCoverage.ts',
          'surfaceFill.ts',
          'surfaceFormat.ts',
          'surfaceHistogram.ts',
          'surfaceMorphological.ts',
          'surfaceNoise.ts',
          'surfacePaletteMap.ts',
          'surfacePixel.ts',
          'surfacePixelate.ts',
          'surfaceQuery.ts',
          'surfaceTone.ts',
        ],
        reason:
          'Initial compiled surface kernel slice; remaining portable modules are admitted as their required emitter constructs and alias rules compile.',
      },
      sourceExclusions: [],
      typeMappings: {},
    },
  ] satisfies RustTarget[],
} as const;
