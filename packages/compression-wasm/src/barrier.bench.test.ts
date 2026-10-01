import { writeFileSync } from 'node:fs';

import * as reference from '@flighthq/compression';
import { CompressionFraming } from '@flighthq/types';

import { compressDeflate, inflateDeflate, initCompressionWasm } from './compressionWasm';

// Does the wasm barrier pay for THIS package? `bitmap-wasm` measured at about half the speed of the
// TypeScript it replaces (see agents/wasm-barrier.md), so the question is empirical here too and the answer
// is not transferable: these codecs are hand-written rather than generated, and they cross the boundary once
// per buffer rather than once per pixel.
//
// Only the DEFLATE decoder is comparable from inside this package: LZMA and both encoders landed upstream
// after the pin, so there is no pinned counterpart to measure against.

function time(iterations: number, fn: () => void): number {
  fn();
  const start = process.hrtime.bigint();
  for (let i = 0; i < iterations; i++) fn();
  return Number(process.hrtime.bigint() - start) / iterations / 1e6;
}

/** A stored-block DEFLATE stream, so the harness needs no encoder at the pin. */
function stored(bytes: Uint8Array): Uint8Array {
  const blocks: number[] = [];
  let offset = 0;
  do {
    const size = Math.min(65535, bytes.length - offset);
    const final = offset + size >= bytes.length ? 1 : 0;
    blocks.push(final, size & 0xff, (size >> 8) & 0xff, ~size & 0xff, (~size >> 8) & 0xff);
    for (let index = 0; index < size; index++) blocks.push(bytes[offset + index]);
    offset += size;
  } while (offset < bytes.length);
  return new Uint8Array(blocks);
}

/** Highly compressible: long runs, so the decoder spends its time in back-reference copies. */
function runs(length: number): Uint8Array {
  const out = new Uint8Array(length);
  const unit = new TextEncoder().encode('the quick brown fox jumps over the lazy dog 0123456789 ');
  for (let index = 0; index < length; index++) out[index] = unit[index % unit.length];
  return out;
}

it('measures the compression barrier against upstream', { timeout: 300_000 }, () => {
  initCompressionWasm();
  const rows: string[] = [];

  for (const size of [1024, 64 * 1024, 1024 * 1024, 8 * 1024 * 1024]) {
    const payload = runs(size);
    const stream = stored(payload);
    const iterations = size > 1024 * 1024 ? 20 : 500;

    const wasm = time(iterations, () => void inflateDeflate(stream, payload.length, CompressionFraming.Raw));
    const ts = time(iterations, () => void reference.inflateDeflate(stream, payload.length, CompressionFraming.Raw));

    // Agreement first: a faster wrong answer is not a result.
    const a = inflateDeflate(stream, payload.length, CompressionFraming.Raw);
    const b = reference.inflateDeflate(stream, payload.length, CompressionFraming.Raw);
    expect(a).toEqual(b);

    const label = size >= 1024 * 1024 ? `${size / 1024 / 1024} MB` : `${size / 1024} KB`;
    rows.push(
      `inflateDeflate stored ${label.padStart(7)}   wasm ${wasm.toFixed(4).padStart(9)} ms   ` +
        `ts ${ts.toFixed(4).padStart(9)} ms   ${(ts / wasm).toFixed(2).padStart(6)}x`,
    );
  }

  // The path that actually exercises the decoder: a real fixed-Huffman stream with back-references, built
  // with this package's own encoder because the pinned upstream has none. Both decoders read the same bytes.
  for (const size of [64 * 1024, 1024 * 1024, 8 * 1024 * 1024]) {
    const payload = runs(size);
    const stream = compressDeflate(payload);
    const iterations = size > 1024 * 1024 ? 20 : 200;

    const wasm = time(iterations, () => void inflateDeflate(stream, payload.length, CompressionFraming.Raw));
    const ts = time(iterations, () => void reference.inflateDeflate(stream, payload.length, CompressionFraming.Raw));
    expect(inflateDeflate(stream, payload.length, CompressionFraming.Raw)).toEqual(
      reference.inflateDeflate(stream, payload.length, CompressionFraming.Raw),
    );

    const label = size >= 1024 * 1024 ? `${size / 1024 / 1024} MB` : `${size / 1024} KB`;
    const ratio = (payload.length / stream.length).toFixed(0);
    rows.push(
      `inflateDeflate huffman ${label.padStart(6)} (${ratio}x)  wasm ${wasm.toFixed(4).padStart(9)} ms   ` +
        `ts ${ts.toFixed(4).padStart(9)} ms   ${(ts / wasm).toFixed(2).padStart(6)}x`,
    );
  }

  writeFileSync('/tmp/compression-barrier.txt', 'wasm vs TypeScript, >1x means wasm faster\n' + rows.join('\n') + '\n');
  expect(rows.length).toBe(7);
});
