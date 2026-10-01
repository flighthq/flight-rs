import type { CompressionFraming, Decompressor } from '@flighthq/types';

import { decompress_deflate, initSync } from './wasm/compression_wasm.js';
import { compressionWasmBytes } from './wasm/compressionWasmBytes';

// The Rust-backed half of the upstream compression registry. Upstream's own comment on
// `registerDecompressor` says it is last-write-wins specifically so "a host can replace a portable decoder
// with a native or wasm one", which is this: the seam is declared upstream rather than invented here.
//
// Nothing is marshalled across the boundary beyond the framing enum. `Decompressor` is already
// byte-shaped — compressed bytes in, raw bytes out, one declared length, one framing — so the wasm ABI is
// the contract with the enum crossing as its wire code.

const FRAMING_CODES: Record<string, number> = { Raw: 0, Rfc1950: 1 };

let initialized = false;

/**
 * Eagerly instantiates the module. `inflateDeflate` initializes lazily too, so calling this is optional;
 * it exists so a caller that wants the one-time cost paid up front can choose when.
 */
export function initCompressionWasm(): void {
  if (initialized) return;
  initSync({ module: compressionWasmBytes });
  initialized = true;
}

/**
 * Drop-in replacement for upstream's `inflateDeflate`.
 *
 * Returns `null` exactly where upstream does — a malformed stream, a failed zlib header or Adler-32 check,
 * an output that would exceed the declared bound, or a framing this decoder does not know. An unknown
 * framing is refused rather than guessed because a raw stream can legitimately begin with bytes that look
 * like a zlib header, so only the container knows which form it carries.
 */
export const inflateDeflate: Decompressor = (compressed, uncompressedLength, framing) => {
  initCompressionWasm();
  const code = FRAMING_CODES[framing as string];
  if (code === undefined) return null;
  return decompress_deflate(compressed as Uint8Array, uncompressedLength, code) ?? null;
};

/**
 * Registers the wasm decoder for `Compression.Deflate` in upstream's registry.
 *
 * Deliberately a separate call rather than an import side effect, matching upstream's
 * `registerDeflateDecompressor`: importing a codec must register nothing, so a build that never
 * decompresses pays for no codec. Takes the registrar rather than importing it so this module does not
 * pull the registry — and therefore the whole upstream package — into a bundle that only wants the decoder.
 */
export function registerDeflateDecompressorWasm(
  register: (compression: 'deflate', decompress: Decompressor) => void,
): void {
  register('deflate', inflateDeflate);
}

export type { CompressionFraming };
