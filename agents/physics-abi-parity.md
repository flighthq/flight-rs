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

**Bit-exact solver reproduction (1 failure).** This one is project-scale and it is worth being precise about why.

`referencePhysics2DAbi.test.ts:51` steps a settling three-box stack 120 times at 1/60 and asserts position, angle, linear and angular velocity with `toBe()` — bit-exact, no tolerance — guarding the degenerate pass with `expect(bodies[3].y).toBeLessThan(2.7)`. Our backend's `step` integrates gravity and nothing else.

Passing it means reproducing upstream's solver with identical floating-point operation order:

| Source that must be mirrored bit-exactly | Lines |
| ---------------------------------------- | ----: |
| `physics2d/src/step.ts`                  |  1379 |
| `physics2d` total, non-test              |  6690 |
| `@flighthq/collision`, non-test          | 10398 |

So roughly 17,000 lines of float-order-sensitive solver and collision code, and there is no partial credit: `toBe()` over 120 iterations either matches or does not. Worse, the usual reasons to write Rust — reordering, vectorising, a better broadphase — are all forbidden by the assertion, so the performance ceiling for a _conforming_ backend is Rust's constant factor and nothing more.

That is a different kind of undertaking from the compression mirror, which was 318 upstream lines with a bit-defined oracle, or bitmap, which the generator produces mechanically.

## Recommendation

Take the first two groups: **13 of the 20 2D failures are reachable** without any solver, and they are what make the ABI _usable_ — a backend that answers queries and validates commands correctly but declines to step is honest and useful, because `Physics2DAbiCapability` exists precisely so a backend can advertise what it does.

Treat contacts and the exact solver as a separate decision. If bit-exact stepping is wanted, it is a project with a 17,000-line mirror at its centre; if what is wanted is a _fast_ solver, the exact-reproduction assertion is the thing to renegotiate with upstream first, because it forbids every optimisation that would motivate the work.
