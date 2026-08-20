# Generator Resume Playbook

This is the durable handoff for continuing the mechanical Flight TypeScript-to-Rust port. It describes the repository after the Pass 27 Stage 2 canonical task-runtime checkpoint. Resume from repository `HEAD`; do not reconstruct earlier passes or edit generated Rust by hand.

## Goal and policy

The repository is a compiler project, not a collection of manually ported crates.

- A small, explicit set of packages may be cultivated by hand.
- `packages/surface-rs` is the compatibility-named cultivated facade for upstream `@flighthq/bitmap`. Its bitmap source/declaration selections are explicit; the `flighthq-surface` crate is generated from those selections and must not be edited by hand. This is the one temporary crate-naming exception: the target's `compatibilityForCrate` marker must match inventory's canonical `flighthq-bitmap`, and generation rejects any disposition that would enable an automatic bitmap candidate while the compatibility-generated definitions exist. Migrate by generating the selection as `flighthq-bitmap`, rewiring the wasm facade, and retaining `flighthq-surface` only as a thin re-export adapter if compatibility requires it.
- This bitmap facade is the only package planned as a standalone wasm package.
- Selected host packages, build tools such as `tool-capture`, and `*-dom` packages may be host-bound or excluded by explicit policy.
- All other upstream packages enter generation by default. A failure must appear as a source, compile, or dependency blocker in the generation report.
- Generated crates are ordinary native-capable Rust crates. Browser behavior belongs behind generated backend seams or cultivated host adapters.
- Improve shared IR and emitter architecture. Never fix a candidate by directly editing `generated/`.

The pinned Flight revision is `cad72aa3ea4e6e76a050918a403dcb10efdfcb0d`, matching `flight-hx` main at `27f4d0ee7ae04d64e66c996f99a0f270b09e12be`. The generator began from cultivated `flight-hx` revision `390a890c542f135278d89eee83f4f54e8fdbfd72`; subsequent compiler work should continue to use the newer `flight-hx` generator as its reference.

The pinned Flight commit is no longer advertised as a direct ref by the configured submodule remote. In a fresh clone, `git fetch origin cad72aa3ea4e6e76a050918a403dcb10efdfcb0d` may fail; fetch the remote's full reachable history (`git fetch --all`) before checking out the recorded gitlink.

## Current checkpoint

`reports/inventory.json`, `generated/manifest.json`, and `reports/generation.json` are current for the pinned upstream revision. That checkpoint summary is:

| State                           | Count |
| ------------------------------- | ----: |
| Inventoried packages            |   143 |
| Eligible automatic packages     |   137 |
| Packages reaching Rust emission |    43 |
| Compiled candidates             |    16 |
| Compile-blocked candidates      |     6 |
| Dependency-blocked candidates   |    19 |
| Source-blocked packages         |    94 |
| Source blockers                 |   455 |
| Promoted generated packages     |     2 |
| Cultivated packages             |     1 |
| Host-bound packages             |     4 |
| Excluded packages               |     1 |

Compiled candidates:

```text
adjustments color device flow haptics input keyboard lifecycle math platform screen signals spring textbidi
textsegment useragent
```

Compile-blocked candidates, ordered roughly from smallest to largest compiler frontier:

```text
accessibility clock importdiagnostics protocol timeline xml
```

Dependency-blocked candidates:

```text
application-gl bitmapfont-formats bitmaptext camera effects geometry lighting materials
motionpath particleemitter particles path-formats scene3d-gl spatial spritesheet textinput
textlayout textshaper-canvas velocity
```

`@flighthq/types` and `@flighthq/easing` are fully promoted executable generated targets. Promotion is intentionally stricter than candidate compilation and is closed over emitted Rust dependencies. Easing deliberately omits the upstream opt-in `enableEasingGuards.ts` development module, with its logging-boundary rationale and fingerprint recorded as a target exclusion.

## What passes 1–18 established

The generator now has these invariants. Preserve them with focused regression tests when changing adjacent code.

- Every upstream package is inventoried even if it cannot yet emit Rust.
- Export coverage, upstream fingerprints, generated provenance, dependencies, and compiler diagnostics are deterministic.
- Candidate compilation is dependency-aware; an unresolved prerequisite is classified separately rather than producing misleading downstream diagnostics.
- Structural records are canonicalized by resolved schema at module signatures.
- Imported nested-record provenance survives lowering.
- Anonymous records can project across nominal Rust records while evaluating owned values once.
- Ordered object fields preserve a non-Copy local that is consumed before a later field reads it, even across source aliases with one Rust representation.
- Rust consumption liveness preserves non-Copy values across sequential statements, loop back-edges, and reused switch discriminants without cloning comparison-only reads.
- Receiver-returning collection mutation preserves JavaScript expression results for both addressable and temporary Rust collections.
- Reused string parameters receive one UTF-16 code-unit view shared by `.length` and `codePointAt`; astral and trailing-surrogate behavior is compile-and-runtime tested.
- Nullable ternaries join as `Option` per branch, contextually non-null indexed reads unwrap matching option elements, computed Copy `LazyLock` scalars dereference at use sites, and numeric `~` applies JavaScript `ToInt32` while nominal bitflag enums retain Rust `Not`.
- Function signature records remain canonical even when a shape occurs only once.
- Generic structural constructors use Rust turbofish syntax.
- A module-global canonical record never captures an unbound generic type parameter.
- Cross-module structural helper calls can inline and project optional fields.
- Mutable/read argument overlap is sequenced through owned temporaries.
- Shared typed-array/byte-buffer views, regex captures, string splitting/parsing, exhaustive switches, interval handles, and selected collection operations lower to native Rust.
- Statically absent native host branches are pruned before their web-only expressions block emission.
- Dynamic host reads, writes, and calls route through the explicit opaque host boundary.
- Automatic candidates resolve package dependencies only to automatic Cargo identities; a differently named partial cultivated crate cannot leak into that graph. Public Flight subpaths such as `/contract` resolve type declarations from the owning package source, including indexed-access return types.
- A dynamically supplied typed task becomes a rejected `HostUnavailable` task without requiring `Default` or fabricating an output value; executable runtime tests cover a non-`Default` output type.
- No generated Promise path fabricates a default task. Web-default task paths that native code replaces are omitted only through explicit partial host targets until configured host-task placeholders land.
- Discriminated open-interface families are discovered from a package-wide semantic type catalog.
- A family is widened only when descendants explicitly redeclare `kind` and every added field is safely default-materializable.
- Open-family constructors and projections use struct update defaults. This promoted the lighting `Light` family without corrupting recursive node hierarchies or callback-bearing adjustment families.
- `application`, `lifecycle`, `input`, `keyboard`, `haptics`, `power`, `platform`, `device`, and `screen` provide generated seams that can be linked by a cultivated native host.
- Configured upstream test files are harvested from their TypeScript ASTs into candidate unit tests. Generation runs every translated assertion before it increments case coverage, and only completely translated files increment file coverage; all other in-scope files remain fingerprinted unsupported report entries.
- Global `Promise<T>` types lower to target-neutral `task<T>` IR, while source-declared `Promise` names remain nominal. The report partitions all 225 task constructions and 173 async scopes; 19 constructions/13 non-opaque scopes execute and 206/160 remain explicitly unsupported without default bodies.
- Imported Flight types stay nominal when their local names collide with browser globals such as `Image`; unshadowed platform types still lower through the explicit host boundary.

Primary implementation entry points:

- `tools/generator/src/emit/core.ts`
  - `collectPackageSemanticTypes` creates the package-wide semantic catalog.
  - The package emission path merges this catalog before explicit semantic mappings.
- `tools/generator/src/conformance/harvest.ts`
  - Discovers package test files, translates the admitted pure assertion subset, fingerprints unsupported files, and produces the generated candidate test modules plus report data.
- `tools/generator/src/emit/rust.ts`
  - `EmitContext.openInterfaceFields` records safe widened layouts.
  - `registerOpenInterfaceFamilies` discovers and validates open families.
  - `emitStructConstructorType` handles constructor syntax, including generic turbofish.
  - Expression, call-argument, structural projection, and object-construction lowering are concentrated in this file.
- `tests/generator/rust-emitter.test.ts`
  - Contains compile-backed emitter regressions. Add the smallest TypeScript fixture that exposes a shared defect, assert meaningful generated Rust when useful, and compile it with `rustc`.

## Native host canary

`crates/flighthq-host-winit` is cultivated host code, not an automatically translated upstream package. It currently proves that the generated backend seams can coexist and link.

- `NativeHostBackends` bundles application, lifecycle, keyboard, haptics, power, screen, platform, and device backends. Generated partial application/input/power/screen targets and a distinctly named host signal-constructor target avoid mixing duplicate automatic package identities in this graph.
- `install_native_host` installs the bundle and creates the generated input manager.
- The canary has compile/link assertions and one integration test.
- It does not yet own a real winit event loop or renderer.

The canary is a nested Cargo workspace on purpose. It must not become a root workspace member: the promoted graph and candidate graph both contain a package named `flighthq-types`, and Cargo rejects both package identities in one workspace.

## Resume loop

Use this loop for every pass:

1. Read `reports/generation.json` and rank blockers by diagnostic count, shared architectural value, and downstream dependents.
2. Inspect the upstream TypeScript and generated Rust for one representative package.
3. Reduce the failure to the smallest compile-backed fixture in `tests/generator/rust-emitter.test.ts`.
4. Implement the shared IR/emitter behavior.
5. Run the focused regression.
6. Regenerate the whole matrix. Never patch the regenerated crate.
7. Compare report counts and package state against the previous commit. A package may move from source-blocked to compile-blocked before it compiles; that is real progress.
8. Verify no previously compiled candidate regressed.
9. Run the full checks and commit a coherent checkpoint.

Useful report queries:

```sh
jq '.summary' reports/generation.json

jq -r '.automaticPackages
  | group_by(.candidate.status)[]
  | "\(.[0].candidate.status) (\(length)): " + (map(.package) | join(", "))' \
  reports/generation.json

jq -r '.automaticPackages[]
  | select(.candidate.status == "compile-blocked")
  | [.package, (.candidate.compileDiagnostics | length),
     ([.candidate.compileDiagnostics[].code]
       | group_by(.)
       | map("\(.[0])×\(length)")
       | join(", "))]
  | @tsv' reports/generation.json

jq '.automaticPackages[]
  | select(.package == "@flighthq/app")
  | {status, blockers, candidate}' reports/generation.json
```

Generation and validation commands:

```sh
npm run generate
npm run check
cargo test --workspace
npm run test:host-winit
git diff --check
```

In the current Quimby image, `rustup` has no global default. Commands that transitively invoke `rustc` need the pinned temporary toolchain:

```sh
env \
  PATH="/tmp/flight-rs-cargo/bin:$PATH" \
  RUSTUP_HOME=/tmp/flight-rs-rustup \
  CARGO_HOME=/tmp/flight-rs-cargo \
  CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=/tmp/flight-rs-rustup/toolchains/1.97.1-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/rust-lld \
  npm run check
```

Use the same environment for direct `cargo` or `rustc` checks if needed. A direct candidate `cargo check` may create `generated/candidates/Cargo.lock`; do not commit that lockfile. Full regeneration removes it. Keep `crates/flighthq-host-winit/Cargo.lock`, which belongs to the cultivated nested workspace.

## Next three passes

### Pass 19: app, socket, and small shared APIs

Start with `@flighthq/app`, whose five diagnostics currently expose:

- a `.then` call on an inert Promise expression lowered as `()`;
- one owned/borrowed mismatch;
- a `String: Pattern` mismatch;
- missing string `.slice`;
- missing string `.length`.

The correct architecture is a typed Promise-chain expression whose callbacks remain type-checked and capturable even when the web backend result is inert. Do not execute a web callback in native placeholder code. String lowering should distinguish byte operations from TypeScript UTF-16/code-point expectations; use a helper when Rust slicing would panic or change semantics.

Then revisit `@flighthq/socket`. Its five diagnostics are one invalid callback `.unwrap()` on a mutex-backed closure and four ownership/type mismatches. Generalize nullable callback access rather than special-casing socket.

Acceptance:

- add compile-backed tests for chained inert Promises and each new string/callback rule;
- move app and preferably socket to `candidate.status == "compiled"`;
- preserve all 28 currently compiled candidates.

### Pass 20: node generic identity and runtime extensions

`@flighthq/node` now reaches compilation, which is an important milestone, but has 187 diagnostics:

- 37 `E0107` generic-arity failures;
- 76 missing-field `E0609` failures, mostly TypeScript runtime extensions projected onto `EntityRuntime`;
- 25 inference `E0283` failures;
- 41 type mismatches, plus a small remainder.

The checkpoint report counted `@flighthq/entity` as compiled even though `EntityRuntimeKey` reads, writes, deletes, membership tests, and computed initializers had been erased into no-ops, constants, or a panic. Pass 19 replaced those approximations with explicit blockers. Pass 20 now lowers statically typed entity operations to a shared native slot; receivers outside the closed entity family retain the blocker.

The implementation does not widen `EntityRuntime` from a handwritten list. A package-visible source catalog preserves generic parameters through aliases and applications, then builds a generated aggregate handle. Compatible extension fields stay flat. Reused field names with incompatible types, plus nested structural fields whose nominal provenance must survive module boundaries, use source-named typed slots inside the same handle. Runtime aliases retain declared arity through zero-storage associated-type markers that normalize to the same handle. A dynamic/opaque map is never used, so node and backend-specific code retain field types.

Field updates bind the runtime and value before taking the aggregate lock once, avoiding self-deadlock when the right-hand side reads the same runtime. Entity object spread creates a fresh identity and slot, copies slot presence into it, and retains the shared runtime handle, so deleting the symbol from the source does not clear the copy. The full `@flighthq/types` family provides the canonical aggregate to automatic candidates. Generic-dependent extensions use source-generated generic slot records in a `TypeId`-keyed side table. Concrete applications recover an `Arc<Mutex<Slot>>` through a checked `Any` downcast; required non-defaultable fields remain explicit uninitialized `Option` slots until written. This preserves field types without opaque values, unsafe initialization, or invented observable defaults. An access whose generic application cannot be recovered statically remains source-blocked.

Promoted targets that emit an entity field or `FlightEntity` implementation must carry `EntityRuntime`, `EntityRuntimeStorage`, and `FlightEntity` at the crate root, whether defined locally or imported. Generation checks that invariant before writing a target, and focused emitter fixtures compile a promoted root independently with `rustc`.

Node has 23 direct dependents, so architectural fixes here unlock much of the render/scene graph. Treat string `.includes` and `as_ref` diagnostics as smaller follow-on clusters after the generic/runtime representation is sound.

Acceptance:

- generic aliases retain declared arity and substitutions across module boundaries;
- `EntityRuntime` extensions are generated from source declarations, not a handwritten field list;
- diagnostic clusters fall materially even if node does not compile in one pass;
- no unsafe defaulting is introduced to make recursive records compile.

### Pass 21: tagged open families and real winit events

The safe default-materializable open-family strategy deliberately rejects callback-bearing or recursive families. Implement a tagged representation for families that cannot be widened safely:

```text
common base fields + discriminant + typed payload variant
```

Construction, narrowing, property projection, spreads, and equality must preserve the TypeScript discriminant semantics. Do not use zeroed memory, fake callbacks, or opaque host values for substrate-neutral data.

`@flighthq/materials` is the main proving ground: 90 of its 153 diagnostics are missing fields on `SurfaceMaterial`, followed by 42 type mismatches. Use a small fixture before applying the representation to that package.

In parallel within this pass, extend the cultivated host-winit canary with a concrete event translation layer only after the relevant generated event types are stable. Keep platform event-loop ownership handwritten and generated input/lifecycle state transitions mechanical. Rendering ownership remains behind the node/material/skeleton/render frontier.

Acceptance:

- a non-defaultable discriminated family compiles in a focused fixture;
- material diagnostics fall without regressing lighting;
- at least one concrete winit event maps into generated input or lifecycle state;
- surface remains the only standalone wasm package.

## Broader blocker frontier

After passes 19–21, prioritize shared capabilities rather than raw diagnostic count:

- `@flighthq/log` has seven direct dependents and is source-blocked by a two-spread structural object plus package export coverage.
- `@flighthq/image` has eleven direct dependents and is blocked by host `new ImageData` plus missing exports.
- `@flighthq/text` has eight direct dependents and needs ordered multiple-object-spread lowering.
- `@flighthq/shape` has seven direct dependents and needs spread/structural generic handling.
- Portable async packages such as filesystem and notification need contextual output recovery and later task operations; image-codec is the current non-opaque straight-line execution canary.
- Dynamic WebGL/WebGPU object literals should eventually use typed backend-capability IR rather than giant opaque records.

The package-level “missing exports” blocker often shrinks automatically after the first emission blocker is fixed. Do not build a broad re-export workaround until regeneration proves the barrel/export graph itself is incomplete.

## Pass 27 design checkpoint

The implementation contract is [Future/task IR design](future-task-ir.md). Send and review that design before task-lowering code. Its critical invariants are:

- every async scope is portable executable, an explicitly configured host placeholder, or source-blocked;
- `Promise<T>` becomes target-neutral task IR with one compiler-runtime type across candidate and promoted graphs;
- host placeholders never execute browser bodies or chained callbacks and never fabricate default output;
- portable tasks preserve eager pre-suspension work, mandatory await yielding, shared one-time execution, typed rejection, and owned state across suspension;
- unsupported Promise composition, detached work, and async iteration remain source-scoped blockers;
- async-task disposition is diagnostic reporting, while exports, portable opacity, upstream conformance, and fully promoted packages remain the four parity metrics.

At the design baseline, eligible generated packages contained 162 async scopes and 190 awaits across 40 non-test sources. At the current upstream pin the inventory contains 173 scopes and 205 awaits. The old report exposed only seven await blockers in six packages because top-level async declarations were body-erased. Removing that silent path was the first code gate; do not interpret the resulting candidate-status correction as a behavioral regression without checking whether the old candidate ever executed its source body.

Stage 1 removed that erasure path. Stage 2 adds the canonical generated `flighthq-runtime`, typed ready/reject and straight-line async/await lowering, deterministic scheduler installation in generated tests, and construction-wide reporting. Stage 2b now propagates declared `Promise<T>` output context through ordinary returns, concise arrows, task factories, and reserved composition nodes; unrecovered construction outputs fell from 18 to 10 while the eight platform-valued async scope outputs remain honestly dynamic. The current partition is 19 executable + 0 host placeholder + 206 unsupported constructions and 13 + 0 + 160 scopes. Ten recovered-output scopes remain blocked because their source still requires `OpaqueHostValue`; 66 of 1,549 emitted automatic sources currently require opaque host values. Stage 3 adds configured host-task boundaries and typed platform capabilities; Stage 4 owns composition and must account for `catch_unwind` interactions in non-async task-returning functions.

## Checkpoint discipline

- Generated output is reviewed as evidence, never edited as input.
- Update `agents/status.md`, `agents/architecture.md`, and this playbook when counts or architecture change materially.
- Commit generator code, tests, regenerated outputs, reports, and documentation together.
- Keep commit messages to one line without trailers.
- Confirm `git status --short` is clean after committing.
- Record the exact passing commands and package-count delta in durable status before handing off.
