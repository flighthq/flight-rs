// Regenerates src/wasm/bitmapWasmBytes.ts by base64-embedding the wasm-pack
// output (src/wasm/bitmap_wasm_bg.wasm). Run as the second half of `npm run
// wasm`, after wasm-pack. Embedding keeps init synchronous and free of any file
// read or network fetch, so the shim is a true drop-in across environments.

import { readFileSync, rmSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

import { renderBitmapWasmBytes } from '../../../tools/generator/src/wasm-artifact.ts';

const here = dirname(fileURLToPath(import.meta.url));
const workspace = join(here, '../../..');
const wasmDir = join(here, '..', 'src', 'wasm');

const bytes = readFileSync(join(wasmDir, 'bitmap_wasm_bg.wasm'));

// wasm-pack scaffolds the out-dir as if it were its own publishable package,
// dropping a `package.json` and a `.gitignore` (`*`). This dir is generated
// output inside @flighthq/bitmap-wasm, not a package: the stray manifest would
// register as a phantom workspace, and the whole dir is git-ignored anyway.
// Strip both so every bake leaves src/wasm holding only the artifacts.
for (const scaffold of ['package.json', '.gitignore', 'bitmap_wasm_bg.wasm', 'bitmap_wasm_bg.wasm.d.ts']) {
  rmSync(join(wasmDir, scaffold), { force: true });
}

writeFileSync(join(wasmDir, 'bitmapWasmBytes.ts'), renderBitmapWasmBytes(workspace, bytes));
