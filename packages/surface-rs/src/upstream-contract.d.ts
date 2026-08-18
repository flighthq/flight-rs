// Compile-time boundary for the filtered repository. Published consumers
// resolve these modules from the real @flighthq dependencies declared by this
// package; this local declaration keeps the blessed facade independently
// buildable without compiling the complete upstream monorepo.

declare module '@flighthq/types' {
  export interface Bitmap {
    readonly data: Uint8ClampedArray<ArrayBuffer>;
    height: number;
    version: number;
    width: number;
  }

  export interface RectangleLike {
    height: number;
    width: number;
    x: number;
    y: number;
  }

  export interface BitmapFingerprint {
    readonly cells: Uint8Array;
    readonly gridSize: number;
  }

  export interface BitmapRegion {
    height: number;
    bitmap: Bitmap;
    width: number;
    x: number;
    y: number;
  }

  export interface BitmapHistogram {
    alpha: number[];
    blue: number[];
    green: number[];
    red: number[];
  }

  export interface BitmapMismatch {
    fraction: number;
    maxChannelDelta: number;
    mismatchedPixels: number;
    totalPixels: number;
  }
}

declare module '@flighthq/bitmap' {
  import type { Bitmap } from '@flighthq/types';

  export function invalidateBitmap(bitmap: Bitmap): void;
  export interface BitmapConvolutionOptions {
    bias?: number;
    edge?: 'clamp' | 'mirror' | 'transparent' | 'wrap';
    divisor?: number;
    matrix: ReadonlyArray<number>;
    matrixX: number;
    matrixY: number;
    preserveAlpha?: boolean;
  }
}
