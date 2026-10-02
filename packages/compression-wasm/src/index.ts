// The wasm-backed names shadow the `export *` below, which is what makes this a substitute for the upstream
// package rather than an addition to it: a caller importing `decompressDeflate` from here gets the Rust
// decoder, while anything upstream adds that this facade does not implement still flows through unchanged.
export {
  compressDeflate,
  compressDeflateZlib,
  compressLzma,
  decompressDeflate,
  decompressLzma,
  initCompressionWasm,
  sdkHostCompressDeflate,
  sdkHostCompressLzma,
  sdkHostDecompressDeflate,
  sdkHostDecompressLzma,
} from './compressionWasm';
export * from '@flighthq/compression';
