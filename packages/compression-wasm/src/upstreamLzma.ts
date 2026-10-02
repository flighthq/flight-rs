// What upstream's `lzma.test.ts` sees in place of `./lzma.ts` when the conformance lane runs.
//
// This is the strongest of the four lanes: its fixtures are precomputed with Python 3's `lzma` module, so the
// decoder is checked against a third-party encoder rather than against anything in either codebase.
export { decompressLzma, sdkHostDecompressLzma } from './compressionWasm';
