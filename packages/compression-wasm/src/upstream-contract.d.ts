// The slice of upstream's declarations this facade compiles against, hand-written because the published
// `@flighthq/*` packages ship extensionless relative imports that Node's ESM resolver rejects — they are
// bundler-only by convention — so `tsc` cannot read them directly.
//
// This file is the one place a mistake here is invisible to every other check: an ambient
// `declare module` shadows node_modules, so a wrong declaration compiles and only fails for a consumer.
// `tsconfig.upstream.json` exists to catch exactly that by excluding this file and compiling against the
// real package instead. Keep the two in step.

declare module '@flighthq/types' {
  export const CompressionFraming: {
    readonly Raw: 'Raw';
    readonly Rfc1950: 'Rfc1950';
  };
  export type CompressionFraming = (typeof CompressionFraming)[keyof typeof CompressionFraming];

  export const Compression: {
    readonly Brotli: 'brotli';
    readonly Deflate: 'deflate';
    readonly Lzma: 'lzma';
  };
  export type Compression = (typeof Compression)[keyof typeof Compression];

  export type Decompressor = (
    compressed: Readonly<Uint8Array>,
    uncompressedLength: number,
    framing: CompressionFraming,
  ) => Uint8Array | null;
}

declare module '@flighthq/compression' {
  import type { Compression, Decompressor } from '@flighthq/types';

  export const inflateDeflate: Decompressor;
  export function registerDeflateDecompressor(): void;
  export function getDecompressor(compression: Compression): Decompressor | null;
  export function hasDecompressor(compression: Compression): boolean;
  export function registerDecompressor(compression: Compression, decompress: Decompressor): void;
  export function unregisterDecompressor(compression: Compression): void;
}
