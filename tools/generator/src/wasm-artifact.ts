import { createHash } from 'node:crypto';
import { existsSync, readdirSync, readFileSync } from 'node:fs';
import path from 'node:path';

export interface WasmArtifactTarget {
  artifactPath: string;
  bytesExport: string;
  crate: string;
  embedScript: string;
}

export const wasmArtifactTargets = {
  bitmap: {
    artifactPath: 'packages/bitmap-wasm/src/wasm/bitmapWasmBytes.ts',
    bytesExport: 'bitmapWasmBytes',
    crate: 'flighthq-bitmap-wasm',
    embedScript: 'packages/bitmap-wasm/scripts/embed-wasm.ts',
  },
  physics2d: {
    artifactPath: 'packages/physics2d-abi-wasm/src/wasm/physics2DAbiWasmBytes.ts',
    bytesExport: 'physics2DAbiWasmBytes',
    crate: 'flighthq-physics2d-abi-wasm',
    embedScript: 'packages/physics2d-abi-wasm/scripts/embed-wasm.ts',
  },
  physics3d: {
    artifactPath: 'packages/physics3d-abi-wasm/src/wasm/physics3DAbiWasmBytes.ts',
    bytesExport: 'physics3DAbiWasmBytes',
    crate: 'flighthq-physics3d-abi-wasm',
    embedScript: 'packages/physics3d-abi-wasm/scripts/embed-wasm.ts',
  },
} as const satisfies Record<string, WasmArtifactTarget>;

const commonBuildInputs = [
  'Cargo.lock',
  'Cargo.toml',
  'rust-toolchain',
  'rust-toolchain.toml',
  '.cargo/config',
  '.cargo/config.toml',
  'tools/generator/src/build-wasm.ts',
] as const;
const inputHashPattern = /^\/\/ wasm-input-sha256: ([a-f\d]{64})$/mu;
const outputHashPattern = /^\/\/ wasm-output-sha256: ([a-f\d]{64})$/mu;
const base64Pattern = /const base64 =\s*\n\s*'([A-Za-z\d+/]*={0,2})';/u;

export function computeBitmapWasmInputHash(workspace: string): string {
  return computeWasmInputHash(workspace, wasmArtifactTargets.bitmap);
}

export function computeWasmInputHash(workspace: string, target: WasmArtifactTarget): string {
  const files = new Set<string>();
  for (const input of [...commonBuildInputs, target.embedScript]) {
    const absolute = path.join(workspace, input);
    if (existsSync(absolute)) files.add(absolute);
  }

  const manifests = [path.join(workspace, 'generated/crates', target.crate, 'Cargo.toml')];
  const visitedManifests = new Set<string>();
  while (manifests.length > 0) {
    const manifest = manifests.pop();
    if (!manifest || visitedManifests.has(manifest)) continue;
    if (!existsSync(manifest)) throw new Error(`Wasm crate manifest is missing: ${relative(workspace, manifest)}`);
    visitedManifests.add(manifest);

    const crateDirectory = path.dirname(manifest);
    collectFiles(crateDirectory, files);
    for (const dependency of localDependencyPaths(readFileSync(manifest, 'utf8'))) {
      const dependencyManifest = path.resolve(crateDirectory, dependency, 'Cargo.toml');
      if (!isInside(workspace, dependencyManifest)) {
        throw new Error(`Wasm crate dependency is outside the workspace: ${dependency}`);
      }
      manifests.push(dependencyManifest);
    }
  }

  const hash = createHash('sha256');
  for (const file of [...files].sort((left, right) =>
    relative(workspace, left).localeCompare(relative(workspace, right)),
  )) {
    hash.update(relative(workspace, file));
    hash.update('\0');
    hash.update(readFileSync(file));
    hash.update('\0');
  }
  return hash.digest('hex');
}

export function renderBitmapWasmBytes(workspace: string, bytes: Uint8Array): string {
  return renderWasmBytes(workspace, wasmArtifactTargets.bitmap, bytes);
}

export function renderWasmBytes(workspace: string, target: WasmArtifactTarget, bytes: Uint8Array): string {
  return renderEmbeddedWasm(target, computeWasmInputHash(workspace, target), bytes);
}

function renderEmbeddedWasm(target: WasmArtifactTarget, inputHash: string, bytes: Uint8Array): string {
  const outputHash = sha256(bytes);
  const base64 = Buffer.from(bytes).toString('base64');
  return `// GENERATED — do not edit by hand. Produced by scripts/embed-wasm.ts from
// generated/crates/${target.crate}. Holds the wasm module as base64 so init is
// synchronous and needs no file read or network fetch in any environment.
// wasm-input-sha256: ${inputHash}
// wasm-output-sha256: ${outputHash}

const base64 =
  '${base64}';

function decodeBase64(value: string): Uint8Array {
  const binary = atob(value);
  const out = new Uint8Array(binary.length);
  for (let i = 0; i < binary.length; i += 1) out[i] = binary.charCodeAt(i);
  return out;
}

export const ${target.bytesExport}: Uint8Array = decodeBase64(base64);
`;
}

export function assertBitmapWasmArtifactFresh(workspace: string): void {
  assertWasmArtifactFresh(workspace, wasmArtifactTargets.bitmap);
}

export function assertWasmArtifactFresh(workspace: string, target: WasmArtifactTarget): void {
  const file = path.join(workspace, target.artifactPath);
  if (!existsSync(file)) throw staleArtifactError(target, 'is missing');

  const content = readFileSync(file, 'utf8');
  const recordedInputHash = inputHashPattern.exec(content)?.[1];
  const recordedOutputHash = outputHashPattern.exec(content)?.[1];
  const base64 = base64Pattern.exec(content)?.[1];
  if (!recordedInputHash || !recordedOutputHash || base64 === undefined) {
    throw staleArtifactError(target, 'does not contain valid freshness metadata');
  }

  const currentInputHash = computeWasmInputHash(workspace, target);
  if (recordedInputHash !== currentInputHash) {
    throw staleArtifactError(target, 'was built from stale Rust or packaging inputs');
  }
  const bytes = Buffer.from(base64, 'base64');
  if (recordedOutputHash !== sha256(bytes)) {
    throw staleArtifactError(target, 'does not match its recorded wasm output hash');
  }
  if (content !== renderEmbeddedWasm(target, currentInputHash, bytes)) {
    throw staleArtifactError(target, 'does not match the canonical embedded module');
  }
}

function collectFiles(directory: string, files: Set<string>): void {
  for (const entry of readdirSync(directory, { withFileTypes: true })) {
    const absolute = path.join(directory, entry.name);
    if (entry.isDirectory()) collectFiles(absolute, files);
    else if (entry.isFile()) files.add(absolute);
  }
}

function localDependencyPaths(manifest: string): string[] {
  const dependencies: string[] = [];
  let dependencySection = false;
  for (const rawLine of manifest.split(/\r?\n/gu)) {
    const line = rawLine.replace(/\s+#.*$/u, '');
    const section = /^\s*\[([^\]]+)\]\s*$/u.exec(line)?.[1];
    if (section !== undefined) {
      dependencySection = /(?:^|\.)(?:dev-|build-)?dependencies(?:\.|$)/u.test(section);
      continue;
    }
    if (!dependencySection) continue;
    const dependency = /\bpath\s*=\s*"([^"]+)"/u.exec(line)?.[1];
    if (dependency !== undefined) dependencies.push(dependency);
  }
  return dependencies;
}

function sha256(bytes: Uint8Array): string {
  return createHash('sha256').update(bytes).digest('hex');
}

function isInside(workspace: string, file: string): boolean {
  const result = path.relative(workspace, file);
  return result !== '..' && !result.startsWith(`..${path.sep}`) && !path.isAbsolute(result);
}

function relative(workspace: string, file: string): string {
  return path.relative(workspace, file).split(path.sep).join('/');
}

function staleArtifactError(target: WasmArtifactTarget, reason: string): Error {
  return new Error(`Generated wasm artifact ${target.artifactPath} ${reason}; run npm run wasm`);
}
