import { readFileSync, rmSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

import { renderWasmBytes, wasmArtifactTargets } from '../../../tools/generator/src/wasm-artifact.ts';

const here = dirname(fileURLToPath(import.meta.url));
const workspace = join(here, '../../..');
const wasmDir = join(here, '..', 'src', 'wasm');
const base = 'physics2d_abi_wasm';
const bytes = readFileSync(join(wasmDir, `${base}_bg.wasm`));

for (const scaffold of ['package.json', '.gitignore', `${base}_bg.wasm`, `${base}_bg.wasm.d.ts`]) {
  rmSync(join(wasmDir, scaffold), { force: true });
}
writeFileSync(
  join(wasmDir, 'physics2DAbiWasmBytes.ts'),
  renderWasmBytes(workspace, wasmArtifactTargets.physics2d, bytes),
);
