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

  // One slot per algorithm, which is why there is no slot taking a `Compression` argument: a host declares
  // exactly what it can do, and an absent slot is a fact a caller can read rather than a runtime failure.
  export interface HostDecompressBrotliCapability {
    decompress: Decompressor;
  }

  export interface HostDecompressDeflateCapability {
    decompress: Decompressor;
  }

  export interface HostDecompressLzmaCapability {
    decompress: Decompressor;
  }

  // Encode takes no `uncompressedLength` — the encoder sees the whole input.
  export interface HostCompressDeflateCapability {
    compress(bytes: Readonly<Uint8Array>, framing: CompressionFraming): Uint8Array;
  }

  export interface HostCompressLzmaCapability {
    compress(bytes: Readonly<Uint8Array>, framing: CompressionFraming): Uint8Array;
  }
}

declare module '@flighthq/compression' {
  import type {
    Decompressor,
    HostCompressDeflateCapability,
    HostCompressLzmaCapability,
    HostDecompressDeflateCapability,
    HostDecompressLzmaCapability,
  } from '@flighthq/types';

  export const decompressDeflate: Decompressor;
  export const sdkHostDecompressDeflate: HostDecompressDeflateCapability;

  export const decompressLzma: Decompressor;
  export const sdkHostDecompressLzma: HostDecompressLzmaCapability;

  export function compressDeflate(bytes: Readonly<Uint8Array>): Uint8Array;
  export function compressDeflateZlib(bytes: Readonly<Uint8Array>): Uint8Array;
  export const sdkHostCompressDeflate: HostCompressDeflateCapability;

  export function compressLzma(bytes: Readonly<Uint8Array>): Uint8Array;
  export const sdkHostCompressLzma: HostCompressLzmaCapability;
}
