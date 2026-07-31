# Status

- The original manual Rust port is archived outside the repository as a complete Git bundle.
- Repository history was filtered to the curated `packages/surface-rs` path.
- Upstream Flight is pinned at `5d24729f7360475e28a105ae0caeeaa2e1328260`, matching the cultivated `flight-hx` generator baseline.
- The generator inventory, neutral IR, and TypeScript lowering began from `flight-hx` commit `390a890c542f135278d89eee83f4f54e8fdbfd72`.
- The compiler-driven automatic matrix currently compiles 27 generated package candidates. Pass 27 Stage 1 removed 33 erased async bodies, inventoried all 162 eligible async scopes, and honestly moved screen out of the compiled set; 61 packages now reach Rust emission and there are 355 source/package blockers.
- `@flighthq/types` and `@flighthq/easing` are fully promoted executable Rust generation targets.
- `packages/surface-rs` and the surface source/declaration admission policy are cultivated. `flighthq-surface` is generated from that selection; surface remains the only planned standalone wasm package.
- A cultivated `flighthq-host-winit` compile canary now links the generated application, lifecycle, input, keyboard, haptics, power, platform, device, and screen seams and installs their backends as one native bundle.
- Node and assets now reach concrete Rust compilation instead of stopping at generic-constructor syntax. The remaining native-render frontier is log plus node/material/skeleton/render, followed by concrete winit event and renderer adapters.
- `agents/generator-resume.md` is the durable resume playbook: it records the current matrix, generator invariants, validation loop, known traps, and acceptance criteria for passes 19–21.
