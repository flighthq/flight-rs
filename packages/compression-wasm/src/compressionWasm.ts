import type {
  Decompressor,
  HostCompressDeflateCapability,
  HostCompressLzmaCapability,
  HostDecompressDeflateCapability,
  HostDecompressLzmaCapability,
} from '@flighthq/types';
import { CompressionFraming } from '@flighthq/types';

import {
  compress_deflate,
  compress_deflate_zlib,
  compress_lzma,
  decompress_deflate,
  decompress_lzma,
  initSync,
} from './wasm/compression_wasm.js';
import { compressionWasmBytes } from './wasm/compressionWasmBytes';

// The Rust-backed half of the upstream compression package. The seam is declared upstream rather than
// invented here: each algorithm is a named Host slot, and upstream's comment on `sdkHostDecompressDeflate`
// says a host "with a native or wasm codec supplies its own slot instead and never bundles this module".
// This supplies those slots.
//
// Nothing is marshalled across the boundary beyond the framing enum. `Decompressor` is already byte-shaped,
// so the wasm ABI is the contract with the enum crossing as its wire code.

const FRAMING_CODES: Record<string, number> = { Raw: 0, Rfc1950: 1 };

let initialized = false;

/**
 * Eagerly instantiates the module. Every exported function initializes lazily too, so calling this is
 * optional; it exists so a caller that wants the one-time cost paid up front can choose when.
 */
export function initCompressionWasm(): void {
  if (initialized) return;
  initSync({ module: compressionWasmBytes });
  initialized = true;
}

/** Shared shape for both decoders: resolve the framing, call wasm, and turn `undefined` back into `null`. */
function decode(
  wasm: (compressed: Uint8Array, uncompressedLength: number, framing: number) => Uint8Array | undefined,
  compressed: Readonly<Uint8Array>,
  uncompressedLength: number,
  framing: CompressionFraming,
): Uint8Array | null {
  initCompressionWasm();
  const code = FRAMING_CODES[framing as string];
  if (code === undefined) return null;
  return wasm(compressed as Uint8Array, uncompressedLength, code) ?? null;
}

/**
 * Drop-in replacement for upstream's `decompressDeflate`.
 *
 * Returns `null` exactly where upstream does — a malformed stream, a failed zlib header or Adler-32 check, an
 * output exceeding the declared bound, or a framing this decoder does not know. An unknown framing is refused
 * rather than guessed, because a raw stream can legitimately begin with bytes that look like a zlib header, so
 * only the container knows which form it carries.
 */
export const decompressDeflate: Decompressor = (compressed, uncompressedLength, framing) =>
  decode(decompress_deflate, compressed, uncompressedLength, framing);

/**
 * LZMA1 alone-format decoding, for the `Compression.Lzma` slot upstream declares.
 *
 * LZMA carries no wrapper, so only `Raw` framing is accepted; zlib framing is a different format rather than a
 * stricter request.
 */
export const decompressLzma: Decompressor = (compressed, uncompressedLength, framing) =>
  decode(decompress_lzma, compressed, uncompressedLength, framing);

/**
 * Encodes to a bare RFC 1951 stream, byte-identical to upstream's `compressDeflate`.
 *
 * Upstream documents its encoder as a pure function of its input, so byte identity is the contract and not
 * merely a nicety: every search bound — the chain depth, the window, the interior-position insertions — is
 * part of it.
 */
export function compressDeflate(bytes: Readonly<Uint8Array>): Uint8Array {
  initCompressionWasm();
  return compress_deflate(bytes as Uint8Array);
}

/** Encodes to RFC 1950: a two-byte header, the same stream, and a big-endian Adler-32 of the input. */
export function compressDeflateZlib(bytes: Readonly<Uint8Array>): Uint8Array {
  initCompressionWasm();
  return compress_deflate_zlib(bytes as Uint8Array);
}

/**
 * Encodes to an LZMA1 alone-format stream, byte-identical to upstream's `compressLzma`.
 *
 * Fixed lc=3/lp=0/pb=2 properties and a declared size in the header, matching upstream — which means a
 * decoder reading it knows the output length up front rather than relying on the end-of-stream marker.
 */
export function compressLzma(bytes: Readonly<Uint8Array>): Uint8Array {
  initCompressionWasm();
  return compress_lzma(bytes as Uint8Array);
}

// The four Host slots, each holding the SAME binding the matching function export does. Upstream's suites
// assert that identity directly (`expect(sdkHostDecompressDeflate.decompress).toBe(decompressDeflate)`), and
// it is the honest shape anyway: the slot is the function, not a wrapper that might drift from it.

/** The deflate decompress slot, wasm-backed. */
export const sdkHostDecompressDeflate: HostDecompressDeflateCapability = { decompress: decompressDeflate };

/** The LZMA decompress slot, wasm-backed. */
export const sdkHostDecompressLzma: HostDecompressLzmaCapability = { decompress: decompressLzma };

/**
 * The deflate compress slot, wasm-backed, dispatching on framing exactly as upstream's does.
 *
 * The framing is a parameter here rather than two slots because upstream models encode that way: one
 * algorithm slot that the caller tells which container form it wants.
 */
export const sdkHostCompressDeflate: HostCompressDeflateCapability = {
  compress(bytes: Readonly<Uint8Array>, framing: CompressionFraming): Uint8Array {
    return framing === CompressionFraming.Rfc1950 ? compressDeflateZlib(bytes) : compressDeflate(bytes);
  },
};

/**
 * The LZMA compress slot, wasm-backed.
 *
 * Throws on any framing but `Raw`, matching upstream including the message: zlib framing is a different
 * format rather than a stricter request, so there is nothing sensible to return.
 */
export const sdkHostCompressLzma: HostCompressLzmaCapability = {
  compress(bytes: Readonly<Uint8Array>, framing: CompressionFraming): Uint8Array {
    if (framing !== CompressionFraming.Raw) throw new Error('lzma: only Raw framing is supported');
    return compressLzma(bytes);
  },
};
