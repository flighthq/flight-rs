# @flighthq/physics2d-abi-wasm

**Deferred — not supported, and not on a path to release.** A persistent Rust/wasm backend for the upstream `@flighthq/physics2d-abi` packed-buffer ABI, kept because the protocol work in it is sound and the conformance number is worth tracking honestly.

Upstream's own suite currently passes **59 of 79** against this backend. The remaining failures are not a long tail of small gaps: most of them need the `spatial` → `collision` → `physics2d` chain, and one asserts **bit-exact reproduction** of upstream's solver over 120 steps with no tolerance. Our `step` integrates gravity and nothing else.

`crates/flighthq-physics-abi-wasm-core` is deliberately **not** being grown by hand toward that number. Doing so would make it a second implementation of physics to maintain, which is the opposite of what this repository is for — upstream is the implementation, and Rust here is a structural port of it. See [`agents/physics-abi-parity.md`](../../agents/physics-abi-parity.md) for the per-failure classification and what it would actually take.

The conformance lane still runs, so the number cannot quietly drift.
