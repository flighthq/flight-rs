# Status

- The original manual Rust port is archived outside the repository as a complete Git bundle.
- Repository history was filtered to the curated `packages/surface-rs` path.
- Upstream Flight is pinned at `cad72aa3ea4e6e76a050918a403dcb10efdfcb0d`, matching `flight-hx` main at `27f4d0ee7ae04d64e66c996f99a0f270b09e12be`.
- The schema-2 inventory reports are current for that pin: 143 packages, 299 manifest export lanes, and 32,998 export records. `generated/manifest.json` and `reports/generation.*` still record the last successful full generation at `5d24729f7360475e28a105ae0caeeaa2e1328260`; rerun generation with Rust/rustfmt installed before treating compiled-candidate counts as current.
- The generator inventory, neutral IR, and TypeScript lowering began from `flight-hx` commit `390a890c542f135278d89eee83f4f54e8fdbfd72`.
- The compiler-driven automatic matrix currently compiles 23 generated package candidates. Pass 27 Stage 2 inventories all 204 task constructions and 162 async scopes, executes 9 constructions/3 non-opaque image-codec scopes through the canonical generated runtime, and leaves 195/159 explicitly unsupported; 59 packages reach Rust emission and there are 358 source/package blockers.
- `@flighthq/types` and `@flighthq/easing` are fully promoted executable Rust generation targets.
- `packages/surface-rs` is the compatibility-named cultivated facade for upstream `@flighthq/bitmap`; its bitmap source/declaration admission policy is explicit. `flighthq-surface` is generated from that selection and remains the only planned standalone wasm package.
- A cultivated `flighthq-host-winit` compile canary now links the generated application, lifecycle, input, keyboard, haptics, power, platform, device, and screen seams and installs their backends as one native bundle.
- The canary resolves generated partial application, input, power, screen, and host-signal-constructor targets so synchronous installation remains compile-checked while automatic task paths stay honest.
- Node and assets now reach concrete Rust compilation instead of stopping at generic-constructor syntax. The remaining native-render frontier is log plus node/material/skeleton/render, followed by concrete winit event and renderer adapters.
- `agents/generator-resume.md` is the durable resume playbook: it records the current matrix, generator invariants, validation loop, known traps, and acceptance criteria for passes 19–21.
