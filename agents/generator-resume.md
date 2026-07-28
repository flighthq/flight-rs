# Generator Resume Playbook

This is the durable handoff for continuing the mechanical Flight TypeScript-to-Rust port. It describes the repository at generator checkpoint `294cf03` (`Promote screen lighting and native host canary`). Resume from repository `HEAD`; do not reconstruct earlier passes or edit generated Rust by hand.

## Goal and policy

The repository is a compiler project, not a collection of manually ported crates.

- A small, explicit set of packages may be cultivated by hand.
- `packages/surface-rs` and the configured surface source/declaration selections are cultivated. The
  `flighthq-surface` crate is generated from those selections and must not be edited by hand.
- Surface is the only package planned as a standalone wasm package.
- Selected host packages, build tools such as `tool-capture`, and `*-dom` packages may be host-bound or excluded by explicit policy.
- All other upstream packages enter generation by default. A failure must appear as a source, compile, or dependency blocker in the generation report.
- Generated crates are ordinary native-capable Rust crates. Browser behavior belongs behind generated backend seams or cultivated host adapters.
- Improve shared IR and emitter architecture. Never fix a candidate by directly editing `generated/`.

The pinned Flight revision is `5d24729f7360475e28a105ae0caeeaa2e1328260`. The generator began from cultivated `flight-hx` revision `390a890c542f135278d89eee83f4f54e8fdbfd72`.

## Current checkpoint

`reports/generation.json` is the machine-readable source of truth. At `294cf03` its summary is:

| State                           | Count |
| ------------------------------- | ----: |
| Inventoried packages            |   131 |
| Eligible automatic packages     |   125 |
| Packages reaching Rust emission |    70 |
| Compiled candidates             |    28 |
| Compile-blocked candidates      |    22 |
| Dependency-blocked candidates   |    19 |
| Source-blocked packages         |    55 |
| Source blockers                 |   294 |
| Promoted generated packages     |     1 |
| Cultivated packages             |     1 |
| Host-bound packages             |     4 |
| Excluded packages               |     1 |

Compiled candidates:

```text
adjustments application bitmapfont camera camera2d color device entity flow
geometry haptics input keyboard lifecycle lighting math motionpath path platform
power screen signals spatial spring textsegment texture types useragent
```

Compile-blocked candidates, ordered roughly from smallest to largest compiler frontier:

```text
app(5) socket(5) clock(7) connectivity(10) accessibility(12) assets(14)
timeline(16) protocol(18) textbidi(19) animation(27) font(27) media(32)
xml(33) audio(36) clip(37) effects(56) path-formats(62) snapshot(66)
particles(118) mesh(130) materials(153) node(187)
```

Dependency-blocked candidates:

```text
bitmapfont-formats bitmaptext displayobject glyphatlas movieclip particleemitter
picking render scene skeleton3d sprite spritesheet textinput textlayout
textshaper-canvas textureatlas textureatlas-formats tileset velocity
```

`@flighthq/easing` is the first fully promoted executable generated target. Promotion is intentionally stricter than candidate compilation.

## What passes 1–18 established

The generator now has these invariants. Preserve them with focused regression tests when changing adjacent code.

- Every upstream package is inventoried even if it cannot yet emit Rust.
- Export coverage, upstream fingerprints, generated provenance, dependencies, and compiler diagnostics are deterministic.
- Candidate compilation is dependency-aware; an unresolved prerequisite is classified separately rather than producing misleading downstream diagnostics.
- Structural records are canonicalized by resolved schema at module signatures.
- Imported nested-record provenance survives lowering.
- Anonymous records can project across nominal Rust records while evaluating owned values once.
- Function signature records remain canonical even when a shape occurs only once.
- Generic structural constructors use Rust turbofish syntax.
- A module-global canonical record never captures an unbound generic type parameter.
- Cross-module structural helper calls can inline and project optional fields.
- Mutable/read argument overlap is sequenced through owned temporaries.
- Shared typed-array/byte-buffer views, regex captures, string splitting/parsing, exhaustive switches, interval handles, and selected collection operations lower to native Rust.
- Statically absent native host branches are pruned before their web-only expressions block emission.
- Dynamic host reads, writes, and calls route through the explicit opaque host boundary.
- Web-default `Promise<T>` paths can lower to typed inert placeholders. This is only valid for backend code that a native host replaces before use; it is not a portable async runtime.
- Discriminated open-interface families are discovered from a package-wide semantic type catalog.
- A family is widened only when descendants explicitly redeclare `kind` and every added field is safely default-materializable.
- Open-family constructors and projections use struct update defaults. This promoted the lighting `Light` family without corrupting recursive node hierarchies or callback-bearing adjustment families.
- `application`, `lifecycle`, `input`, `keyboard`, `haptics`, `power`, `platform`, `device`, and `screen` provide generated seams that can be linked by a cultivated native host.

Primary implementation entry points:

- `tools/generator/src/emit/core.ts`
  - `collectPackageSemanticTypes` creates the package-wide semantic catalog.
  - The package emission path merges this catalog before explicit semantic mappings.
- `tools/generator/src/emit/rust.ts`
  - `EmitContext.openInterfaceFields` records safe widened layouts.
  - `registerOpenInterfaceFamilies` discovers and validates open families.
  - `emitStructConstructorType` handles constructor syntax, including generic turbofish.
  - Expression, call-argument, structural projection, and object-construction lowering are concentrated in this file.
- `tests/generator/rust-emitter.test.ts`
  - Contains compile-backed emitter regressions. Add the smallest TypeScript fixture that exposes a shared defect, assert meaningful generated Rust when useful, and compile it with `rustc`.

## Native host canary

`crates/flighthq-host-winit` is cultivated host code, not an automatically translated upstream package. It currently proves that the generated backend seams can coexist and link.

- `NativeHostBackends` bundles application, lifecycle, keyboard, haptics, power, screen, platform, and device backends.
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

The checkpoint report counted `@flighthq/entity` as compiled even though `EntityRuntimeKey` reads, writes,
deletes, membership tests, and computed initializers had been erased into no-ops, constants, or a panic. Pass 19
replaced those approximations with explicit blockers. Pass 20 now lowers statically typed entity operations to a
shared native slot; receivers outside the closed entity family retain the blocker.

The implementation does not widen `EntityRuntime` from a handwritten list. A package-visible source catalog
preserves generic parameters through aliases and applications, then builds a generated aggregate handle.
Compatible extension fields stay flat. Reused field names with incompatible types, plus nested structural fields
whose nominal provenance must survive module boundaries, use source-named typed slots inside the same handle.
Runtime aliases retain declared arity through zero-storage associated-type markers that normalize to the same
handle. A dynamic/opaque map is never used, so node and backend-specific code retain field types.

Field updates bind the runtime and value before taking the aggregate lock once, avoiding self-deadlock when the
right-hand side reads the same runtime. Entity object spread copies slot presence into a new slot while retaining
the shared runtime handle, so deleting the symbol from the source does not clear the copy. A package-local
extension that would add storage to an imported aggregate is rejected explicitly; it must first join the
configured canonical `@flighthq/types` runtime family.

The checked-in generation report predates this implementation. Regenerate the full matrix before claiming
diagnostic or package-count movement; the focused emitter fixtures are compile-backed when `rustc` is available.

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
- Portable async packages such as filesystem, notification, and image-codec need a real Future/task IR; do not expand the inert backend Promise placeholder into general async semantics.
- Dynamic WebGL/WebGPU object literals should eventually use typed backend-capability IR rather than giant opaque records.

The package-level “missing exports” blocker often shrinks automatically after the first emission blocker is fixed. Do not build a broad re-export workaround until regeneration proves the barrel/export graph itself is incomplete.

## Checkpoint discipline

- Generated output is reviewed as evidence, never edited as input.
- Update `agents/status.md`, `agents/architecture.md`, and this playbook when counts or architecture change materially.
- Commit generator code, tests, regenerated outputs, reports, and documentation together.
- Keep commit messages to one line without trailers.
- Confirm `git status --short` is clean after committing.
- Record the exact passing commands and package-count delta in durable status before handing off.
