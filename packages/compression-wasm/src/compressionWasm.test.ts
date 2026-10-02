import * as reference from '@flighthq/compression';
import { CompressionFraming } from '@flighthq/types';

import {
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

// Upstream's own four suites are the conformance evidence and run in `vitest.config.upstream.ts`. These are
// the facade's own concerns, which those suites cannot see: the wasm boundary itself, and the Host slots this
// package exists to fill.

function deflated(bytes: Uint8Array): Uint8Array {
  // Decoded by us from a stream neither implementation produced — the differential direction that matters for
  // a drop-in. Built by hand rather than with upstream's encoder on purpose: a stored block is the one DEFLATE
  // shape no encoder here emits, so it exercises a path round-tripping our own output never reaches. BFINAL=1,
  // BTYPE=00, then a byte-aligned LEN/NLEN pair and the bytes verbatim.
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
  it('initializes synchronously, so a decode needs no await anywhere', () => {
    // Upstream's decoder is synchronous and every parser resolving through the registry depends on that.
    // A facade that needed instantiation to be awaited would not be a drop-in, whatever its output.
    expect(() => initCompressionWasm()).not.toThrow();
    expect(decompressDeflate(deflated(new Uint8Array([1, 2, 3])), 3, CompressionFraming.Raw)).toEqual(
      new Uint8Array([1, 2, 3]),
    );
  });

  it('agrees with the upstream decoder byte for byte, including on refusals', () => {
    const payload = new Uint8Array(512);
    for (let index = 0; index < payload.length; index++) payload[index] = (index * 7) % 251;
    const stream = deflated(payload);

    for (const declared of [0, payload.length]) {
      expect(decompressDeflate(stream, declared, CompressionFraming.Raw)).toEqual(
        reference.decompressDeflate(stream, declared, CompressionFraming.Raw),
      );
    }
    // A bound one byte short, a truncated stream, and a raw stream claimed as zlib: upstream returns null
    // for each, and a decoder that returned bytes for any of them would not be substitutable.
    for (const [input, declared, framing] of [
      [stream, payload.length - 1, CompressionFraming.Raw],
      [stream.subarray(0, 6), 0, CompressionFraming.Raw],
      [stream, 0, CompressionFraming.Rfc1950],
    ] as const) {
      expect(decompressDeflate(input, declared, framing)).toBeNull();
      expect(reference.decompressDeflate(input, declared, framing)).toBeNull();
    }
  });

  it('refuses a framing it does not know rather than guessing one', () => {
    // The framing is the container's fact to supply, because a raw stream can open with bytes that form a
    // valid zlib header. Guessing would decode one file correctly and the next one wrongly.
    expect(decompressDeflate(deflated(new Uint8Array([9])), 1, 'Unknown' as never)).toBeNull();
  });

  it('fills the deflate decompress slot upstream declares for exactly this', () => {
    // Upstream replaced its last-write-wins registry with one named Host slot per algorithm, and says a host
    // "with a native or wasm codec supplies its own slot instead". The slot holds the function itself rather
    // than a wrapper, so there is no second code path that could drift from the exported decoder.
    expect(sdkHostDecompressDeflate.decompress).toBe(decompressDeflate);

    const stream = deflated(new Uint8Array([4, 5, 6]));
    expect(sdkHostDecompressDeflate.decompress(stream, 3, CompressionFraming.Raw)).toEqual(new Uint8Array([4, 5, 6]));
  });
});

// The LZMA streams below are verbatim from upstream's `lzma.test.ts`, generated with Python 3.12's `lzma`
// module, so the oracle is a third-party encoder rather than either implementation. Upstream's own suites now
// cover these codecs too (they run in the conformance lanes); these keep the boundary-level assertions that
// suite has no reason to make, such as the exact alone-format header bytes.

function base64(input: string): Uint8Array {
  const alphabet = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/';
  const out: number[] = [];
  let accumulator = 0;
  let bits = 0;
  for (const character of input.replace(/=+$/u, '')) {
    accumulator = (accumulator << 6) | alphabet.indexOf(character);
    bits += 6;
    if (bits >= 8) {
      bits -= 8;
      out.push((accumulator >> bits) & 0xff);
    }
  }
  return new Uint8Array(out);
}

function adler32(input: Uint8Array): number {
  let first = 1;
  let second = 0;
  for (const byte of input) {
    first = (first + byte) % 65521;
    second = (second + first) % 65521;
  }
  return ((second << 16) | first) >>> 0;
}

const LZMA_LITERAL = 'XQAAgAAWAAAAAAAAAAAzGwlhGvxuQ0djOEOOegx+SF/9GpGR4mriTv//OtQAAA==';
const LZMA_EOS = 'XQAAgAD//////////wAzGwlhGvxuQ0djOEOOegx+SF/9GpGR4mriTv//OtQAAA==';

describe('lzma decoding across the wasm boundary', () => {
  it('decodes an alone-format stream, declared length or not', () => {
    const expected = new TextEncoder().encode('flighthq scene-formats');
    expect(decompressLzma(base64(LZMA_LITERAL), 0, CompressionFraming.Raw)).toEqual(expected);
    expect(decompressLzma(base64(LZMA_LITERAL), expected.length, CompressionFraming.Raw)).toEqual(expected);
  });

  it('decodes an end-of-stream-terminated stream whose header declares no size', () => {
    // The two paths through the size logic: the header declares a length, or it declares 0xFF…FF and the
    // decode has to run to the end marker instead.
    expect(decompressLzma(base64(LZMA_EOS), 0, CompressionFraming.Raw)).toEqual(
      new TextEncoder().encode('flighthq scene-formats'),
    );
  });

  it('refuses a bound one byte short, and refuses zlib framing outright', () => {
    expect(decompressLzma(base64(LZMA_LITERAL), 21, CompressionFraming.Raw)).toBeNull();
    // LZMA carries no wrapper, so zlib framing is a different format rather than a stricter request.
    expect(decompressLzma(base64(LZMA_LITERAL), 0, CompressionFraming.Rfc1950)).toBeNull();
    expect(decompressLzma(base64(LZMA_LITERAL), 0, 'Unknown' as never)).toBeNull();
  });
});

describe('deflate encoding across the wasm boundary', () => {
  const text = new TextEncoder().encode('flighthq scene-formats');

  it('is byte-identical to upstream for a known input', () => {
    // Upstream documents its encoder as a pure function of its input, so byte identity is the contract.
    // These are upstream's own output length and Adler-32 for this input, which pins the bytes compactly.
    const raw = compressDeflate(text);
    expect(raw.length).toBe(24);
    expect(adler32(raw)).toBe(0x8b9009d2);

    const zlib = compressDeflateZlib(text);
    expect(zlib.length).toBe(30);
    expect(adler32(zlib)).toBe(0xc4510bba);
  });

  it("round-trips through this package's own decoder in both framings", () => {
    // The property a consumer depends on: the two halves of this facade agree with each other.
    expect(decompressDeflate(compressDeflate(text), text.length, CompressionFraming.Raw)).toEqual(text);
    expect(decompressDeflate(compressDeflateZlib(text), text.length, CompressionFraming.Rfc1950)).toEqual(text);
  });

  it('wraps the raw stream in a header and an Adler-32 of the UNCOMPRESSED bytes', () => {
    const raw = compressDeflate(text);
    const zlib = compressDeflateZlib(text);
    expect([...zlib.subarray(2, zlib.length - 4)]).toEqual([...raw]);
    const checksum = adler32(text);
    expect([...zlib.subarray(zlib.length - 4)]).toEqual([
      (checksum >>> 24) & 0xff,
      (checksum >>> 16) & 0xff,
      (checksum >>> 8) & 0xff,
      checksum & 0xff,
    ]);
  });

  it('never meaningfully grows incompressible input', () => {
    // The stored-block fallback: a Huffman block over random bytes is larger than the bytes.
    let state = 7;
    const noise = new Uint8Array(4096).map(() => {
      state = (Math.imul(state, 1103515245) + 12345) & 0x7fffffff;
      return (state >>> 16) & 0xff;
    });
    expect(compressDeflate(noise).length).toBeLessThanOrEqual(noise.length + 5);
    expect(decompressDeflate(compressDeflate(noise), noise.length, CompressionFraming.Raw)).toEqual(noise);
  });
});

describe('every Host slot this package backs', () => {
  // Four slots, and the facade must fill all four or a host wiring it up gets a silent mix of wasm and
  // TypeScript codecs. `index.ts` re-exports upstream wholesale, so an unfilled slot does not fail to
  // resolve — it quietly resolves to upstream's own implementation. These assert each one is ours.
  it('fills both decompress slots with the wasm decoders', () => {
    expect(sdkHostDecompressDeflate.decompress).toBe(decompressDeflate);
    expect(sdkHostDecompressLzma.decompress).toBe(decompressLzma);
    expect(sdkHostDecompressLzma.decompress(base64(LZMA_LITERAL), 0, CompressionFraming.Raw)).toEqual(
      new TextEncoder().encode('flighthq scene-formats'),
    );
  });

  it('dispatches the deflate compress slot on framing, as upstream does', () => {
    const bytes = new TextEncoder().encode('framing dispatch');
    expect(sdkHostCompressDeflate.compress(bytes, CompressionFraming.Raw)).toEqual(compressDeflate(bytes));
    expect(sdkHostCompressDeflate.compress(bytes, CompressionFraming.Rfc1950)).toEqual(compressDeflateZlib(bytes));
  });

  it('refuses a framing LZMA has no form for, rather than returning the wrong bytes', () => {
    const bytes = new Uint8Array([1, 2, 3]);
    expect(sdkHostCompressLzma.compress(bytes, CompressionFraming.Raw)).toEqual(compressLzma(bytes));
    expect(() => sdkHostCompressLzma.compress(bytes, CompressionFraming.Rfc1950)).toThrow(
      'lzma: only Raw framing is supported',
    );
  });
});

describe('lzma encoding across the wasm boundary', () => {
  it('is byte-identical to upstream for a known input', () => {
    // Upstream's length and Adler-32 for this input; pins the bytes without embedding them.
    const out = compressLzma(new TextEncoder().encode('flighthq scene-formats'));
    expect(out.length).toBe(40);
    expect(adler32(out)).toBe(0x8cf00b9c);
  });

  it('writes the alone-format header, declaring the size rather than the unknown marker', () => {
    // Properties byte 93 is lc=3/lp=0/pb=2. Declaring the real size means a decoder knows where to stop
    // instead of depending on the end-of-stream marker.
    const out = compressLzma(new TextEncoder().encode('flighthq'));
    expect(out[0]).toBe(93);
    const declared = out[5] | (out[6] << 8) | (out[7] << 16) | (out[8] << 24);
    expect(declared).toBe(8);
    expect([...out.subarray(9, 13)]).toEqual([0, 0, 0, 0]);
  });

  it("round-trips through this package's own LZMA decoder", () => {
    for (const text of ['', 'a', 'flighthq scene-formats', 'abcABC123'.repeat(400)]) {
      const bytes = new TextEncoder().encode(text);
      expect(decompressLzma(compressLzma(bytes), bytes.length, CompressionFraming.Raw)).toEqual(bytes);
      expect(decompressLzma(compressLzma(bytes), 0, CompressionFraming.Raw)).toEqual(bytes);
    }
  });
});
