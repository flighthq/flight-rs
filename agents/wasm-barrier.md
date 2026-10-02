# The wasm barrier

A `-wasm` package is a **hybrid by design**. There is no requirement for every function to cross into Rust, and good reasons for several never to. This records which functions belong behind the barrier, which belong in TypeScript, and — measured rather than assumed — what the barrier currently costs.

## The uncomfortable measurement

`npm run bench:barrier` compares each wasm-backed function against the upstream TypeScript it replaces, same inputs, same process. **Greater than 1x means wasm is faster.**

| Function              | Size  |       wasm | TypeScript | ratio |
| --------------------- | ----- | ---------: | ---------: | ----: |
| `setBitmapAlpha`      | 16²   |  0.0037 ms |  0.0008 ms | 0.22x |
| `setBitmapAlpha`      | 256²  |  0.1336 ms |  0.1024 ms | 0.77x |
| `setBitmapAlpha`      | 1024² |  2.1836 ms |  1.6021 ms | 0.73x |
| `multiplyBitmapAlpha` | 1024² |  9.4126 ms |  3.2108 ms | 0.34x |
| `convolveBitmap` 5×5  | 256²  | 22.6907 ms | 10.3259 ms | 0.46x |
| `convolveBitmap` 5×5  | 512²  | 89.2859 ms | 41.4650 ms | 0.46x |
| `pixelateBitmap` 8    | 1024² | 13.7738 ms |  6.4457 ms | 0.47x |
| `dilateBitmap` r3     | 512²  | 98.2844 ms | 51.9644 ms | 0.53x |

**The shipped bitmap wasm is about half the speed of the TypeScript it replaces, at every size and on every function tried — including the compute-heavy ones.** A 5×5 convolution is 25 multiply-adds per channel per pixel and still loses by 2.2x. There is no crossover point to find.

That matters for planning: **widening the set of wasm-backed functions would currently make `bitmap-wasm` slower, not faster.** The appropriate improvement for the barrier is to make the existing 34 beat TypeScript first; after that, widening has a point.

## Compression crosses the same barrier and wins

The controlled comparison. Same boundary, same toolchain, same machine, same day — the only difference is that `compression-wasm` is a **hand-written mirror** and `bitmap-wasm` is **generated**. Greater than 1x means wasm faster.

| Codec                   | Payload     |      wasm | upstream TS |             ratio |
| ----------------------- | ----------- | --------: | ----------: | ----------------: |
| deflate decode, stored  | 1 KB → 8 MB |         — |           — | **1.69x – 2.49x** |
| deflate decode, Huffman | 64 KB       | 0.1073 ms |   0.3267 ms |         **3.04x** |
| deflate decode, Huffman | 8 MB        |  14.40 ms |    37.26 ms |         **2.59x** |
| LZMA decode             | 64 KB runs  | 0.2024 ms |     0.74 ms |         **3.66x** |
| LZMA decode             | 1 MB text   |  11.22 ms |    37.52 ms |         **3.34x** |
| deflate encode          | 64 KB runs  | 0.1932 ms |   1.0545 ms |         **5.46x** |
| deflate encode          | 1 MB text   |  38.37 ms |    97.76 ms |         **2.55x** |
| LZMA encode             | 64 KB runs  | 0.2550 ms |   1.2200 ms |         **4.78x** |
| LZMA encode             | 1 MB text   |  47.94 ms |   133.86 ms |         **2.79x** |

**Every compression codec is 1.4x to 5.5x faster across the barrier.** So the barrier is not the problem, and wasm is not the problem — the same mechanism that loses 2x for bitmap wins up to 5x here.

What differs is the Rust on the other side. The compression mirror uses `usize` induction variables and indices, an `enum Framing` compared by discriminant, and crosses the boundary once per buffer. The generated bitmap code uses `f64` induction variables cast `as usize` per access, and compares an owned `String` per kernel tap. Those two choices are the whole gap.

That is also the strongest argument for the mirror discipline: a hand-written structural port of upstream's algorithm, held to upstream's own tests, beats TypeScript comfortably. A mechanical transliteration of upstream's _types_ does not.

## It is not the barrier — it is the generated code

### Measured, so the marshalling hypothesis can be closed

The obvious suspicion about any wasm boundary is that the copying dominates, and here it is wrong. `passArray8ToWasm0` copies the whole pixel buffer into wasm memory on every call, and the mutable-slice convention copies it back out — 8 MB of traffic for one 1024² operation, which sounds decisive until it is timed against the same two memcpys:

| Operation        | Size  |     total |      copy |   compute | TypeScript | zero-copy would be |
| ---------------- | ----- | --------: | --------: | --------: | ---------: | -----------------: |
| `setBitmapAlpha` | 256²  | 0.1474 ms | 0.0093 ms | 0.1381 ms |  0.1054 ms |              0.76x |
| `setBitmapAlpha` | 1024² | 2.7702 ms | 0.1670 ms | 2.6032 ms |  1.8566 ms |              0.71x |

Copying is **6%** of the call at 1024², and this is the most bandwidth-bound function in the package — the case most favourable to the marshalling theory. Subtract the copy entirely and wasm still loses, 0.71x.

**So a zero-copy facade would not fix this.** Keeping pixel buffers resident in wasm memory, a persistent-handle API, bypassing the generated glue to pass a pointer — all of that is real engineering, all of it is available, and none of it would make `bitmap-wasm` faster than the TypeScript it shadows. The time is inside the kernel, and the second measurement is what licenses ignoring the first fix. `npm run bench:barrier` reports both tables.

### The two defects

Two defects in lowering account for it, and both are visible in `generated/crates/flighthq-bitmap/src/bitmap_convolution.rs`:

**1. Every loop counter and index is `f64`.**

```rust
let mut py = 0.0_f64;
while (py < source.height) {
    ...
    let weight = options.matrix[(weight_row_start + kx) as usize].clone();
```

Induction variables and array indices in floating point, cast `as usize` at each access. That defeats integer indexing, bounds-check elision and any vectorisation, and it is a faithful transliteration of JavaScript's single number type rather than a port of the algorithm.

**2. Closed string-literal unions lower to `String`, so mode checks become string comparisons inside the kernel.**

```rust
pub type BitmapEdgeMode = String;        // generated/crates/flighthq-types/src/bitmap_edge_mode.rs
```

```rust
let edge = ((options.edge).clone()).unwrap_or("clamp".to_owned());
while (kx < matrix_x) {
    ...
    if (edge == "transparent") { ... } else if (edge == "wrap") { ... } else if (edge == "mirror") { ... }
```

Up to three string comparisons **per kernel tap** — seventy-five per pixel for a 5×5 — against an owned `String`. V8 compares interned strings by identity and specialises the branch; this does neither.

Both are general lowering problems rather than package-specific ones: an integer induction variable where the source range is integral, and a Rust enum for a closed string union. Each would improve every generated function that loops or branches on a mode, which in bitmap alone covers the convolution, blend and channel paths.

## What belongs on each side

The judgement is already being applied, and until now was undocumented. Six functions are generated into the core crate and deliberately **not** exposed across the barrier by `port.config.ts` — `getBitmapPixel`, `getBitmapPixelLuminance`, `getBitmapPixelRgb`, `setBitmapPixel`, `setBitmapPixelRgb`, `invalidateBitmap` — because a per-pixel call pays the crossing cost for a byte of work.

**Leave in TypeScript, permanently:**

- **Host and DOM interop.** `createBitmapFromImageSource`, `captureBitmapFromImageResource`, `encodeBitmap`, `explainBitmapReadback`. These need the platform, not arithmetic. (`createBitmapFromCanvas` and `drawBitmap` were on this list until upstream removed them; `createBitmapFromImageSource` now covers the canvas case.)
- **Single-pixel accessors.** The six above. The crossing dominates the work by orders of magnitude.
- **Allocation and entity construction.** `createBitmap`, `cloneBitmap`, `createBitmapRegion`, `splitBitmapChannels`. These allocate through the entity package; identity and lifetime live in JavaScript and there is no compute to win.
- **String formatting and parsing.** `formatBitmapFingerprint`, `parseBitmapFingerprint`.
- **Constants.** `BITMAP_FINGERPRINT_COMPUTATION_ID`, the `BITMAP_NOISE_CHANNEL_*` set. Data, not code.

This list is enforced rather than merely written down. `tests/generator/facade-packaging.test.ts` asserts that no `wasmFacades` export set contains any of these names, and — because a guard naming things that no longer exist is a guard that has stopped working — that every name on it is still exported by upstream. That second half is what caught `createBitmapFromCanvas` and `drawBitmap` going away.

**Worth crossing, once the barrier pays:** the per-pixel kernels over flat buffers with no callback and no allocation — the blur family, the glow/bevel/shadow family, median and sharpen, the geometric resamplers, the gradient fills, the composite and channel operations. Roughly forty of the seventy-five currently deferred exports.

So the "75 deferred" figure should not be read as 75 missing things. Around a third of them should never cross, and the rest are waiting on the barrier being worth crossing at all.

## The hand-written mirrors: every kernel now beats TypeScript

`crates/flighthq-bitmap-core` mirrors six kernels with integer induction variables and indices, an `EdgeMode` enum, and no change to any floating-point arithmetic. Measured through the facade against upstream on identical inputs:

| Kernel                | Size  | generated |  mirrored | TypeScript | before |     after |
| --------------------- | ----- | --------: | --------: | ---------: | ------ | --------: |
| `multiplyBitmapAlpha` | 1024² |  9.904 ms |  1.005 ms |   3.194 ms | 0.31x  | **3.18x** |
| `multiplyBitmapAlpha` | 256²  |  0.625 ms |  0.061 ms |   0.191 ms | 0.33x  | **3.13x** |
| `setBitmapAlpha`      | 256²  |  0.157 ms |  0.050 ms |   0.109 ms | 0.63x  | **2.17x** |
| `setBitmapAlpha`      | 1024² |  2.285 ms |  0.769 ms |   1.502 ms | 0.68x  | **1.95x** |
| `pixelateBitmap` 8    | 1024² | 13.499 ms |  3.944 ms |   6.867 ms | 0.47x  | **1.74x** |
| `pixelateBitmap` 8    | 256²  |  0.811 ms |  0.237 ms |   0.399 ms | 0.54x  | **1.69x** |
| `convolveBitmap` 5x5  | 256²  | 23.085 ms |  7.070 ms |  10.915 ms | 0.46x  | **1.54x** |
| `convolveBitmap` 5x5  | 512²  | 93.924 ms | 27.938 ms |  42.484 ms | 0.44x  | **1.52x** |
| `dilateBitmap` r3     | 256²  | 24.961 ms |  9.804 ms |  14.152 ms | 0.56x  | **1.44x** |
| `dilateBitmap` r3     | 512²  | 99.996 ms | 39.531 ms |  52.713 ms | 0.52x  | **1.33x** |

Upstream's suite still passes 372 of 372, so every one of these is byte-identical. The crate also carries 108 differential cases whose expected bytes come from upstream's own TypeScript, which is what makes that a measured claim rather than a hope.

This settles the diagnosis above. The boundary was never the cost: removing `f64` counters and float indexing — and nothing else — moved five kernel families from losing by 2-3x to winning by 1.3-3.2x.

### Two findings worth keeping

**`multiplyBitmapAlpha` needed more than integer indexing.** At 0.68x with integer indices it still lost, because per pixel it converted `u8` to `f64`, multiplied, rounded and converted back for one byte of output. It is now a 256-entry table: the function is `clamp_byte(v * f)` over a `u8`, so there are exactly 256 possible results and the table holds all of them, each from the same `f64` arithmetic upstream performs. That took it to 3.18x, the largest single win in the package — and it is a memoisation rather than an approximation, with a test sweeping all 256 values across 65 factors against the per-pixel form.

**At 16² the mirrors still lose** — 0.42x for `setBitmapAlpha`, 0.69x for `multiplyBitmapAlpha`. No kernel work fixes that: a 1 KB operation is dominated by crossing the boundary at all. The remaining argument about small calls is about call size, not about kernels, and the single-pixel accessors below are the extreme of it.

### What is still generated

Twenty-seven of the facade's thirty-three wasm-backed exports still use generated kernels and are therefore still slower than upstream. The ones worth mirroring next are the families with real arithmetic per pixel — the colour matrix and curve/levels/palette paths, `copyBitmapPixels`, the noise fills. `getBitmapHistogram`, `getBitmapCoverage` and `getBitmapMismatch` reduce to a few numbers and should win easily on the same reasoning.

Each one needs its own differential fixture before it is believed. Mutation testing is why: swapping two colour channels in the convolution accumulator passed **all eleven** of upstream's hand-written cases, and replacing a rounding clamp with a truncating cast passed **every test in the crate** until the fixture began generating fractional alpha.

## Open: no facade substitutes the `./contract` lane

Every upstream package these facades stand in for exposes two entry points — `.` and `./contract` — and **no facade exposes `./contract`**. A consumer who substitutes by package name (aliasing `@flighthq/bitmap` to `@flighthq/bitmap-wasm`, which is how these are meant to be adopted) therefore gets the wasm implementation on the root lane and an unresolved import on the contract lane.

The contract lane is not just an alias for the root. It carries one extra module in each case — `bitmapReadbackResolver.ts` for bitmap, `deflateFormat.ts` (`computeAdler32`, the DEFLATE length/distance tables) for compression — so it is a genuinely wider surface rather than a second name for the same one.

Closing it is a handful of lines per package: a `src/contract.ts` that shadows the same wasm-backed names and re-exports `@flighthq/<pkg>/contract`, plus an `exports` entry. It is deliberately **not** done yet, for two reasons worth stating rather than discovering later:

- **It widens a published API.** `bitmap-wasm` is the one published facade, and its surface is gated by `published-api-contract.ts` and `npm run typecheck:published`. Adding an entry point there is a release decision.
- **It doubles the ambient-shim surface.** `src/upstream-contract.d.ts` exists because the published `@flighthq/*` packages ship bundler-only extensionless imports that `tsc` cannot read, and its own comment warns that a wrong declaration there is invisible to every other check. A `./contract` lane needs a second `declare module` per package, carrying the extra names, kept in step by hand.

Whether the lane is worth that depends on something not known here: whether anything outside upstream actually imports it. Upstream uses it internally (`@flighthq/types/contract` throughout); a downstream application would normally import the root.

## How to make the barrier pay

In order of leverage:

1. **Integer induction variables and indices** where the source range is integral. Removes the `f64` arithmetic and the per-access cast from every generated loop.
2. **Closed string unions as Rust enums.** Removes string comparison from kernel inner loops.
3. **Then re-measure.** `npm run bench:barrier` is the instrument, and the table above is the baseline to beat.
4. **Only then widen the exposed set**, kernel families first.

A hand-touched kernel is a legitimate alternative to waiting for lowering — the wasm packages are separately maintained, and `crates/flighthq-compression-core` is the precedent. But a hand-written kernel still has to be a structural port of upstream's algorithm, held to upstream's own tests, and it should be taken only where the generated version has been measured and found wanting. Convolution is the obvious first candidate: it is the hottest, the furthest behind, and its two defects are the general ones above in their clearest form.
