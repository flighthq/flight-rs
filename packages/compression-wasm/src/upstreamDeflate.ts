// What upstream's `deflate.test.ts` sees in place of `./deflate` when the conformance lane runs.
//
// Upstream's suite imports exactly two names from that module — `inflateDeflate` and
// `registerDeflateDecompressor` — and asserts, among other things, that the registry hands back the SAME
// function reference it registered. So the registrar has to register this decoder rather than upstream's,
// and it has to register the identical binding the suite imported.
//
// The registry itself is upstream's, imported here rather than reimplemented: substituting the decoder is
// the point, and substituting anything else would weaken the test.

import { registerDecompressor } from '@flighthq/compression';
import { Compression } from '@flighthq/types';

export { inflateDeflate } from './compressionWasm';
import { inflateDeflate } from './compressionWasm';

export function registerDeflateDecompressor(): void {
  registerDecompressor(Compression.Deflate, inflateDeflate);
}
