export interface SourceExclusion {
  reason: string;
  source: string;
}

export interface RustTarget {
  conformanceTemplate?: string;
  crate: string;
  package: string;
  sourceExclusions: SourceExclusion[];
  typeMappings: Record<string, { reason: string; rust: string; source: string }>;
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
      conformanceTemplate: 'tools/generator/templates/easing_conformance.rs',
      crate: 'flighthq-easing',
      package: '@flighthq/easing',
      sourceExclusions: [
        {
          source: 'createEasingSamples.ts',
          reason:
            'Float32Array out-parameter ownership requires the general typed-array borrow rule; emitting a Vec would break alias identity.',
        },
        {
          source: 'easePiecewise.ts',
          reason:
            'Readonly structural-array ownership and reduce lowering must be generalized before emitting the segment table.',
        },
      ],
      typeMappings: {
        EasingFunction: {
          source: 'upstream/packages/types/src/EasingFunction.ts',
          rust: "std::sync::Arc<dyn Fn(f64) -> f64 + Send + Sync + 'static>",
          reason: 'A shared immutable callable preserves TypeScript function-value semantics across returned closures.',
        },
        ScalarRemap: {
          source: 'upstream/packages/types/src/ScalarRemap.ts',
          rust: "std::sync::Arc<dyn Fn(f64) -> f64 + Send + Sync + 'static>",
          reason: 'ScalarRemap has the same callable representation as EasingFunction.',
        },
        StepPosition: {
          source: 'upstream/packages/types/src/StepPosition.ts',
          rust: "&'static str",
          reason: 'The initial target preserves the closed string vocabulary while enum lowering is generalized.',
        },
      },
    },
  ] satisfies RustTarget[],
} as const;
