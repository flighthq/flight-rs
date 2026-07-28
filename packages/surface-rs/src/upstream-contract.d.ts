// Compile-time boundary for the filtered repository. Published consumers
// resolve these modules from the real @flighthq dependencies declared by this
// package; this local declaration keeps the blessed facade independently
// buildable without compiling the complete upstream monorepo.

declare module '@flighthq/types' {
  export interface Surface {
    readonly data: Uint8ClampedArray<ArrayBuffer>;
    height: number;
    version: number;
    width: number;
  }

  export interface SurfaceRegion {
    height: number;
    surface: Surface;
    width: number;
    x: number;
    y: number;
  }
}

declare module '@flighthq/image' {
  import type { Surface } from '@flighthq/types';

  export function invalidateImageResource(resource: Surface): void;
}

declare module '@flighthq/surface' {
  export interface SurfaceConvolutionOptions {
    bias?: number;
    edge?: 'clamp' | 'mirror' | 'transparent' | 'wrap';
    divisor?: number;
    matrix: ReadonlyArray<number>;
    matrixX: number;
    matrixY: number;
    preserveAlpha?: boolean;
  }
}
