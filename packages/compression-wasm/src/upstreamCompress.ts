// What upstream's `compress.test.ts` sees in place of `./compress.ts` when the conformance lane runs.
//
// That suite also imports `./deflate.ts` to verify its round trips, and the lane deliberately does NOT
// substitute it. Upstream's own comment explains why that decoder was chosen — it "was written independently
// of this encoder and is already pinned by its own tests" — so leaving it upstream's checks the wasm encoder
// against an independent decoder. Substituting both would check this crate against itself, where a single
// shared misreading of the format cancels out and passes.
export { compressDeflate, compressDeflateZlib, sdkHostCompressDeflate } from './compressionWasm';
