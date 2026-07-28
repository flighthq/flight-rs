import { execFileSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
import path from 'node:path';

const workspace = path.resolve(import.meta.dirname, '../../..');
const output = path.join(workspace, 'packages/surface-rs/src/wasm');
const crate = 'flighthq-surface-wasm';
const artifact = path.join(workspace, 'target/wasm32-unknown-unknown/release/flighthq_surface_wasm.wasm');
const toolRoot = path.join(workspace, 'target/tools/wasm-bindgen');
const localWasmBindgen = path.join(toolRoot, 'bin', process.platform === 'win32' ? 'wasm-bindgen.exe' : 'wasm-bindgen');

run('npm', ['run', 'generate']);
run('rustup', ['target', 'add', 'wasm32-unknown-unknown']);
run('cargo', ['build', '-p', crate, '--release', '--target', 'wasm32-unknown-unknown']);

const lock = readFileSync(path.join(workspace, 'Cargo.lock'), 'utf8');
const version = lock.match(/\[\[package\]\]\nname = "wasm-bindgen"\nversion = "([^"]+)"/u)?.[1];
if (!version) throw new Error('Cargo.lock does not contain the generated wasm-bindgen dependency');

let wasmBindgen = 'wasm-bindgen';
if (toolVersion(wasmBindgen) !== `wasm-bindgen ${version}`) {
  wasmBindgen = localWasmBindgen;
  if (toolVersion(wasmBindgen) !== `wasm-bindgen ${version}`) {
    run('cargo', ['install', 'wasm-bindgen-cli', '--version', version, '--locked', '--force', '--root', toolRoot]);
  }
}

run(wasmBindgen, [artifact, '--target', 'web', '--out-dir', output, '--out-name', 'surface_wasm']);
run('tsx', [path.join(workspace, 'packages/surface-rs/scripts/embed-wasm.ts')]);

function toolVersion(command: string): string {
  try {
    return execFileSync(command, ['--version'], { encoding: 'utf8' }).trim();
  } catch {
    return '';
  }
}

function run(command: string, arguments_: string[]): void {
  execFileSync(command, arguments_, {
    cwd: workspace,
    env: process.env,
    stdio: 'inherit',
  });
}
