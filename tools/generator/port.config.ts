export interface SourceExclusion {
  reason: string;
  source: string;
}

export interface RustTarget {
  conformanceTemplate?: string;
  crate: string;
  declarationSelection?: Record<string, { names: string[]; reason: string }>;
  dependencies: Record<string, { crate: string }>;
  package: string;
  sourceSelection?: {
    reason: string;
    sources: string[];
  };
  sourceExclusions: SourceExclusion[];
  typeMappings: Record<string, { reason: string; rust: string; rustDefinition?: string; source: string }>;
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
  ] satisfies RustTarget[],
} as const;
