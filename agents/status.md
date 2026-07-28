# Status

- The original manual Rust port is archived outside the repository as a complete Git bundle.
- Repository history was filtered to the curated `packages/surface-rs` path.
- Upstream Flight is pinned at `5d24729f7360475e28a105ae0caeeaa2e1328260`, matching the cultivated `flight-hx` generator baseline.
- The generator inventory, neutral IR, and TypeScript lowering began from `flight-hx` commit `390a890c542f135278d89eee83f4f54e8fdbfd72`.
- The compiler-driven automatic matrix currently compiles 22 generated package candidates, including application, camera, entity, geometry, input, lifecycle, path, power, signals, spatial, and texture.
- `@flighthq/easing` remains the first fully promoted executable Rust generation target.
- `packages/surface-rs` and `flighthq-surface` are the selected cultivated wasm boundary; surface is the only planned standalone wasm package.
- The native host frontier is useragent → platform/device, core/backend separation for screen, and the log/node/material/render dependency chain before a cultivated `flighthq-host-winit` adapter is introduced.
