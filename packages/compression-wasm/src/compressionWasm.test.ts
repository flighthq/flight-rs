import { getDecompressor, unregisterDecompressor } from '@flighthq/compression';
import * as reference from '@flighthq/compression';
import { Compression, CompressionFraming } from '@flighthq/types';

import { inflateDeflate, initCompressionWasm, registerDeflateDecompressorWasm } from './compressionWasm';

// Upstream's own suite is the conformance evidence and runs in `vitest.config.upstream.ts`. These are the
// facade's own concerns, which that suite cannot see: the wasm boundary, and the registry seam this package
// exists to fill.

function deflated(bytes: Uint8Array): Uint8Array {
  // Encoded by upstream, decoded by us — the differential direction that matters for a drop-in. Upstream's
  // pinned package has no encoder, so this builds a stored-block stream by hand: BFINAL=1, BTYPE=00, then a
  // byte-aligned LEN/NLEN pair and the bytes verbatim.
  const out = new Uint8Array(5 + bytes.length);
  out[0] = 0x01;
  out[1] = bytes.length & 0xff;
  out[2] = (bytes.length >> 8) & 0xff;
  out[3] = ~bytes.length & 0xff;
  out[4] = (~bytes.length >> 8) & 0xff;
  out.set(bytes, 5);
  return out;
}

describe('compression wasm facade', () => {
  afterEach(() => unregisterDecompressor(Compression.Deflate));

  it('initializes synchronously, so a decode needs no await anywhere', () => {
    // Upstream's decoder is synchronous and every parser resolving through the registry depends on that.
    // A facade that needed instantiation to be awaited would not be a drop-in, whatever its output.
    expect(() => initCompressionWasm()).not.toThrow();
    expect(inflateDeflate(deflated(new Uint8Array([1, 2, 3])), 3, CompressionFraming.Raw)).toEqual(
      new Uint8Array([1, 2, 3]),
    );
  });

  it('agrees with the upstream decoder byte for byte, including on refusals', () => {
    const payload = new Uint8Array(512);
    for (let index = 0; index < payload.length; index++) payload[index] = (index * 7) % 251;
    const stream = deflated(payload);

    for (const declared of [0, payload.length]) {
      expect(inflateDeflate(stream, declared, CompressionFraming.Raw)).toEqual(
        reference.inflateDeflate(stream, declared, CompressionFraming.Raw),
      );
    }
    // A bound one byte short, a truncated stream, and a raw stream claimed as zlib: upstream returns null
    // for each, and a decoder that returned bytes for any of them would not be substitutable.
    for (const [input, declared, framing] of [
      [stream, payload.length - 1, CompressionFraming.Raw],
      [stream.subarray(0, 6), 0, CompressionFraming.Raw],
      [stream, 0, CompressionFraming.Rfc1950],
    ] as const) {
      expect(inflateDeflate(input, declared, framing)).toBeNull();
      expect(reference.inflateDeflate(input, declared, framing)).toBeNull();
    }
  });

  it('refuses a framing it does not know rather than guessing one', () => {
    // The framing is the container's fact to supply, because a raw stream can open with bytes that form a
    // valid zlib header. Guessing would decode one file correctly and the next one wrongly.
    expect(inflateDeflate(deflated(new Uint8Array([9])), 1, 'Unknown' as never)).toBeNull();
  });

  it('fills the registry slot upstream declares for exactly this', () => {
    // `registerDecompressor` is documented upstream as last-write-wins so a host can replace the portable
    // decoder with a native or wasm one. This asserts the seam works and that registration is explicit —
    // importing the facade must register nothing, or a build that never decompresses pays for a codec.
    expect(getDecompressor(Compression.Deflate)).toBeNull();

    registerDeflateDecompressorWasm((compression, decompress) =>
      reference.registerDecompressor(compression as Compression, decompress),
    );

    expect(getDecompressor(Compression.Deflate)).toBe(inflateDeflate);
    const stream = deflated(new Uint8Array([4, 5, 6]));
    expect(getDecompressor(Compression.Deflate)?.(stream, 3, CompressionFraming.Raw)).toEqual(
      new Uint8Array([4, 5, 6]),
    );
  });
});
