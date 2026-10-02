# Cross-module anonymous record naming binds the wrong type

An inline object type in TypeScript — `{ bindGroupLayout: GPUBindGroupLayout; pipeline: GPURenderPipeline }` — has no name, so the emitter invents one: `<Owner>Record<N>`, where `N` is a counter. **That counter is assigned per module, over that module's own walk order.** When two modules reference the same structural shape, each numbers it independently, and nothing reconciles them.

At the `develop` pin this produces a wrong binding that compiles.

## The evidence

Upstream declares the same inline shape in two files:

```ts
// upstream/packages/types/src/WgpuDeviceRuntime.ts:27
mipmapPipelineCache: Map<GPUTextureFormat, { bindGroupLayout: GPUBindGroupLayout; pipeline: GPURenderPipeline }>;
// upstream/packages/types/src/WgpuRenderState.ts:186
mipmapPipelineCache: Map<GPUTextureFormat, { bindGroupLayout: GPUBindGroupLayout; pipeline: GPURenderPipeline }>;
```

What the generator emitted:

```rust
// generated/crates/flighthq-types/src/wgpu_render_state.rs:304 — the SURFACE EXTENT record
pub struct WgpuRenderStateRuntimeRecord1 {
    pub height: f64,
    pub width: f64,
}

// generated/crates/flighthq-types/src/wgpu_render_state.rs:323 — references a name nothing declares
pub mipmap_pipeline_cache: Vec<(crate::OpaqueHostValue, crate::WgpuDeviceRuntimeRecord1)>,

// generated/crates/flighthq-types/src/wgpu_device_runtime.rs:22 — references the WRONG struct, and compiles
pub mipmap_pipeline_cache: Vec<(crate::OpaqueHostValue, crate::WgpuRenderStateRuntimeRecord1)>,
```

Each module attributed the shared record to the other module's owner. Of the two resulting names, one was never declared and one already meant something else:

- `WgpuDeviceRuntimeRecord1` is declared **nowhere**. That is the `E0425` that fails `npm run generate`, because the conformance harvest needs a compiled candidate workspace.
- `WgpuRenderStateRuntimeRecord1` **is** declared — as `{ height, width }`, upstream's `borrowedSurfaceExtent`. So `wgpu_device_runtime.rs` types a cache of WebGPU pipeline handles as a pair of numbers.

**The second is the serious one, and it type-checks.** `Vec<(OpaqueHostValue, WgpuRenderStateRuntimeRecord1)>` is perfectly valid Rust. The compile error beside it is the lucky half of the same defect; had the counters happened to agree on both names, nothing would have failed and the wrong field type would have shipped. Byte-parity and idempotence checks would not notice either — the output is stable, just wrong.

## Why the existing mechanism does not cover it

Structural naming already exists and is already keyed by content rather than position:

```ts
anonymousTypes.set(key, `${prefix}Record${stableTypeIdentity(key)}`); // rust.ts:8503, 8545
```

But it is reached only for _structural utility types_ — `FlightPartial` and `FlightOmit`, via `isStructuralUtilityType`. A plain inline object literal takes the positional path instead (`rust.ts:7782, 8266, 8619, 8641, 8681`), and `importedNestedStructuralNames` numbers an imported owner's records `1..N` over its own walk, which is what diverges from the declaring module's numbering.

## What was measured, and what was done

The corpus says the live surface is small and the mechanism is unreliable:

|                                                                           |       |
| ------------------------------------------------------------------------- | ----: |
| synthesized record declarations                                           |   310 |
| of those, already named by structural hash (`FlightPartial`/`FlightOmit`) |    58 |
| named by per-module counter                                               |   252 |
| **references that cross a module boundary** (`crate::<Owner>Record<N>`)   | **3** |
| of those three: correct                                                   |     1 |
| of those three: named a struct nothing declared                           |     1 |
| of those three: resolved to a DIFFERENT struct, and compiled              |     1 |

So the guess was right one time in three. **The fix was to stop guessing**: a nested anonymous record inside an imported type is no longer given that module's private name for it. It stays anonymous, and the referencing module declares its own copy from the shape it can actually see. That needs only local knowledge, which is the point — the counter depends on the declaring module's walk order and is not computable from anywhere else.

The consequence is that such a record becomes nominally distinct per module. That is the honest outcome rather than a regression: if anything genuinely needs to assign one across a module boundary, it now fails as a compile error instead of binding the wrong fields silently.

`tests/generator/anonymous-record-identity.test.ts` locks the property: every crate-root reference to a synthesized record resolves to exactly one declaration, and a name referenced across modules may not be declared with two different shapes. It is scoped to names actually referenced across modules, which is why it tolerates the long-standing duplication below.

## The dormant half, deliberately left alone

`SharedStructuralRecord1` is declared in **45 modules with roughly 30 different shapes**, `SharedStructuralRecord2` in 10, and so on — all glob-re-exported into the crate root, which is the source of the five `ambiguous glob re-exports` warnings. Rust resolves such a name to whichever re-export wins.

This is latent rather than live: these records are referenced from inside their own modules, where the local declaration shadows the glob, so the warnings are warnings. Worth noting that the hashed names collide too (`FlightOmitRecord2968336371` is declared seven times) — but those collisions are **benign by construction**, because an identical hash means an identical shape. That is the real difference between the two schemes, and it is not cosmetic.

## The across-the-board migration, and why it is not queued

Naming every anonymous record by `stableTypeIdentity(key)` would make the scheme content-derived throughout: no module could disagree about a name, no two shapes could collide on one, and the glob ambiguity above would disappear. It is the right end state.

It is deliberately not being bought now:

- **It renames roughly 252 generated types.** Disposable output, so churn rather than risk — but a large diff.
- **`tests/generator/rust-emitter.test.ts` pins the positional convention on purpose**, asserting on `VariantRecordRecord1`, `SharedStructuralRecord1`. Within one module a counter genuinely reads better than a hash, and those assertions encode that.
- **The party who needs it does not exist yet.** Byte parity on generated crates is a stated `flight-compiler` adoption gate, and a walk-order counter can never be reproduced by an independent implementation, while a content hash can. So a content-derived scheme is a _prerequisite_ for that gate — which makes it the compiler's design decision to inherit rather than something to pre-pay here and then discard. On the current measurement (`docs/flight-compiler-adoption.md`: 34% of `@flighthq/types`, 0 of 44 for `@flighthq/bitmap`, five prerequisites none started) that gate is not close.

**Do the migration when** byte parity with `flight-compiler` becomes a live gate, or when cross-module references to anonymous records stop being rare — the count in the table above is the number to watch, and the test named earlier is what will report it.
