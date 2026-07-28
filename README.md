# flight-rs

`flight-rs` is a mechanically generated Rust port of the Flight SDK.

The repository deliberately separates durable source from disposable output:

- [`upstream/`](upstream/) pins the authoritative Flight TypeScript source.
- [`tools/generator/`](tools/generator/) analyzes TypeScript, lowers it into a normalized IR, and emits Rust.
- [`generated/`](generated/) is reproducible Rust output.
- [`packages/surface-rs/`](packages/surface-rs/) is the first explicitly blessed TypeScript-to-wasm facade.
- [`reports/`](reports/) accounts for upstream API coverage and every unsupported source.

The generator architecture is cultivated from [`flighthq/flight-hx`](https://github.com/flighthq/flight-hx): AST and export inventory first, normalized lowering second, deterministic target emission third, and explicit semantic mappings instead of edits to generated files.

## Setup

```sh
git submodule update --init --recursive
npm ci
npm run generate
npm run wasm
npm run check
cargo test --workspace
```

`npm run wasm` installs the pinned Rust target and an exactly matched `wasm-bindgen` CLI when needed, compiles the generated adapter, and bakes synchronous module bytes into the blessed facade. `npm run check` exercises both generator tests and the real wasm facade parity suite.

The compiled target set currently covers all of `@flighthq/easing`, the portable image-resource state slice, the required type closure, and an initial 11-module `@flighthq/surface` kernel slice. The facade shadows only those mechanically generated operations; every deferred surface export continues to come directly from cultivated TypeScript. Translation coverage expands by improving general lowering rules, while source and declaration deferrals remain fingerprinted in [`reports/generation.json`](reports/generation.json).

## Preserved manual port

The pre-filter handwritten Rust lineage is preserved separately as `manual-rust-port-b9022b67.bundle` (SHA-256 `8d3886c00175b97cdb43d4b629e112a242473f5c129d4ed01cc9f175c4a3717c`). It is an archive, not an input to generation. Restore it into a disposable repository with:

```sh
git clone manual-rust-port-b9022b67.bundle archived-flight-rs
```
