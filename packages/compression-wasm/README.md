# @flighthq/compression-wasm

Rust/wasm codecs for [`@flighthq/compression`](https://github.com/flighthq/flight/tree/main/packages/compression), covering its whole surface except Brotli — which upstream deliberately does not ship either, because its decoder needs a large static dictionary that is data rather than rules.

| Export | Upstream counterpart | Held to it by |
| --- | --- | --- |
| `decompressDeflate` | `decompressDeflate` | **upstream's own `deflate.test.ts`, unmodified** |
| `decompressLzma` | `decompressLzma` | **upstream's own `lzma.test.ts`, unmodified** (Python `lzma`, `FORMAT_ALONE` fixtures) |
| `compressDeflate`, `compressDeflateZlib` | same | **upstream's own `compress.test.ts`, unmodified** |
| `compressLzma` | `compressLzma` | **upstream's own `lzmaCompress.test.ts`, unmodified** |
| `sdkHostDecompressDeflate`, `sdkHostDecompressLzma`, `sdkHostCompressDeflate`, `sdkHostCompressLzma` | same | the suites above, which assert each slot holds the exported function itself |

## Filling the Host slots

Upstream models each codec as a named slot on the Host, one per algorithm, and says a host "with a native or wasm codec supplies its own slot instead and never bundles this module". This supplies those slots — the seam is declared upstream rather than invented here:

```ts
import { sdkHostCompressDeflate, sdkHostDecompressDeflate, sdkHostDecompressLzma } from '@flighthq/compression-wasm';

const host = {
  decompress: { deflate: sdkHostDecompressDeflate, lzma: sdkHostDecompressLzma },
  compress: { deflate: sdkHostCompressDeflate },
};
```

A slot is one algorithm, so a host fills only what it has and an absent slot stays a fact a caller can read rather than a runtime failure discovered mid-parse. Nothing registers on import, so a build that never decompresses pays for no codec.

The decoders are drop-ins — the same `Decompressor` contract, returning `null` in exactly the same cases, including a failed zlib header or Adler-32 check, an output exceeding the declared bound, and a framing they do not recognise. The framing is never guessed, because a raw stream can legitimately begin with bytes that form a valid zlib header, so only the container knows which form it carries.

**Byte identity, not round-tripping, is the contract for the encoders.** Upstream documents its output as a pure function of its input, so every search bound is part of the agreement — the chain depth, the window, the interior-position insertions, the exact hash. Two mutations that still round-trip perfectly are caught by byte identity and would have passed a round-trip-only suite.

## Conformance

All four of upstream's compression suites run against this implementation, unmodified, in `vitest.config.upstream.ts`. Each lane redirects exactly one import inside one upstream test file; everything else — fixtures, framing constants, helpers — stays upstream's.

The encoder lanes deliberately leave the decoder resolving to upstream, so a round trip is checked by an implementation written independently of the encoder under test. Substituting both ends would check this crate against itself, where one shared misreading of the format cancels out and passes.

A conformance lane that stops matching is the failure mode worth naming, because it looks identical to success: with a specifier changed to one that never matches, the deflate suite still reported 25 of 25 passing, testing upstream against itself. So each lane asserts before any test runs that its target file still imports what it substitutes, counts its own substitutions, and fails having made none. `tests/generator/conformance-lane.test.ts` asserts the same property for every facade in the repository.

Implementation is a hand-written mirror in `crates/flighthq-compression-core`. See [`agents/compression-mirror.md`](../../agents/compression-mirror.md) for what mirroring obliges and why the fixtures are shared rather than reimplemented.

Unpublished.
