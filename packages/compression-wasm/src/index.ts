export {
  compressDeflate,
  compressDeflateZlib,
  compressLzma,
  decompressLzma,
  inflateDeflate,
  initCompressionWasm,
  registerCompressionWasmDecompressors,
  registerDeflateDecompressorWasm,
} from './compressionWasm';
export * from '@flighthq/compression';
