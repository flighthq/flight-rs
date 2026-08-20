# Future/task IR design

## Status and scope

This is the design and implementation checkpoint for Pass 27. Stages 1, 2, and 2b plus the first Stage 4 collection-composition slice are implemented; callback composition, configured host-task boundaries, detach, and async iteration remain later stages.

Stage 1 established the honesty boundary: global `Promise<T>` references lower to target-neutral `task<T>` IR, every eligible async scope has a stable source and lexical identity, and Rust emission rejects unsupported tasks instead of replacing their bodies with defaults. Stage 2 generates one canonical `flighthq-runtime`, lowers typed ready/rejected tasks and straight-line async functions, and installs a deterministic scheduler in generated test targets. Stage 2b propagates declared task output types into unannotated task factories, including concise arrows, generic rejection paths, and reserved composition nodes. The first Stage 4 slice lowers homogeneous `Promise.all` collections to explicit `taskAll` IR and a canonical Rust join that polls every unsettled observation handle in each turn, preserves input order, and propagates a later-slot error without waiting for an earlier pending slot. Empty joins settle successfully. Mixed task/value assimilation, tuple identity, and `allSettled` remain explicit blockers.

At the current upstream pin, the report partitions all 225 task constructions as 19 portable executable + 0 configured host placeholder + 206 unsupported, and all 173 async scopes as 13 + 0 + 160. The executable count is deliberately flat: the one recovered homogeneous upstream join is in `permission.ts`, whose unrelated task-aware `try`/`catch` still prevents the containing module from emitting. The other three `taskAll` sites expose unrecovered outputs instead of retaining a generic Stage 4 reservation. Contextual recovery reduced the pre-composition unrecovered construction frontier from 18 to 10 without changing opacity: those ten are the eight constructions corresponding to browser/platform-valued async scopes plus two WebGPU device factories. Ten otherwise typed scopes remain explicitly blocked because their source still requires `OpaqueHostValue`; 66 of 1,549 emitted automatic sources require opaque host values instead of widening erasure. When a typed task is read from the dynamic host boundary, the runtime returns a typed `HostUnavailable` rejection without imposing `Default` on or fabricating the task output.

The original design baseline's pre-bound source measurement found all 162 outputs compatible with `Clone + Send + 'static`: 83 scopes carried explicit closed `Promise<T>` annotations and 79 were unannotated. Normalized IR was stricter than that syntactic split: 85 outputs contained dynamic data and only 77 were genuinely recovered. Dynamic outputs remain unsupported rather than becoming executable `FlightTask<OpaqueHostValue>`; the current generated report is authoritative for the evolving disposition counts.

The pinned upstream tree has async syntax in 25 eligible generated packages, spanning 45 non-test source files, 173 async scopes, 205 `await` expressions, and three `for await` loops. Before Stage 1, the generation report exposed only seven `await` emission blockers in six packages because the Rust emitter replaced top-level async function bodies with `Default::default()`. That replacement could make a candidate compile without running its source body. `@flighthq/screen`, for example, was reported compiled even though both of its async bodies were erased.

Pass 27 is an honesty boundary as well as a lowering pass:

- every eligible async scope must become an executable portable task, an explicitly configured host placeholder, or a source-scoped blocker;
- no async body may disappear merely because its declaration carries the `async` modifier;
- the inert web-backend Promise placeholder must not become the implementation of portable async;
- portable task support must introduce no `OpaqueHostValue`, invented output, panic stub, unsafe initialization, or implicit executor assumption;
- candidate compilation remains a diagnostic. Export coverage, portable opaque density, upstream conformance, and fully promoted packages remain the parity metrics.

The six-package await frontier is useful prioritization, not the acceptance metric. A package may move only when all of its async scopes are represented honestly.

## Semantic split

TypeScript uses `Promise<T>` for both portable orchestration and browser-only default backends. Native Rust needs two different execution dispositions even though both positions have the same source type.

### Portable task

A portable task contains source-derived control flow that native code executes. It can await installed backend operations, compose other portable tasks, reject, recover, and return a typed value. Its body must be emitted and tested.

Examples include `decodeImage`, `encodeImage`, `findFiles`, `resolveScenesWithOptions`, and the format loaders in `@flighthq/scene-resources`. These functions are portable even when an installed backend eventually performs host I/O.

### Host placeholder

A host placeholder represents an async callback or method inside a web-only default backend that native code must replace before use. Its browser body does not execute, and fulfillment or rejection callbacks chained inside that browser implementation do not execute on native merely to make the module compile.

Host placeholders are not default values. Calling one produces a typed, source-identified `HostUnavailable` task outcome. That outcome propagates through generated chains without invoking their callbacks. An installed native backend returns ordinary portable tasks instead.

Classification is explicit and source-derived:

1. A declared `host-backend` package policy may classify the package's backend implementation scopes.
2. A substrate-neutral package uses a configured source/declaration boundary with a reason, such as a web backend factory whose returned object owns async browser methods.
3. Every other async scope is portable by default.

Function names such as `createWeb*`, use of a dynamic value, or the presence of a browser global are not sufficient classification rules. A stable selector comprises package, source, lexical declaration/property path, and the selected AST fingerprint. Nested async methods inherit a configured enclosing host boundary. Drift that prevents the selector from matching is a blocker rather than a silent reclassification.

An async portable wrapper does not become host-bound because it calls an abstract backend. The wrapper executes and awaits the task returned by whichever backend is installed.

## Target-neutral IR

`Promise` must stop being a magic named type interpreted only by the Rust emitter. The target-neutral type model gains a task kind:

```ts
type IrType =
  | { kind: 'task'; output: IrType }
  | /* existing kinds */;
```

All resolved global `Promise<T>` references lower to `task<T>`. A lexically shadowed identifier named `Promise` remains an ordinary source symbol.

Function and closure bodies carry an execution form rather than an optional boolean:

```ts
type IrFunctionExecution =
  | { kind: 'sync'; body: IrStatement[] }
  | {
      kind: 'portableTask';
      body: IrStatement[];
      origin: SourceOrigin;
    }
  | {
      capability: string;
      kind: 'hostTaskPlaceholder';
      origin: SourceOrigin;
      reason: string;
    };
```

The placeholder form deliberately has no executable browser body. The lowering report still fingerprints that body and associates it with the explicit boundary, so drift remains visible.

Task operations are explicit IR nodes rather than property-name special cases in Rust emission:

- `taskReady` and `taskReject` for resolved and rejected tasks;
- `await` for a suspension and typed output projection;
- `taskThen`, `taskCatch`, and `taskFinally` for source Promise chains;
- `taskAll` and `taskAllSettled` for collection joins;
- `taskDetach` for a deliberately unobserved task, including `void task`;
- async `forOf` with a typed async-iterator source.

The first implementation need not support every node. An unsupported node remains in the IR long enough to produce a source-scoped diagnostic with its origin. It must never fall back to a synchronous call, unit, a default task, or an ignored callback.

Return types are normalized once. An `async function f(): Promise<T>` has a portable-task execution body whose output is `T`; it does not contain a nested `task<task<T>>`. Returning another task adopts its outcome, matching Promise resolution. The same flattening rule applies to `then` callbacks.

Every nested task expression receives a stable origin. Named declarations use their lexical path. Object methods, returned closures, and anonymous callbacks use the nearest named path plus their AST fingerprint, not an array ordinal that shifts when unrelated code is inserted.

## Canonical Rust runtime identity

`Promise<T>` is currently emitted separately into every generated crate. That cannot represent portable tasks across package boundaries: two structurally identical Rust future wrappers from different crates are nominally different types.

Pass 27 introduces one compiler-runtime crate, `flighthq-runtime`, generated deterministically from a maintained generator template under `generated/crates`. It is an internal support crate, not an upstream Flight package and not part of the 125-package promotion denominator. Candidate, promoted, host, and wasm graphs resolve the same path, following the existing dependency-closed identity rule.

Generated crates that contain task types or values depend on that runtime and refer to one canonical set of types:

```rust
pub struct FlightTask<T> { /* shared observation handle */ }

#[derive(Clone, Debug)]
pub enum FlightTaskError {
    Rejection(FlightRejection),
    HostUnavailable(FlightHostUnavailable),
    RuntimeUnavailable(FlightRuntimeUnavailable),
}

pub type FlightTaskOutcome<T> = Result<T, FlightTaskError>;
```

`FlightTask<T>` implements `Future<Output = FlightTaskOutcome<T>>` when `T` satisfies the generated task-output bounds. It is a cloneable observation handle over a single execution and a cached outcome, because a TypeScript Promise may have multiple observers but runs once. The task driver owns and polls the inner future; observers never poll it while holding the shared-state lock.

Multiple observers require a repeatable output. The first implementation requires task outputs to be `Clone + Send + 'static`, matching the existing generated callback boundary. A task output that cannot satisfy those bounds is a source blocker until the ownership representation is generalized. It is not boxed as opaque state.

Only `Rejection` is catchable by source `.catch` or `try`/`catch`. `HostUnavailable` and `RuntimeUnavailable` are compiler/runtime boundary failures and propagate without executing source recovery callbacks. This preserves the rule that native placeholder code must not run browser callback bodies.

`FlightRejection` is a typed portable representation for the rejection shapes the lowerer supports, initially null, boolean, number, string, and `Error` name/message data. Throwing or inspecting an arbitrary JavaScript object is unsupported until it has a source-derived tagged representation. `OpaqueHostValue` is not an error channel.

The runtime template is generator input and its output is disposable. Generated package files do not receive handwritten task implementations.

## Scheduling and Promise ordering

Rust futures are normally lazy; JavaScript async functions start synchronously and always suspend at `await`, even when the awaited Promise is already settled. A naive `Box::pin(async move { ... })` changes observable ordering and drops unobserved work. Pass 27 must preserve these properties explicitly:

1. Calling a portable async function evaluates arguments, defaults, and its body through the first suspension before returning.
2. Every source `await` schedules its continuation after the current turn. A ready task does not let the continuation run inline.
3. One source task executes once, and cloned handles observe the cached result.
4. Dropping all observation handles does not cancel source work merely because JavaScript would have had no observer.
5. `taskDetach` keeps the task alive and reports scheduler absence; it is not emitted as `let _ = task`.

The runtime therefore separates a driver from observation handles. `FlightTask::start` performs the initial poll through the first explicit task-yield boundary, then gives the pending driver to a `FlightTaskScheduler` seam. The driver retains itself until settlement and wakes all observers. Generated native tests install a deterministic current-thread scheduler. Cultivated native and wasm hosts install their event-loop bridge; the compiler runtime does not choose Tokio, async-std, a worker thread, or a wasm executor for them.

If a task reaches a suspension without an installed scheduler, it settles as `RuntimeUnavailable` with source identity. It does not hang, continue inline, or fabricate an output. A task that completes before suspension needs no scheduler, but observer notification still observes the task-yield rule.

The scheduler seam is a target capability, not a browser backend. Its API and error are portable and contain no host object. Host-specific futures enter through installed backend functions and are converted to the canonical `FlightTask` at that seam.

## Ownership across suspension

An emitted task cannot retain a borrowed Rust parameter across suspension unless its lifetime is part of every downstream callback and interface type. The generator instead materializes the source value ownership needed by the task at the call boundary:

- strings, arrays, records, entities, callbacks, and shared handles are cloned or moved once before task construction;
- default parameter expressions are evaluated at call time, before task execution starts;
- receiver and argument evaluation order remains left to right;
- a mutable source object that is intentionally shared keeps its generated shared identity rather than receiving a deep copy;
- local values live in the generated task state across suspension;
- a raw borrow, lock guard, or non-`Send` value that would cross suspension is a source blocker.

No generated mutex guard may be held across `await`. The emitter evaluates and stores the required value, releases the guard, and only then suspends. This is especially important for entity-runtime fields and callback locks.

The Rust public API returns owned `FlightTask<T>` handles. It does not expose synthetic lifetimes derived from temporary input borrows.

## Control flow and errors

Inside a portable task, a source `await task` projects the task outcome:

- success yields the typed value;
- a source rejection propagates unless enclosed by source recovery;
- host/runtime boundary failures always propagate;
- an ordinary non-task value is normalized to `taskReady` and still observes the mandatory yield;
- a structural thenable is assimilated only when the type checker and IR recover its typed continuation contract; a dynamic or unresolved thenable is a source blocker.

`try`/`catch`/`finally` needs task-aware control flow. The emitter cannot use panic catching as Promise rejection. It emits an inner task-result region, matches only `FlightTaskError::Rejection`, binds the typed portable rejection where supported, and runs `finally` on success, rejection, early return, and boundary failure. Arbitrary catch-value reflection remains unsupported.

`throw` in a portable task creates a rejection. Genuine generator invariant failures may panic during generation or tests; emitted source behavior may not use panic as an async rejection substitute.

Promise combinators preserve source ordering:

- `then` runs only after successful settlement and flattens a returned task;
- `catch` runs only for a source rejection;
- `finally` observes no value, runs once, and preserves the prior outcome unless it rejects;
- `all` preserves input order and short-circuits on the first rejection;
- `allSettled` waits for every input and emits a source-derived tagged settlement record;
- placeholder boundary failures propagate through all combinators without invoking browser callbacks.

The implemented `taskAll` subset accepts a homogeneous `Vec<FlightTask<T>>` and returns `FlightTask<Vec<T>>`. It does not lower to a sequential async loop: the aggregate future polls every unsettled handle on every wake, stores successful values by source index, and settles immediately when any polled handle reports an error. Each handle poll releases its shared-state lock before returning `Pending`, and the aggregate retains no guard across suspension. Dropping the aggregate after an error drops observation handles only; the input task drivers retain themselves under the existing no-cancellation rule.

Async iteration requires a canonical `FlightAsyncIterator<T>` protocol that produces `FlightTask<Option<T>>`. Until that protocol and early-break disposal are implemented, each `for await` remains an explicit blocker. It must not lower to synchronous `IntoIterator`.

## Reporting

Generation adds an `asyncTasks` section derived from the lowered IR, with package and aggregate counts for:

- every task construction, including async scopes, ready/reject, composition, and joins;
- eligible async scopes;
- executable portable scopes;
- configured host-placeholder scopes;
- unsupported scopes grouped by stable reason;
- `await`, Promise-combinator, detached-task, and async-iteration operations;
- sources that previously would have used erased async bodies.

The invariant is:

```text
eligible async scopes = portable executable + host placeholder + unsupported
eligible task constructions = portable executable + host placeholder + unsupported
```

Every entry carries package, source, lexical identity, and fingerprint. The report fails generation if an async scope has no disposition or if a configured host selector no longer matches.

These are diagnostic coverage fields, not a fifth headline parity metric. Pass and promotion handoffs continue to lead with:

1. generated / eligible upstream exports;
2. portable opaque sources / portable emitted sources;
3. translated-and-passing upstream test files / all upstream test files;
4. fully promoted upstream packages / 125.

Async upstream tests remain unsupported by the conformance harvester until the generated test target can install the deterministic task scheduler and await the canonical task outcome. They are reported rather than treated as passing because the source crate compiles.

## Implementation sequence

The code follows this design in separate reviewable increments:

1. **IR and honesty (implemented).** Add task types/execution forms and stable nested origins. Remove the async-body erasure path. Emit complete async-task inventory and blockers before changing package status.
2. **Canonical runtime and straight-line tasks (implemented).** Generate the shared runtime crate. Support typed ready/rejected tasks, async functions, returns, and `await`, with owned suspension state and scheduling-order runtime fixtures.
3. **Host boundary and callbacks.** Add explicit configured host-task boundaries and typed `HostUnavailable` outcomes. Make async callbacks and installed backend task signatures use the canonical identity.
4. **Composition and errors (in progress).** Homogeneous `taskAll` is implemented with concurrent observation, ordered output, empty-input success, and early error propagation. Add `then`, `catch`, `finally`, task flattening, `try`/`catch`/`finally`, mixed/tuple `all`, and `allSettled` without panic or opaque rejection data.
5. **Detached work and async iteration.** Add scheduler-owned detach and the typed async-iterator protocol, including cleanup on break and error.
6. **Package and conformance harvest.** Re-run the full frontier after each semantic increment. Extend generated conformance to the smallest upstream async files only after its scheduler harness is generated.

Later increments may be split, but they may not be reordered in a way that restores silent erasure or lets host placeholders count as portable execution.

## Acceptance gates

Pass 27 is complete only when all of the following hold:

- no `declaration.async ? Default::default()` or equivalent body-erasure path remains;
- every eligible async scope satisfies the reporting partition;
- global `Promise<T>` has target-neutral task IR and one canonical Rust identity across crate graphs;
- a generated compile-and-runtime fixture proves eager prefix execution, mandatory await yielding, nested await, one execution with multiple observers, and owned input survival after the caller drops its input;
- runtime fixtures prove fulfillment-chain flattening, catch/finally behavior, typed rejection, `all`/`allSettled` ordering, and no lock held across suspension for each implemented operation;
- a host-placeholder fixture proves its body and chained callbacks never execute and that the source-identified boundary failure is observable;
- detached work and async iteration either pass runtime fixtures or remain source-scoped blockers;
- no portable opaque source count or fully promoted package regresses silently;
- `npm run ci` passes, including deterministic regeneration, candidate compilation, generated conformance, root Cargo tests, wasm checks, and the native-host canary;
- the handoff reports the four parity metrics plus async-task disposition and package-frontier deltas.

Promotion criteria consume this result later: a package cannot be fully promoted while any of its portable async scopes is unsupported, represented by a host placeholder outside a declared host seam, untested where upstream conformance exists, or able to return a panic/default stub.
