# flight-compiler adoption

[`flighthq/flight-compiler`](https://github.com/flighthq/flight-compiler) is building `@flighthq/tool-compiler`, the one versioned TypeScript compiler intended to replace the generators in `flight-hx`, `flight-cpp`, and this repository. Its roadmap puts Rust in phase 6, deliberately after the Haxe seam is proven, and states plainly that "no downstream repository installs this package in place of its generator yet."

This is the downstream register for that migration — the same role `flight-cpp/docs/flight-compiler-adoption.md` plays for C++. It exists so the Rust side of the handover is written down somewhere before it is needed, and so the compiler's Rust readiness figure is backed by a measurement taken here rather than by a planning estimate.

Nothing in this repository installs or calls `@flighthq/tool-compiler` today, and nothing here should until the gates below are met. `tools/generator/` remains the only source generator.

## Division of ownership

The compiler owns the source-compilation path: workspace and package analysis, export lanes, declaration fingerprints and provenance, the target-neutral IR, semantic patches and audits, coverage models, orchestration, and concrete Rust lowering and emission.

This repository keeps its target ecosystem: the maintained Rust runtime and standard-library crates, Cargo workspace structure, the wasm and native host seams, the blessed TypeScript facades in `packages/`, examples, execution integration tests, packaging, and releases. Generated Rust stays disposable output.

The rule that follows from that split: a representation defect in the compiler is never worked around by editing `generated/`, and never by weakening a runtime assertion. That is already this repository's standing rule for its own generator, and it does not change when the generator moves.

## Rust target contracts that must survive the move

`agents/architecture.md` records three paragraphs of Rust-target semantics written specifically as contracts to preserve when work moves into `flight-compiler`. They are the substance of this register — a reimplementation that passes byte parity on today's corpus can still lose them, because most are about constructs the corpus does not currently reach.

- **Portable value representation.** The canonical recursive `FlightValue`, `OpaqueHostValue` only as a compatibility alias for statically unrecovered boundaries, and the container rules for `JSON.stringify` lowering — including which residues stay explicit failures rather than invented empty records.
- **Representation-driven semantics.** Clone placement derived from consumption liveness rather than source alias identity; mutating collection methods that return their receiver; one UTF-16 view per reused string parameter so `.length` and `codePointAt` share code-unit indexing; `Option` at nullable conditional joins; `ToInt32` for numeric bitwise-not without disturbing nominal bitflag enums.
- **Package planning and tasks.** One Cargo identity per upstream package, with a compatibility-named crate required to name the canonical identity it shadows and structurally exclude the automatic candidate; declared task outputs flowing into unannotated `Promise.resolve`/`reject` factories; a dynamic host call's task represented by a typed `HostUnavailable` rejection that never manufactures a successful output; `Vec<FlightTask<T>> -> FlightTask<Vec<T>>` with concurrent observation and stable index storage.

`tests/generator/compiler-adoption.test.ts` asserts those paragraphs are still present, so this register cannot end up indexing prose that has been deleted.

## Measured position, 2026-10-02

The compiler's roadmap reports Rust generator parity at 5–10% as a planning estimate dated 2026-08-18, and records downstream drop-in integration at 0% because no target has produced a Rust corpus ledger. The numbers below are measured from this repository. They are a **lower bound**, for a reason stated in full under Caveats.

This measurement was **re-taken when the Flight pin moved to `develop`**. The compiler revision is unchanged from the previous measurement, so the Flight pin is the only variable that moved — which makes the comparison below a clean reading of what a four-month-newer SDK does to the same compiler.

| Input                          | Revision                                   |
| ------------------------------ | ------------------------------------------ |
| flight-rs                      | `8b6bc4b09da0f3c023204406103298b342ce94db` |
| Flight (this repository's pin) | `a62784923f3be814463286c9fb28edfe6ab15789` |
| flight-compiler                | `ec8da2a6399ca6b916466779c86ca0a039ed4847` |
| `@flighthq/tool-compiler`      | `0.0.0`, unpublished                       |

| Upstream package   | Modules | Emitted | Refused | Emitted % |
| ------------------ | ------: | ------: | ------: | --------: |
| `@flighthq/types`  |    1026 |     353 |     673 |       34% |
| `@flighthq/bitmap` |      44 |       0 |      44 |        0% |

### What moved, and the direction is the finding

Against the previous pin the same compiler emitted 464 of 882 `@flighthq/types` modules (53%). The SDK has since grown to 1026 modules and emission **fell to 353 (34%)** — fewer modules emitted in absolute terms, against a larger input.

This is the measurement's whole purpose, and it is why `tests/generator/compiler-adoption.test.ts` requires the recorded pin to be the current one. A figure carried forward across a pin move would have reported 53% while the truth was 34%, and the error would have grown silently in the direction that matters — upstream moves faster than the compiler.

**673 refusals come from 114 root causes.** The rest — 559 — are `dependency … was refused` cascades, modules refused only because something they import was. That ratio is the actionable part: the root list is short.

| `@flighthq/types` root refusal                     | Count |
| -------------------------------------------------- | ----: |
| host-type binding plan incomplete                  |   103 |
| intersection type needing record or trait lowering |     9 |
| other (`external type`, `Partial<T>`)              |     2 |

The binding refusals name the same host surfaces as before, now with WebGPU well represented alongside WebGL: `WebGLProgram` (17), `AbortSignal` (16), `GPUBindGroupLayout` (15), `GPURenderPipeline` (15), `WebGLUniformLocation` (14), `WebGLTexture` (10), `CanvasRenderingContext2D` (9), `GPUTextureFormat` (9).

**`@flighthq/bitmap` is unchanged: 44 modules, none emitted.** Its refusal profile is also unchanged in substance, which matters more than the figure — the blocker is the same one.

| `@flighthq/bitmap` refusal                                           | Count |
| -------------------------------------------------------------------- | ----: |
| `operator …` requires Rust type-directed lowering                    |    19 |
| external symbol binding plan incomplete (mostly `SharedArrayBuffer`) |     9 |
| module evaluation dependency missing (`@flighthq/types/contract`)    |     4 |
| open structural construction target                                  |     3 |
| dependency refused (cascade)                                         |     2 |
| `??` requires an Option-shaped left operand                          |     2 |
| external constructor ABI plan incomplete (`Array`)                   |     2 |
| `typeof` on unknown                                                  |     1 |
| mutable module variable needs synchronization lowering               |     1 |
| syntactic interface heritage                                         |     1 |

**The two packages fail for different reasons, and the difference is the finding.** `@flighthq/types` is declaration-shaped, and its root refusals are dominated by absent host-type bindings. Those are configuration plus a bounded lowering list.

`@flighthq/bitmap` is imperative numeric code, and its single largest family is:

```
operator < on number and unknown requires Rust type-directed lowering
operator + on unknown and number requires Rust type-directed lowering
operator * on unknown and unknown requires Rust type-directed lowering
```

That family appears **zero** times among `@flighthq/types`' root refusals. It is the open decision the compiler's roadmap records under "The analysis checker has no library types, decided open 2026-08-22", whose own worked example is `index < values.length` refusing as `operator < on number and unknown`. The checker is built with `noLib: true`, so `Uint8ClampedArray.length` has no type and every loop bound over pixel data goes unknown.

So the practical statement for this repository is narrower and harder than "Rust is at 5–10%": **the one package this repository publishes a facade for is blocked on the deepest unresolved decision in the compiler, not on a tail of small lowerings.** `packages/bitmap-wasm` substitutes `@flighthq/bitmap`, and its Rust core is exactly the pixel-loop code that needs library types to lower at all. A Rust backend could reach useful coverage across the SDK's declaration surface while remaining unable to emit the one crate this repository ships.

### Caveats

These figures come from the single-package command-line lane — `flight-compile <dir> --target rust --package <name> --report` — because that is the only entry point this repository can drive today. That lane has no package graph and **no way to supply a binding profile**: there is no `--bindings` flag. `flight-cpp` closes whole refusal families by electing host types through profile files such as `bindings/web-types.json` and `bindings/runtime.json`, and the compiler's own notes record one such election moving its corpus from 1,108 to 1,339 emitted modules.

Every `binding plan is incomplete` refusal above is therefore a lane artifact rather than demonstrated incapacity, and the real number is higher than 353/1026. Because 103 of the 114 root refusals are exactly that family, a binding profile is the single highest-leverage thing missing — and the 559 cascades mean each root closed should free several modules rather than one.

The `operator … on unknown` family is not a lane artifact: it is a lowering refusal that no binding profile addresses. Read the tables as "what a downstream repository can measure before it has built a corpus driver", not as the compiler's capability.

### Reproduction

From a `flight-compiler` checkout at the revision above, with `npm ci && npm run build` done:

```sh
BIN=packages/tool-compiler/dist/packages/compiler-command-line/src/compilerCommandLineEntryPoint.js
node "$BIN" /path/to/flight-rs/upstream/packages/bitmap/src \
  --target rust --package @flighthq/bitmap --out /tmp/rs-bitmap --report
node "$BIN" /path/to/flight-rs/upstream/packages/types/src \
  --target rust --package @flighthq/types --out /tmp/rs-types --report
```

`--report` prints refusals and exits zero, so the run is an instrument rather than a gate. Nothing is written into this repository; point `--out` outside it.

## What this repository must build before phase 6

In dependency order. None of this is started, and none of it should start before the compiler's Haxe seam is proven — the roadmap's sequencing exists because Rust would otherwise pay twice for every seam correction.

1. **A pinned dependency on the compiler.** `flight-compiler` resolves its own downstream checkouts through a `dependencies.lock.json` naming repository, branch, and commit, and materializes them under `.dependencies/`. This repository needs the mirror of that so a measurement names one compiler revision instead of whatever was checked out.
2. **A corpus driver.** The ledger the compiler's `readiness:corpus` instrument reads is `manifest.json` and `refusals.json` under a target's `generated/`, and its own comment says the SDK corpus "can only be generated downstream, because the request needs a target repository's package graph, package targets, and binding profiles." Until this repository has that driver, Rust readiness is an estimate and C++ is the only target with a number. This is the single highest-value item here, and it is generator work.
3. **A binding profile for the Rust target.** The host types that refuse above are elections, not lowerings. `flight-cpp` owns its profile; the Rust equivalent belongs here alongside the runtime crates that implement them.
4. **A parity harness against `reports/`.** This repository already emits `reports/inventory.json`, `reports/lowering.json`, `reports/generation.json`, and `reports/api.json` with a `schemaVersion` and the upstream commit. Those are the natural comparison surface for the compiler's own inventory and coverage models, and comparing them is cheaper than comparing emitted crates.
5. **Byte parity on generated crates, then removal.** The compiler's definition of drop-in requires identical generated source, file paths, inventories, patch audits, and reports against the same pinned Flight revision, a packed artifact installing into a clean consumer, deterministic re-generation leaving the tree clean, and only then removal of `tools/generator/` and the duplicated compiler-owned tests.

## Open questions for the compiler side

Neither of these is answerable here, and both change what this repository has to build.

- **The library-types decision governs whether Rust adoption is possible at all for imperative packages.** The compiler records three options: keep the isolated `noLib` program and refuse everything whose meaning lives in the library; pass the real program that `createTypeScriptProject` already builds, accepting that the pinned TypeScript version becomes part of compiler identity; or take the middle path of an optional checker recorded in the report. On the evidence above, the first option is not a viable end state for `@flighthq/bitmap` — the whole package is library-typed array arithmetic. This repository's own generator resolves those types today, so the capability is not in question; the determinism policy is.
- **There is no `flight-rs-adoption.md` on the compiler side, and `flight-rs` is absent from its `dependencies.lock.json`**, which locks `flight-cpp` and `flight-hx`. The compiler's README names `flight-hx` and `flight-rs` as its integration targets while `flight-cpp` — described there as having been incubated in the compiler repository before moving out — holds the second lock slot. That is consistent with Rust being phase 6 and is not a defect, but it means no register on either side currently tracks Rust asks, which is what this document starts.
