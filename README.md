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
npm run check
cargo test --workspace
```

The initial executable target is `@flighthq/easing`, chosen as a small conformance canary for numeric expressions, control flow, closures, and optional parameters. Translation coverage expands by improving general rules; source exclusions remain visible in the generation report.
