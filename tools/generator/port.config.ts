import type { IrHostConstructorCapability } from './src/model/ir.ts';

export interface SourceExclusion {
  reason: string;
  source: string;
}

export interface NativeHostConstructor {
  capability: IrHostConstructorCapability;
  global: string;
  reason: string;
  resultType: string;
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

export type PackageDisposition = 'cultivated' | 'excluded' | 'host-backend' | 'host-bound';

export interface PackagePolicyRule {
  disposition: PackageDisposition;
  match: string;
  reason: string;
}

export const portConfig = {
  generatedDirectory: 'generated',
  reportsDirectory: 'reports',
  upstreamDirectory: 'upstream',
  typeLowering: {
    entityRuntimeFamily: {
      entityType: 'Entity',
      package: '@flighthq/types',
      reason:
        'EntityRuntime is the declared root for generated native entity storage; package-visible extensions may join only when they resolve against this canonical source family.',
      runtimeType: 'EntityRuntime',
    },
    nativeHostConstructors: [
      {
        capability: 'ImageData',
        global: 'ImageData',
        reason:
          'ImageData construction crosses a declared native backend seam while retaining typed pixel and dimension inputs.',
        resultType: 'FlightImageData',
      },
      {
        capability: 'OffscreenCanvas',
        global: 'OffscreenCanvas',
        reason:
          'OffscreenCanvas construction crosses a declared native backend seam with typed dimensions and a distinct native handle.',
        resultType: 'FlightOffscreenCanvas',
      },
      {
        capability: 'URL',
        global: 'URL',
        reason: 'URL construction crosses a declared native backend seam with typed value and optional base inputs.',
        resultType: 'FlightUrl',
      },
    ] satisfies NativeHostConstructor[],
    genericIntersectionBaseOverrides: [
      {
        name: 'Node',
        reason:
          'Flight node aliases intersect the concrete Node storage record with a generic traits parameter; Rust keeps the storage record while traits move through generated runtime fields.',
      },
    ],
    transparentTypeWrappers: [
      {
        name: 'EntityWithoutRuntime',
        reason:
          'EntityWithoutRuntime removes the TypeScript symbol member from the public shape; generated Rust carries it in the source-derived aggregate EntityRuntime slot and rejects receivers outside the statically closed entity family.',
      },
    ],
  },
  packagePolicy: [
    {
      disposition: 'host-backend',
      match: '@flighthq/*-canvas',
      reason:
        'Canvas backend packages mechanically emit against dynamic browser handles until typed backend-capability IR replaces their opaque native seam.',
    },
    {
      disposition: 'host-backend',
      match: '@flighthq/*-gl',
      reason:
        'WebGL backend packages mechanically emit against dynamic browser handles until typed backend-capability IR replaces their opaque native seam.',
    },
    {
      disposition: 'host-backend',
      match: '@flighthq/*-wgpu',
      reason:
        'WebGPU backend packages mechanically emit against dynamic browser handles until typed backend-capability IR replaces their opaque native seam.',
    },
    {
      disposition: 'cultivated',
      match: '@flighthq/surface',
      reason:
        'The TypeScript/wasm facade and its explicit source/declaration admission policy are cultivated; flighthq-surface itself is generated from those selections.',
    },
    {
      disposition: 'excluded',
      match: '@flighthq/tool-capture',
      reason: 'Build-time browser capture tooling is not part of the native Flight runtime crate graph.',
    },
    {
      disposition: 'host-bound',
      match: '@flighthq/host-capacitor',
      reason: 'The Capacitor adapter is a platform integration package rather than a mechanical core-library port.',
    },
    {
      disposition: 'host-bound',
      match: '@flighthq/host-electron',
      reason: 'The Electron adapter is a platform integration package rather than a mechanical core-library port.',
    },
    {
      disposition: 'host-bound',
      match: '@flighthq/host-tauri',
      reason: 'The Tauri adapter is a platform integration package rather than a mechanical core-library port.',
    },
    {
      disposition: 'host-bound',
      match: '@flighthq/*-dom',
      reason: 'DOM substrate packages remain host-bound; native hosts consume the generated substrate-neutral crates.',
    },
  ] satisfies PackagePolicyRule[],
  // Grandfathered substrate-neutral opaque-source counts from the pass-23 report.
  // Entries may shrink as typed lowering lands; growth requires a declared host-backend policy.
  opaqueHostValueBaseline: {
    '@flighthq/accessibility': 1,
    '@flighthq/animation': 1,
    '@flighthq/app': 1,
    '@flighthq/application': 2,
    '@flighthq/assets': 1,
    '@flighthq/audio': 2,
    '@flighthq/bitmapfont-formats': 3,
    '@flighthq/bitmaptext': 2,
    '@flighthq/capture': 1,
    '@flighthq/clip': 1,
    '@flighthq/connectivity': 1,
    '@flighthq/debug': 1,
    '@flighthq/device': 1,
    '@flighthq/displayobject': 6,
    '@flighthq/displayobject-canvas': 15,
    '@flighthq/displayobject-gl': 15,
    '@flighthq/displayobject-wgpu': 13,
    '@flighthq/effects': 2,
    '@flighthq/effects-canvas': 7,
    '@flighthq/effects-gl': 8,
    '@flighthq/effects-wgpu': 2,
    '@flighthq/entity': 1,
    '@flighthq/font': 1,
    '@flighthq/glyphatlas': 2,
    '@flighthq/image': 2,
    '@flighthq/input': 1,
    '@flighthq/interaction': 9,
    '@flighthq/keyboard': 1,
    '@flighthq/lifecycle': 1,
    '@flighthq/materials': 1,
    '@flighthq/media': 3,
    '@flighthq/mesh': 2,
    '@flighthq/movieclip': 2,
    '@flighthq/node': 6,
    '@flighthq/particleemitter': 10,
    '@flighthq/particles': 1,
    '@flighthq/particles-formats': 5,
    '@flighthq/platform': 1,
    '@flighthq/power': 1,
    '@flighthq/protocol': 1,
    '@flighthq/render': 10,
    '@flighthq/render-gl': 15,
    '@flighthq/render-wgpu': 9,
    '@flighthq/scene': 7,
    '@flighthq/scene-formats': 2,
    '@flighthq/scene-gl': 24,
    '@flighthq/scene-resources': 13,
    '@flighthq/scene-wgpu': 18,
    '@flighthq/screen': 1,
    '@flighthq/shading': 2,
    '@flighthq/shape': 2,
    '@flighthq/shell': 1,
    '@flighthq/skeleton3d': 2,
    '@flighthq/snapshot': 4,
    '@flighthq/socket': 1,
    '@flighthq/sprite': 3,
    '@flighthq/spritesheet': 4,
    '@flighthq/spritesheet-formats': 7,
    '@flighthq/text': 3,
    '@flighthq/text-markup': 1,
    '@flighthq/textinput': 4,
    '@flighthq/textlayout': 1,
    '@flighthq/textsegment': 1,
    '@flighthq/textshaper': 1,
    '@flighthq/textshaper-canvas': 1,
    '@flighthq/textureatlas': 1,
    '@flighthq/textureatlas-formats': 1,
    '@flighthq/tilemap-formats': 2,
    '@flighthq/tileset': 1,
    '@flighthq/tween': 1,
    '@flighthq/types': 76,
    '@flighthq/useragent': 1,
    '@flighthq/velocity': 2,
    '@flighthq/video': 1,
    '@flighthq/xml': 2,
  } satisfies Record<string, number>,
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
        'compareSurfaceFingerprints',
        'concatSurfaceColorMatrix',
        'convolveSurface',
        'copySurfaceAlpha',
        'copySurfacePixels',
        'createSurfaceFingerprint',
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
      declarationSelection: {
        'Entity.ts': {
          names: ['Entity', 'EntityRuntime'],
          reason:
            'Entity-backed promoted records require the native runtime root storage and trait; utility aliases and the symbol key remain deferred with their consuming operations.',
        },
      },
      dependencies: {},
      package: '@flighthq/types',
      sourceSelection: {
        sources: [
          'AlphaType.ts',
          'ColorTransform.ts',
          'EasingFunction.ts',
          'EasingSegment.ts',
          'Entity.ts',
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
        'surfaceFingerprint.ts': {
          names: ['compareSurfaceFingerprints', 'createSurfaceFingerprint'],
          reason:
            'String slicing and numeric text parsing remain at the TypeScript boundary; typed-array fingerprint construction and comparison are independently portable.',
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
          'surfaceFingerprint.ts',
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
