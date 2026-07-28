import { execFileSync } from 'node:child_process';
import { mkdtempSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import ts from 'typescript';

import { emitRustModule } from '../../tools/generator/src/emit/rust.ts';
import { lowerTypeScriptSource } from '../../tools/generator/src/lower/typescript.ts';

describe('Rust emission', () => {
  it('deterministically emits and compiles numeric control flow and closures', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/math/src/sample.ts',
      `
        export const EPSILON = 1e-6;
        export function clamp(value: number, min: number, max: number): number {
          if (Math.abs(value) < EPSILON) return 0;
          return value < min ? min : value > max ? max : value;
        }
        export function createScale(scale: number): (value: number) => number {
          return (value) => value * scale;
        }
        export function requirePositive(value: number): number {
          if (value < 0) throw new Error(\`expected positive value, received \${value}\`);
          return value;
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/math', '/workspace');
    const module = {
      declarations: lowered.declarations,
      source: 'upstream/packages/math/src/sample.ts',
      typeImports: [],
    };
    const output = emitRustModule(module);

    expect(lowered.diagnostics).toEqual([]);
    expect(output).toBe(emitRustModule(module));
    expect(output).toContain('pub fn clamp');
    expect(output).toContain('pub fn create_scale');
    expect(output).toContain('format!("expected positive value, received {}", value)');
    expect(output).toContain('std::sync::Arc::new');

    const fixture = mkdtempSync(path.join(tmpdir(), 'flight-rs-emitter-'));
    const sourceFile = path.join(fixture, 'lib.rs');
    writeFileSync(sourceFile, output);
    expect(() =>
      execFileSync('rustc', ['--crate-type', 'lib', '--emit', 'metadata', '--edition', '2024', sourceFile], {
        cwd: fixture,
        stdio: 'pipe',
      }),
    ).not.toThrow();
  });

  it('compiles structural arrays, collection methods, and typed-array ownership', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/math/src/collections.ts',
      `
        export interface Weighted {
          readonly weight?: number;
        }
        export interface Bounds {
          readonly height: number;
          readonly width: number;
        }
        export interface NestedCallbacks {
          readonly callbacks?: {
            readonly onValue?: (value: number) => void;
          };
        }
        export function collectWeights(values: ReadonlyArray<Readonly<Weighted>>): Float32Array {
          const total = values.reduce((sum, value) => sum + (value.weight ?? 1), 0);
          const out = new Float32Array(values.length);
          for (let i = 0; i < values.length; i++) {
            out[i] = (values[i].weight ?? 1) / total;
          }
          return out;
        }
        export function positiveBounds(width: number, height: number): Bounds | null {
          return width > 0 && height > 0 ? { width, height } : null;
        }
        export function countBytes(values: Uint8Array): number[] {
          const bins = new Array<number>(256).fill(0);
          for (const value of values) bins[value]++;
          return bins;
        }
        export function copyLookup(
          out: Uint8Array,
          values: Readonly<Uint8Array | Uint8ClampedArray | null>,
        ): void {
          out[0] = values !== null ? values[0] : 0;
        }
        export function copySharedLookup(out: Uint8Array, values: Uint8Array): void {
          copyLookup(out, values);
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/math', '/workspace');
    const output = emitRustModule({
      declarations: lowered.declarations,
      source: 'upstream/packages/math/src/collections.ts',
      typeImports: [],
    });

    expect(lowered.diagnostics).toEqual([]);
    expect(output).toContain('pub struct Weighted');
    expect(output).toContain('pub struct NestedCallbacksRecord1');
    expect(output).toContain('.iter().cloned().fold');
    expect(output).toContain('Vec<f32>');
    expect(output).toContain('Some(Bounds {');
    expect(output).toContain('vec![0.0_f64; (256.0_f64) as usize]');
    expect(output).toContain('values: Option<Vec<u8>>');
    expect(output).toContain('copy_lookup(out, Some((values).clone()))');

    const fixture = mkdtempSync(path.join(tmpdir(), 'flight-rs-emitter-'));
    const sourceFile = path.join(fixture, 'lib.rs');
    writeFileSync(sourceFile, output);
    expect(() =>
      execFileSync('rustc', ['--crate-type', 'lib', '--emit', 'metadata', '--edition', '2024', sourceFile], {
        cwd: fixture,
        stdio: 'pipe',
      }),
    ).not.toThrow();
  });

  it('preserves for and do-while updates when continue is lowered', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/math/src/loops.ts',
      `
        export function sumOdds(limit: number): number {
          let total = 0;
          for (let i = 0; i < limit; i++) {
            if (i % 2 === 0) continue;
            total += i;
          }
          return total;
        }
        export function countWithDoWhile(limit: number): number {
          let value = 0;
          do {
            value++;
            continue;
          } while (value < limit);
          return value;
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/math', '/workspace');
    const output = emitRustModule({
      declarations: lowered.declarations,
      source: 'upstream/packages/math/src/loops.ts',
      typeImports: [],
    });

    expect(lowered.diagnostics).toEqual([]);
    expect(output).toMatch(/i \+= 1\.0; i \};\s+continue;/u);
    expect(output).toMatch(/if !\(\(value < limit\)\) \{ break; \}\s+continue;/u);

    const fixture = mkdtempSync(path.join(tmpdir(), 'flight-rs-emitter-'));
    const sourceFile = path.join(fixture, 'lib.rs');
    writeFileSync(sourceFile, output);
    expect(() =>
      execFileSync('rustc', ['--crate-type', 'lib', '--emit', 'metadata', '--edition', '2024', sourceFile], {
        cwd: fixture,
        stdio: 'pipe',
      }),
    ).not.toThrow();
  });

  it('compiles numeric enums, bit flags, and merged namespace helpers', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/types/src/Flags.ts',
      `
        export enum Flags {
          None = 0,
          First = 1 << 0,
          Highest = 1 << 31,
        }
        export namespace Flags {
          export function any(flags: Flags, test: Flags): boolean {
            return (flags & test) !== 0;
          }
          export function add(flags: Flags, value: Flags): Flags {
            return flags | value;
          }
          export function remove(flags: Flags, value: Flags): Flags {
            return flags & ~value;
          }
        }
        export enum Mode {
          First,
          Second,
        }
        export function secondMode(): Mode {
          return Mode.Second;
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/types', '/workspace');
    const output = emitRustModule({
      declarations: lowered.declarations,
      enumNames: ['Flags', 'Mode'],
      source: 'upstream/packages/types/src/Flags.ts',
      typeImports: [],
    });

    expect(lowered.diagnostics).toEqual([]);
    expect(output).toContain('pub struct Flags(pub u32)');
    expect(output).toContain('pub const Highest: Self = Self(2147483648_u32)');
    expect(output).toContain('pub fn any');
    expect(output).toContain('return Mode::Second');

    const fixture = mkdtempSync(path.join(tmpdir(), 'flight-rs-emitter-'));
    const sourceFile = path.join(fixture, 'lib.rs');
    writeFileSync(sourceFile, output);
    expect(() =>
      execFileSync('rustc', ['--crate-type', 'lib', '--emit', 'metadata', '--edition', '2024', sourceFile], {
        cwd: fixture,
        stdio: 'pipe',
      }),
    ).not.toThrow();
  });

  it('compiles data-carrying Error subclasses', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/types/src/PortError.ts',
      `
        export class PortTimeoutError extends Error {
          readonly channel: string;
          readonly timeoutMs: number;

          constructor(channel: string, timeoutMs: number) {
            super(\`Channel "\${channel}" timed out after \${timeoutMs}ms\`);
            this.name = 'PortTimeoutError';
            this.channel = channel;
            this.timeoutMs = timeoutMs;
          }
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/types', '/workspace');
    const output = emitRustModule({
      declarations: lowered.declarations,
      source: 'upstream/packages/types/src/PortError.ts',
      typeImports: [],
    });

    expect(lowered.diagnostics).toEqual([]);
    expect(output).toContain('pub struct PortTimeoutError');
    expect(output).toContain('impl std::error::Error for PortTimeoutError');
    expect(output).toContain('pub fn new(channel: String, timeout_ms: f64)');

    const fixture = mkdtempSync(path.join(tmpdir(), 'flight-rs-emitter-'));
    const sourceFile = path.join(fixture, 'lib.rs');
    writeFileSync(sourceFile, output);
    expect(() =>
      execFileSync('rustc', ['--crate-type', 'lib', '--emit', 'metadata', '--edition', '2024', sourceFile], {
        cwd: fixture,
        stdio: 'pipe',
      }),
    ).not.toThrow();
  });

  it('synthesizes typed lazy statics for object constants', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/types/src/Roles.ts',
      `
        export const Roles = {
          copy: 'copy',
          pasteAndMatchStyle: 'pasteAndMatchStyle',
        };
        export function copyRole(): string {
          return Roles.copy;
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/types', '/workspace');
    const output = emitRustModule({
      declarations: lowered.declarations,
      source: 'upstream/packages/types/src/Roles.ts',
      typeImports: [],
    });

    expect(lowered.diagnostics).toEqual([]);
    expect(output).toContain('pub struct Roles');
    expect(output).toContain('pub static ROLES: std::sync::LazyLock<Roles>');
    expect(output).toContain('return (ROLES.copy).clone()');

    const fixture = mkdtempSync(path.join(tmpdir(), 'flight-rs-emitter-'));
    const sourceFile = path.join(fixture, 'lib.rs');
    writeFileSync(sourceFile, output);
    expect(() =>
      execFileSync('rustc', ['--crate-type', 'lib', '--emit', 'metadata', '--edition', '2024', sourceFile], {
        cwd: fixture,
        stdio: 'pipe',
      }),
    ).not.toThrow();
  });
});
