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

### Coverage

- **The facade exists**: `packages/compression-wasm`, with upstream's own `deflate.test.ts` running against it (25 of 25) through `vitest.config.upstream.ts`.
- **Nothing is unmirrored now.** Both decoders and both encoders are mirrored, and all five are exposed through `packages/compression-wasm`. Brotli stays out, matching upstream's own deliberate non-goal.
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

### The rep-shuffle defect, found here and fixed upstream

Resolved. Upstream fixed it in `5de055f94` with regression fixtures in `a62784923`, both on `origin/develop`, and **this mirror has followed**. The record stays because the shape of the bug is the argument for how mirrors are tested.

**What it was.** `lzma.ts` ran `rep2 = rep1` for all three sub-cases of a rep match, including the one where the distance came from `rep1`. The reference decoder shifts `rep2` only when the distance came from `rep2` or `rep3`. So after the first `rep1` match the decoder's `rep2` held a duplicate of `rep1`, and the next `rep2`/`rep3` match resolved a wrong distance. `lzmaCompress.ts` had the shuffle right all along, so the two halves of one package disagreed.

**What it cost**, measured before the fix by round-tripping `compressLzma` output through `decompressLzma` over token-shaped and small-alphabet inputs:

|                                    |  Before | After |
| ---------------------------------- | ------: | ----: |
| round-trips checked                |    1200 |  1200 |
| failed                             | **161** | **0** |
| …returned `null`                   |      35 |     0 |
| …**returned wrong bytes silently** | **126** |     0 |

Whether a given stream corrupted or refused was luck: a wrong distance still in range let the decode continue and emit plausible wrong bytes, one out of range tripped a bound and returned `null`. The quiet face was the common one, 126 to 35 — which is why this was a data-integrity bug rather than an availability one.

It also made upstream refuse valid streams from other encoders, which is how it was first noticed: Python `lzma` output above roughly 2 KB of low-entropy text returned `null` while Python itself round-tripped it.

**Why upstream's suite could not see it.** The trigger is a `rep1` match followed later by a `rep2` or `rep3` match, which needs mixed content. All eight decoder fixtures were high-ratio — the largest low-ratio case 1212 bytes of a repeated sentence, the big ones 64 KB of `i % 7`, 1 KB of `i % 256`, `'abcABC123'` ×600 — and the encoder round-trip cases were the same shapes. Nothing in the suite was ordinary mixed content above 2 KB, which is exactly where the failure rate reached double digits. `.awd` and `.swf` bodies are the real inputs, and they are not `i % 7`.

**How it was found**, since the method generalises. Upstream's own encoder round-tripping cleanly on the shapes first tried ruled out "the decoder is simply broken" and pointed at a path `compressLzma` rarely emitted. Narrowing by Python encoder knob was actively misleading — `nice_len` 16 passed, 21 and 22 failed, 23 passed — and that non-monotonicity was the signal to stop guessing and trace instead. A traced copy of this mirror, logging every literal/match/rep event with its output range and diffed against known plaintext, put the first wrong byte at index 2239 of a 2304-byte stream, written by a rep match resolving a distance nothing had set. A rep distance nothing set is a clobbered rep slot, and the shuffle is three lines long.

**Verification after the fix**, against upstream `origin/develop`:

- upstream's encoder/decoder round-trip: 0 failures in 1200;
- 60 of 60 streams of exactly the shapes that used to break decode byte-for-byte identically through this mirror;
- the previously refused Python streams — 2304 bytes, 64 KB, 8 MB — decode through both;
- upstream's two new regression fixtures are mirrored here verbatim, and the two reproducers this crate found are kept as positive cases, so the fix is guarded by output from three independent encoders.

The mirror's rule held and was worth holding: it reproduced both faces of the defect until upstream fixed it, which is what kept the differential oracle meaningful, and flipping it afterwards was a one-line change plus swapping two pinned tests from negative to positive.

### Measured throughput

Low-ratio payloads are comparable for the first time: before the fix, upstream refused them.

| Payload          | ratio | upstream TS | this mirror | mirror vs TS |
| ---------------- | ----: | ----------: | ----------: | -----------: |
| repetitive 64 KB |  437x |     34 MB/s |    884 MB/s |          26x |
| repetitive 1 MB  | 3603x |    338 MB/s |    847 MB/s |         2.5x |
| repetitive 8 MB  | 6322x |    416 MB/s |    892 MB/s |         2.1x |
| text 64 KB       |  4.9x |     18 MB/s |    104 MB/s |         5.8x |
| text 1 MB        |  5.6x |     31 MB/s |    122 MB/s |         3.9x |
| text 8 MB        |  5.8x |     32 MB/s |    125 MB/s |         3.9x |

**2.1x to 26x**, and the spread says where the work is. On a large high-ratio stream both implementations sit in the same byte-at-a-time match copy and the gap narrows to about 2x. On low-ratio input the range coder dominates — one `decode_bit` per output bit, each with a probability update — and the mirror holds a steady ~4x. The 26x at 64 KB repetitive is mostly JavaScript's fixed cost per call showing up against a stream that decodes in 70 microseconds.

Since LZMA's compute lives in the range coder, the ~4x on realistic mixed content is the number that matters for `.awd` and `.swf` bodies, not the high-ratio headline.

Native figures again; wasm will be slower.

### The seam changed with the pin, as predicted here

Before the pin moved, substitution happened through a runtime registry: `registerDecompressor(Compression.Deflate, …)`, documented as last-write-wins specifically so a host could replace a portable decoder with a native or wasm one.

On `develop` that registry is **deleted**. The seam is now one named Host slot per algorithm: `@flighthq/types` declares `HostDecompressDeflateCapability`, `HostDecompressLzmaCapability`, `HostCompressDeflateCapability` and `HostCompressLzmaCapability`, and `@flighthq/compression` ships `sdkHostDecompressDeflate`, `sdkHostDecompressLzma`, `sdkHostCompressDeflate` and `sdkHostCompressLzma` as the slots Flight itself fills. Upstream's comment on the first of those names the change directly — "named for the Host slot it fills rather than for the registry it used to feed". Its deflate decoder was also renamed `inflateDeflate` → `decompressDeflate`.

The facade now fills all four slots, and the crate was untouched by any of it: it implements the algorithms, and the seam only decides how they are handed to a host. That is the division this file argues for throughout, and the pin move is the evidence it holds.

One hazard the move exposed, worth keeping in mind for every facade built this way. `packages/compression-wasm/src/index.ts` re-exports upstream wholesale (`export * from '@flighthq/compression'`) and shadows what it implements, so anything upstream adds flows through for free. The cost is that a slot the facade FAILS to fill does not fail to resolve — it silently resolves to upstream's TypeScript codec, and a host wiring up what looks like a wasm package gets a mix. After the rename, `decompressDeflate` would have come straight from upstream had the facade not been updated, with nothing failing anywhere. `src/compressionWasm.test.ts` asserts each of the four slots is ours for exactly this reason.

## Gaps worth knowing

`cargo fmt` and `cargo clippy` are not in any gate — not in `npm run check`, `npm run ci`, or `ci.yml`. Both hand-written crates are nevertheless rustfmt-clean and clippy-clean, and new mirrors should stay that way. The workspace as a whole is not: `cargo clippy --workspace` reports over two thousand warnings, almost all from generated crates, which is why gating it would mean a ratchet rather than a flag.

## Does the wasm barrier pay here?

Yes, unlike `bitmap-wasm`. Every codec is faster across the barrier, in both directions, on identical bytes — and since the pin moved to `develop` the whole table is reproducible in-tree with `npm run bench:barrier`, because upstream now ships the LZMA decoder and both encoders. Byte identity is asserted alongside each timing: a codec that got faster by producing different output has not got faster at anything.

| Codec                         | 64 KB |  1 MB |
| ----------------------------- | ----: | ----: |
| `decompressDeflate` (stored)  | 1.83x | 2.58x |
| `decompressDeflate` (huffman) | 4.38x | 2.90x |
| `compressDeflate`             | 3.53x | 3.67x |
| `compressDeflateZlib`         | 5.83x | 4.39x |
| `compressLzma`                | 3.73x | 4.07x |
| `decompressLzma`              | 1.13x | 1.23x |

**These numbers took two attempts, and the first set was wrong.** At ten iterations the 1 MB zlib encode measured 35.5 ms and would have been published as a 10.31x speedup; at two hundred it measures 15.1 ms, for 4.39x. The difference was entirely under-warmed JIT in the TypeScript baseline. The benchmark now warms five times and uses iteration counts high enough that the figures do not move when the counts change — worth stating because a flattering benchmark number is the one least likely to be questioned.

LZMA decode is the narrowest win at 1.13x–1.23x, which is the honest shape of that codec rather than a deficiency: the decoder is a bit-at-a-time range decoder with a serial dependency on probability-model state, so there is little for either implementation to exploit.

### Upstream finding: `computeAdler32` iterates a typed array with `for...of`

Isolated while explaining why the zlib encode gains more than the raw one. `upstream/packages/compression/src/deflateFormat.ts` reads:

```ts
for (const byte of input) {
  first += byte;
  ...
}
```

Measured over 1 MB, fifty iterations, against the identical arithmetic with an indexed loop:

| Form          |     1 MB |
| ------------- | -------: |
| `for...of`    | 12.70 ms |
| indexed `for` |  1.44 ms |

**8.84x, with the same result** — verified equal on the same input, not merely assumed. The iterator protocol over a `Uint8Array` is what costs it. This is the whole of the zlib encode's extra cost: `compressDeflateZlib` is `compressDeflate` plus an allocation, a `set`, and this checksum, and at 1 MB the measured delta between them is 12.88 ms against the 12.70 ms this loop accounts for.

It is a one-line change upstream with no behavioural difference, on a path every zlib-framed read and write crosses — `decompressDeflate` verifies the same checksum on RFC 1950 input. Worth reporting the way the LZMA rep-shuffle defect was. It is also a reminder that part of a wasm speedup can be an artifact of the baseline rather than merit: fix this loop and `compressDeflateZlib`'s margin drops toward `compressDeflate`'s.

The contrast with `bitmap-wasm` — about half the speed of its TypeScript — is the argument for this crate's discipline rather than an accident of workload. Same boundary, same toolchain; what differs is `usize` induction variables and an `enum` compared by discriminant here, against `f64` indices and an owned `String` compared per kernel tap in generated code. See [`agents/wasm-barrier.md`](wasm-barrier.md).
