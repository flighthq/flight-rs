# @flighthq/compression-wasm

Rust/wasm codecs for [`@flighthq/compression`](https://github.com/flighthq/flight/tree/main/packages/compression), covering its whole surface except Brotli — which upstream deliberately does not ship either, because its decoder needs a large static dictionary that is data rather than rules.

| Export | Upstream counterpart | Held to it by |
| --- | --- | --- |
| `inflateDeflate` | `inflateDeflate` / `decompressDeflate` | **upstream's own `deflate.test.ts`, 25 of 25** |
| `decompressLzma` | `decompressLzma` | upstream's LZMA fixtures (Python `lzma`, `FORMAT_ALONE`) |
| `compressDeflate`, `compressDeflateZlib` | `compressDeflate`, `compressDeflateZlib` | byte identity over a 12-case corpus |
| `compressLzma` | `compressLzma` | byte identity over the same corpus |

Upstream's decompressor registry is documented as last-write-wins specifically so "a host can replace a portable decoder with a native or wasm one". This is that replacement — the seam is declared upstream rather than invented here:

```ts
import { registerDecompressor } from '@flighthq/compression';
import { registerCompressionWasmDecompressors } from '@flighthq/compression-wasm';

registerCompressionWasmDecompressors(registerDecompressor);
```

That fills both the `deflate` and `lzma` slots. `registerDeflateDecompressorWasm` fills only `deflate`, matching upstream's single-codec registrar. Registration is explicit either way: importing the package registers nothing, so a build that never decompresses pays for no codec.

The decoders are drop-ins — the same `Decompressor` contract, returning `null` in exactly the same cases, including a failed zlib header or Adler-32 check, an output exceeding the declared bound, and a framing they do not recognise. The framing is never guessed, because a raw stream can legitimately begin with bytes that form a valid zlib header, so only the container knows which form it carries.

**Byte identity, not round-tripping, is the contract for the encoders.** Upstream documents its output as a pure function of its input, so every search bound is part of the agreement — the chain depth, the window, the interior-position insertions, the exact hash. Two mutations that still round-trip perfectly are caught by byte identity and would have passed a round-trip-only suite.

## Coverage and the pin

Only the DEFLATE decoder has a pinned upstream suite to run against: LZMA and both encoders landed upstream after this repository's submodule pin, so their fixtures are embedded from upstream's own test files instead. That keeps the oracle third-party — the LZMA streams were generated with Python's `lzma` module, by neither implementation.

Implementation is a hand-written mirror in `crates/flighthq-compression-core` (2730 lines, 51 tests). See [`agents/compression-mirror.md`](../../agents/compression-mirror.md) for what mirroring obliges and why the fixtures are shared rather than reimplemented.

Unpublished.
