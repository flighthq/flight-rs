# Architecture

## Objective

The repository is a compiler-like TypeScript-to-Rust translation system:

```text
upstream revision
  + generator revision
  + target configuration
  + semantic mappings
  = reproducible Rust crates
```

Handwritten Rust implementations and direct edits under `generated/` are not sources of truth.

## Pipeline

```text
Flight package barrels and TypeScript sources
                    ↓
       TypeScript AST and export graph
                    ↓
       normalized language-neutral IR
                    ↓
      Rust ownership and type lowering
                    ↓
      deterministic modules and crates
                    ↓
        rustfmt, cargo check, and tests
```

The inventory covers every upstream package regardless of current Rust emission support. The generation report separately identifies translated, mapped, and unsupported sources.

## Rust API

- `@flighthq/<name>` maps to crate `flighthq-<name>`.
- Defining TypeScript files map to snake_case Rust modules.
- Exported camelCase free functions map deterministically to snake_case free functions.
- Allocation, teardown, sentinel, out-parameter, and aliasing vocabulary remains explicit.
- Target-specific capabilities sit behind maintained runtime seams rather than contaminating bulk generated code.

## Blessed facades

`packages/surface-rs` is maintained because a JavaScript/wasm package seam is not derivable solely from Rust source. Its Rust implementation is not blessed: the backing crate must become generated output.
