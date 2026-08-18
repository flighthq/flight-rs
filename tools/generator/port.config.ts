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

export interface ConformanceHarvestPackage {
  package: string;
  sources: string[];
  unsupportedReason: string;
}

export interface RustTarget {
  conformanceTemplate?: string;
  crate: string;
  declarationSelection?: Record<string, { names: string[]; reason: string }>;
  dependencies: Record<string, { crate: string }>;
  fullyPromoted?: boolean;
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
  conformanceHarvest: [
    {
      package: '@flighthq/math',
      sources: ['clamp.test.ts', 'comparison.test.ts', 'constants.test.ts'],
      unsupportedReason:
        'Outside the first pure scalar-expression harvest; translator support must be added before this upstream test file is admitted.',
    },
    {
      package: '@flighthq/color',
      sources: ['srgbTransfer.test.ts'],
      unsupportedReason:
        'Outside the first pure scalar-expression harvest; translator support must be added before this upstream test file is admitted.',
    },
  ] satisfies ConformanceHarvestPackage[],
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
      match: '@flighthq/bitmap',
      reason:
        'The TypeScript/wasm facade and its explicit source/declaration admission policy are cultivated; the compatibility-named flighthq-surface crate is generated from the upstream bitmap replacement.',
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
      reason: 'Curated JavaScript/wasm compatibility boundary for the generated @flighthq/bitmap implementation.',
    },
  ],
  wasmFacades: [
    {
      coreCrate: 'flighthq-surface',
      crate: 'flighthq-surface-wasm',
      exports: [
        'applyBitmapCurve',
        'applyBitmapLevels',
        'applyBitmapPaletteMap',
        'buildBitmapBrightnessColorMatrix',
        'buildBitmapContrastColorMatrix',
        'buildBitmapGrayscaleColorMatrix',
        'buildBitmapHueRotationColorMatrix',
        'buildBitmapInvertColorMatrix',
        'buildBitmapSaturationColorMatrix',
        'buildBitmapSepiaColorMatrix',
        'colorMatrixBitmap',
        'compareBitmapFingerprints',
        'concatBitmapColorMatrix',
        'convolveBitmap',
        'copyBitmapAlpha',
        'copyBitmapPixels',
        'createBitmapFingerprint',
        'dilateBitmap',
        'erodeBitmap',
        'fillBitmapNoise',
        'fillBitmapPerlinNoise',
        'fillBitmapRectangle',
        'fillBitmapTurbulence',
        'getBitmapColorBoundsRectangle',
        'getBitmapCoverage',
        'getBitmapHistogram',
        'getBitmapMismatch',
        'mergeBitmapChannels',
        'multiplyBitmapAlpha',
        'pixelateBitmap',
        'premultiplyBitmapPixels',
        'setBitmapAlpha',
        'setBitmapColorMatrixIdentity',
        'unpremultiplyBitmapPixels',
      ],
      rustTemplate: 'tools/generator/templates/surface_wasm.rs',
    },
  ] satisfies WasmFacadeTarget[],
  targets: [
    {
      crate: 'flighthq-types',
      dependencies: {},
      fullyPromoted: true,
      package: '@flighthq/types',
      sourceExclusions: [],
      typeMappings: {},
    },
    {
      crate: 'flighthq-application',
      declarationSelection: {
        'application.ts': {
          names: ['_loopBackend', 'setLoopBackend'],
          reason: 'The native-host canary needs only the synchronous loop-backend installation seam.',
        },
        'window.ts': {
          names: ['_windowBackend', 'setWindowBackend'],
          reason: 'The native-host canary needs only the synchronous window-backend installation seam.',
        },
      },
      dependencies: {
        '@flighthq/types': { crate: 'flighthq-types' },
      },
      package: '@flighthq/application',
      sourceExclusions: [],
      sourceSelection: {
        sources: ['application.ts', 'window.ts'],
        reason:
          'The cultivated host installs synchronous native backends without admitting the package web task factories or unresolved host-task calls.',
      },
      typeMappings: {},
    },
    {
      crate: 'flighthq-host-signals',
      declarationSelection: {
        'signal.ts': {
          names: ['createSignal'],
          reason:
            'The synchronous input-manager seam needs the source-derived signal constructor without admitting unrelated signal operators.',
        },
      },
      dependencies: {
        '@flighthq/types': { crate: 'flighthq-types' },
      },
      package: '@flighthq/signals',
      sourceExclusions: [],
      sourceSelection: {
        sources: ['signal.ts'],
        reason:
          'The partial target supplies the typed signal constructor used by the cultivated native-host input seam.',
      },
      typeMappings: {},
    },
    {
      crate: 'flighthq-input',
      declarationSelection: {
        'inputManager.ts': {
          names: ['createInputManager', 'createInputSignals'],
          reason:
            'The native-host canary creates the generated synchronous input state machine; pointer-lock task helpers remain on the automatic frontier.',
        },
      },
      dependencies: {
        '@flighthq/signals': { crate: 'flighthq-host-signals' },
        '@flighthq/types': { crate: 'flighthq-types' },
      },
      package: '@flighthq/input',
      sourceExclusions: [],
      sourceSelection: {
        sources: ['inputManager.ts'],
        reason:
          'The partial target preserves synchronous input-manager construction without admitting DOM listeners or unresolved pointer-lock tasks.',
      },
      typeMappings: {},
    },
    {
      crate: 'flighthq-power',
      declarationSelection: {
        'power.ts': {
          names: ['_backend', 'setPowerBackend'],
          reason: 'The native-host canary needs only the synchronous power-backend installation seam.',
        },
      },
      dependencies: {
        '@flighthq/types': { crate: 'flighthq-types' },
      },
      package: '@flighthq/power',
      sourceExclusions: [],
      sourceSelection: {
        sources: ['power.ts'],
        reason:
          'The partial target preserves synchronous native power-backend installation without admitting web Promise composition.',
      },
      typeMappings: {},
    },
    {
      conformanceTemplate: 'tools/generator/templates/easing_conformance.rs',
      crate: 'flighthq-easing',
      dependencies: {
        '@flighthq/types': { crate: 'flighthq-types' },
      },
      fullyPromoted: true,
      package: '@flighthq/easing',
      sourceExclusions: [
        {
          source: 'enableEasingGuards.ts',
          reason:
            'Upstream deliberately isolates this opt-in development guard so its @flighthq/log dependency stays outside the core easing graph; the generated core crate preserves the always-loaded portable surface.',
        },
      ],
      typeMappings: {},
    },
    {
      conformanceTemplate: 'tools/generator/templates/image_conformance.rs',
      crate: 'flighthq-image',
      declarationSelection: {
        'imageResource.ts': {
          names: ['isImageResourceEmpty'],
          reason:
            'Host-element construction, cloning, resizing, and invalidation remain on the TypeScript side; the dimension-only emptiness query is independently portable.',
        },
      },
      dependencies: {
        '@flighthq/types': { crate: 'flighthq-types' },
      },
      package: '@flighthq/image',
      sourceSelection: {
        sources: ['imageResource.ts'],
        reason:
          'Browser element/load/decode operations and host-source invalidation are host-bound and remain in the TypeScript boundary.',
      },
      sourceExclusions: [],
      typeMappings: {},
    },
    {
      crate: 'flighthq-screen',
      declarationSelection: {
        'screen.ts': {
          names: ['_backend', 'setScreenBackend'],
          reason:
            'The cultivated native-host canary needs only the synchronous backend installation seam; the automatic package now exercises portable permission tasks through the canonical task runtime independently.',
        },
      },
      dependencies: {
        '@flighthq/types': { crate: 'flighthq-types' },
      },
      package: '@flighthq/screen',
      sourceExclusions: [],
      sourceSelection: {
        sources: ['screen.ts'],
        reason:
          'The partial target preserves the synchronous native ScreenBackend installation seam used by the host canary; the automatic target separately emits both portable async declarations.',
      },
      typeMappings: {},
    },
    {
      conformanceTemplate: 'tools/generator/templates/surface_conformance.rs',
      crate: 'flighthq-surface',
      declarationSelection: {
        'bitmap.ts': {
          names: ['invalidateBitmap'],
          reason:
            'The selected bitmap kernels retain their source-derived invalidation helper without admitting allocation through the entity package.',
        },
        'bitmapCompare.ts': {
          names: ['getBitmapMismatch'],
          reason:
            'Diff-bitmap construction remains at the TypeScript boundary; the allocation-free mismatch summary is independently portable.',
        },
        'bitmapChannel.ts': {
          names: ['mergeBitmapChannels'],
          reason:
            'Bitmap allocation through the entity package remains at the TypeScript boundary; the region-based merge kernel is independently portable.',
        },
        'bitmapCopy.ts': {
          names: ['copyBitmapPixels'],
          reason:
            'Channel-specific copying joins with the generated ImageChannel type and relative re-export resolution.',
        },
        'bitmapFill.ts': {
          names: ['fillBitmapRectangle'],
          reason:
            'Flood fill joins after reusable module-level scratch buffers lower to generated Rust synchronization primitives.',
        },
        'bitmapFingerprint.ts': {
          names: ['compareBitmapFingerprints', 'createBitmapFingerprint'],
          reason:
            'String slicing and numeric text parsing remain at the TypeScript boundary; typed-array fingerprint construction and comparison are independently portable.',
        },
        'bitmapFormat.ts': {
          names: ['premultiplyBitmapPixels', 'unpremultiplyBitmapPixels'],
          reason:
            'Pixel-order conversion joins after tuple destructuring and typed-array subarray/set methods lower without losing alias safety.',
        },
        'bitmapHistogram.ts': {
          names: ['getBitmapHistogram'],
          reason:
            'Histogram equalization joins after its palette-map dependency enters the generated bitmap slice; the allocation-only histogram query is independently portable.',
        },
        'bitmapPixel.ts': {
          names: [
            'LUMA_B',
            'LUMA_G',
            'LUMA_R',
            'getBitmapPixel',
            'getBitmapPixelLuminance',
            'getBitmapPixelRgb',
            'setBitmapPixel',
            'setBitmapPixelRgb',
          ],
          reason:
            'The ImageChannel-parameterized reader joins when relative TypeScript re-export imports resolve directly to their generated dependency crate.',
        },
      },
      dependencies: {
        '@flighthq/types': { crate: 'flighthq-types' },
      },
      package: '@flighthq/bitmap',
      sourceSelection: {
        sources: [
          'bitmap.ts',
          'bitmapAlpha.ts',
          'bitmapChannel.ts',
          'bitmapColorMatrix.ts',
          'bitmapCompare.ts',
          'bitmapConvolution.ts',
          'bitmapCopy.ts',
          'bitmapCoverage.ts',
          'bitmapFill.ts',
          'bitmapFingerprint.ts',
          'bitmapFormat.ts',
          'bitmapHistogram.ts',
          'bitmapMorphological.ts',
          'bitmapNoise.ts',
          'bitmapPaletteMap.ts',
          'bitmapPixel.ts',
          'bitmapPixelate.ts',
          'bitmapQuery.ts',
          'bitmapTone.ts',
        ],
        reason:
          'Compatibility-named compiled bitmap kernel slice; remaining portable modules are admitted as their required emitter constructs and alias rules compile.',
      },
      sourceExclusions: [],
      typeMappings: {},
    },
  ] satisfies RustTarget[],
} as const;
