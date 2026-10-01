# Hand-written Rust mirrors

A **mirror** is a hand-written Rust crate under `crates/` that reproduces an upstream package's behaviour exactly, as opposed to a mechanically generated crate under `generated/crates/`. `crates/flighthq-physics-abi-wasm-core` was the first; `crates/flighthq-compression-core` is the second and the one this document describes.

Mirrors are not a loophole in the mechanical-port rule. They exist where the upstream implementation's _shape_ is the thing that makes it slow, so carrying that shape over faithfully — which is what a correct lowering does — produces Rust nobody wants. The DEFLATE decoder is the clearest case in the SDK: `readBit()` returns one bit per method call and `decodeSymbol` walks the code space a bit at a time. A mechanical port of that is a faithful port of the problem.

What a mirror must not relax is behaviour. For every input it must return what upstream returns, including every refusal, and it has to be held to that by upstream's own tests rather than by tests written alongside it.

## `flighthq-compression-core`

Mirrors `upstream/packages/compression/src/deflate.ts` — RFC 1951 (raw DEFLATE) and RFC 1950 (zlib) decoding. 682 lines including tests, no dependencies, `#![forbid(unsafe_code)]`.

```rust
pub enum Framing { Raw, Rfc1950 }
pub fn decompress_deflate(compressed: &[u8], uncompressed_length: usize, framing: Framing) -> Option<Vec<u8>>
```

`None` stands for upstream's `null`, and `uncompressed_length` is the container-declared ceiling, or 0 for none — the same contract as upstream's `Decompressor`, which is already byte-shaped and so needs no translation layer to cross a wasm boundary.

**The expansion cap is mirrored at upstream's value and is load-bearing.** `MAX_INFLATE_BYTES` is 256 MB because the quantity that sizes the allocation is the compression _ratio_, which is not in the file, is not bounded by its length, and no per-field check can reach. Upstream's comment is explicit that a kilobyte of nested maximum-length back-references expands to gigabytes and takes the process with it. This is reachable from any untrusted `.swf` or `.awd`, so the cap is security behaviour and not a tuning constant.

### Conformance

**22 of 22 of upstream's own `deflate.test.ts` cases pass**, and the crate's suite is a direct mirror of that file — the same cases in the same order, including every refusal: truncation, invalid block types, undersized and illegal zlib wrappers (forbidden method, oversized window, bad check bits, preset dictionary), a corrupt or missing Adler-32 trailer, reserved fixed length symbols, a back-reference before the start of output, reserved dynamic distance symbols, dynamic code-length repeats that overrun the declared table for all three repeat symbols, a previous-length repeat with no previous length, and forbidden dynamic literal counts.

**The fixtures are shared, not reimplemented.** The base64 streams are extracted verbatim from upstream's `FIXTURES`. The bit-level malformed streams were produced by _running_ upstream's own `createFixedBackReferenceStream`, `createDynamicRepeatOverflowStream`, `createValidDynamicRepeatStream`, `createReservedDynamicDistanceStream` and `createRawStoredStreamStartingWithZlibHeader` helpers and recording the bytes they emit. That choice matters: a bit packer ported into Rust could drift from the original and still agree with a decoder ported the same way, and the two errors would cancel. Sharing the fixture makes that impossible.

Regenerate them by lifting the helper block out of upstream's test file, calling each builder, and printing hex. When the pin moves, re-extract rather than hand-editing — a hand-split base64 constant silently dropped four characters the first time, and the only symptom was two failing round-trips.

### Measured throughput

Decode throughput on zlib streams produced by node, same payload shapes throughout, output verified identical every run. TypeScript figures are upstream's decoder; native zlib is node's `inflateSync` as a C reference point.

| Payload          | ratio | upstream TS | this mirror | node zlib (C) | mirror vs TS |
| ---------------- | ----: | ----------: | ----------: | ------------: | -----------: |
| repetitive 64 KB |  242x |     49 MB/s |    485 MB/s |      193 MB/s |         9.9x |
| repetitive 1 MB  |  335x |     66 MB/s |    473 MB/s |      320 MB/s |         7.2x |
| repetitive 8 MB  |  343x |     72 MB/s |    474 MB/s |      619 MB/s |         6.6x |
| text 64 KB       |  4.4x |     44 MB/s |    201 MB/s |      470 MB/s |         4.6x |
| text 1 MB        |  4.4x |     44 MB/s |    207 MB/s |      454 MB/s |         4.7x |
| text 8 MB        |  4.5x |     69 MB/s |    197 MB/s |      437 MB/s |         2.9x |

**2.9x to 9.9x over the TypeScript**, and faster than node's zlib on the repetitive shapes at 64 KB and 1 MB — mirroring upstream's semantics exactly did not cost the speedup. An 8 MB body falls from 115 ms to 41 ms, which is the difference between a visible import stall and an unnoticed one.

**These are native numbers and wasm will be slower.** This is the ceiling for this implementation, not a prediction for a facade.

On text the mirror is roughly half zlib's speed, and the reason is nameable rather than mysterious: `decode_symbol` walks the canonical code space one bit at a time, where zlib decodes several bits through a lookup table. A multi-bit table is the obvious next optimisation and it is available without touching observable behaviour, since the symbol decoded is identical either way.

Reproduce with `cargo run --release -p flighthq-compression-core --example throughput -- <iterations> <file>…`, where each file is `<declared-length>:<zlib bytes>`. Generate those with `zlib.deflateSync` over the payload shapes above.

### What this is not, yet

- **No wasm facade.** There is no `packages/compression-wasm`, no `port.config.ts` entry, and no generated binding crate. The facade needs a `wasmFacades` entry, a Rust template, and a TS package, following `physics2d-abi-wasm`.
- **The LZMA encoder is not mirrored.** `compressDeflate`/`compressDeflateZlib` are (see below); upstream's `compressLzma` is not yet.
- **No Brotli**, and that is upstream's position too rather than a gap here: the decoder needs a large static dictionary that is data rather than rules, so upstream declares the slot and expects a caller to fill it.

## `flighthq-compression-core::compress`

Mirrors `upstream/packages/compression/src/compress.ts` at upstream `85d85a3b1`, with the shared code tables from `deflateFormat.ts`:

```rust
pub fn compress_deflate(bytes: &[u8]) -> Vec<u8>        // bare RFC 1951
pub fn compress_deflate_zlib(bytes: &[u8]) -> Vec<u8>   // RFC 1950 wrapper + Adler-32
```

One fixed-Huffman block over the whole input with greedy LZ77 through a hash chain, falling back to stored blocks when the Huffman encoding would not be smaller. Like `lzma`, this mirrors a file that is **not in the pinned submodule**.

### The oracle is byte identity, not round-tripping

Upstream documents its output as a pure function of its input — no timestamps, no heuristics that vary — so the mirror does not merely have to produce something that decodes back, it has to produce **the same bytes**. That makes every search bound part of the contract: the 32-candidate chain limit, the 32768-byte window, the insertion of positions interior to a match, and the exact hash function. Change any one and the output still round-trips perfectly and is still wrong.

That distinction is load-bearing, and it is measured rather than assumed. Three mutations, each caught:

| Mutation                                   | Still round-trips? | Byte identity |
| ------------------------------------------ | ------------------ | ------------- |
| chain depth 32 → 16                        | yes                | **fails**     |
| drop the interior-position chain insertion | yes                | **fails**     |
| `write_code` packs LSB-first               | no                 | **fails**     |

The first two would have passed a round-trip-only suite. Upstream's own comment calls the interior insertion "deliberately not pinned by a test: an assertion tight enough to detect it would pin a heuristic, not a behaviour" — which is true of a round-trip assertion and not of this one, because for a mirror the heuristic _is_ the behaviour.

Twelve corpus cases carry upstream's output length and Adler-32, which pins the bytes in eight characters rather than kilobytes of fixtures. They are chosen for branches, not variety: `empty` and `one-byte` for the degenerate paths, `all-byte-values` for every literal code width, `incompressible-4096` for the stored-block fallback, `incompressible-70000` for _multiple_ stored blocks, `single-byte-run-600` for runs past the 258-byte maximum match, `cycle-251-x40000` for distances deep into the window. The corpus generator is transcribed on both sides rather than shared, so neither implementation can drift into agreement through a shared helper.

### Measured throughput

| Payload    | upstream TS | this mirror | gain | output                   |
| ---------- | ----------: | ----------: | ---: | ------------------------ |
| 64 KB text |     10 MB/s |     38 MB/s | 3.8x | 20157 bytes, identical   |
| 1 MB text  |     10 MB/s |     36 MB/s | 3.6x | 313753 bytes, identical  |
| 8 MB text  |     10 MB/s |     33 MB/s | 3.3x | 2506380 bytes, identical |

**3.3x to 3.8x**, and the identical output sizes at 1 MB and 8 MB are independent confirmation of byte identity beyond what the corpus covers. A narrower spread than either decoder, which is expected: encoding time is dominated by the hash-chain match search rather than by per-bit work, so there is less constant-factor overhead for Rust to remove.

Reproduce with `--example throughput -- <iterations> deflate-encode <plaintext files>`.

## `flighthq-compression-core::lzma`

Mirrors `upstream/packages/compression/src/lzma.ts` at upstream `85d85a3b1` — LZMA1, alone-format header, through the same `Decompressor` contract:

```rust
pub fn decompress_lzma(compressed: &[u8], uncompressed_length: usize, framing: Framing) -> Option<Vec<u8>>
```

**This mirrors a file that is not in the pinned submodule.** LZMA landed upstream after the current pin, so what this reproduces is the behaviour that arrives when the pin moves. That is why the fixtures are embedded: nothing here reads a tree the repository does not check out.

Only `Framing::Raw` is accepted, because LZMA carries no wrapper — zlib framing is a different format, not a stricter request.

### Conformance

**16 of 16 of upstream's `lzma.test.ts` decoder cases pass**, plus one pinned regression described below. Upstream's two `sdkHostDecompressLzma` cases are not mirrored: they assert the Host capability slot carries the portable decoder, which is a TypeScript wiring fact with no counterpart in a crate.

The fixtures are upstream's, extracted mechanically. Their provenance is stronger than the deflate set: upstream generated them with **Python 3.12's `lzma` module** in `FORMAT_ALONE`, so the oracle is a third-party encoder rather than either implementation of the decoder. Both upstream and this mirror are measured against bytes neither produced.

One bug this caught, worth recording because it is the kind a hand-written mirror invites. Upstream computes the middle-slot distance model base as `dist - distSlot - 1`, which is legitimately **-1** when `distSlot` is 4, and JavaScript survives it because the bit-tree index starts at 1 so `offset + m` lands on element 0. Written as an unsigned subtraction it becomes a refusal, and every stream whose first distance uses slot 4 — most of them — stops decoding. Upstream's own LOREM fixture caught it. The mirror now takes the base as `dist - distSlot` and indexes `base + m - 1`, which is the same element with no negative intermediate.

### An upstream defect this mirror reproduces, and its cause

`decompressLzma` refuses valid LZMA streams. Measured at upstream `85d85a3b1`, against streams Python's own `lzma` module round-trips:

| Content           |         Size | Python  | upstream `decompressLzma` | this mirror |
| ----------------- | -----------: | ------- | ------------------------- | ----------- |
| low-entropy text  |       2048 B | decodes | decodes                   | decodes     |
| low-entropy text  |       2304 B | decodes | **null**                  | **None**    |
| low-entropy text  | 64 KB – 8 MB | decodes | **null**                  | **None**    |
| highly repetitive | 64 KB – 8 MB | decodes | decodes                   | decodes     |

**The cause is one misplaced line in the rep-distance shuffle.** Upstream runs `rep2 = rep1` for all three sub-cases of a rep match, including the one where the distance came from `rep1`. The LZMA reference decoder shifts `rep2` only when the distance came from `rep2` or `rep3`:

```cpp
if (DecodeBit(&IsRepG1[state]) == 0) dist = rep1;
else {
  if (DecodeBit(&IsRepG2[state]) == 0) dist = rep2;
  else { dist = rep3; rep3 = rep2; }
  rep2 = rep1;            // inside the else
}
rep1 = rep0; rep0 = dist;
```

So after the first `rep1` match, `rep2` holds a duplicate of `rep1`; the next `rep2` or `rep3` match resolves a wrong distance, output diverges silently, and the decode then dies on an impossible distance or a size mismatch — surfacing as `null` rather than as wrong bytes.

Traced on the 2304-byte reproducer: decoding is byte-correct through output index 2238, and the first wrong byte is at 2239, written by a rep match that resolved distance 389 from the corrupted `rep2`.

**Verified outside this repository, by applying the reordering to a copy:**

- the 2304-byte reproducer decodes byte-for-byte;
- 64 KB, 1 MB and 8 MB text payloads decode byte-for-byte;
- 19 of 19 Python encoder variants decode (`nice_len` 14–23, `mode_fast`, `depth`, `dict_size`, `pb`, `lc`/`lp`);
- all 38 other tests in this crate still pass, including every one of upstream's 16 LZMA fixtures and 22 deflate cases;
- the only test that changes is `refuses_the_same_valid_stream_upstream_refuses`.

Why upstream's own tests do not catch it. **All eight of its fixtures are high-ratio**, and so is its own encoder's output — `compressLzma` round-trips through `decompressLzma` at every size I tried, because the sequence that triggers the bug is a `rep1` match followed later by a `rep2` or `rep3` match, which needs mixed content. The largest low-ratio fixture is 1212 bytes of a repeated sentence; the big ones are 64 KB of `i % 7`, 1 KB of `i % 256`, and `'abcABC123'` repeated 600 times. Nothing in the suite is ordinary mixed content above 2 KB. That matters beyond this crate, because `.awd` and `.swf` bodies are the real inputs and they are not `i % 7`.

**Do not fix the mirror ahead of upstream.** A mirror that decodes more than upstream is no longer a drop-in, and the differential oracle stops holding. When `refuses_the_same_valid_stream_upstream_refuses` starts failing, upstream has fixed its decoder: apply the same reordering and delete the case.

### Measured throughput

Same method as deflate. Only payloads upstream can decode are comparable, so the low-ratio column stops at 2 KB.

| Payload          | ratio | upstream TS | this mirror | mirror vs TS |
| ---------------- | ----: | ----------: | ----------: | -----------: |
| repetitive 64 KB |  437x |     48 MB/s |    839 MB/s |        17.5x |
| repetitive 1 MB  | 3603x |    413 MB/s |    754 MB/s |         1.8x |
| repetitive 8 MB  | 6322x |    412 MB/s |    767 MB/s |         1.9x |
| text 2 KB        |  3.0x |      7 MB/s |    107 MB/s |        15.3x |

**1.8x to 17.5x**, and the spread is the interesting part. On a large high-ratio stream both implementations spend their time in the same byte-at-a-time match copy, so the gap narrows to under 2x. On small or low-ratio input the range coder dominates — one `decode_bit` per output bit, each with a probability update — and that is where Rust wins by an order of magnitude. LZMA's compute is in the range coder, so the realistic inputs favour the mirror more than the headline repetitive numbers suggest.

Native figures again; wasm will be slower.

### The seam changes with the pin

At our pin, substitution happens through a runtime registry: `registerDecompressor(Compression.Deflate, …)`, documented as last-write-wins specifically so a host can replace a portable decoder with a native or wasm one.

At `origin/main` that registry is **deleted**. The seam becomes the Host capability group — `@flighthq/types` declares `HostDecompressCapabilities { brotli?, deflate?, lzma? }` and `HostCompressCapabilities { deflate? }`, and `@flighthq/compression` ships `sdkHostDecompressDeflate` and `sdkHostCompressDeflate` as the slots Flight itself fills.

A facade built against the pin would therefore target an API upstream has removed. Unlike `bitmap-wasm`, this one wants the pin move first. The crate in this directory is unaffected either way: it implements the algorithm, and the seam only decides how it is registered.

## Gaps worth knowing

`cargo fmt` and `cargo clippy` are not in any gate — not in `npm run check`, `npm run ci`, or `ci.yml`. Both hand-written crates are nevertheless rustfmt-clean and clippy-clean, and new mirrors should stay that way. The workspace as a whole is not: `cargo clippy --workspace` reports over two thousand warnings, almost all from generated crates, which is why gating it would mean a ratchet rather than a flag.
