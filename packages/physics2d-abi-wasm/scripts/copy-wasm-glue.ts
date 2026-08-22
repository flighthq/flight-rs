import { copyFileSync, mkdirSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

export const wasmGlueFiles = ['physics2d_abi_wasm.js', 'physics2d_abi_wasm.d.ts'] as const;
const scriptPath = fileURLToPath(import.meta.url);

export function copyWasmGlue(): void {
  const here = dirname(scriptPath);
  const source = join(here, '..', 'src', 'wasm');
  const output = join(here, '..', 'dist', 'wasm');
  mkdirSync(output, { recursive: true });
  for (const file of wasmGlueFiles) copyFileSync(join(source, file), join(output, file));
}

if (resolve(process.argv[1] ?? '') === resolve(scriptPath)) copyWasmGlue();
