import { existsSync } from 'node:fs';
import path from 'node:path';

import { defineConfig, mergeConfig } from 'vitest/config';

import baseConfig from '../../vitest.config.base';

// The conformance suite imports the wasm-bindgen output (`src/wasm/surface_wasm.js`), which only
// exists after `npm run build:wasm`. In this Rust-first repo, missing baked wasm is a hard error for
// the surface-rs contract lane, not a green "no tests" state.
const wasmBuilt = existsSync(path.resolve(__dirname, 'src/wasm/surface_wasm.js'));

if (!wasmBuilt) {
  throw new Error('[surface-rs] missing src/wasm/surface_wasm.js; run `npm run build:wasm` before testing.');
}

export default mergeConfig(
  baseConfig,
  defineConfig({
    test: {
      environment: 'jsdom',
      include: ['src/**/*.test.ts'],
      sequence: { groupOrder: 2 },
    },
  }),
);
