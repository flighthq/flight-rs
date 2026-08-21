import { execFileSync } from 'node:child_process';
import { homedir } from 'node:os';
import { readFileSync } from 'node:fs';
import path from 'node:path';

const workspace = path.resolve(import.meta.dirname, '../../..');
const output = path.join(workspace, 'packages/bitmap-rs/src/wasm');
const crate = 'flighthq-bitmap-wasm';
const artifact = path.join(workspace, 'target/wasm32-unknown-unknown/release/flighthq_bitmap_wasm.wasm');
const toolRoot = path.join(workspace, 'target/tools/wasm-bindgen');
const localWasmBindgen = path.join(toolRoot, 'bin', process.platform === 'win32' ? 'wasm-bindgen.exe' : 'wasm-bindgen');

// This module is embedded verbatim in the published @flighthq/bitmap-rs, so its bytes must depend
// on the sources alone. rustc otherwise records the absolute path of each panicking source file —
// including dependency sources under CARGO_HOME — which bakes a build-machine path into a published
// artifact and makes the output differ between clones. Nothing downstream catches that: the
// freshness check compares the embedded bytes against their own recorded hash, so two modules built
// in different directories each verify against themselves.
//
// Cargo's `trim-paths` profile option would express this directly but is not stabilized in the
// pinned toolchain, so remap the two roots explicitly. The remapped prefixes are fixed strings, so
// every machine produces identical bytes.
const cargoHome = process.env.CARGO_HOME ?? path.join(homedir(), '.cargo');
const remap = [`--remap-path-prefix=${cargoHome}=/cargo`, `--remap-path-prefix=${workspace}=/flight-rs`];
const buildEnvironment = {
  ...process.env,
  RUSTFLAGS: [process.env.RUSTFLAGS, ...remap].filter(Boolean).join(' '),
};

run('npm', ['run', 'generate']);
run('rustup', ['target', 'add', 'wasm32-unknown-unknown']);
run('cargo', ['build', '-p', crate, '--release', '--target', 'wasm32-unknown-unknown'], buildEnvironment);

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

run(wasmBindgen, [artifact, '--target', 'web', '--out-dir', output, '--out-name', 'bitmap_wasm']);
run('tsx', [path.join(workspace, 'packages/bitmap-rs/scripts/embed-wasm.ts')]);

function toolVersion(command: string): string {
  try {
    return execFileSync(command, ['--version'], { encoding: 'utf8' }).trim();
  } catch {
    return '';
  }
}

function run(command: string, arguments_: string[], environment: NodeJS.ProcessEnv = process.env): void {
  execFileSync(command, arguments_, {
    cwd: workspace,
    env: environment,
    stdio: 'inherit',
  });
}
