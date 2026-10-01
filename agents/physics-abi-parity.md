# Physics ABI parity

`packages/physics2d-abi-wasm` and `packages/physics3d-abi-wasm` substitute `@flighthq/physics2d-abi` and `@flighthq/physics3d-abi` — flat command-buffer protocols — with a persistent Rust/wasm backend in `crates/flighthq-physics-abi-wasm-core`. Both are private and partial.

The conformance lane runs upstream's own suite against our backend, the same bar `bitmap-wasm` (376/376) and `compression-wasm` (25/25) meet:

| Facade               | Upstream suite  |     Passing |
| -------------------- | --------------- | ----------: |
| `physics2d-abi-wasm` | `physics2d-abi` | **59 / 79** |
| `physics3d-abi-wasm` | `physics3d-abi` | **59 / 73** |

## What the remaining 2D failures actually require

Classified by what each needs, because the groups have very different costs and only one of them is large.

**Pure ABI decoding and bookkeeping — no physics (4 failures).** Reachable without any solver or geometry work.

- rejects invalid authored state before it can make the world unsteppable
- rejects trailing bytes on a variable-length collider record
- derives mass from colliders rather than from the wire
- advances the world it names and leaves a sibling world alone

**Geometric queries (8 failures, plus 1 gated behind them).** The backend has no query support at all: the template's `query` is an honest stub that returns `false`, and `CAPABILITIES` advertises `PersistentWorlds | SelectiveReadback` only, never `Queries`.

- point: names the containing body and zeroes the geometric row; empty space; no stale republish on reuse
- ray: every crossing with fraction, point and normal; nearest only; no stale rows after a reuse
- region: capacity exhaustion through `requiredCount` rather than silent truncation
- shape cast: stops at the first body along the sweep; a clean miss is an empty answer
- _and_ "declares its version and every capability the reference implements", which cannot pass until the capability is real

Feasible: the suite uses AABB colliders and one circle cast, and asserts counts and ids exactly but geometry with `toBeCloseTo(…, 9)`. Prerequisite: `Collider` currently stores only `body_id`, so the shape kind and scalars have to be captured when `SET_COLLIDER` is decoded.

**Contacts and joints (5 failures).** Needs real collision detection and manifold generation, not just geometry.

- reports the resting contact with its manifold points
- publishes a prefix rather than a subsequence when the point capacity binds
- separates the began selection from the standing set
- stops at the first contact whose manifold points do not fit
- reports a joint that broke, which the world no longer holds

**Bit-exact solver reproduction (1 failure).**

`referencePhysics2DAbi.test.ts:51` steps a settling three-box stack 120 times at 1/60 and asserts position, angle, linear and angular velocity with `toBe()` — bit-exact, no tolerance. Our backend's `step` integrates gravity and nothing else.

**This is not a request to write a solver.** `referencePhysics2DAbi.ts` is upstream source like any other, and upstream already has the implementation: `physics2d` drives it. So the question is not "who writes a solver" but "why is upstream's not generated", and the answer is in `reports/generation.json` — the whole chain is **source-blocked on a handful of generator lowering gaps**, not on size:

| Upstream package          | Status         | Blockers |
| ------------------------- | -------------- | -------: |
| `@flighthq/math`          | compiled       |        0 |
| `@flighthq/spatial`       | source-blocked |        2 |
| `@flighthq/collision`     | source-blocked |        6 |
| `@flighthq/physics2d`     | source-blocked |        6 |
| `@flighthq/physics2d-abi` | source-blocked |        5 |
| `@flighthq/physics3d`     | source-blocked |        9 |
| `@flighthq/physics3d-abi` | source-blocked |        5 |

And the blockers repeat rather than being distinct problems. `Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery` accounts for 6 of collision's, 4 of physics2d's, both of spatial's and 1 of physics2d-abi's. Three of physics2d-abi's are the same `new-expression Rust lowering is not implemented: OpaqueHostValue::Object`, one of them in `referencePhysics2DAbi.ts` itself. The rest are one `dynamic for-in Rust enumeration`, and two packages needing export re-exposure.

So bit-exactness is not the obstacle it looks like. Generated code is a transcription of upstream's arithmetic in upstream's order, which is exactly what `toBe()` after 120 steps demands. A **hand-written** solver would have to earn that agreement; a generated one gets it by construction.

## Recommendation

**Fix the lowering, do not hand-write the solver.** Upstream is the implementation; this repository's job is to put it in Rust, and where that needs human help the help belongs in a general lowering rule rather than in a second copy of the behaviour. Five or so capabilities — typed recovery for the `OpaqueHostValue` sources, `new`-expression lowering, dynamic `for-in`, and export re-exposure for two packages — unblock the whole `spatial → collision → physics2d → physics2d-abi` chain, and `physics3d` behind it.

**`crates/flighthq-physics-abi-wasm-core` is a waypoint, not the destination.** Its 729 lines integrate gravity and decline everything else. That is a _different_ implementation of the ABI rather than a port of upstream's, which is the thing to retire once the chain generates — not something to grow toward 79/79 by adding more hand-written physics.

The two groups that are still worth doing by hand are the ones that are not physics at all: the four ABI decoding and bookkeeping failures, and the geometric queries. Those are protocol behaviour the backend owns in any case, they are small, and they make the backend honestly useful in the meantime, since `Physics2DAbiCapability` exists precisely so a backend can advertise what it does.

An earlier version of this document recommended renegotiating the exact-reproduction assertion with upstream if a faster solver was wanted. **That was wrong and is withdrawn.** The assertion is the oracle that proves the port is faithful; a backend that diverged from it would be a second implementation to maintain, which is the outcome this repository exists to avoid. If a different solver is ever wanted, it should be written in TypeScript upstream and ported from there like everything else.
