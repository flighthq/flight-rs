import { execFileSync } from 'node:child_process';
import { mkdirSync, mkdtempSync, writeFileSync } from 'node:fs';
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
        export function angle(y: number, x: number): number {
          return Math.atan2(y, x) + Math.acos(1);
        }
        export function hex(value: number): string {
          return (value & 255).toString(16).padStart(2, '0');
        }
        export function cacheKey(value: string): string {
          return \`\${value}\\u0000end\`;
        }
        export function utf16Length(value: string): number {
          return value.length;
        }
        export function twiceLength(value: string): number {
          return utf16Length(value) + utf16Length(value);
        }
        export function selectValue(out: number[], index: number, value: number): void {
          switch (index) {
            case 0:
              out[0] = value;
              break;
            default:
              throw new Error('unsupported index');
          }
        }
        interface Point {
          x: number;
        }
        function setX(out: Point, x: number): void {
          out.x = x;
        }
        export function doubleX(out: Point): void {
          setX(out, out.x * 2);
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
    expect(output).toContain('(y).atan2(x)');
    expect(output).toContain('fn __flight_number_to_string');
    expect(output).toContain('fn __flight_pad_start');
    expect(output).toContain('\\u{0000}');
    expect(output).toContain('value.encode_utf16().count() as f64');
    expect(output).toContain('utf16_length((value).clone())');
    expect(output).toContain('let __flight_argument_1 = (out.x * 2.0_f64);');
    expect(output).not.toContain('break;');

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
        export interface OptionalNames {
          readonly names?: readonly string[] | null;
        }
        export interface OptionalValue {
          readonly value?: number;
        }
        export interface StringMeasureBackend {
          readonly measure: (value: string) => number;
        }
        export interface Defaults {
          count: number;
          name: string;
        }
        export interface BasePosition {
          x: number;
        }
        export interface PositionedValue extends BasePosition {
          label: string;
        }
        export interface AdjustmentOptions {
          kind: string;
          colorMatrix: readonly number[];
          intensity?: number;
        }
        function measureString(value: string): number {
          return value.length;
        }
        function fillDefaults(options?: Partial<Defaults>): Defaults {
          return {
            count: options?.count ?? 0,
            name: options?.name ?? 'default',
          };
        }
        export function createStringMeasureBackend(): StringMeasureBackend {
          return { measure: measureString };
        }
        export function createDefaults(): Defaults {
          return fillDefaults({ count: 2 });
        }
        function readBasePosition(value: Readonly<BasePosition>): number {
          return value.x;
        }
        export function readPositionedValue(value: Readonly<PositionedValue>): number {
          return readBasePosition(value);
        }
        export function createAdjustment(
          options: Readonly<Omit<AdjustmentOptions, 'kind' | 'colorMatrix'>> = {},
        ): AdjustmentOptions {
          return { kind: 'test', ...options, colorMatrix: [1] };
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
        export function cloneFloats(values: Float32Array): Float32Array {
          const copy = new Float32Array(values);
          return copy;
        }
        export function normalizedFloat(values: Float32Array, index: number): number {
          const normalized = values[index] / 2;
          return normalized + 1;
        }
        export function reserveInts(values: Int16Array, capacity: number): Int16Array {
          const out = new Int16Array(capacity);
          out.set(values);
          return out;
        }
        export function cloneNames(values: readonly string[]): string[] {
          return values.slice();
        }
        export function findName(values: readonly string[], name: string): number {
          return values.indexOf(name);
        }
        export function collectionSize(map: Map<string, number>, set: Set<number>): number {
          return map.size + set.size;
        }
        export function clearValues(values: number[]): void {
          values.length = 0;
        }
        export function emptyNames(): OptionalNames {
          return {};
        }
        export function countNames(values?: readonly string[] | null): number {
          if (values === undefined) return 0;
          return values.length;
        }
        export function positiveInfinity(): number {
          return Number.POSITIVE_INFINITY;
        }
        export function optionalValue(options?: Readonly<OptionalValue>): number {
          return options?.value ?? 0;
        }
        export function allValues(values: readonly (number | null)[]): boolean {
          return values.every((value) => value !== null);
        }
        function isPositive(value: number): boolean {
          return value > 0;
        }
        export function somePositive(values: readonly number[]): boolean {
          return values.some(isPositive);
        }
        export function serializeWeights(values: readonly Weighted[]): string {
          return JSON.stringify(values);
        }
        export function hasValues(values: readonly number[] | null): boolean {
          return Array.isArray(values);
        }
        function callPair(callback: (x: number, y: number) => void): void {
          callback(1, 2);
        }
        export function capturedMinimum(): number {
          let minimum = Infinity;
          const expand = (x: number, _y: number) => {
            if (x < minimum) minimum = x;
          };
          callPair(expand);
          return minimum;
        }
        export function falseByte(values: Uint8Array, index: number): boolean {
          return !values[index];
        }
        export function ownOptional(values: number[] | null): number[] {
          if (values !== null) return values;
          return [];
        }
        export function clearNarrowed(): number[] | null {
          let values: number[] | null = [];
          if (values !== null) values = null;
          return values;
        }
        export function nonEmpty(values: number[] | null): number[] | null {
          return values && values.length > 0 ? values : null;
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
    expect(output).toContain('pub struct NestedCallbacksRecord');
    expect(output).toContain('.iter().cloned().fold');
    expect(output).toContain('Vec<f32>');
    expect(output).toContain('Some(Bounds {');
    expect(output).toContain('vec![0.0_f64; (256.0_f64) as usize]');
    expect(output).toContain('values: Option<Vec<u8>>');
    expect(output).toContain('copy_lookup(out, Some(');
    expect(output).toContain('pub names: Option<Vec<String>>');
    expect(output).toContain('.iter().map(|value| (*value) as f32).collect()');
    expect(output).toContain('let copy: Vec<f32>');
    expect(output).toContain('let normalized = ((values[index as usize] as f64) / 2.0_f64)');
    expect(output).toContain('let __flight_values: Vec<i16>');
    expect(output).toContain('.position(|item| item == &__flight_value).map_or(-1.0_f64');
    expect(output).toMatch(/map\.len\(\) as f64.*set\.len\(\) as f64/u);
    expect(output).toContain('values.clear()');
    expect(output).toContain('names: None');
    expect(output).toContain('(values).is_none()');
    expect(output).toContain('f64::INFINITY');
    expect(output).toContain('options.as_ref().and_then(|value| value.value)');
    expect(output).toContain('measure: std::sync::Arc::new(std::sync::Mutex::new(Box::new(');
    expect(output).toContain('count: Some(2.0_f64)');
    expect(output).toContain('name: None');
    expect(output).toContain('pub struct FlightOmitRecord2');
    expect(output).toContain('options: Option<FlightOmitRecord2>');
    expect(output).toContain('.iter().cloned().all(|value: Option<f64>| -> bool');
    expect(output).toContain('.iter().cloned().any(|__flight_item| is_positive(__flight_item))');
    expect(output).toContain('format!("[{}]", __flight_items.join(","))');
    expect(output).toContain('(values).is_some()');
    expect(output).toContain('&{ let __flight_source = &(value); BasePosition {');
    expect(output).toContain('x: __flight_source.x');

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

  it('narrows discriminated unions and wraps named union assignments', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/camera/src/projection.ts',
      `
        export interface OrthographicProjection {
          kind: 'orthographic';
          halfWidth: number;
        }
        export interface PerspectiveProjection {
          kind: 'perspective';
          fovY: number;
        }
        export type Projection = OrthographicProjection | PerspectiveProjection;
        export interface CameraState {
          projection: Projection;
        }
        export function projectionWidth(projection: Projection): number {
          if (projection.kind === 'perspective') return projection.fovY;
          return projection.halfWidth;
        }
        export function setOrthographic(
          camera: CameraState,
          projection: OrthographicProjection,
        ): void {
          camera.projection = projection;
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/camera', '/workspace');
    const output = emitRustModule({
      declarations: lowered.declarations,
      source: 'upstream/packages/camera/src/projection.ts',
      typeImports: [],
    });

    expect(lowered.diagnostics).toEqual([]);
    expect(output).toContain('matches!(&(projection), Projection::B(_))');
    expect(output).toContain('Projection::A(');
    expect(output).toContain('TypeScript union narrowing failed');
  });

  it('resolves TypeScript value imports to emitted Rust function bindings', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/math/src/imports.ts',
      `
        import { importedFunction } from './dependency';
        export function callImported(value: number): number {
          return importedFunction(value);
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/math', '/workspace');
    const output = emitRustModule({
      declarations: lowered.declarations,
      imports: [
        {
          module: 'crate',
          names: [{ imported: 'importedFunction', local: 'importedFunction' }],
        },
      ],
      source: 'upstream/packages/math/src/imports.ts',
      typeImports: [],
    });

    expect(lowered.diagnostics).toEqual([]);
    expect(output).toContain('use crate::{imported_function};');
    expect(output).toContain('return imported_function(value)');

    const fixture = mkdtempSync(path.join(tmpdir(), 'flight-rs-emitter-'));
    const sourceFile = path.join(fixture, 'generated.rs');
    writeFileSync(sourceFile, output);
    writeFileSync(
      path.join(fixture, 'lib.rs'),
      'pub fn imported_function(value: f64) -> f64 { value }\nmod generated;\n',
    );
    expect(() =>
      execFileSync('rustc', ['--crate-type', 'lib', '--emit', 'metadata', '--edition', '2024', 'lib.rs'], {
        cwd: fixture,
        stdio: 'pipe',
      }),
    ).not.toThrow();
  });

  it('types host-bound Promise callbacks while keeping the native placeholder inert', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/power/src/promise.ts',
      `
        interface Manager {
          level: number;
        }
        export function observeManager(promise: Promise<Manager> | null): void {
          if (promise === null) return;
          promise
            .then((manager) => {
              void manager.level;
            })
            .catch(() => {});
        }
        export async function requestManager(): Promise<boolean> {
          await Promise.resolve();
          return true;
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/power', '/workspace');
    const output = emitRustModule({
      declarations: lowered.declarations,
      source: 'upstream/packages/power/src/promise.ts',
      typeImports: [],
    });

    expect(lowered.diagnostics).toEqual([]);
    expect(output).toContain('move |manager: Manager| -> ()');
    expect(output).toContain('crate::Promise::<()>::default()');
    expect(output).toContain('pub fn request_manager() -> crate::Promise<bool> {\n  Default::default()');

    const fixture = mkdtempSync(path.join(tmpdir(), 'flight-rs-emitter-'));
    writeFileSync(path.join(fixture, 'generated.rs'), output);
    writeFileSync(
      path.join(fixture, 'lib.rs'),
      [
        'pub struct Promise<T> { marker: std::marker::PhantomData<fn() -> T> }',
        'impl<T> Clone for Promise<T> { fn clone(&self) -> Self { Self { marker: std::marker::PhantomData } } }',
        'impl<T> Default for Promise<T> { fn default() -> Self { Self { marker: std::marker::PhantomData } } }',
        'mod generated;',
        '',
      ].join('\n'),
    );
    expect(() =>
      execFileSync('rustc', ['--crate-type', 'lib', '--emit', 'metadata', '--edition', '2024', 'lib.rs'], {
        cwd: fixture,
        stdio: 'pipe',
      }),
    ).not.toThrow();
  });

  it('lowers shared byte-buffer views, regex captures, and exhaustive switches', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/useragent/src/portable.ts',
      `
        export function firstNativeByte(): number {
          const buffer = new ArrayBuffer(2);
          new Uint16Array(buffer)[0] = 0x0102;
          return new Uint8Array(buffer)[0];
        }
        export function execVersion(value: string): string {
          const match = /version\\/([\\d.]+)/i.exec(value);
          return match ? match[1] : '';
        }
        export function matchVersion(value: string): string {
          const match = value.match(/version\\/([\\d.]+)/i);
          return match ? match[1] : '';
        }
        export function platform(value: string): string {
          switch (value) {
            case 'web':
              return 'browser';
            default:
              return 'native';
          }
        }
        export function majorVersion(value: string): number {
          const part = value.split('.')[0];
          const parsed = parseInt(part, 10);
          return isNaN(parsed) ? 0 : parsed;
        }
        export function nativeUserAgent(): string {
          return typeof navigator !== 'undefined' ? navigator.userAgent : '';
        }
        export function nativeTouchPoints(): number {
          const nav = typeof navigator === 'undefined' ? null : navigator;
          return nav !== null && 'maxTouchPoints' in nav ? nav.maxTouchPoints : -1;
        }
        export function nativeStoredValue(): string {
          const existing = typeof localStorage !== 'undefined' ? localStorage.getItem('key') : null;
          if (existing !== null) return existing;
          return '';
        }
        interface Mode {
          value: number;
        }
        export function ensureMode(out: Mode[]): Mode[] {
          out.length = 1;
          if (out[0] === undefined) out[0] = { value: 0 };
          return out;
        }
        export function positiveOptional(value?: number): boolean {
          return typeof value === 'number' && value > 0;
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/useragent', '/workspace');
    const output = emitRustModule({
      declarations: lowered.declarations,
      source: 'upstream/packages/useragent/src/portable.ts',
      typeImports: [],
    });

    expect(lowered.diagnostics).toEqual([]);
    expect(output).toContain('let mut buffer: Vec<u8>');
    expect(output).toContain('let __flight_bytes = __flight_value.to_ne_bytes()');
    expect(output).toContain('.captures(&');
    expect(output).toContain('collect::<Vec<_>>()');
    expect(output).toContain('i64::from_str_radix');
    expect(output).toContain('.is_nan()');
    expect(output).toContain('pub fn native_user_agent() -> String {\n  return "".to_owned();');
    expect(output).toContain('pub fn native_touch_points() -> f64');
    expect(output).toContain('let nav: Option<crate::OpaqueHostValue> = None;');
    expect(output).toContain('return (-1.0_f64);');
    expect(output).not.toContain('host.maxTouchPoints');
    expect(output).toContain('let existing: Option<crate::OpaqueHostValue> = None;');
    expect(output).toContain('.get((0.0_f64) as usize).is_none()');
    expect(output).toContain('.as_ref().is_some_and(|value| *value > 0.0_f64)');
    expect(output).toContain('exhaustive TypeScript switch completed without returning');
  });

  it('uses imported generic signatures to type structural call arguments', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/math/src/factory.ts',
      `
        import { createEntity } from '@flighthq/entity';
        export interface Point {
          x: number;
        }
        export function createPoint(x: number): Point {
          return createEntity({ x });
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const dependency = ts.createSourceFile(
      '/workspace/upstream/packages/entity/src/entity.ts',
      `
        export function createEntity<Type extends object>(obj?: Type): Type {
          return obj!;
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/math', '/workspace');
    const dependencyFunction = lowerTypeScriptSource(dependency, '@flighthq/entity', '/workspace').declarations.find(
      (declaration) => declaration.kind === 'function',
    );
    expect(dependencyFunction).toBeDefined();
    const output = emitRustModule({
      declarations: lowered.declarations,
      imports: [
        {
          module: 'crate',
          names: [{ imported: 'createEntity', local: 'createEntity' }],
        },
      ],
      semanticFunctions: dependencyFunction ? [dependencyFunction] : [],
      source: 'upstream/packages/math/src/factory.ts',
      typeImports: [],
    });

    expect(lowered.diagnostics).toEqual([]);
    expect(output).toContain('create_entity(Some(Point {');

    const fixture = mkdtempSync(path.join(tmpdir(), 'flight-rs-emitter-'));
    writeFileSync(path.join(fixture, 'generated.rs'), output);
    writeFileSync(
      path.join(fixture, 'lib.rs'),
      [
        'fn create_entity(obj: Option<generated::Point>) -> generated::Point { obj.unwrap() }',
        'mod generated;',
        '',
      ].join('\n'),
    );
    expect(() =>
      execFileSync('rustc', ['--crate-type', 'lib', '--emit', 'metadata', '--edition', '2024', 'lib.rs'], {
        cwd: fixture,
        stdio: 'pipe',
      }),
    ).not.toThrow();
  });

  it('inlines cross-module structural helpers and projects optional cast fields', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/adjustments/src/caller.ts',
      `
        import { readMatrix } from './helper';
        export interface Adjustment {
          kind: string;
        }
        export function resolveMatrix(adjustment: Adjustment): readonly number[] | null {
          return readMatrix(adjustment);
        }
        export interface Backend {
          run: () => void;
        }
        export function createBackend(): Backend {
          const backend: Backend & { refresh?: () => void } = {
            run() {},
            refresh() {},
          };
          return backend;
        }
        export function refreshBackend(backend: Backend): void {
          const maybeRefreshable = backend as unknown as { refresh?: () => void };
          if (typeof maybeRefreshable.refresh === 'function') maybeRefreshable.refresh();
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const helper = ts.createSourceFile(
      '/workspace/upstream/packages/adjustments/src/helper.ts',
      `
        interface MatrixAdjustment {
          kind: string;
          matrix: readonly number[];
        }
        export function readMatrix(value: Readonly<{ kind: string }>): readonly number[] | null {
          const matrix = (value as Readonly<Partial<MatrixAdjustment>>).matrix;
          return Array.isArray(matrix) ? matrix : null;
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/adjustments', '/workspace');
    const helperLowered = lowerTypeScriptSource(helper, '@flighthq/adjustments', '/workspace');
    const helperFunction = helperLowered.declarations.find(
      (declaration): declaration is Extract<(typeof helperLowered.declarations)[number], { kind: 'function' }> =>
        declaration.kind === 'function',
    );
    const helperTypes = Object.fromEntries(
      helperLowered.declarations.flatMap((declaration) =>
        declaration.kind === 'type' ? [[declaration.name, declaration.type]] : [],
      ),
    );
    const output = emitRustModule({
      declarations: lowered.declarations,
      semanticFunctions: helperFunction ? [helperFunction] : [],
      semanticTypes: helperTypes,
      source: 'upstream/packages/adjustments/src/caller.ts',
      typeImports: [],
    });

    expect(lowered.diagnostics).toEqual([]);
    expect(helperLowered.diagnostics).toEqual([]);
    expect(output).not.toContain('read_matrix(adjustment)');
    expect(output).toContain('let matrix = None::<Vec<f64>>');
    expect(output).toContain('refresh: None');
    expect(output).toContain('.as_ref().map_or("undefined", |_| "function")');
    expect(output).toContain('run: (__flight_source.run).clone()');

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

  it('canonicalizes repeated structural parameters across functions in one module', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/adjustments/src/signature.ts',
      `
        function signature(values: ReadonlyArray<Readonly<{ kind: string }>>): string {
          return JSON.stringify(values);
        }
        export function publicSignature(values: ReadonlyArray<Readonly<{ kind: string }>>): string {
          return signature(values);
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/adjustments', '/workspace');
    const output = emitRustModule({
      declarations: lowered.declarations,
      source: 'upstream/packages/adjustments/src/signature.ts',
      typeImports: [],
    });

    expect(lowered.diagnostics).toEqual([]);
    expect(output).toContain('pub struct SharedStructuralRecord1');
    expect(output).toContain('signature(values)');

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

  it('canonicalizes structural signatures against inferred top-level records', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/screen/src/scratch.ts',
      `
        const scratchPoint = { x: 0, y: 0 };
        function readPoint(value: Readonly<{ x: number; y: number }>): number {
          return value.x + value.y;
        }
        export function readScratchPoint(): number {
          return readPoint(scratchPoint);
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/screen', '/workspace');
    const output = emitRustModule({
      declarations: lowered.declarations,
      source: 'upstream/packages/screen/src/scratch.ts',
      typeImports: [],
    });

    expect(lowered.diagnostics).toEqual([]);
    expect(output).toContain('pub struct SharedStructuralRecord1');
    expect(output).toContain('SharedStructuralRecord1 {');

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

  it('preserves fields across discriminated open-interface families', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/lighting/src/openFamily.ts',
      `
        export interface Light {
          kind: string;
        }
        export interface PointLight extends Light {
          kind: 'PointLight';
          range: number;
        }
        export function getRange(light: Light): number {
          const point = light as PointLight;
          return point.range;
        }
        export function getPointRange(point: PointLight): number {
          return getRange(point);
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/lighting', '/workspace');
    const output = emitRustModule({
      declarations: lowered.declarations,
      source: 'upstream/packages/lighting/src/openFamily.ts',
      typeImports: [],
    });

    expect(lowered.diagnostics).toEqual([]);
    expect(output).toMatch(/pub struct Light \{[\s\S]*pub range: f64,/u);
    expect(output).toContain('..Default::default()');

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

  it('selects nested structural union variants for object arguments', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/log/src/union.ts',
      `
        export type LogData = string | Readonly<Record<string, unknown>>;
        export type LogDataProvider = () => LogData;
        export function log(data: LogData | LogDataProvider): void {
          void data;
        }
        export function warn(): void {
          log({ message: 'generated warning' });
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/log', '/workspace');
    const output = emitRustModule({
      declarations: lowered.declarations,
      source: 'upstream/packages/log/src/union.ts',
      typeImports: [],
    });

    expect(lowered.diagnostics).toEqual([]);
    expect(output).toContain(
      'crate::FlightUnion2::<LogData, LogDataProvider>::A(crate::FlightUnion2::<String, Vec<(String, crate::OpaqueHostValue)>>::B(',
    );

    const fixture = mkdtempSync(path.join(tmpdir(), 'flight-rs-emitter-'));
    writeFileSync(path.join(fixture, 'generated.rs'), output);
    writeFileSync(
      path.join(fixture, 'lib.rs'),
      [
        '#[derive(Clone)]',
        'pub enum OpaqueHostValue { String(String) }',
        '#[derive(Clone)]',
        'pub enum FlightUnion2<A, B> { A(A), B(B) }',
        'mod generated;',
        '',
      ].join('\n'),
    );
    expect(() =>
      execFileSync('rustc', ['--crate-type', 'lib', '--emit', 'metadata', '--edition', '2024', 'lib.rs'], {
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

  it('uses Rust turbofish syntax for generic structural projections', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/node/src/genericProjection.ts',
      `
        interface Source<T> {
          readonly value: T;
        }
        interface Target<T> {
          readonly value: T;
        }
        export function project<T>(source: Source<T>): Target<T> {
          return source;
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/node', '/workspace');
    const output = emitRustModule({
      declarations: lowered.declarations,
      source: 'upstream/packages/node/src/genericProjection.ts',
      typeImports: [],
    });

    expect(lowered.diagnostics).toEqual([]);
    expect(output).toContain('Target::<T> {');

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

  it('preserves string value namespaces alongside their TypeScript type aliases', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/types/src/GamepadAxisKind.ts',
      `
        export const GamepadAxisKind = {
          STICK_LEFT_X: 'StickLeftX',
          STICK_LEFT_Y: 'StickLeftY',
        };
        export type GamepadAxisKind = (typeof GamepadAxisKind)[keyof typeof GamepadAxisKind];
        export const FallbackAxis: string = 'None';
        export const DefaultAxes: readonly (GamepadAxisKind | undefined)[] = [
          GamepadAxisKind.STICK_LEFT_X,
        ];
        export function firstAxis(): GamepadAxisKind {
          return GamepadAxisKind.STICK_LEFT_X;
        }
        export function fallbackAxis(): string {
          return FallbackAxis;
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/types', '/workspace');
    const output = emitRustModule({
      declarations: lowered.declarations,
      source: 'upstream/packages/types/src/GamepadAxisKind.ts',
      typeImports: [],
    });

    expect(lowered.diagnostics).toEqual([]);
    expect(output).toContain('pub struct GamepadAxisKindValues');
    expect(output).toContain('pub static GAMEPAD_AXIS_KIND');
    expect(output).toContain('pub type GamepadAxisKind = String');
    expect(output).toContain("pub const FALLBACK_AXIS: &'static str");
    expect(output).toContain('vec![Some((GAMEPAD_AXIS_KIND.stick_left_x).clone())]');
    expect(output).toContain('FALLBACK_AXIS).clone()).to_owned()');

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

  it('projects structurally compatible object spreads across distinct Rust structs', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/input/src/manager.ts',
      `
        interface Signals {
          readonly label: string;
          readonly count: number;
        }
        interface Manager extends Signals {
          readonly enabled: boolean;
        }
        function createSignals(): Signals {
          return { label: 'input', count: 1 };
        }
        export function createManager(): Manager {
          return { ...createSignals(), enabled: true };
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/input', '/workspace');
    const output = emitRustModule({
      declarations: lowered.declarations,
      source: 'upstream/packages/input/src/manager.ts',
      typeImports: [],
    });

    expect(lowered.diagnostics).toEqual([]);
    expect(output).toContain('let __flight_spread_0 = create_signals()');
    expect(output).toContain('label: (__flight_spread_0.label).clone()');
    expect(output).not.toContain('..(create_signals()).clone()');

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

  it('keeps mutated numeric records as mutex-backed state instead of value namespaces', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/input/src/state.ts',
      `
        const eventData: { value: number } = { value: 0 };
        export function updateEventData(value: number): number {
          eventData.value = value;
          return eventData.value;
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/input', '/workspace');
    const output = emitRustModule({
      declarations: lowered.declarations,
      source: 'upstream/packages/input/src/state.ts',
      typeImports: [],
    });

    expect(lowered.diagnostics).toEqual([]);
    expect(output).toContain('static EVENT_DATA: std::sync::LazyLock<std::sync::Mutex<EventData>>');
    expect(output).not.toContain('struct eventData;');

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

  it('compiles callback-valued weak maps with static closure access and nullable narrowing', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/lifecycle/src/subscriptions.ts',
      `
        export interface Owner {
          readonly name: string;
        }
        const subscriptions = new WeakMap<Owner, () => void>();
        export function attach(owner: Owner): void {
          subscriptions.set(owner, () => {
            subscriptions.delete(owner);
          });
        }
        export function attachCallback(owner: Owner, cleanup: () => void): void {
          subscriptions.set(owner, cleanup);
        }
        export function attachEmpty(owner: Owner): void {
          attachCallback(owner, () => {});
        }
        export function detach(owner: Owner): void {
          const unsubscribe = subscriptions.get(owner);
          if (unsubscribe !== undefined) unsubscribe();
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/lifecycle', '/workspace');
    const output = emitRustModule({
      declarations: lowered.declarations,
      source: 'upstream/packages/lifecycle/src/subscriptions.ts',
      typeImports: [],
    });

    expect(lowered.diagnostics).toEqual([]);
    expect(output).not.toContain('let subscriptions = subscriptions.clone()');
    expect(output).toContain('unsubscribe.as_ref().unwrap()');
    expect(output).toContain('cleanup: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send +');
    expect(output).not.toContain('cleanup: &mut impl FnMut');

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

  it('preserves TypeScript symbol identity in generated collection keys', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/application/src/symbols.ts',
      `
        const first = Symbol();
        const second = Symbol();
        const values = new Map<symbol, number>();
        export function store(value: number): void {
          values.set(first, value);
        }
        export function load(): number {
          return values.get(first) ?? 0;
        }
        export function distinct(): boolean {
          return first !== second;
        }
        export function createKey(): symbol {
          return Symbol();
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/application', '/workspace');
    const output = emitRustModule({
      declarations: lowered.declarations,
      source: 'upstream/packages/application/src/symbols.ts',
      typeImports: [],
    });

    expect(lowered.diagnostics).toEqual([]);
    expect(output).toContain('std::sync::LazyLock<crate::FlightSymbol>');
    expect(output).toContain('crate::FlightSymbol::new()');
    expect(output).toContain('let __flight_key = *FIRST');

    const fixture = mkdtempSync(path.join(tmpdir(), 'flight-rs-emitter-'));
    const sourceFile = path.join(fixture, 'lib.rs');
    writeFileSync(
      sourceFile,
      output.replace(
        '// Source:',
        `
        #[derive(Clone, Copy, PartialEq)]
        pub struct FlightSymbol(u64);
        impl FlightSymbol {
          pub fn new() -> Self { Self(1) }
          pub fn for_name(_: &str) -> Self { Self(1) }
        }
        // Source:`,
      ),
    );
    expect(() =>
      execFileSync('rustc', ['--crate-type', 'lib', '--emit', 'metadata', '--edition', '2024', sourceFile], {
        cwd: fixture,
        stdio: 'pipe',
      }),
    ).not.toThrow();
  });

  it('fills contextual callback parameters omitted by TypeScript implementations', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/application/src/backend.ts',
      `
        export interface Backend {
          readonly open: (name: string, width: number) => boolean;
          readonly close: (name: string) => void;
          readonly impact: (amount?: number) => boolean;
          readonly read: (index?: number) => number;
        }
        export function createBackend(): Backend {
          return {
            open() {
              return true;
            },
            close() {},
            impact(amount) {
              const value = amount === undefined ? 0 : Math.min(1, amount);
              return value > 0;
            },
            read(index = 0) {
              return [1][index]!;
            },
          };
        }
        function notify(): void {}
        export function createListener(): () => void {
          return () => notify();
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/application', '/workspace');
    const output = emitRustModule({
      declarations: lowered.declarations,
      source: 'upstream/packages/application/src/backend.ts',
      typeImports: [],
    });

    expect(lowered.diagnostics).toEqual([]);
    expect(output).toContain('|__flight_unused_0: String, __flight_unused_1: f64|');
    expect(output).toContain('|__flight_unused_0: String|');
    expect(output).toContain('FnMut(Option<f64>) -> bool');
    expect(output).toContain('let index = index.unwrap_or(0.0_f64)');
    expect(output).toContain('move || -> () { notify() }');

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

  it('routes dynamically typed host reads, calls, and writes through the native host boundary', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/application/src/host.ts',
      `
        export function readWidth(element: any): number {
          return element.bounds.width;
        }
        export function attach(element: any, listener: () => void): void {
          element.addEventListener('resize', listener);
          element.style.touchAction = 'none';
        }
        export function isVisible(element: any): boolean {
          return !element.hidden;
        }
        export function isStandard(element: any): boolean {
          return element.mapping === 'standard';
        }
        export function readAxes(element: any): number[] {
          return Array.from(element.axes);
        }
        export function readText(element: any): string {
          return element.data ?? '';
        }
        export function hasTarget(element: any): boolean {
          return element.target !== null;
        }
        export function readCharCode(element: any): number {
          return element.key.toLowerCase().charCodeAt(0);
        }
        export function readLanguage(element: any | null): string {
          return element?.language ?? '';
        }
        export function hasValue(element: any | null): boolean {
          return element !== null && 'value' in element;
        }
        export function sumCoalesced(element: any): number {
          const values = typeof element.getCoalescedEvents === 'function' ? element.getCoalescedEvents() : null;
          if (!values) return 0;
          let total = 0;
          for (const value of values) total += value.x;
          return total;
        }
        export function countSegments(segmenter: any): number {
          let count = 0;
          for (const _segment of segmenter.segment('value')) {
            count += _segment.index + _segment.segment.length;
          }
          return count;
        }
        export function readAxis(gamepad: any | null, index: number): number {
          if (gamepad === null) return 0;
          return gamepad.axes[index];
        }
        export function readButton(gamepad: any | null, index: number): boolean {
          if (gamepad === null) return false;
          return gamepad.buttons[index].pressed;
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/application', '/workspace');
    const output = emitRustModule({
      declarations: lowered.declarations,
      source: 'upstream/packages/application/src/host.ts',
      typeImports: [],
    });

    expect(lowered.diagnostics).toEqual([]);
    expect(output).toContain('crate::host_value::<f64>("host.width")');
    expect(output).toContain('crate::host_value::<()>("host.addEventListener")');
    expect(output).toContain('crate::host_set("host.touchAction"');
    expect(output).toContain('crate::host_value::<bool>("host.hidden")');
    expect(output).toContain('crate::host_value::<String>("host.mapping")');
    expect(output).toContain('crate::host_value::<Vec<f64>>("host.Array.from")');
    expect(output).toContain('crate::host_value::<Option<String>>("host.data")');
    expect(output).toContain('crate::host_value::<Option<crate::OpaqueHostValue>>("host.target")');
    expect(output).toContain('crate::host_value::<f64>("host.call")');
    expect(output).toContain('crate::host_value::<Option<String>>("host.language")');
    expect(output).toContain('let values: Option<Vec<crate::OpaqueHostValue>> = None;');
    expect(output).toContain('for _segment in (crate::host_value::<Vec<crate::OpaqueHostValue>>("host.call"))');
    expect(output).toContain('crate::host_value::<f64>("host.index")');
    expect(output).toContain('crate::host_value::<String>("host.segment").encode_utf16().count() as f64');
    expect(output).toContain('expect("TypeScript nullable iterable was not narrowed").iter().cloned()');
    expect(output).toContain('crate::host_value::<f64>("host.index")');
    expect(output).toContain('crate::host_value::<bool>("host.pressed")');

    const fixture = mkdtempSync(path.join(tmpdir(), 'flight-rs-emitter-'));
    const sourceFile = path.join(fixture, 'lib.rs');
    writeFileSync(
      sourceFile,
      output.replace(
        '// Source:',
        `
        #[derive(Clone, Default)]
        pub struct OpaqueHostValue;
        pub fn host_value<T: Default>(_: &str) -> T { T::default() }
        pub fn host_set<T>(_: &str, value: T) -> T { value }
        // Source:`,
      ),
    );
    expect(() =>
      execFileSync('rustc', ['--crate-type', 'lib', '--emit', 'metadata', '--edition', '2024', sourceFile], {
        cwd: fixture,
        stdio: 'pipe',
      }),
    ).not.toThrow();
  });

  it('resolves imported indexed-access fields and exhaustive try/catch returns', () => {
    const fixture = mkdtempSync(path.join(tmpdir(), 'flight-rs-emitter-'));
    const typesDirectory = path.join(fixture, 'upstream/packages/types/src');
    mkdirSync(typesDirectory, { recursive: true });
    writeFileSync(
      path.join(typesDirectory, 'PointerEventData.ts'),
      `export type PointerType = 'mouse' | 'pen' | 'touch' | 'unknown';`,
    );
    writeFileSync(
      path.join(typesDirectory, 'InputPointerData.ts'),
      `
        import type { PointerType } from './PointerEventData';
        export interface InputPointerData {
          pointerType: PointerType;
        }
      `,
    );
    const inputSource = path.join(fixture, 'upstream/packages/input/src/input.ts');
    mkdirSync(path.dirname(inputSource), { recursive: true });
    const source = ts.createSourceFile(
      inputSource,
      `
        import type { InputPointerData } from '@flighthq/types';
        export function readPointerType(event: any): InputPointerData['pointerType'] {
          return event.pointerType;
        }
        export function recover(value: string): string {
          try {
            return value;
          } catch {
            return 'fallback';
          }
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/input', fixture);
    const output = emitRustModule({
      declarations: lowered.declarations,
      source: 'upstream/packages/input/src/input.ts',
      typeImports: [],
    });

    expect(lowered.diagnostics).toEqual([]);
    expect(output).toContain('pub fn read_pointer_type(event: crate::OpaqueHostValue) -> String');
    expect(output).toContain('crate::host_value::<String>("host.pointerType")');
    expect(output).toContain('return __flight_try_return.expect("TypeScript try/catch completed without returning");');

    const sourceFile = path.join(fixture, 'lib.rs');
    writeFileSync(
      sourceFile,
      output.replace(
        '// Source:',
        `
        #[derive(Clone, Default)]
        pub struct OpaqueHostValue;
        pub fn host_value<T: Default>(_: &str) -> T { T::default() }
        // Source:`,
      ),
    );
    expect(() =>
      execFileSync('rustc', ['--crate-type', 'lib', '--emit', 'metadata', '--edition', '2024', sourceFile], {
        cwd: fixture,
        stdio: 'pipe',
      }),
    ).not.toThrow();
  });

  it('initializes self-referential callbacks through a shared recursive slot', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/application/src/recursive.ts',
      `
        export function createTick(): (value: number) => void {
          function tick(value: number): void {
            if (value > 0) tick(value - 1);
          }
          return tick;
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/application', '/workspace');
    const output = emitRustModule({
      declarations: lowered.declarations,
      source: 'upstream/packages/application/src/recursive.ts',
      typeImports: [],
    });

    expect(lowered.diagnostics).toEqual([]);
    expect(output).toContain('let __flight_recursive_tick');
    expect(output).toContain('*__flight_recursive_tick.lock().unwrap() = Some(tick.clone())');
    expect(output).not.toContain('let tick = tick.clone()');

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

  it('preserves nullable locals across owned insertion and mutable flow narrowing', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/application/src/state.ts',
      `
        export interface State {
          value: number;
        }
        export interface Holder {
          state: State | null;
          visible?: boolean;
        }
        const states = new Map<string, State>();
        export function getState(key: string): State {
          let state = states.get(key);
          if (state === undefined) {
            state = { value: 0 };
            states.set(key, state);
          }
          return state;
        }
        export function updateState(key: string): void {
          const state = states.get(key);
          if (state !== undefined) {
            state.value = 1;
          }
        }
        function mutateState(state: State): void {
          state.value = 2;
        }
        export function updateHolder(holder: Holder): void {
          if (holder.state !== null) {
            mutateState(holder.state);
          }
        }
        export function isHolderVisible(holder: Holder): boolean {
          return holder.state !== null && holder.visible ? true : false;
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/application', '/workspace');
    const output = emitRustModule({
      declarations: lowered.declarations,
      source: 'upstream/packages/application/src/state.ts',
      typeImports: [],
    });

    expect(lowered.diagnostics).toEqual([]);
    expect(output).toContain('.clone().unwrap()');
    expect(output).toContain('.as_mut().unwrap().value');
    expect(output).toContain('.state.as_mut().unwrap()');
    expect(output).toContain('(holder.visible).unwrap_or(false)');

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

  it('compiles array membership, host array conversion, and nullable value equality', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/application/src/array-identity.ts',
      `
        export interface Item {
          readonly name: string;
        }
        const selected = new Map<string, Item>();
        export function includes(values: readonly Item[], value: Item): boolean {
          return values.includes(value);
        }
        export function selectedIs(key: string, value: Item): boolean {
          return selected.get(key) === value;
        }
        export function fromHost(value: any): any[] {
          return Array.from(value);
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/application', '/workspace');
    const output = emitRustModule({
      declarations: lowered.declarations,
      source: 'upstream/packages/application/src/array-identity.ts',
      typeImports: [],
    });

    expect(lowered.diagnostics).toEqual([]);
    expect(output).toContain('.iter().any(|item| item == &__flight_value)');
    expect(output).toContain('== Some(');
    expect(output).toContain('host.Array.from');

    const fixture = mkdtempSync(path.join(tmpdir(), 'flight-rs-emitter-'));
    const sourceFile = path.join(fixture, 'lib.rs');
    writeFileSync(
      sourceFile,
      output.replace(
        '// Source:',
        `
        #[derive(Clone, Default)]
        pub struct OpaqueHostValue;
        pub fn host_value<T: Default>(_: &str) -> T { T::default() }
        // Source:`,
      ),
    );
    expect(() =>
      execFileSync('rustc', ['--crate-type', 'lib', '--emit', 'metadata', '--edition', '2024', sourceFile], {
        cwd: fixture,
        stdio: 'pipe',
      }),
    ).not.toThrow();
  });

  it('uses cancellable native handles for numerically cast interval ids', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/input/src/timer.ts',
      `
        export function createTimer(): () => void {
          let intervalId = 0;
          const callback = () => {};
          intervalId = setInterval(callback, 10) as unknown as number;
          return () => {
            clearInterval(intervalId);
            intervalId = 0;
          };
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/input', '/workspace');
    const output = emitRustModule({
      declarations: lowered.declarations,
      source: 'upstream/packages/input/src/timer.ts',
      typeImports: [],
    });

    expect(lowered.diagnostics).toEqual([]);
    expect(output).toContain('Option<crate::FlightTimeout>');
    expect(output).toContain('crate::set_interval');
    expect(output).toContain('crate::clear_interval');
    expect(output).toContain('= None');

    const fixture = mkdtempSync(path.join(tmpdir(), 'flight-rs-emitter-'));
    const sourceFile = path.join(fixture, 'lib.rs');
    writeFileSync(
      sourceFile,
      output.replace(
        '// Source:',
        `
        #[derive(Clone)]
        pub struct FlightTimeout;
        pub fn set_interval<F: FnMut() + Send + 'static>(_: F, _: f64) -> FlightTimeout { FlightTimeout }
        pub fn clear_interval(_: FlightTimeout) {}
        // Source:`,
      ),
    );
    expect(() =>
      execFileSync('rustc', ['--crate-type', 'lib', '--emit', 'metadata', '--edition', '2024', sourceFile], {
        cwd: fixture,
        stdio: 'pipe',
      }),
    ).not.toThrow();
  });

  it('reports every EntityRuntimeKey storage operation instead of emitting runtime stubs', () => {
    const fixtures = [
      'return source[EntityRuntimeKey];',
      'source[EntityRuntimeKey] = value;',
      'source[EntityRuntimeKey].binding = value;',
      'return delete source[EntityRuntimeKey];',
      'return EntityRuntimeKey in source;',
      'return { [EntityRuntimeKey]: value };',
    ];

    for (const [index, body] of fixtures.entries()) {
      const source = ts.createSourceFile(
        `/workspace/upstream/packages/entity/src/runtime-${String(index)}.ts`,
        `export function runtimeOperation(source: any, value: any): any { ${body} }`,
        ts.ScriptTarget.Latest,
        true,
        ts.ScriptKind.TS,
      );
      const lowered = lowerTypeScriptSource(source, '@flighthq/entity', '/workspace');

      expect(lowered.diagnostics).toEqual([]);
      expect(() =>
        emitRustModule({
          declarations: lowered.declarations,
          source: `upstream/packages/entity/src/runtime-${String(index)}.ts`,
          typeImports: [],
        }),
      ).toThrow(
        'EntityRuntimeKey storage requires an aggregate native entity runtime representation; refusing to erase observable runtime state',
      );
    }
  });

  it('reports unknown DOM typeof properties instead of assuming they are functions', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/application/src/capability.ts',
      `
        export function supportsUnknownCapability(): boolean {
          return typeof window.unknownCapability === 'function';
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/application', '/workspace');

    expect(lowered.diagnostics).toEqual([]);
    expect(() =>
      emitRustModule({
        declarations: lowered.declarations,
        source: 'upstream/packages/application/src/capability.ts',
        typeImports: [],
      }),
    ).toThrow('typeof window.unknownCapability has no configured host-property tag');
  });
});
