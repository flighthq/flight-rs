// What upstream's `lzmaCompress.test.ts` sees in place of `./lzmaCompress.ts` when the conformance lane runs.
//
// As with the deflate encoder lane, `./lzma.ts` is left resolving to upstream so the round trips decode with
// upstream's own LZMA decoder rather than this crate's.
export { compressLzma, sdkHostCompressLzma } from './compressionWasm';
