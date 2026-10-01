# @flighthq/compression-wasm

A Rust/wasm DEFLATE decoder for [`@flighthq/compression`](https://github.com/flighthq/flight/tree/main/packages/compression).

Upstream's decompressor registry is documented as last-write-wins specifically so "a host can replace a portable decoder with a native or wasm one". This is that replacement: the seam is declared upstream rather than invented here.

```ts
import { registerDecompressor } from '@flighthq/compression';
import { registerDeflateDecompressorWasm } from '@flighthq/compression-wasm';

registerDeflateDecompressorWasm(registerDecompressor);
```

Registration is explicit, matching upstream: importing the package registers nothing, so a build that never decompresses pays for no codec.

`inflateDeflate` is a drop-in for upstream's — the same `Decompressor` contract, returning `null` in exactly the same cases, including a failed zlib header or Adler-32 check, an output exceeding the declared bound, and a framing it does not recognise. The framing is never guessed, because a raw stream can legitimately begin with bytes that form a valid zlib header.

The implementation is a hand-written mirror of upstream's decoder in `crates/flighthq-compression-core`, held to it by **upstream's own test suite**: `npm run test:upstream` runs `upstream/packages/compression/src/deflate.test.ts` unchanged against this wasm module. See [`agents/compression-mirror.md`](../../agents/compression-mirror.md) for what that obliges and how the fixtures are shared.

Unpublished. The crate also carries LZMA decoding and DEFLATE encoding, which upstream added after the current submodule pin; those are not exposed here yet.
