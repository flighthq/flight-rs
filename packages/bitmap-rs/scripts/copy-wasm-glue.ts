// Copies the wasm-bindgen JS glue (and its types) from src/wasm into dist/wasm
// after `tsc -b`. tsc resolves `./wasm/surface_wasm.js` to its sibling .d.ts for
// types but does not emit the .js itself (it is not a TypeScript source), so the
// built dist needs the glue placed alongside the compiled shim. The wasm bytes
// are embedded in surfaceWasmBytes (compiled by tsc), so the raw .wasm is not
// shipped.

import { copyFileSync, mkdirSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

/**
 * Every file the built `dist` needs that `tsc -b` will not emit for it. Exported so the packaging
 * test can prove this list still covers each non-TypeScript module `src` imports: a glue file that
 * is imported but not copied produces a tarball that installs and then fails at import time.
 */
export const wasmGlueFiles = ['surface_wasm.js', 'surface_wasm.d.ts'] as const;

const scriptPath = fileURLToPath(import.meta.url);

export function copyWasmGlue(): void {
  const here = dirname(scriptPath);
  const srcWasm = join(here, '..', 'src', 'wasm');
  const distWasm = join(here, '..', 'dist', 'wasm');

  mkdirSync(distWasm, { recursive: true });
  for (const file of wasmGlueFiles) {
    copyFileSync(join(srcWasm, file), join(distWasm, file));
  }
}

if (resolve(process.argv[1] ?? '') === resolve(scriptPath)) copyWasmGlue();
