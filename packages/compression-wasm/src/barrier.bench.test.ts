import { writeFileSync } from 'node:fs';

import * as reference from '@flighthq/compression';
import { CompressionFraming } from '@flighthq/types';

import {
  compressDeflate,
  compressDeflateZlib,
  compressLzma,
  decompressDeflate,
  decompressLzma,
  initCompressionWasm,
} from './compressionWasm';

// Does the wasm barrier pay for THIS package? `bitmap-wasm` measured at about half the speed of the
// TypeScript it replaces (see agents/wasm-barrier.md), so the question is empirical here too and the answer
// is not transferable: these codecs are hand-written rather than generated, and they cross the boundary once
// per buffer rather than once per pixel.
//
// Every exported codec is measured, in both directions. That became possible when the pin moved to `develop`:
// upstream now ships the LZMA decoder and both encoders, so each of them has a counterpart to compare against
// rather than only DEFLATE decode. A package claiming "all features supported" should be able to say what each
// of those features costs.

// Five warm-up calls rather than one, and iteration counts high enough that the result does not move when they
// change. Both were needed: at ten iterations the 1 MB zlib encode read 35.5 ms, and at two hundred it reads
// 19.7 ms — the first number was under-warmed JIT, and it would have been published as a 10.31x speedup.
function time(iterations: number, fn: () => void): number {
  for (let warmup = 0; warmup < 5; warmup++) fn();
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

    const wasm = time(iterations, () => void decompressDeflate(stream, payload.length, CompressionFraming.Raw));
    const ts = time(iterations, () => void reference.decompressDeflate(stream, payload.length, CompressionFraming.Raw));

    // Agreement first: a faster wrong answer is not a result.
    const a = decompressDeflate(stream, payload.length, CompressionFraming.Raw);
    const b = reference.decompressDeflate(stream, payload.length, CompressionFraming.Raw);
    expect(a).toEqual(b);

    const label = size >= 1024 * 1024 ? `${size / 1024 / 1024} MB` : `${size / 1024} KB`;
    rows.push(
      `decompressDeflate stored ${label.padStart(7)}   wasm ${wasm.toFixed(4).padStart(9)} ms   ` +
        `ts ${ts.toFixed(4).padStart(9)} ms   ${(ts / wasm).toFixed(2).padStart(6)}x`,
    );
  }

  // The path that actually exercises the decoder: a real fixed-Huffman stream with back-references, built
  // with this package's own encoder because the pinned upstream has none. Both decoders read the same bytes.
  for (const size of [64 * 1024, 1024 * 1024, 8 * 1024 * 1024]) {
    const payload = runs(size);
    const stream = compressDeflate(payload);
    const iterations = size > 1024 * 1024 ? 20 : 200;

    const wasm = time(iterations, () => void decompressDeflate(stream, payload.length, CompressionFraming.Raw));
    const ts = time(iterations, () => void reference.decompressDeflate(stream, payload.length, CompressionFraming.Raw));
    expect(decompressDeflate(stream, payload.length, CompressionFraming.Raw)).toEqual(
      reference.decompressDeflate(stream, payload.length, CompressionFraming.Raw),
    );

    const label = size >= 1024 * 1024 ? `${size / 1024 / 1024} MB` : `${size / 1024} KB`;
    const ratio = (payload.length / stream.length).toFixed(0);
    rows.push(
      `decompressDeflate huffman ${label.padStart(6)} (${ratio}x)  wasm ${wasm.toFixed(4).padStart(9)} ms   ` +
        `ts ${ts.toFixed(4).padStart(9)} ms   ${(ts / wasm).toFixed(2).padStart(6)}x`,
    );
  }

  // The encoders, and the LZMA pair. Byte identity is asserted alongside the timing on purpose: a codec that
  // got faster by producing different output has not got faster at anything that matters, and for a
  // deterministic encoder the comparison costs nothing extra.
  for (const size of [64 * 1024, 1024 * 1024]) {
    const payload = runs(size);
    const label = size >= 1024 * 1024 ? `${size / 1024 / 1024} MB` : `${size / 1024} KB`;
    const iterations = size > 64 * 1024 ? 200 : 500;

    for (const [name, ours, theirs] of [
      ['compressDeflate', compressDeflate, reference.compressDeflate],
      ['compressDeflateZlib', compressDeflateZlib, reference.compressDeflateZlib],
      ['compressLzma', compressLzma, reference.compressLzma],
    ] as const) {
      const wasm = time(iterations, () => void ours(payload));
      const ts = time(iterations, () => void theirs(payload));
      expect(ours(payload), `${name} is byte-identical to upstream`).toEqual(theirs(payload));
      rows.push(
        `${name} ${label}`.padEnd(33) +
          ` wasm ${wasm.toFixed(4).padStart(9)} ms   ts ${ts.toFixed(4).padStart(9)} ms   ` +
          `${(ts / wasm).toFixed(2).padStart(6)}x`,
      );
    }

    const lzma = reference.compressLzma(payload);
    const wasm = time(iterations, () => void decompressLzma(lzma, payload.length, CompressionFraming.Raw));
    const ts = time(iterations, () => void reference.decompressLzma(lzma, payload.length, CompressionFraming.Raw));
    expect(decompressLzma(lzma, payload.length, CompressionFraming.Raw)).toEqual(payload);
    rows.push(
      `decompressLzma ${label}`.padEnd(33) +
        ` wasm ${wasm.toFixed(4).padStart(9)} ms   ts ${ts.toFixed(4).padStart(9)} ms   ` +
        `${(ts / wasm).toFixed(2).padStart(6)}x`,
    );
  }

  writeFileSync('/tmp/compression-barrier.txt', 'wasm vs TypeScript, >1x means wasm faster\n' + rows.join('\n') + '\n');
  // 7 decode rows plus four codecs at each of two sizes.
  expect(rows.length).toBe(15);
});
