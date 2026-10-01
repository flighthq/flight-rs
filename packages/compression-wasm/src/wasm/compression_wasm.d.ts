/* tslint:disable */
/* eslint-disable */

/**
 * Encodes to a bare RFC 1951 stream.
 */
export function compress_deflate(bytes: Uint8Array): Uint8Array;

/**
 * Encodes to RFC 1950: a two-byte header, the same stream, and a big-endian Adler-32 of the input.
 */
export function compress_deflate_zlib(bytes: Uint8Array): Uint8Array;

/**
 * Encodes to an LZMA1 alone-format stream: a 13-byte header then the range-coded body.
 */
export function compress_lzma(bytes: Uint8Array): Uint8Array;

/**
 * Decompresses a DEFLATE stream, or returns `undefined` when it is malformed or the framing is unknown.
 *
 * `framing` is 0 for a raw RFC 1951 stream and 1 for RFC 1950 zlib framing; any other value is refused
 * rather than guessed, because a raw stream can begin with bytes that look like a zlib header.
 */
export function decompress_deflate(compressed: Uint8Array, uncompressed_length: number, framing: number): Uint8Array | undefined;

/**
 * Decompresses an LZMA1 alone-format stream, or returns `undefined` when it is malformed.
 *
 * LZMA carries no wrapper, so only raw framing is accepted; zlib framing is a different format rather than
 * a stricter request, and upstream refuses it too.
 */
export function decompress_lzma(compressed: Uint8Array, uncompressed_length: number, framing: number): Uint8Array | undefined;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly compress_deflate: (a: number, b: number) => [number, number];
    readonly compress_deflate_zlib: (a: number, b: number) => [number, number];
    readonly compress_lzma: (a: number, b: number) => [number, number];
    readonly decompress_deflate: (a: number, b: number, c: number, d: number) => [number, number];
    readonly decompress_lzma: (a: number, b: number, c: number, d: number) => [number, number];
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
