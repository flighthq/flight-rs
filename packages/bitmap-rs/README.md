# @flighthq/bitmap-rs

A wasm-backed drop-in for [`@flighthq/bitmap`](https://www.npmjs.com/package/@flighthq/bitmap). The API is identical; the bulk pixel kernels run in Rust compiled to WebAssembly instead of TypeScript.

```sh
npm install @flighthq/bitmap-rs
```

Swap the import and nothing else changes:

```diff
-import { convolveBitmap, getBitmapHistogram } from '@flighthq/bitmap';
+import { convolveBitmap, getBitmapHistogram } from '@flighthq/bitmap-rs';
```

Call `initBitmapWasm()` once before the first pixel operation, or let the first call initialize lazily. Initialization is synchronous: the module is embedded as base64 in the bundle, so there is no `fetch`, no separate `.wasm` asset to host, and no `await` in your startup path.

```ts
import { createBitmap, createBitmapRegion } from '@flighthq/bitmap';
import { getBitmapHistogram, initBitmapWasm } from '@flighthq/bitmap-rs';

initBitmapWasm();
const bitmap = createBitmap(256, 256, 0xff102030);
const histogram = getBitmapHistogram(createBitmapRegion(bitmap, 0, 0, 256, 256));
```

## What is actually Rust

This package re-exports the whole of `@flighthq/bitmap` and shadows the subset of functions the generator has compiled to Rust. Everything else remains the upstream TypeScript implementation, so the module is always API-complete regardless of how much has been ported.

The wasm-backed functions are the ones listed under `wasmFacades` in `tools/generator/port.config.ts`, currently 34 of them: the color-matrix builders and `colorMatrixBitmap`; `convolveBitmap`, `dilateBitmap`, `erodeBitmap`, and `pixelateBitmap`; `applyBitmapCurve`, `applyBitmapLevels`, and `applyBitmapPaletteMap`; the alpha operations; the noise, Perlin, and turbulence fills plus `fillBitmapRectangle`; the copy and channel-merge operations; and the query functions (`getBitmapHistogram`, `getBitmapCoverage`, `getBitmapColorBoundsRectangle`, `getBitmapMismatch`, and the fingerprint pair).

Every one of them is differentially tested against the upstream TypeScript implementation, and the Rust output must match byte for byte.

## Requirements and caveats

- **Bundler required.** Like every `@flighthq` package, the published JavaScript uses extensionless relative imports, which Node's ESM resolver rejects. Use a bundler (Vite, webpack, Rollup, esbuild) or any toolchain that applies Node-style resolution to ESM. Bare `node script.mjs` will not resolve this package or its dependencies.
- **Support level is experimental.** The Rust slice is generated, and which functions it covers grows between releases. The API surface does not change — it is always `@flighthq/bitmap`'s.
- `@flighthq/bitmap` and `@flighthq/types` are peer-level runtime dependencies pinned to `^0.3.0`.

## License

MIT. See [LICENSE.md](LICENSE.md).
