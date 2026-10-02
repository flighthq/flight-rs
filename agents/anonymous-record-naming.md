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

## The fix, and why it was not just applied

Name every anonymous record that is visible outside its module by `stableTypeIdentity(key)` rather than by a counter. The name then becomes a pure function of the shape, so two modules cannot disagree about it and two shapes cannot collide on it.

It was left for a decision because the blast radius is wider than the defect:

- **Roughly 239 generated type names change**, from `OwnerRecord1` to `OwnerRecord<hash>`. Generated output is disposable, so that is churn rather than risk — but it is a large diff to review.
- **`tests/generator/rust-emitter.test.ts` pins the positional convention deliberately**, asserting on `VariantRecordRecord1`, `VariantRecordRecord2`, `SharedStructuralRecord1`. Those assertions encode readability of single-module output, which the counter genuinely serves better than a hash does.

A narrower variant is available and may be the better trade: keep the counter for records declared and used within one module, and switch to structural identity only where a record crosses a module boundary. It needs one thing the current code does not track — whether a record is referenced outside its declaring module — because a module cannot otherwise know which scheme to use for its own declaration.

## Until then

`npm run generate` fails at the conformance harvest, so the `develop` pin cannot complete a clean run. The candidate workspace is otherwise at **one** compile error, down from 202.
