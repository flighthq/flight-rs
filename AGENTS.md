# Flight Rust Generator

This repository mechanically translates the Flight TypeScript SDK in [`upstream/`](upstream/) into Rust. The upstream submodule is the API and behavioral source of truth.

## Source boundaries

- `upstream/` is read-only generator input.
- `tools/generator/` contains the analyzer, target-neutral intermediate representation, Rust emitter, semantic mappings, and drift checks.
- `generated/` is disposable Rust output. Never fix a problem by editing it directly.
- `packages/` contains explicitly blessed TypeScript facades, each named `<upstream package>-wasm` after the package it substitutes. Bitmap wraps a generated core; the physics ABI facades implement independent persistent Rust/wasm backends.
- `reports/` records inventory, lowering, generation coverage, exclusions, and source fingerprints.
- `tests/` covers generator rules and compiled generated output.

## Rules

- Parse TypeScript with the TypeScript AST and resolve package exports before emission.
- Preserve Flight's searchable free-function API. Rust uses deterministic snake_case names but does not turn free functions into methods.
- Every upstream declaration is generated, semantically mapped, or reported as unsupported with a stable source identity. Never silently drop input.
- Generated output must be deterministic and idempotent.
- Prefer general lowering rules over package-specific patches. Genuine exceptions belong in `tools/generator/port.config.ts` with a reason.
- Keep commits to a single-line Conventional Commit subject.

## Commands

- `npm run generate`
- `npm run generate:check`
- `npm run typecheck`
- `npm run test`
- `npm run check`
- `cargo test --workspace`
