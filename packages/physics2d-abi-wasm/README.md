# @flighthq/physics2d-abi-wasm

An experimental Rust/WebAssembly backend for the persistent packed-buffer contract in `@flighthq/physics2d-abi`.

This package is a private prototype and is not published. Do not use it as a substitute for the upstream package yet.

Only the backend constructor changes. Command writers, layout constants, buffers, and convenience wrappers continue to come from the authoritative upstream package:

```diff
-import { createPhysics2DAbi } from '@flighthq/physics2d-abi';
+import { createPhysics2DAbi } from '@flighthq/physics2d-abi-wasm';
```

The wasm module is embedded and initialized synchronously on the first constructor call. `initPhysics2DAbiWasm()` is available for eager initialization; no separate `.wasm` asset or fetch is required.

## Current capabilities

This first backend advertises `PersistentWorlds | SelectiveReadback`. It executes the versioned command stream for world configuration, bodies, collider and joint identity, forces, impulses, torque, and wake operations; steps body state in Rust; and publishes body and joint readback.

It does not yet advertise `Queries` or `ContactHooks`. Query methods return `false`, contact readback is empty, and stepping with hooks returns `Declined`. These gaps are explicit in the capability mask rather than silently falling back to the TypeScript reference backend.

The unchanged upstream conformance suite currently passes 59 of 79 tests. Missing query and contact-hook capabilities are only part of the gap: input validation, contact readback, mass derivation, broken-joint reporting, isolation, and solver behavior also diverge. Run `npm run test:upstream` in this directory to see the executable gap list; the command is expected to fail until the backend is complete.

The prototype currently requires a bundler for the same extensionless-ESM reason as other `@flighthq` packages.

## License

MIT. See [LICENSE.md](LICENSE.md).
