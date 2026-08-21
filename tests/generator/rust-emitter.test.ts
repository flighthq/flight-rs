import { execFileSync } from 'node:child_process';
import { mkdirSync, mkdtempSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import ts from 'typescript';

import { emitNativeHostCapabilityRuntime } from '../../tools/generator/src/emit/native-host.ts';
import { emitFlightTaskRuntime } from '../../tools/generator/src/emit/runtime.ts';
import { emitRustModule } from '../../tools/generator/src/emit/rust.ts';
import { lowerTypeScriptSource } from '../../tools/generator/src/lower/typescript.ts';

describe('Rust emission', () => {
  it('aligns host, numeric namespace, and erased generic field types with emitted Rust', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/types/src/schema-types.ts',
      `
        export const WireCode = { One: 1, Two: 2 } as const;
        export type WireCode = (typeof WireCode)[keyof typeof WireCode];
        export interface Phantom<Value> { count: number; }
        export interface Holder<Value> { phantom: Phantom<Value>; }
        export interface OptionalValue<Value> { value?: Value; }
        export interface Registry<Value> { entries: readonly OptionalValue<Value>[]; }
        export interface CallbackRegistries { callbacks: Registry<(value: string) => void>; }
        export interface BinaryView { buffer: ArrayBufferLike; }
        export const Severity = { Drop: 'Drop', Skip: 'Skip' } as const;
        export type Severity = (typeof Severity)[keyof typeof Severity];
        export interface Diagnostic { detail?: Readonly<Record<string, boolean | number | string>>; severity: Severity; }
        export function detailText(diagnostic: Diagnostic): string {
          let text = '';
          if (diagnostic.detail !== undefined) {
            const keys = Object.keys(diagnostic.detail).sort();
            for (const key of keys) text += \` \${key}=\${diagnostic.detail[key]}\`;
          }
          return text;
        }
        export function appendOptional(values: string[] | undefined): void {
          if (values === undefined) return;
          values.push('next');
        }
        export function hasOptional(values: Readonly<Record<string, string>> | null): boolean {
          return values !== null;
        }
        export function hasNoValues(): boolean { return hasOptional(null); }
        type Guard = (value: number) => void;
        let guard: Guard | null = null;
        export function setGuard(next: Guard | null): void { guard = next; }
        export function callGuard(value: number): void {
          if (guard === null) return;
          guard(value);
        }
        export interface Schedule { at?: Date; code: WireCode; }
        export type VendorKind = \`\${string}.\${string}\`;
        export function sameNumber(left: number, right: number): boolean { return Object.is(left, right); }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/types', '/workspace');
    const output = emitRustModule({
      declarations: lowered.declarations,
      source: 'upstream/packages/types/src/schema-types.ts',
      typeImports: [],
    });

    expect(lowered.diagnostics).toEqual([]);
    expect(output).toContain('pub struct Phantom {');
    expect(output).toContain('pub struct Holder {');
    expect(output).toContain('pub phantom: Phantom,');
    expect(output).toContain('impl<Value> Default for OptionalValue<Value>');
    expect(output).toContain('impl<Value> Default for Registry<Value>');
    expect(output).toContain(
      "pub callbacks: Registry<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>>>,",
    );
    expect(output).toContain('pub buffer: Vec<u8>,');
    expect(output).toContain('pub type Severity = String;');
    expect(output).toContain('.iter().map(|(entry_key, _)| entry_key.clone()).collect::<Vec<_>>()');
    expect(output).toContain('.find(|(entry_key, _)| entry_key == &(key).clone())');
    expect(output).toContain('text.push_str(&');
    expect(output).toContain('pub fn append_optional(values: &mut Option<Vec<String>>)');
    expect(output).toContain('return has_optional(&(None));');
    expect(output).toContain('(*GUARD.lock().unwrap()).as_ref().unwrap()');
    expect(output).toContain('pub at: Option<crate::OpaqueHostValue>,');
    expect(output).toContain('pub code: f64,');
    expect(output).toContain('pub type VendorKind = String;');
    expect(output).toContain(
      '__flight_left.to_bits() == __flight_right.to_bits() || (__flight_left.is_nan() && __flight_right.is_nan())',
    );
    expect(emitFlightTaskRuntime()).toContain('pub enum FlightUnion2<A, B>');
    expect(emitFlightTaskRuntime()).toContain('std::fmt::Display for FlightUnion2<A, B>');

    const fixture = mkdtempSync(path.join(tmpdir(), 'flight-rs-schema-types-'));
    const sourceFile = path.join(fixture, 'lib.rs');
    writeFileSync(
      sourceFile,
      output.replace(
        '// Source:',
        `
        #[derive(Clone)] pub struct OpaqueHostValue;
        #[derive(Clone)] pub enum FlightUnion2<A, B> { A(A), B(B) }
        impl<A: std::fmt::Display, B: std::fmt::Display> std::fmt::Display for FlightUnion2<A, B> {
          fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self { Self::A(value) => value.fmt(formatter), Self::B(value) => value.fmt(formatter) }
          }
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

  it('synthesizes records nested through inherited fields and union intersections', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/types/src/nested-aliases.ts',
      `
        export interface BaseRecord {
          metadata?: { label: string };
        }
        export interface ExtendedRecord extends BaseRecord {
          value: number;
        }
        export type VariantRecord = BaseRecord | (BaseRecord & { values: readonly number[] });
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/types', '/workspace');
    const output = emitRustModule({
      declarations: lowered.declarations,
      source: 'upstream/packages/types/src/nested-aliases.ts',
      typeImports: [],
    });

    expect(lowered.diagnostics).toEqual([]);
    expect(output).toContain('pub type VariantRecord = crate::FlightUnion2<');
    expect(output).toMatch(/pub struct ExtendedRecordRecord\d+/u);
    expect(output).toMatch(/pub struct VariantRecordRecord\d+/u);

    const fixture = mkdtempSync(path.join(tmpdir(), 'flight-rs-nested-aliases-'));
    const sourceFile = path.join(fixture, 'lib.rs');
    writeFileSync(sourceFile, `${output}\n#[derive(Clone, PartialEq)] pub enum FlightUnion2<A, B> { A(A), B(B) }\n`);
    expect(() =>
      execFileSync('rustc', ['--crate-type', 'lib', '--emit', 'metadata', '--edition', '2024', sourceFile], {
        cwd: fixture,
        stdio: 'pipe',
      }),
    ).not.toThrow();
  });

  it('emits typed record for-in loops while rejecting dynamic enumeration', () => {
    const lower = (body: string) =>
      lowerTypeScriptSource(
        ts.createSourceFile(
          '/workspace/upstream/packages/example/src/for-in.ts',
          body,
          ts.ScriptTarget.Latest,
          true,
          ts.ScriptKind.TS,
        ),
        '@flighthq/example',
        '/workspace',
      );
    const typed = lower(`
      export function recordKeys(values: Record<string, number>): string[] {
        const result: string[] = [];
        for (const key in values) result.push(key);
        return result;
      }
    `);
    const dynamic = lower(`
      export function dynamicKeys(values: any): string[] {
        const result: string[] = [];
        for (const key in values) result.push(key);
        return result;
      }
    `);
    const output = emitRustModule({
      declarations: typed.declarations,
      source: 'upstream/packages/example/src/for-in.ts',
      typeImports: [],
    });

    expect(typed.diagnostics).toEqual([]);
    expect(output).toContain('let __flight_keys: Vec<String> = (values).iter().map(|(key, _)| key.clone()).collect();');
    expect(output).toContain('for key in __flight_keys');
    expect(() =>
      emitRustModule({
        declarations: dynamic.declarations,
        source: 'upstream/packages/example/src/for-in.ts',
        typeImports: [],
      }),
    ).toThrow('dynamic for-in Rust enumeration is not implemented');
  });

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
        export function nextEven(value: number): number {
          return (value + 2) & ~1;
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
    expect(output).toContain('!__flight_js_to_i32(1.0_f64)');
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
        type Direction = 'L' | 'R';
        type DeclineReason = 'invalid' | 'inverted';
        type IndexReason = DeclineReason | 'missing';
        interface IndexNotice { reason?: IndexReason; }
        export interface RecursiveNode {
          value: number;
          parent: RecursiveNode | null;
        }
        interface TestSignal<Value> { slots: Value[]; }
        interface SignalOwner { signal: TestSignal<(value: number) => void> | null; }
        function clearTestSignal<Value>(signal: TestSignal<Value>): void { signal.slots.length = 0; }
        function connectTestSignal<Value>(signal: TestSignal<Value>, slot: Value): void { signal.slots.push(slot); }
        const lookup = [1, 2, 3];
        const lookupCount = lookup.length;
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
        export function fillBytes(values: Uint8Array, value: number): Uint8Array {
          return values.fill(value);
        }
        export function prefillInts(length: number): Int32Array {
          return new Int32Array(length).fill(length);
        }
        export function lastLookupIndex(): number {
          return lookupCount - 1;
        }
        export function optionalDirection(code: number): Direction | null {
          return code === 1 ? 'R' : code === 2 ? 'L' : null;
        }
        export function reuseDirection(seed: Direction): Direction {
          let current: Direction = seed;
          current = seed;
          for (let index = 0; index < 2; index++) current = seed;
          return current;
        }
        export function firstPresentDirection(values: (Direction | null)[]): Direction {
          if (values[0] !== null) return values[0] as Direction;
          return 'L';
        }
        export function switchDirection(direction: Direction): Direction {
          switch (direction) {
            case 'L':
              return direction;
            default:
              return 'R';
          }
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
        export function sumEntries(map: Map<number, number>): number {
          let total = 0;
          for (const [key, value] of map) total += key + value;
          return total;
        }
        export function hasInvalidReason(notice: Readonly<IndexNotice>): boolean {
          return notice.reason === 'invalid';
        }
        export function attachParent(node: RecursiveNode, parent: RecursiveNode): void {
          node.parent = parent;
        }
        export function copyParent(node: RecursiveNode, source: RecursiveNode): void {
          node.parent = source.parent;
        }
        export function parentOf(node: Readonly<RecursiveNode>): RecursiveNode | null {
          return node.parent;
        }
        export function createChild(parent: RecursiveNode): RecursiveNode {
          return { value: 1, parent };
        }
        function detachParent(parent: RecursiveNode, child: RecursiveNode): void {
          parent.value = 0;
          child.parent = null;
        }
        export function detach(node: RecursiveNode): void {
          if (node.parent !== null) detachParent(node.parent, node);
        }
        export function clearOwnerSignal(owner: SignalOwner): void {
          if (owner.signal !== null) clearTestSignal(owner.signal);
        }
        export function connectOwnerSignal(owner: SignalOwner): void {
          const slot = (_value: number): void => {};
          if (owner.signal !== null) connectTestSignal(owner.signal, slot);
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
    expect(output).toContain('let __flight_value = (value) as u8; let __flight_collection = &mut *values;');
    expect(output).toContain('__flight_collection.fill(__flight_value);');
    expect(output).toContain('let mut __flight_collection = vec![0_i32; (length) as usize];');
    expect(output).toContain('let __flight_value = (length) as i32; __flight_collection.fill(__flight_value);');
    expect(output).toContain('static LOOKUP_COUNT: std::sync::LazyLock<f64>');
    expect(output).toContain('return (*LOOKUP_COUNT - 1.0_f64);');
    expect(output).toContain('if (code == 1.0_f64) { Some("R".to_owned()) }');
    expect(output).toContain('let mut current: Direction = (seed).clone();');
    expect(output).toContain('current = (seed).clone();');
    expect(output).toContain('return values[0.0_f64 as usize].clone().unwrap();');
    expect(output).toContain('let __switch_value = (direction).clone();');
    expect(output).toContain('values: &Option<Vec<u8>>');
    expect(output).toContain('copy_lookup(out, &(Some(');
    expect(output).toContain('pub names: Option<Vec<String>>');
    expect(output).toContain('.iter().map(|value| (*value) as f32).collect()');
    expect(output).toContain('let copy: Vec<f32>');
    expect(output).toContain('let normalized = ((values[index as usize] as f64) / 2.0_f64)');
    expect(output).toContain('let __flight_values: Vec<i16>');
    expect(output).toContain('.position(|item| item == &__flight_value).map_or(-1.0_f64');
    expect(output).toMatch(/map\.len\(\) as f64.*set\.len\(\) as f64/u);
    expect(output).toContain('let key = __iteration0.0.clone();');
    expect(output).toContain('let value = __iteration0.1.clone();');
    expect(output).toContain('.as_ref().map(|value| value.to_string()) == Some("invalid".to_owned())');
    expect(output).toContain('pub parent: Option<Box<RecursiveNode>>');
    expect(output).toContain('node.parent = Some(Box::new((*parent).clone()))');
    expect(output).toContain('node.parent = ((source.parent).as_deref().cloned()).map(Box::new)');
    expect(output).toContain('return (node.parent).as_deref().cloned()');
    expect(output).toContain('parent: Some(Box::new((*parent).clone()))');
    expect(output).toContain('__flight_argument_0 = node.parent.replace(Box::new(Default::default()))');
    expect(output).toContain('detach_parent(&mut *__flight_argument_0, node)');
    expect(output).toContain('if node.parent.is_some() { node.parent = Some(__flight_argument_0); }');
    expect(output).not.toContain('unsafe {');
    expect(output).toContain('clear_test_signal(owner.signal.as_mut().unwrap())');
    expect(output).toContain('connect_test_signal(owner.signal.as_mut().unwrap(), (slot).clone())');
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
    expect(output).toContain('crate::flight_json_stringify');
    expect(output).toContain('(values).is_some()');
    expect(output).toContain('&{ let __flight_source = &(value); BasePosition {');
    expect(output).toContain('x: __flight_source.x');

    const fixture = mkdtempSync(path.join(tmpdir(), 'flight-rs-emitter-'));
    expect(() => compileRustLibraryWithRuntime(output, fixture)).not.toThrow();
  });

  it('compiles and runs JavaScript string operations over UTF-16 code units', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/textbidi/src/code-points.ts',
      `
        export function inspectCodePoint(value: string, index: number): number[] {
          return [value.length, value.codePointAt(index) as number];
        }
        export function findString(value: string, search: string, position: number): number {
          return value.indexOf(search, position);
        }
        export function sliceString(value: string, start: number, end?: number): string {
          return value.slice(start, end).toLowerCase();
        }
        export function readStringUnit(value: string, index: number): string {
          return value[index];
        }
        export function stringFromPoint(codePoint: number): string {
          return String.fromCodePoint(codePoint);
        }
        export function hasString(value: string): boolean {
          return value ? true : false;
        }
        export function joinStrings(left: string, right: string): string {
          return left + right;
        }
        export function joinEntries(values: string[], separator?: string): string {
          return values.join(separator);
        }
        export function joinNumbers(values: number[]): string {
          return values.join(',');
        }
        export function normalizeEntries(values: Record<string, string>): string[] {
          values['added'] = 'yes';
          return Object.entries(values)
            .filter(([key]) => key.length > 0)
            .map(([key, value]) => \`\${key}=\${value}\`);
        }
        export function isAllowed(value: string): boolean {
          const allowed = new Set(['alpha', 'beta']);
          return allowed.has(value);
        }
        export function hasEntry(values: Record<string, string>, key: string): boolean {
          return key in values;
        }
        export function assignedValue(value: string): string {
          let output = '';
          return (output = value);
        }
        export function normalizeOptional(value?: string): string {
          if (value === undefined || value.trim() === '') return '';
          return value;
        }
        export function encodeComponent(value: string): string {
          return encodeURIComponent(value);
        }
        export function safeDecodeComponent(value: string): string {
          try {
            return decodeURIComponent(value);
          } catch {
            return 'invalid';
          }
        }
        export function numberValue(value: string): number {
          return Number(value);
        }
        export interface Registry {
          values?: Map<string, string>;
        }
        export function putRegistry(registry: Registry, key: string, value: string): string {
          (registry.values ??= new Map()).set(key, value);
          return registry.values.get(key) ?? '';
        }
        export function readRegistry(registry: Readonly<Registry>, key: string): string {
          return registry.values?.get(key) ?? '';
        }
        export function initializeOptional(value?: string): string {
          return (value ??= 'default');
        }
        export interface RequiredValue {
          value: number;
        }
        export function retainRequired(value: RequiredValue): number {
          value.value ??= 3;
          return value.value;
        }
        export interface OptionalRunner {
          run?: (value: string) => void;
        }
        export function runOptional(runner: OptionalRunner, value: string): void {
          runner.run?.(value);
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/textbidi', '/workspace');
    const output = emitRustModule({
      declarations: lowered.declarations,
      source: 'upstream/packages/textbidi/src/code-points.ts',
      typeImports: [],
    });

    expect(lowered.diagnostics).toEqual([]);
    expect(output).toContain('let __flight_utf16_value: Vec<u16> = value.encode_utf16().collect();');
    expect(output).toContain('(__flight_utf16_value.len() as f64)');
    expect(output).toContain('let __flight_units: &[u16] = &__flight_utf16_value');
    expect(output).toContain('__flight_string_index_of');
    expect(output).toContain('__flight_string_slice');
    expect(output).toContain('__flight_string_from_code_point');
    expect(output).toContain('__flight_encode_uri_component');
    expect(output).toContain('__flight_decode_uri_component');
    expect(output).toContain('__flight_number_from_string');
    expect(output).toContain('!(value).is_empty()');

    const fixture = mkdtempSync(path.join(tmpdir(), 'flight-rs-utf16-code-point-'));
    const binary = path.join(fixture, 'utf16-code-point');
    writeFileSync(path.join(fixture, 'generated.rs'), output);
    writeFileSync(
      path.join(fixture, 'main.rs'),
      [
        'mod generated;',
        'fn main() {',
        '  assert_eq!(generated::inspect_code_point("A😀Z".to_owned(), 1.0), vec![4.0, 0x1F600_u32 as f64]);',
        '  assert_eq!(generated::inspect_code_point("A😀Z".to_owned(), 2.0), vec![4.0, 0xDE00_u32 as f64]);',
        '  assert_eq!(generated::inspect_code_point("A😀Z".to_owned(), f64::NAN), vec![4.0, 65.0]);',
        '  assert!(generated::inspect_code_point("A😀Z".to_owned(), 4.0)[1].is_nan());',
        '  assert_eq!(generated::find_string("A😀Z😀".to_owned(), "😀".to_owned(), 0.0), 1.0);',
        '  assert_eq!(generated::find_string("A😀Z😀".to_owned(), "😀".to_owned(), 2.0), 4.0);',
        '  assert_eq!(generated::slice_string("A😀Z".to_owned(), 1.0, Some(3.0)), "😀");',
        '  assert_eq!(generated::slice_string("A😀Z".to_owned(), -1.0, None), "z");',
        '  assert_eq!(generated::read_string_unit("A😀Z".to_owned(), 3.0), "Z");',
        '  assert_eq!(generated::read_string_unit("A😀Z".to_owned(), 4.0), "");',
        '  assert_eq!(generated::string_from_point(0x1F600_u32 as f64), "😀");',
        '  assert!(!generated::has_string(String::new()));',
        '  assert!(generated::has_string("value".to_owned()));',
        '  assert_eq!(generated::join_strings("left".to_owned(), "right".to_owned()), "leftright");',
        '  assert_eq!(generated::join_entries(&vec!["a".to_owned(), "b".to_owned()], Some(" / ".to_owned())), "a / b");',
        '  assert_eq!(generated::join_numbers(&vec![1.0, 2.5]), "1,2.5");',
        '  let mut entries = vec![("first".to_owned(), "1".to_owned())];',
        '  assert_eq!(generated::normalize_entries(&mut entries), vec!["first=1".to_owned(), "added=yes".to_owned()]);',
        '  assert!(generated::is_allowed("alpha".to_owned()));',
        '  assert!(!generated::is_allowed("gamma".to_owned()));',
        '  assert!(generated::has_entry(&entries, "first".to_owned()));',
        '  assert!(!generated::has_entry(&entries, "missing".to_owned()));',
        '  assert_eq!(generated::assigned_value("kept".to_owned()), "kept");',
        '  assert_eq!(generated::normalize_optional(None), "");',
        '  assert_eq!(generated::normalize_optional(Some("  ".to_owned())), "");',
        '  assert_eq!(generated::normalize_optional(Some("value".to_owned())), "value");',
        '  assert_eq!(generated::encode_component("a b/😀".to_owned()), "a%20b%2F%F0%9F%98%80");',
        '  assert_eq!(generated::safe_decode_component("a%20b%2F%F0%9F%98%80".to_owned()), "a b/😀");',
        '  assert_eq!(generated::safe_decode_component("%GG".to_owned()), "invalid");',
        '  assert_eq!(generated::number_value(" 42.5 ".to_owned()), 42.5);',
        '  assert_eq!(generated::number_value("0x10".to_owned()), 16.0);',
        '  assert!(generated::number_value("not a number".to_owned()).is_nan());',
        '  let mut registry = generated::Registry::default();',
        '  assert_eq!(generated::put_registry(&mut registry, "key".to_owned(), "value".to_owned()), "value");',
        '  assert_eq!(generated::read_registry(&registry, "key".to_owned()), "value");',
        '  assert_eq!(generated::read_registry(&generated::Registry::default(), "missing".to_owned()), "");',
        '  assert_eq!(generated::initialize_optional(None), "default");',
        '  assert_eq!(generated::initialize_optional(Some("kept".to_owned())), "kept");',
        '  assert_eq!(generated::retain_required(&mut generated::RequiredValue::default()), 0.0);',
        '  generated::run_optional(&mut generated::OptionalRunner::default(), "ignored".to_owned());',
        '}',
        '',
      ].join('\n'),
    );
    expect(() => compileRustExecutable('main.rs', binary, fixture)).not.toThrow();
    expect(() => execFileSync(binary, [], { cwd: fixture, stdio: 'pipe' })).not.toThrow();
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

  it('emits typed straight-line tasks and reports opaque output or composition instead of fabricating tasks', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/power/src/promise.ts',
      `
        export async function echoAfterReady(input: string): Promise<string> {
          const ready = Promise.resolve<string>(input);
          const value = await ready;
          return value;
        }
        export async function adoptReady(input: string): Promise<string> {
          return Promise.resolve(input);
        }
        export async function inferredFlag(input: boolean) {
          return input;
        }
        export function readyFlag(): Promise<boolean> {
          return Promise.resolve(true);
        }
        export function rejectedFlag(): Promise<boolean> {
          return Promise.reject('nope');
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
    expect(lowered.asyncTasks).toHaveLength(3);
    expect(lowered.asyncTasks[0]).toMatchObject({
      execution: {
        kind: 'portableTask',
        origin: { lexicalPath: 'echoAfterReady' },
      },
      operations: { awaits: 1, promiseResolve: 1 },
      matchesLegacyErasurePath: true,
      output: { kind: 'primitive', name: 'String' },
    });
    expect(lowered.taskConstructions.map((item) => item.kind)).toEqual([
      'async-scope',
      'ready',
      'async-scope',
      'ready',
      'async-scope',
      'ready',
      'reject',
    ]);
    expect(lowered.taskConstructions.at(-1)?.output).toEqual({
      kind: 'primitive',
      name: 'Bool',
    });
    expect(output).toContain('pub fn echo_after_ready(input: String) -> crate::FlightTask<String>');
    expect(output).toContain('crate::FlightTask::start(async move');
    expect(output).toContain('.await?');
    expect(output).toContain('pub fn adopt_ready(input: String) -> crate::FlightTask<String>');
    expect(output).toContain('return crate::FlightTask::ready(');
    expect(output).toContain('.await;');
    expect(output).toContain('pub fn inferred_flag(input: bool) -> crate::FlightTask<bool>');
    expect(output).toContain('pub fn ready_flag() -> crate::FlightTask<bool>');
    expect(output).toContain('pub fn rejected_flag() -> crate::FlightTask<bool>');
    expect(output).toContain('crate::FlightRejection::String("nope".to_owned())');
    expect(output).not.toContain('Promise');
    expect(output).not.toContain('Default::default()');

    const fixture = mkdtempSync(path.join(tmpdir(), 'flight-rs-emitter-'));
    mkdirSync(path.join(fixture, 'src'), { recursive: true });
    writeFileSync(path.join(fixture, 'src', 'generated.rs'), output);
    writeFileSync(
      path.join(fixture, 'src', 'lib.rs'),
      [
        'pub use flighthq_runtime::*;',
        'mod generated;',
        'pub use generated::*;',
        '#[cfg(test)] mod tests {',
        '  use super::*;',
        '  #[test] fn generated_task_preserves_owned_input_and_nested_await() {',
        '    let scheduler = install_deterministic_flight_task_scheduler();',
        '    let source = String::from("owned");',
        '    let task = echo_after_ready(source);',
        '    assert_eq!(scheduler.block_on(task), Ok(String::from("owned")));',
        '    assert_eq!(scheduler.block_on(adopt_ready(String::from("adopted"))), Ok(String::from("adopted")));',
        '    assert_eq!(scheduler.block_on(inferred_flag(true)), Ok(true));',
        '    assert_eq!(scheduler.block_on(ready_flag()), Ok(true));',
        '    assert_eq!(scheduler.block_on(rejected_flag()), Err(FlightTaskError::Rejection(FlightRejection::String(String::from("nope")))));',
        '  }',
        '}',
        '',
      ].join('\n'),
    );
    writeFileSync(
      path.join(fixture, 'Cargo.toml'),
      [
        '[package]',
        'name = "generated-task-fixture"',
        'version = "0.0.0"',
        'edition = "2024"',
        '[dependencies]',
        `flighthq-runtime = { path = ${JSON.stringify(path.join(process.cwd(), 'generated/crates/flighthq-runtime'))} }`,
        '',
      ].join('\n'),
    );
    expect(() =>
      execFileSync('cargo', ['test', '--quiet'], {
        cwd: fixture,
        env: { ...process.env, CARGO_TARGET_DIR: path.join(fixture, 'target') },
        stdio: 'pipe',
      }),
    ).not.toThrow();

    const opaque = lowerTypeScriptSource(
      ts.createSourceFile(
        '/workspace/upstream/packages/power/src/opaque.ts',
        'export async function opaque(value: any) { return value; }',
        ts.ScriptTarget.Latest,
        true,
        ts.ScriptKind.TS,
      ),
      '@flighthq/power',
      '/workspace',
    );
    expect(() =>
      emitRustModule({
        declarations: opaque.declarations,
        source: 'opaque.ts',
        typeImports: [],
      }),
    ).toThrow('portableTask opaque: async output type is not recovered');

    const composition = lowerTypeScriptSource(
      ts.createSourceFile(
        '/workspace/upstream/packages/power/src/composition.ts',
        'export function next(listener: () => void): Promise<void> { return Promise.resolve().then(listener); }',
        ts.ScriptTarget.Latest,
        true,
        ts.ScriptKind.TS,
      ),
      '@flighthq/power',
      '/workspace',
    );
    expect(composition.taskConstructions.map((item) => item.kind)).toEqual(['then', 'ready']);
    expect(() =>
      emitRustModule({
        declarations: composition.declarations,
        source: 'composition.ts',
        typeImports: [],
      }),
    ).toThrow('taskThen Rust lowering is reserved for Pass 27 Stage 4');
  });

  it('emits homogeneous taskAll joins and rejects mixed task/value collections', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/permissions/src/task-all.ts',
      `
        export function gather(value: string): Promise<string[]> {
          return Promise.all([Promise.resolve(value), Promise.resolve('tail')]);
        }
        export function gatherMapped(values: string[]): Promise<string[]> {
          return Promise.all(values.map((value) => Promise.resolve(value)));
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/permissions', '/workspace');
    const output = emitRustModule({
      declarations: lowered.declarations,
      source: 'upstream/packages/permissions/src/task-all.ts',
      typeImports: [],
    });

    expect(lowered.diagnostics).toEqual([]);
    expect(output).toContain('pub fn gather(value: String) -> crate::FlightTask<Vec<String>>');
    expect(output).toContain('pub fn gather_mapped(values: &Vec<String>) -> crate::FlightTask<Vec<String>>');
    expect(output).toContain('crate::FlightTask::all(vec![');
    expect(output).not.toContain('Promise');

    const fixture = mkdtempSync(path.join(tmpdir(), 'flight-rs-task-all-'));
    mkdirSync(path.join(fixture, 'src'), { recursive: true });
    writeFileSync(path.join(fixture, 'src', 'generated.rs'), output);
    writeFileSync(
      path.join(fixture, 'src', 'lib.rs'),
      [
        'pub use flighthq_runtime::*;',
        'mod generated;',
        'pub use generated::*;',
        '#[cfg(test)] mod tests {',
        '  use super::*;',
        '  #[test] fn generated_task_all_preserves_input_order() {',
        '    let scheduler = install_deterministic_flight_task_scheduler();',
        '    assert_eq!(scheduler.block_on(gather(String::from("head"))), Ok(vec![String::from("head"), String::from("tail")]));',
        '    assert_eq!(scheduler.block_on(gather_mapped(&vec![String::from("first"), String::from("second")])), Ok(vec![String::from("first"), String::from("second")]));',
        '  }',
        '}',
        '',
      ].join('\n'),
    );
    writeFileSync(
      path.join(fixture, 'Cargo.toml'),
      [
        '[package]',
        'name = "generated-task-all-fixture"',
        'version = "0.0.0"',
        'edition = "2024"',
        '[dependencies]',
        `flighthq-runtime = { path = ${JSON.stringify(path.join(process.cwd(), 'generated/crates/flighthq-runtime'))} }`,
        '',
      ].join('\n'),
    );
    expect(() =>
      execFileSync('cargo', ['test', '--quiet'], {
        cwd: fixture,
        env: { ...process.env, CARGO_TARGET_DIR: path.join(fixture, 'target') },
        stdio: 'pipe',
      }),
    ).not.toThrow();

    const mixed = lowerTypeScriptSource(
      ts.createSourceFile(
        '/workspace/upstream/packages/permissions/src/mixed-task-all.ts',
        `export function mixed(): Promise<string[]> {
          return Promise.all([Promise.resolve('task'), 'value']);
        }`,
        ts.ScriptTarget.Latest,
        true,
        ts.ScriptKind.TS,
      ),
      '@flighthq/permissions',
      '/workspace',
    );
    expect(() =>
      emitRustModule({
        declarations: mixed.declarations,
        source: 'mixed-task-all.ts',
        typeImports: [],
      }),
    ).toThrow('taskAll currently requires homogeneous task inputs matching its array output');
  });

  it('emits task-aware try/catch without catching host boundary failures', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/permissions/src/task-try.ts',
      `
        export function unavailable(host: any): Promise<string> {
          return host.load();
        }
        export async function recover(input: string): Promise<string> {
          try {
            await Promise.reject<void>('nope');
            return 'missed';
          } catch {
            return Promise.resolve(input);
          }
        }
        export async function preserve(input: string): Promise<string> {
          try {
            return await Promise.resolve(input);
          } catch {
            return 'unexpected';
          }
        }
        export async function recoverAndContinue(input: string, reject: boolean): Promise<string> {
          try {
            if (reject) {
              await Promise.reject<void>('nope');
            }
            if (input === 'return') {
              return input;
            }
          } catch {}
          return 'continued';
        }
        export async function preserveBoundary(host: any): Promise<string> {
          try {
            return await unavailable(host);
          } catch {
            return 'incorrectly caught';
          }
        }
        export async function makeRuntimeUnavailable(input: string): Promise<string> {
          return await Promise.resolve(input);
        }
        export async function preserveRuntimeBoundary(task: Promise<string>): Promise<string> {
          try {
            return await task;
          } catch {
            return 'incorrectly caught';
          }
        }
        export async function throwAndRecover(): Promise<string> {
          try {
            throw new Error('boom');
          } catch {
            return 'recovered';
          }
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/permissions', '/workspace');
    const output = emitRustModule({
      declarations: lowered.declarations,
      source: 'upstream/packages/permissions/src/task-try.ts',
      typeImports: [],
    });

    expect(lowered.diagnostics).toEqual([]);
    expect(output).toContain('Err(crate::FlightTaskError::Rejection(_))');
    expect(output).toContain('Err(__flight_error) => Err(__flight_error)');
    expect(output).toContain('if let Some(__flight_return) = __flight_try_return');
    expect(output).not.toContain('catch_unwind');

    const fixture = mkdtempSync(path.join(tmpdir(), 'flight-rs-task-try-'));
    mkdirSync(path.join(fixture, 'src'), { recursive: true });
    writeFileSync(path.join(fixture, 'src', 'generated.rs'), output);
    writeFileSync(
      path.join(fixture, 'src', 'lib.rs'),
      [
        'pub use flighthq_runtime::*;',
        '#[derive(Clone, Default)] pub struct OpaqueHostValue;',
        'pub fn host_value<T: Default>(_: &str) -> T { T::default() }',
        'mod generated;',
        'pub use generated::*;',
        '#[cfg(test)] mod tests {',
        '  use super::*;',
        '  #[test] fn generated_task_try_recovers_only_source_rejections() {',
        '    let runtime_failure = make_runtime_unavailable(String::from("unavailable"));',
        '    let scheduler = install_deterministic_flight_task_scheduler();',
        '    assert_eq!(scheduler.block_on(recover(String::from("recovered"))), Ok(String::from("recovered")));',
        '    assert_eq!(scheduler.block_on(preserve(String::from("preserved"))), Ok(String::from("preserved")));',
        '    assert_eq!(scheduler.block_on(recover_and_continue(String::from("return"), false)), Ok(String::from("return")));',
        '    assert_eq!(scheduler.block_on(recover_and_continue(String::from("ignored"), true)), Ok(String::from("continued")));',
        '    assert_eq!(scheduler.block_on(throw_and_recover()), Ok(String::from("recovered")));',
        '    assert!(matches!(scheduler.block_on(preserve_boundary(OpaqueHostValue)), Err(FlightTaskError::HostUnavailable(_))));',
        '    assert!(matches!(scheduler.block_on(preserve_runtime_boundary(runtime_failure)), Err(FlightTaskError::RuntimeUnavailable(_))));',
        '  }',
        '}',
        '',
      ].join('\n'),
    );
    writeFileSync(
      path.join(fixture, 'Cargo.toml'),
      [
        '[package]',
        'name = "generated-task-try-fixture"',
        'version = "0.0.0"',
        'edition = "2024"',
        '[dependencies]',
        `flighthq-runtime = { path = ${JSON.stringify(path.join(process.cwd(), 'generated/crates/flighthq-runtime'))} }`,
        '',
      ].join('\n'),
    );
    const catchBinding = lowerTypeScriptSource(
      ts.createSourceFile(
        '/workspace/upstream/packages/permissions/src/task-catch-binding.ts',
        `export async function unsupported(): Promise<string> {
          try { return await Promise.reject<string>('nope'); }
          catch (error) { return String(error); }
        }`,
        ts.ScriptTarget.Latest,
        true,
        ts.ScriptKind.TS,
      ),
      '@flighthq/permissions',
      '/workspace',
    );
    expect(() =>
      emitRustModule({
        declarations: catchBinding.declarations,
        source: 'task-catch-binding.ts',
        typeImports: [],
      }),
    ).toThrow(/task-catch-binding\.ts:\d+:\d+: portable task catch bindings are not implemented/u);

    const finallyBlock = lowerTypeScriptSource(
      ts.createSourceFile(
        '/workspace/upstream/packages/permissions/src/task-finally.ts',
        `export async function unsupported(): Promise<string> {
          try { return await Promise.resolve('value'); }
          catch { return 'recovered'; }
          finally { Promise.resolve(); }
        }`,
        ts.ScriptTarget.Latest,
        true,
        ts.ScriptKind.TS,
      ),
      '@flighthq/permissions',
      '/workspace',
    );
    expect(() =>
      emitRustModule({
        declarations: finallyBlock.declarations,
        source: 'task-finally.ts',
        typeImports: [],
      }),
    ).toThrow(/task-finally\.ts:\d+:\d+: portable task try\/catch\/finally lowering is not implemented/u);

    expect(() =>
      execFileSync('cargo', ['test', '--quiet'], {
        cwd: fixture,
        env: { ...process.env, CARGO_TARGET_DIR: path.join(fixture, 'target') },
        stdio: 'pipe',
      }),
    ).not.toThrow();
  });

  it('represents typed dynamic host tasks without requiring a default output value', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/input/src/hostTask.ts',
      'export function load(host: any): Promise<string> { return host.load(); }',
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/input', '/workspace');
    const output = emitRustModule({
      declarations: lowered.declarations,
      source: 'upstream/packages/input/src/hostTask.ts',
      typeImports: [],
    });

    expect(lowered.diagnostics).toEqual([]);
    expect(output).toContain('pub fn load(host: crate::OpaqueHostValue) -> crate::FlightTask<String>');
    expect(output).toContain('crate::host_task::<String>("host.load")');
    expect(output).not.toContain('host_value::<crate::FlightTask<String>>');

    const fixture = mkdtempSync(path.join(tmpdir(), 'flight-rs-emitter-'));
    mkdirSync(path.join(fixture, 'src'), { recursive: true });
    writeFileSync(path.join(fixture, 'src', 'generated.rs'), output);
    writeFileSync(
      path.join(fixture, 'src', 'lib.rs'),
      [
        'pub use flighthq_runtime::*;',
        '#[derive(Clone, Default)] pub struct OpaqueHostValue;',
        'pub fn host_value<T: Default>(_: &str) -> T { T::default() }',
        'mod generated;',
        'pub use generated::*;',
        '',
      ].join('\n'),
    );
    writeFileSync(
      path.join(fixture, 'Cargo.toml'),
      [
        '[package]',
        'name = "generated-host-task-fixture"',
        'version = "0.0.0"',
        'edition = "2024"',
        '[dependencies]',
        `flighthq-runtime = { path = ${JSON.stringify(path.join(process.cwd(), 'generated/crates/flighthq-runtime'))} }`,
        '',
      ].join('\n'),
    );
    expect(() =>
      execFileSync('cargo', ['check', '--quiet'], {
        cwd: fixture,
        env: { ...process.env, CARGO_TARGET_DIR: path.join(fixture, 'target') },
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
        export function normalizeCapturedVersion(value: string): string {
          const match = /version\\/([\\d_]+)/i.exec(value);
          return match ? match[1].replace(/_/g, '.') : '';
        }
        export function optionalCapture(value: string): string {
          return value.replace(/#(\\d+)|#x([\\da-f]+)/gi, (reference, decimal, hexadecimal) => {
            const numeric = decimal ?? hexadecimal;
            return numeric !== undefined ? numeric : reference;
          });
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
    expect(output).toContain('captures.get(index).map(|matched| matched.as_str().to_owned())');
    expect(output).toContain('[1.0_f64 as usize].clone().unwrap()');
    expect(output).toContain('decimal: Option<String>, hexadecimal: Option<String>');
    expect(output).toContain('captures.get(1).map(|matched| matched.as_str().to_owned())');
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
    expect(() => compileRustLibraryWithRuntime(output, fixture)).not.toThrow();
  });

  it('compiles and runs recursive portable JSON with JavaScript container semantics', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/log/src/json.ts',
      `
        export function recursiveJson(): string {
          const base: Record<string, unknown> = { first: 1, omitted: undefined };
          return JSON.stringify({
            ...base,
            nested: { escaped: 'line\\n"\\\\' },
            array: [undefined, NaN, Infinity, () => 1],
            zero: -0,
            first: 'last',
          });
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/log', '/workspace');
    const output = emitRustModule({
      declarations: lowered.declarations,
      source: 'upstream/packages/log/src/json.ts',
      typeImports: [],
    });

    expect(lowered.diagnostics).toEqual([]);
    expect(output).toContain('crate::FlightValue::Record');
    expect(output).toContain('crate::FlightValue::Array');
    expect(output).toContain('crate::FlightValue::Function');

    const fixture = mkdtempSync(path.join(tmpdir(), 'flight-rs-recursive-json-'));
    writeFileSync(path.join(fixture, 'flight_runtime.rs'), emitFlightTaskRuntime());
    writeFileSync(path.join(fixture, 'generated.rs'), output);
    writeFileSync(
      path.join(fixture, 'main.rs'),
      [
        'mod flight_runtime;',
        'pub use flight_runtime::*;',
        'mod generated;',
        'fn main() {',
        '  assert_eq!(generated::recursive_json(), "{\\"first\\":\\"last\\",\\"nested\\":{\\"escaped\\":\\"line\\\\n\\\\\\\"\\\\\\\\\\"},\\"array\\":[null,null,null,null],\\"zero\\":0}");',
        '}',
        '',
      ].join('\n'),
    );
    const binary = path.join(fixture, 'recursive-json');
    expect(() => compileRustExecutable('main.rs', binary, fixture)).not.toThrow();
    expect(() => execFileSync(binary, [], { cwd: fixture, stdio: 'pipe' })).not.toThrow();
  });

  it('rejects JSON fields whose Rust storage collapses omitted and explicit null', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/log/src/optional-null.ts',
      `
        interface OptionalNull { value?: string | null; }
        export function optionalNullJson(value: OptionalNull): string {
          return JSON.stringify(value);
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/log', '/workspace');

    expect(lowered.diagnostics).toEqual([]);
    expect(() =>
      emitRustModule({
        declarations: lowered.declarations,
        source: 'upstream/packages/log/src/optional-null.ts',
        typeImports: [],
      }),
    ).toThrow('portable field value cannot distinguish an omitted property from explicit null');
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
        'pub type FlightValue = OpaqueHostValue;',
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

  it('resolves named union aliases before emitting typeof matches', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/log/src/typeof-union.ts',
      `
        export type LogData = string | Readonly<Record<string, unknown>>;
        export function isLogString(data: LogData): boolean {
          return typeof data === 'string';
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/log', '/workspace');
    const output = emitRustModule({
      declarations: lowered.declarations,
      source: 'upstream/packages/log/src/typeof-union.ts',
      typeImports: [],
    });

    expect(lowered.diagnostics).toEqual([]);
    expect(output).toContain(
      'match &(data) { crate::FlightUnion2::A(_) => "string", crate::FlightUnion2::B(value) => "object" }',
    );

    const fixture = mkdtempSync(path.join(tmpdir(), 'flight-rs-typeof-union-'));
    const binary = path.join(fixture, 'typeof-union');
    writeFileSync(path.join(fixture, 'generated.rs'), output);
    writeFileSync(
      path.join(fixture, 'main.rs'),
      [
        '#[derive(Clone)]',
        'pub enum OpaqueHostValue { String(String) }',
        'pub type FlightValue = OpaqueHostValue;',
        '#[derive(Clone)]',
        'pub enum FlightUnion2<A, B> { A(A), B(B) }',
        'mod generated;',
        'fn main() {',
        '  let text = FlightUnion2::A("hello".to_owned());',
        '  let record = FlightUnion2::B(vec![("message".to_owned(), OpaqueHostValue::String("hello".to_owned()))]);',
        '  assert!(generated::is_log_string(&text));',
        '  assert!(!generated::is_log_string(&record));',
        '}',
        '',
      ].join('\n'),
    );
    expect(() => compileRustExecutable('main.rs', binary, fixture)).not.toThrow();
    expect(() => execFileSync(binary, [], { cwd: fixture, stdio: 'pipe' })).not.toThrow();
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

  it('infers spread records and preserves ordered JavaScript overwrite semantics', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/log/src/ordered-record-spreads.ts',
      `
        export function mergeRecords(
          parent: Readonly<Record<string, unknown>>,
          fields: Readonly<Record<string, unknown>>,
        ): Record<string, unknown> {
          const merged = {
            first: 'first',
            ...parent,
            middle: 'middle',
            ...fields,
            shared: 'final',
          };
          return merged;
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/log', '/workspace');
    const output = emitRustModule({
      declarations: lowered.declarations,
      source: 'upstream/packages/log/src/ordered-record-spreads.ts',
      typeImports: [],
    });

    expect(lowered.diagnostics).toEqual([]);
    expect(output).toContain('let __flight_spread_1 = (*parent).clone();');
    expect(output).toContain('let __flight_spread_3 = (*fields).clone();');
    expect(output).toContain('__flight_record.iter_mut().find');
    expect(output.indexOf('let __flight_key_0')).toBeLessThan(output.indexOf('let __flight_spread_1'));
    expect(output.indexOf('let __flight_spread_1')).toBeLessThan(output.indexOf('let __flight_key_2'));
    expect(output.indexOf('let __flight_key_2')).toBeLessThan(output.indexOf('let __flight_spread_3'));
    expect(output.indexOf('let __flight_spread_3')).toBeLessThan(output.indexOf('let __flight_key_4'));

    const fixture = mkdtempSync(path.join(tmpdir(), 'flight-rs-record-spreads-'));
    const binary = path.join(fixture, 'record-spreads');
    writeFileSync(path.join(fixture, 'generated.rs'), output);
    writeFileSync(
      path.join(fixture, 'main.rs'),
      [
        '#[derive(Clone, Debug, PartialEq)]',
        'pub enum OpaqueHostValue { String(String) }',
        'pub type FlightValue = OpaqueHostValue;',
        'mod generated;',
        'fn value(text: &str) -> OpaqueHostValue { OpaqueHostValue::String(text.to_owned()) }',
        'fn main() {',
        '  let parent = vec![("shared".to_owned(), value("parent")), ("parent".to_owned(), value("parent"))];',
        '  let fields = vec![("shared".to_owned(), value("child")), ("child".to_owned(), value("child"))];',
        '  let merged = generated::merge_records(&parent, &fields);',
        '  let keys: Vec<&str> = merged.iter().map(|(key, _)| key.as_str()).collect();',
        '  assert_eq!(keys, vec!["first", "shared", "parent", "middle", "child"]);',
        '  assert_eq!(merged.iter().find(|(key, _)| key == "shared").map(|(_, value)| value), Some(&value("final")));',
        '}',
        '',
      ].join('\n'),
    );
    expect(() => compileRustExecutable('main.rs', binary, fixture)).not.toThrow();
    expect(() => execFileSync(binary, [], { cwd: fixture, stdio: 'pipe' })).not.toThrow();
  });

  it('preserves a local across ordered fields with equivalent Rust alias types', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/adjustments/src/ordered-alias-fields.ts',
      `
        interface Shape {
          readonly label: string;
          readonly count: number;
        }
        type ShapeLike = Shape;
        type OwnedShape = Shape;
        interface Result {
          readonly value: OwnedShape;
          readonly label: string;
        }
        export function createResult(input: Readonly<ShapeLike>): Result {
          const value: ShapeLike = { ...input };
          return {
            value,
            label: value.label,
          };
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/adjustments', '/workspace');
    const output = emitRustModule({
      declarations: lowered.declarations,
      source: 'upstream/packages/adjustments/src/ordered-alias-fields.ts',
      typeImports: [],
    });

    expect(lowered.diagnostics).toEqual([]);
    expect(output).toContain('value: (value).clone()');

    const fixture = mkdtempSync(path.join(tmpdir(), 'flight-rs-ordered-alias-fields-'));
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
        import type { InputPointerData } from '@flighthq/types/contract';
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

  it('retains EntityRuntimeKey rejection when the receiver has no static entity representation', () => {
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

  it('compiles source-derived aggregate entity runtime storage and preserves it across projections', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/types/src/entity-runtime.ts',
      `
        export interface Entity {}
        export interface EntityRuntime {
          binding?: string;
        }
        export interface Node extends Entity {
          name: string;
        }
        export interface NodeRuntime<Traits> extends EntityRuntime {
          callback: () => void;
          count: number;
        }
        export interface GlNode extends Entity {}
        export interface GlNodeRuntime extends EntityRuntime {
          backendState: string;
        }
        export interface WgpuNode extends Entity {}
        export interface WgpuNodeRuntime extends EntityRuntime {
          backendState: number;
        }
        export function createRuntime<Traits>(): NodeRuntime<Traits> {
          return {
            binding: null,
            callback() {},
            count: 1,
          };
        }
        export function createNode(runtime: NodeRuntime<string>): Node {
          return { name: 'node', [EntityRuntimeKey]: runtime };
        }
        export function attachRuntime<Type>(source: Type, runtime: NodeRuntime<string>): Type {
          source[EntityRuntimeKey] = runtime;
          return source;
        }
        export function readCount(source: Node): number {
          return source[EntityRuntimeKey].count;
        }
        export function writeCount(source: Node, value: number): void {
          source[EntityRuntimeKey].count = value;
        }
        export function bumpCount(source: Node): void {
          source[EntityRuntimeKey].count = source[EntityRuntimeKey].count + 1;
        }
        export function incrementCount(source: Node): number {
          return source[EntityRuntimeKey].count++;
        }
        export function removeRuntime(source: Node): boolean {
          return delete source[EntityRuntimeKey];
        }
        export function hasRuntime(source: Node): boolean {
          return EntityRuntimeKey in source;
        }
        export function hasRuntimeComparison(source: Node): boolean {
          return source[EntityRuntimeKey] !== undefined;
        }
        export function setBinding(source: Node, binding: string): void {
          source[EntityRuntimeKey].binding = binding;
        }
        export function hasGenericRuntime<Type>(source: Type): boolean {
          return EntityRuntimeKey in source;
        }
        export function readProjected(source: Node): number {
          const projected = source as Entity;
          return projected[EntityRuntimeKey].count;
        }
        export function copyKeepsRuntime(source: Node): boolean {
          const copied: Node = { ...source };
          delete source[EntityRuntimeKey];
          return EntityRuntimeKey in copied;
        }
        export function cloneWithoutRuntime<Type extends Entity>(source: Readonly<Type>): Type {
          const copy = { ...source } as Record<PropertyKey, unknown>;
          copy[EntityRuntimeKey] = undefined;
          return copy as unknown as Type;
        }
        export function stripRuntime<Type extends Entity>(source: Readonly<Type>): Type {
          const copy = { ...source } as Record<PropertyKey, unknown>;
          delete copy[EntityRuntimeKey];
          return copy as unknown as Type;
        }
        export function readGlState(source: GlNode): string {
          const runtime = source[EntityRuntimeKey] as GlNodeRuntime;
          return runtime.backendState;
        }
        export function readWgpuState(source: WgpuNode): number {
          const runtime = source[EntityRuntimeKey] as WgpuNodeRuntime;
          return runtime.backendState;
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/types', '/workspace');
    const output = emitRustModule({
      declarations: lowered.declarations,
      source: 'upstream/packages/types/src/entity-runtime.ts',
      typeImports: [],
    });

    expect(lowered.diagnostics).toEqual([]);
    expect(output).toContain('pub struct EntityRuntimeStorage');
    expect(output).toContain('pub type NodeRuntime<Traits> =');
    expect(output).toContain('std::marker::PhantomData<Traits>');
    expect(output).toContain('FlightEntityRuntimeMarker');
    expect(output).toContain('pub fn create_runtime<Traits: Clone>() -> NodeRuntime<Traits>');
    expect(output).toContain('pub struct GlNodeRuntimeStorage');
    expect(output).toContain('pub struct WgpuNodeRuntimeStorage');
    expect(output).toContain('pub gl_node_runtime: crate::GlNodeRuntimeStorage');
    expect(output).toContain('pub wgpu_node_runtime: crate::WgpuNodeRuntimeStorage');
    expect(output).toContain('.gl_node_runtime.backend_state');
    expect(output).toContain('.wgpu_node_runtime.backend_state');
    expect(output).toContain('pub trait FlightEntity');
    expect(output).toContain('__flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<EntityRuntime>>>');
    expect(output).toContain('Type: Clone + FlightEntity');
    expect(output).toContain('pub fn has_generic_runtime<Type: Clone + FlightEntity>');
    expect(output).toContain('pub fn clone_without_runtime<Type: Clone + FlightEntity>');
    expect(output).toContain('FlightEntity::__flight_fresh_clone');
    expect(output).toContain('FlightEntity::__flight_entity_runtime(&(copy)).lock().unwrap() = None');
    expect(output).toContain('.lock().unwrap().take().is_some()');
    expect(output).toContain('.lock().unwrap().is_some()');
    expect(output).toContain('let __flight_value = Some((binding).clone())');
    expect(output).toContain('__flight_entity_runtime: std::sync::Arc::clone(');
    expect(output).toContain(
      'std::sync::Arc::new(std::sync::Mutex::new(__flight_entity_spread.__flight_entity_runtime.lock().unwrap().clone()))',
    );
    expect(output).toContain(
      'let __flight_value = (({ let __flight_runtime = FlightEntity::__flight_entity_runtime(source).lock().unwrap().clone().expect("entity runtime was read before initialization"); __flight_runtime }).inner.lock().unwrap().count + 1.0_f64);',
    );
    expect(output).toContain(
      'let mut __flight_storage = __flight_runtime.inner.lock().unwrap(); __flight_storage.count = __flight_value;',
    );
    expect(output).toContain(
      'let mut __flight_storage = __flight_runtime.inner.lock().unwrap(); __flight_storage.count += 1.0;',
    );
    expect(output).not.toContain('refusing to erase observable runtime state');

    const fixture = mkdtempSync(path.join(tmpdir(), 'flight-rs-entity-runtime-'));
    const sourceFile = path.join(fixture, 'lib.rs');
    writeFileSync(sourceFile, output);
    expect(() =>
      execFileSync('rustc', ['--crate-type', 'lib', '--emit', 'metadata', '--edition', '2024', sourceFile], {
        cwd: fixture,
        stdio: 'pipe',
      }),
    ).not.toThrow();
  });

  it('stores generic-dependent entity runtime fields in checked typed slots', () => {
    const rootSource = ts.createSourceFile(
      '/workspace/upstream/packages/types/src/entity.ts',
      `
        export interface Entity {}
        export interface EntityRuntime {
          binding?: string;
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const loweredRoot = lowerTypeScriptSource(rootSource, '@flighthq/types', '/workspace');
    const rootOutput = emitRustModule({
      declarations: loweredRoot.declarations,
      source: 'upstream/packages/types/src/entity.ts',
      typeImports: [],
    });
    expect(loweredRoot.diagnostics).toEqual([]);
    expect(rootOutput).toContain('generic_slots: std::collections::HashMap<std::any::TypeId');
    expect(rootOutput).toContain('pub fn __flight_generic_slot<Slot: Default + Send');

    const typesSource = ts.createSourceFile(
      '/workspace/upstream/packages/types/src/generic-runtime.ts',
      `
        export interface Entity {}
        export interface EntityRuntime {
          binding?: string;
        }
        export interface NodeRuntime<Traits> extends EntityRuntime {
          count: number;
          genericValue: Traits;
          children: Traits[] | null;
          traits?: Traits;
        }
        export function readCount(runtime: NodeRuntime<string>): number {
          return runtime.count;
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const loweredTypes = lowerTypeScriptSource(typesSource, '@flighthq/types', '/workspace');
    const output = emitRustModule({
      declarations: loweredTypes.declarations,
      source: 'upstream/packages/types/src/generic-runtime.ts',
      typeImports: [],
    });

    expect(loweredTypes.diagnostics).toEqual([]);
    expect(output).toContain('pub type NodeRuntime<Traits> =');
    expect(output).toContain('std::marker::PhantomData<Traits>');
    expect(output).toContain('pub count: f64');
    expect(output).toContain('pub generic_value: Option<Traits>');
    expect(output).toContain('pub __flight_marker: std::marker::PhantomData<Traits>');
    expect(output).toContain('generic_slots: std::collections::HashMap<std::any::TypeId');
    expect(output).toContain('downcast_ref::<std::sync::Arc<std::sync::Mutex<Slot>>>');
    expect(output).not.toContain('aggregate native entity runtime closure is unavailable');

    const fixture = mkdtempSync(path.join(tmpdir(), 'flight-rs-generic-runtime-'));
    const sourceFile = path.join(fixture, 'lib.rs');
    writeFileSync(sourceFile, output);
    expect(() =>
      execFileSync('rustc', ['--crate-type', 'lib', '--emit', 'metadata', '--edition', '2024', sourceFile], {
        cwd: fixture,
        stdio: 'pipe',
      }),
    ).not.toThrow();

    const semanticTypes = Object.fromEntries(
      loweredTypes.declarations.flatMap((declaration) =>
        declaration.kind === 'type' ? [[declaration.name, declaration.type] as const] : [],
      ),
    );
    const semanticTypeParameters = Object.fromEntries(
      loweredTypes.declarations.flatMap((declaration) =>
        declaration.kind === 'type' ? [[declaration.name, declaration.typeParameters] as const] : [],
      ),
    );
    const consumerSource = ts.createSourceFile(
      '/workspace/upstream/packages/node/src/generic-runtime.ts',
      `
        export function readGenericValue(runtime: NodeRuntime<string>): string {
          return runtime.genericValue;
        }
        export function writeGenericValue(runtime: NodeRuntime<string>, value: string): void {
          runtime.genericValue = value;
          runtime.children = [value];
          runtime.traits = value;
        }
        export function readFirstChild(runtime: NodeRuntime<string>): string {
          return runtime.children![0];
        }
        export function readTraits(runtime: NodeRuntime<string>): string {
          return runtime.traits!;
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const loweredConsumer = lowerTypeScriptSource(consumerSource, '@flighthq/node', '/workspace');

    expect(loweredConsumer.diagnostics).toEqual([]);
    const consumerOutput = emitRustModule({
      declarations: loweredConsumer.declarations,
      imports: [
        {
          module: 'crate',
          names: [{ imported: 'NodeRuntime', kind: 'type', local: 'NodeRuntime' }],
        },
      ],
      semanticTypeParameters,
      semanticTypes,
      source: 'upstream/packages/node/src/generic-runtime.ts',
      typeImports: [],
    });
    expect(consumerOutput).toContain('__flight_generic_slot::<crate::NodeRuntimeStorage<String>>()');

    writeFileSync(path.join(fixture, 'types.rs'), output);
    writeFileSync(path.join(fixture, 'consumer.rs'), consumerOutput);
    writeFileSync(
      path.join(fixture, 'main.rs'),
      [
        'mod types;',
        'pub use types::*;',
        'mod consumer;',
        'fn main() {',
        '  let runtime = EntityRuntime::default();',
        '  consumer::write_generic_value(runtime.clone(), "typed".to_owned());',
        '  assert_eq!(consumer::read_generic_value(runtime.clone()), "typed");',
        '  assert_eq!(consumer::read_first_child(runtime.clone()), "typed");',
        '  assert_eq!(consumer::read_traits(runtime), "typed");',
        '}',
      ].join('\n'),
    );
    const binary = path.join(fixture, 'generic-runtime');
    expect(() => compileRustExecutable('main.rs', binary, fixture)).not.toThrow();
    expect(() => execFileSync(binary, [], { cwd: fixture, stdio: 'pipe' })).not.toThrow();
  });

  it('compiles a promoted entity target with source-derived crate-root runtime support', () => {
    const entitySource = ts.createSourceFile(
      '/workspace/upstream/packages/types/src/Entity.ts',
      `
        export interface Entity {}
        export interface EntityRuntime {
          binding: object | null;
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const loweredEntity = lowerTypeScriptSource(entitySource, '@flighthq/types', '/workspace');
    const entityOutput = emitRustModule({
      declarations: loweredEntity.declarations,
      source: 'upstream/packages/types/src/Entity.ts',
      typeImports: [],
    });
    const semanticTypes = Object.fromEntries(
      loweredEntity.declarations.flatMap((declaration) =>
        declaration.kind === 'type' ? [[declaration.name, declaration.type] as const] : [],
      ),
    );
    const semanticTypeParameters = Object.fromEntries(
      loweredEntity.declarations.flatMap((declaration) =>
        declaration.kind === 'type' ? [[declaration.name, declaration.typeParameters] as const] : [],
      ),
    );
    const promotedSource = ts.createSourceFile(
      '/workspace/upstream/packages/types/src/Surface.ts',
      `
        export interface Surface extends Entity {
          width: number;
        }
        export type SurfaceLike = Surface;
        export interface PlainSurface {
          width: number;
        }
        export interface GenericSurface<State> extends Entity {
          state: State;
        }
        export function readSurface(surface: SurfaceLike): number {
          return surface.width;
        }
        export function readProjectedSurface(surface: PlainSurface): number {
          return readSurface(surface);
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const loweredPromoted = lowerTypeScriptSource(promotedSource, '@flighthq/types', '/workspace');
    const promotedOutput = emitRustModule({
      declarations: loweredPromoted.declarations,
      imports: [
        {
          module: 'crate',
          names: [
            { imported: 'Entity', kind: 'type', local: 'Entity' },
            { imported: 'EntityRuntime', kind: 'type', local: 'EntityRuntime' },
          ],
        },
      ],
      semanticTypeParameters,
      semanticTypes,
      source: 'upstream/packages/types/src/Surface.ts',
      typeImports: [],
    });

    expect(loweredEntity.diagnostics).toEqual([]);
    expect(loweredPromoted.diagnostics).toEqual([]);
    expect(entityOutput).toContain('pub struct EntityRuntimeStorage');
    expect(entityOutput).toContain('pub trait FlightEntity');
    expect(promotedOutput).toContain('impl crate::FlightEntity for Surface');
    expect(promotedOutput).toContain('impl<State: Clone> crate::FlightEntity for GenericSurface<State>');
    expect(promotedOutput).toContain('__flight_entity_runtime: Default::default()');

    const fixture = mkdtempSync(path.join(tmpdir(), 'flight-rs-promoted-entity-'));
    writeFileSync(path.join(fixture, 'entity.rs'), entityOutput);
    writeFileSync(path.join(fixture, 'surface.rs'), promotedOutput);
    writeFileSync(
      path.join(fixture, 'lib.rs'),
      [
        '#[derive(Clone, Default)]',
        'pub struct OpaqueHostValue;',
        'mod entity;',
        'pub use entity::*;',
        'mod surface;',
        'pub use surface::*;',
      ].join('\n'),
    );
    expect(() =>
      execFileSync('rustc', ['--crate-type', 'lib', '--emit', 'metadata', '--edition', '2024', 'lib.rs'], {
        cwd: fixture,
        stdio: 'pipe',
      }),
    ).not.toThrow();
  });

  it('applies generic arguments through aliases before structural field inference', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/types/src/generic-alias.ts',
      `
        export interface ValueBox<Value> {
          value: Value;
        }
        export type ValueAlias<Value> = ValueBox<Value>;
        export interface StringValue extends ValueAlias<string> {
          label: string;
        }
        export function readString(source: ValueAlias<string>): string {
          return source.value;
        }
        export function readExtendedString(source: StringValue): string {
          return source.value;
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/types', '/workspace');
    const output = emitRustModule({
      declarations: lowered.declarations,
      source: 'upstream/packages/types/src/generic-alias.ts',
      typeImports: [],
    });

    expect(lowered.diagnostics).toEqual([]);
    expect(output).toContain('pub struct ValueBox<Value>');
    expect(output).toContain('pub type ValueAlias<Value> = ValueBox<Value>;');
    expect(output).toContain('pub struct StringValue');
    expect(output).toContain('pub value: String');
    expect(output).toContain('source: &ValueAlias<String>');
    expect(output).toContain('-> String');

    const fixture = mkdtempSync(path.join(tmpdir(), 'flight-rs-generic-alias-'));
    const sourceFile = path.join(fixture, 'lib.rs');
    writeFileSync(sourceFile, output);
    expect(() =>
      execFileSync('rustc', ['--crate-type', 'lib', '--emit', 'metadata', '--edition', '2024', sourceFile], {
        cwd: fixture,
        stdio: 'pipe',
      }),
    ).not.toThrow();
  });

  it('preserves imported semantic type parameters through alias applications', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/node/src/imported-generic.ts',
      `
        export function readImportedString(source: ValueAlias<string>): string {
          return source.value;
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/node', '/workspace');
    const valueParameter = {
      arguments: [],
      kind: 'named' as const,
      name: 'Value',
    };
    const semanticTypes = {
      ValueAlias: {
        arguments: [valueParameter],
        kind: 'named' as const,
        name: 'ValueBox',
      },
      ValueBox: {
        extends: [],
        fields: [{ name: 'value', optional: false, type: valueParameter }],
        kind: 'anonymous' as const,
      },
    };
    const output = emitRustModule({
      declarations: lowered.declarations,
      imports: [
        {
          module: 'external',
          names: [{ imported: 'ValueAlias', kind: 'type', local: 'ValueAlias' }],
        },
      ],
      semanticTypeParameters: { ValueAlias: ['Value'], ValueBox: ['Value'] },
      semanticTypes,
      source: 'upstream/packages/node/src/imported-generic.ts',
      typeImports: [],
    });

    expect(lowered.diagnostics).toEqual([]);
    expect(output).toContain('source: &ValueAlias<String>');
    expect(output).toContain('return (source.value).clone();');

    const fixture = mkdtempSync(path.join(tmpdir(), 'flight-rs-imported-generic-'));
    const sourceFile = path.join(fixture, 'lib.rs');
    writeFileSync(
      sourceFile,
      output.replace(
        '// Source:',
        `
        mod external {
          #[derive(Clone)]
          pub struct ValueBox<Value> {
            pub value: Value,
          }
          pub type ValueAlias<Value> = ValueBox<Value>;
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

  it('compiles an entity runtime aggregate and a downstream consumer as separate crates', () => {
    const typesSource = ts.createSourceFile(
      '/workspace/upstream/packages/types/src/entity-runtime.ts',
      `
        export interface Entity {}
        export interface EntityRuntime {
          count: number;
        }
        export interface Node extends Entity {
          name: string;
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const loweredTypes = lowerTypeScriptSource(typesSource, '@flighthq/types', '/workspace');
    const typesOutput = emitRustModule({
      declarations: loweredTypes.declarations,
      source: 'upstream/packages/types/src/entity-runtime.ts',
      typeImports: [],
    });
    const semanticTypes = Object.fromEntries(
      loweredTypes.declarations.flatMap((declaration) =>
        declaration.kind === 'type' ? [[declaration.name, declaration.type] as const] : [],
      ),
    );
    const semanticTypeParameters = Object.fromEntries(
      loweredTypes.declarations.flatMap((declaration) =>
        declaration.kind === 'type' ? [[declaration.name, declaration.typeParameters] as const] : [],
      ),
    );

    const consumerSource = ts.createSourceFile(
      '/workspace/upstream/packages/node/src/entity-runtime.ts',
      `
        export function attachRuntime(source: Node, runtime: EntityRuntime): Node {
          source[EntityRuntimeKey] = runtime;
          return source;
        }
        export function readCount(source: Node): number {
          return source[EntityRuntimeKey].count;
        }
        export function hasRuntime(source: Node): boolean {
          return EntityRuntimeKey in source;
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const loweredConsumer = lowerTypeScriptSource(consumerSource, '@flighthq/node', '/workspace');
    const consumerOutput = emitRustModule({
      declarations: loweredConsumer.declarations,
      imports: [
        {
          module: 'flighthq_types',
          names: [
            { imported: 'EntityRuntime', kind: 'type', local: 'EntityRuntime' },
            { imported: 'Node', kind: 'type', local: 'Node' },
          ],
        },
      ],
      semanticTypeParameters,
      semanticTypes,
      source: 'upstream/packages/node/src/entity-runtime.ts',
      typeImports: [],
    });

    expect(loweredTypes.diagnostics).toEqual([]);
    expect(loweredConsumer.diagnostics).toEqual([]);
    expect(consumerOutput).toContain('flighthq_types::FlightEntity::__flight_entity_runtime(source)');
    expect(consumerOutput).toContain('.inner.lock().unwrap().count');

    const fixture = mkdtempSync(path.join(tmpdir(), 'flight-rs-cross-crate-runtime-'));
    const typesFile = path.join(fixture, 'types.rs');
    const typesMetadata = path.join(fixture, 'libflighthq_types.rmeta');
    const consumerFile = path.join(fixture, 'consumer.rs');
    writeFileSync(typesFile, typesOutput);
    writeFileSync(consumerFile, consumerOutput);
    expect(() =>
      execFileSync(
        'rustc',
        [
          '--crate-name',
          'flighthq_types',
          '--crate-type',
          'lib',
          '--emit',
          'metadata',
          '--edition',
          '2024',
          typesFile,
          '-o',
          typesMetadata,
        ],
        { cwd: fixture, stdio: 'pipe' },
      ),
    ).not.toThrow();
    expect(() =>
      execFileSync(
        'rustc',
        [
          '--crate-name',
          'flighthq_node',
          '--crate-type',
          'lib',
          '--emit',
          'metadata',
          '--edition',
          '2024',
          '--extern',
          `flighthq_types=${typesMetadata}`,
          consumerFile,
        ],
        { cwd: fixture, stdio: 'pipe' },
      ),
    ).not.toThrow();
  });

  it('rejects package-local runtime storage that cannot join an imported aggregate', () => {
    const typesSource = ts.createSourceFile(
      '/workspace/upstream/packages/types/src/entity-runtime.ts',
      `
        export interface Entity {}
        export interface EntityRuntime {
          binding?: string;
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const loweredTypes = lowerTypeScriptSource(typesSource, '@flighthq/types', '/workspace');
    const typesOutput = emitRustModule({
      declarations: loweredTypes.declarations,
      source: 'upstream/packages/types/src/entity-runtime.ts',
      typeImports: [],
    });
    expect(loweredTypes.diagnostics).toEqual([]);
    expect(typesOutput).toContain('pub struct EntityRuntimeStorage');

    const renderSource = ts.createSourceFile(
      '/workspace/upstream/packages/render-gl/src/entity-runtime.ts',
      `
        export interface GlNodeRuntime extends EntityRuntime {
          backendState: string;
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const loweredRender = lowerTypeScriptSource(renderSource, '@flighthq/render-gl', '/workspace');
    const semanticTypes = Object.fromEntries(
      loweredTypes.declarations.flatMap((declaration) =>
        declaration.kind === 'type' ? [[declaration.name, declaration.type] as const] : [],
      ),
    );

    expect(loweredRender.diagnostics).toEqual([]);
    expect(() =>
      emitRustModule({
        declarations: loweredRender.declarations,
        imports: [
          {
            module: 'flighthq_types',
            names: [
              { imported: 'Entity', kind: 'type', local: 'Entity' },
              {
                imported: 'EntityRuntime',
                kind: 'type',
                local: 'EntityRuntime',
              },
            ],
          },
        ],
        semanticTypes,
        source: 'upstream/packages/render-gl/src/entity-runtime.ts',
        typeImports: ['Entity', 'EntityRuntime'],
      }),
    ).toThrow(
      'imported EntityRuntime aggregate cannot acquire package-local storage fields: GlNodeRuntime.backendState',
    );
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

    const shadowedSource = ts.createSourceFile(
      '/workspace/upstream/packages/application/src/shadowed-capability.ts',
      `
        export function supportsShadowedCapability(window: { unknownCapability: () => void }): boolean {
          return typeof window.unknownCapability === 'function';
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const loweredShadowed = lowerTypeScriptSource(shadowedSource, '@flighthq/application', '/workspace');

    expect(loweredShadowed.diagnostics).toEqual([]);
    expect(() =>
      emitRustModule({
        declarations: loweredShadowed.declarations,
        source: 'upstream/packages/application/src/shadowed-capability.ts',
        typeImports: [],
      }),
    ).not.toThrow();
  });

  it('compiles and runs all contextual object literal recovery paths', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/effects-gl/src/contextual-objects.ts',
      `
        export interface DeclaredOptions {
          readonly offset: number;
          readonly scale: number;
        }
        interface SemanticOptions {
          readonly bias: number;
          readonly input: number;
        }
        function evaluate(options: DeclaredOptions): number {
          return options.offset + options.scale;
        }
        export function declaredParameterPath(): number {
          return evaluate({ offset: 2, scale: 3 });
        }
        export function declaredReturnPath(value: number): DeclaredOptions {
          return { offset: value, scale: 4 };
        }
        export function semanticSignaturePath(input: number): number {
          const options: any = { bias: 4, input };
          return options.input + options.bias;
        }
        export function synthesizedRecordPath(input: number): number {
          const packet: any = { left: input, right: 2 };
          return packet.left * packet.right;
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/effects-gl', '/workspace');
    const output = emitRustModule({
      declarations: lowered.declarations,
      source: 'upstream/packages/effects-gl/src/contextual-objects.ts',
      typeImports: [],
    });

    expect(lowered.diagnostics).toEqual([]);
    expect(output).toContain('evaluate(&DeclaredOptions {');
    expect(output).toContain('let options = SemanticOptions {');
    expect(output).toMatch(/struct SynthesizedRecordPathSynthesizedRecord\d+/u);
    expect(output).not.toContain('OpaqueHostValue');

    const fixture = mkdtempSync(path.join(tmpdir(), 'flight-rs-contextual-objects-'));
    const binary = path.join(fixture, 'contextual-objects');
    writeFileSync(path.join(fixture, 'generated.rs'), output);
    writeFileSync(
      path.join(fixture, 'main.rs'),
      [
        'mod generated;',
        'fn main() {',
        '  assert_eq!(generated::declared_parameter_path(), 5.0);',
        '  let returned = generated::declared_return_path(6.0);',
        '  assert_eq!(returned.offset + returned.scale, 10.0);',
        '  assert_eq!(generated::semantic_signature_path(3.0), 7.0);',
        '  assert_eq!(generated::synthesized_record_path(8.0), 16.0);',
        '}',
        '',
      ].join('\n'),
    );
    expect(() => compileRustExecutable('main.rs', binary, fixture)).not.toThrow();
    expect(() => execFileSync(binary, [], { cwd: fixture, stdio: 'pipe' })).not.toThrow();
  });

  it('compiles typed ImageData and OffscreenCanvas constructors against an installable native backend', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/image/src/constructors.ts',
      `
        export function imageFromPixels(
          data: Uint8ClampedArray,
          width: number,
          height: number,
        ) {
          return new ImageData(data, width, height);
        }
        export function imageFromOptionalHeight(
          data: Uint8ClampedArray,
          width: number,
          height?: number,
        ) {
          return new ImageData(data, width, height);
        }
        export function imageFromNullablePixels(
          data: Uint8ClampedArray | null,
          width: number,
          height: number,
        ) {
          return new ImageData(data, width, height);
        }
        export function blankImage(width: number, height: number) {
          return new ImageData(width, height);
        }
        export function createCanvas(width: number, height: number) {
          return new OffscreenCanvas(width, height);
        }
        export function createUrl(value: string, base: string) {
          return new URL(value, base);
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/image', '/workspace');
    const output = emitRustModule({
      declarations: lowered.declarations,
      source: 'upstream/packages/image/src/constructors.ts',
      typeImports: [],
    });

    expect(lowered.diagnostics).toEqual([]);
    expect(output).toContain('-> crate::FlightImageData');
    expect(output).toContain('crate::FlightImageDataRequest::Pixels');
    expect(output).toContain('crate::FlightImageDataRequest::Dimensions');
    expect(output).toContain('-> crate::FlightOffscreenCanvas');
    expect(output).toContain('crate::host_offscreen_canvas(width, height)');
    expect(output).toContain('-> crate::FlightUrl');
    expect(output).toContain('crate::host_url');
    expect(output).toContain('height: height');
    expect(output).toContain('match __flight_data');
    expect(output).not.toContain('OpaqueHostValue::Object');
    expect(output).not.toContain('height: Some((height).clone().unwrap())');

    const fixture = mkdtempSync(path.join(tmpdir(), 'flight-rs-host-constructors-'));
    const binary = path.join(fixture, 'host-constructors');
    writeFileSync(path.join(fixture, 'generated.rs'), output);
    writeFileSync(
      path.join(fixture, 'main.rs'),
      [
        ...emitNativeHostCapabilityRuntime(),
        'mod generated;',
        'struct TestBackend;',
        'impl NativeHostConstructors for TestBackend {',
        '  fn image_data(&self, request: FlightImageDataRequest) -> FlightImageData {',
        '    let description = match request {',
        '      FlightImageDataRequest::Dimensions { width, height } => format!("dimensions:{width}:{height}"),',
        '      FlightImageDataRequest::Pixels { data, width, height } => format!("pixels:{}:{width}:{height:?}", data.len()),',
        '    };',
        '    FlightImageData::from_native(description)',
        '  }',
        '  fn offscreen_canvas(&self, width: f64, height: f64) -> FlightOffscreenCanvas {',
        '    FlightOffscreenCanvas::from_native(format!("canvas:{width}:{height}"))',
        '  }',
        '  fn url(&self, value: String, base: Option<String>) -> FlightUrl {',
        '    FlightUrl::from_native(format!("url:{value}:{base:?}"))',
        '  }',
        '}',
        'fn main() {',
        '  install_native_host_constructors(TestBackend).unwrap();',
        '  let pixels = vec![1, 2, 3, 4];',
        '  let image = generated::image_from_optional_height(&pixels, 1.0, None);',
        '  assert_eq!(image.downcast_ref::<String>().map(String::as_str), Some("pixels:4:1:None"));',
        '  let dimensions = generated::image_from_nullable_pixels(&None, 2.0, 3.0);',
        '  assert_eq!(dimensions.downcast_ref::<String>().map(String::as_str), Some("dimensions:2:3"));',
        '  let canvas = generated::create_canvas(4.0, 8.0);',
        '  assert_eq!(canvas.downcast_ref::<String>().map(String::as_str), Some("canvas:4:8"));',
        '  let url = generated::create_url("child".to_owned(), "https://example.com".to_owned());',
        '  assert_eq!(url.downcast_ref::<String>().map(String::as_str), Some("url:child:Some(\\"https://example.com\\")"));',
        '}',
        '',
      ].join('\n'),
    );
    expect(() => compileRustExecutable('main.rs', binary, fixture)).not.toThrow();
    expect(() => execFileSync(binary, [], { cwd: fixture, stdio: 'pipe' })).not.toThrow();
  });
});

function compileRustLibraryWithRuntime(output: string, fixture: string): void {
  writeFileSync(path.join(fixture, 'flight_runtime.rs'), emitFlightTaskRuntime());
  writeFileSync(path.join(fixture, 'generated.rs'), output);
  const sourceFile = path.join(fixture, 'lib.rs');
  writeFileSync(
    sourceFile,
    ['mod flight_runtime;', 'pub use flight_runtime::*;', 'mod generated;', 'pub use generated::*;', ''].join('\n'),
  );
  execFileSync('rustc', ['--crate-type', 'lib', '--emit', 'metadata', '--edition', '2024', sourceFile], {
    cwd: fixture,
    stdio: 'pipe',
  });
}

function compileRustExecutable(source: string, output: string, cwd: string): void {
  const sysroot = execFileSync('rustc', ['--print', 'sysroot'], {
    cwd,
    encoding: 'utf8',
    stdio: ['ignore', 'pipe', 'pipe'],
  }).trim();
  const host = execFileSync('rustc', ['--version', '--verbose'], {
    cwd,
    encoding: 'utf8',
    stdio: ['ignore', 'pipe', 'pipe'],
  })
    .split('\n')
    .find((line) => line.startsWith('host: '))
    ?.slice('host: '.length);
  if (!host) throw new Error('rustc did not report its host target');
  const linker = path.join(sysroot, 'lib', 'rustlib', host, 'bin', 'rust-lld');
  execFileSync(
    'rustc',
    [
      '--edition',
      '2024',
      '--target',
      'x86_64-unknown-linux-musl',
      '-C',
      `linker=${linker}`,
      '-C',
      'link-self-contained=yes',
      source,
      '-o',
      output,
    ],
    { cwd, stdio: 'pipe' },
  );
}
