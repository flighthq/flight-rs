import ts from 'typescript';

import { portConfig } from '../../tools/generator/port.config.ts';
import { lowerTypeScriptSource } from '../../tools/generator/src/lower/typescript.ts';

describe('configured type lowering exceptions', () => {
  it('recovers async outputs from declared sites, the semantic catalog, and synthesized records', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/application/src/contextual-tasks.ts',
      `
        interface DeclaredPayload {
          value: number;
        }
        interface ReturnBackend {
          fromReturn(): Promise<DeclaredPayload>;
        }
        interface CallBackend {
          fromCall(): Promise<boolean>;
        }
        type Loader = () => Promise<DeclaredPayload>;
        function install(_backend: CallBackend): void {}
        export function returnBackend(): ReturnBackend {
          return {
            async fromReturn() {
              return { value: 1 };
            },
          };
        }
        export function installBackend(): void {
          install({
            async fromCall() {
              return true;
            },
          });
        }
        export function returnLoader(): Loader {
          return async () => ({ value: 2 });
        }
        export const catalogBackend = {
          async fromCatalog() {
            return null;
          },
        };
        export async function synthesized() {
          return { count: 3, label: 'synthesized' };
        }
        export async function genuinelyDynamic(value: any) {
          return value;
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/application', '/workspace', {
      types: {
        CatalogBackend: {
          extends: [],
          fields: [
            {
              name: 'fromCatalog',
              optional: false,
              type: {
                kind: 'function',
                parameters: [],
                returns: {
                  kind: 'task',
                  output: {
                    inner: { arguments: [], kind: 'named', name: 'CatalogPayload' },
                    kind: 'nullable',
                  },
                },
              },
            },
          ],
          kind: 'anonymous',
        },
        CatalogPayload: {
          extends: [],
          fields: [{ name: 'label', optional: false, type: { kind: 'primitive', name: 'String' } }],
          kind: 'anonymous',
        },
      },
    });
    const outputs = Object.fromEntries(
      lowered.asyncTasks.map((scope) => [scope.execution.origin.lexicalPath, scope.output]),
    );
    const loaderOutput = lowered.asyncTasks.find((scope) =>
      scope.execution.origin.lexicalPath.startsWith('returnLoader.anonymous:'),
    )?.output;

    expect(lowered.diagnostics).toEqual([]);
    expect(outputs['returnBackend.fromReturn']).toEqual({ arguments: [], kind: 'named', name: 'DeclaredPayload' });
    expect(outputs['installBackend.fromCall']).toEqual({ kind: 'primitive', name: 'Bool' });
    expect(loaderOutput).toEqual({
      arguments: [],
      kind: 'named',
      name: 'DeclaredPayload',
    });
    expect(outputs['catalogBackend.fromCatalog']).toEqual({
      inner: { arguments: [], kind: 'named', name: 'CatalogPayload' },
      kind: 'nullable',
    });
    expect(outputs.synthesized).toEqual({
      extends: [],
      fields: [
        { name: 'count', optional: false, type: { kind: 'primitive', name: 'Float' } },
        { name: 'label', optional: false, type: { kind: 'primitive', name: 'String' } },
      ],
      kind: 'anonymous',
    });
    expect(outputs.genuinelyDynamic).toEqual({ kind: 'dynamic' });
  });

  it('assigns stable source identities and task types to every async scope', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/application/src/tasks.ts',
      `
        export async function load(value: number): Promise<number> {
          await Promise.resolve(value);
          const nested = async (): Promise<number> => {
            await Promise.all([Promise.resolve(value)]);
            return value;
          };
          const handlers = {
            async run(): Promise<void> {
              await nested();
            },
          };
          [value].map(async (item) => {
            await Promise.reject(item);
          });
          await handlers.run();
          return nested();
        }
        export function pending(): Promise<void> {
          return Promise.resolve();
        }
        export function qualified(value: number): globalThis.Promise<number> {
          return Promise.resolve(value);
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/application', '/workspace');
    const declaration = lowered.declarations.find((item) => item.kind === 'function' && item.name === 'load');

    expect(lowered.diagnostics).toEqual([]);
    expect(declaration).toMatchObject({
      execution: {
        kind: 'portableTask',
        origin: {
          lexicalPath: 'load',
          packageName: '@flighthq/application',
          source: 'upstream/packages/application/src/tasks.ts',
        },
      },
      returns: { kind: 'task', output: { kind: 'primitive', name: 'Float' } },
    });
    expect(lowered.declarations.find((item) => item.kind === 'function' && item.name === 'pending')).toMatchObject({
      returns: { kind: 'task', output: { kind: 'primitive', name: 'Void' } },
    });
    expect(lowered.declarations.find((item) => item.kind === 'function' && item.name === 'qualified')).toMatchObject({
      returns: { kind: 'task', output: { kind: 'primitive', name: 'Float' } },
    });
    expect(lowered.asyncTasks.map((scope) => scope.execution.origin.lexicalPath)).toEqual([
      'load',
      'load.nested',
      'load.handlers.run',
      expect.stringMatching(/^load\.anonymous:[0-9a-f]{12}$/u),
    ]);
    expect(lowered.asyncTasks.map((scope) => scope.matchesLegacyErasurePath)).toEqual([true, false, false, false]);
    expect(lowered.asyncTasks.map((scope) => scope.operations)).toMatchObject([
      { awaits: 2, promiseResolve: 1 },
      { awaits: 1, promiseAll: 1, promiseResolve: 1 },
      { awaits: 1 },
      { awaits: 1, promiseReject: 1 },
    ]);
    for (const scope of lowered.asyncTasks) {
      expect(scope.execution.origin.fingerprint).toMatch(/^sha256:[0-9a-f]{64}$/u);
    }
  });

  it('keeps a source-declared Promise nominal instead of treating its name as the global task type', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/types/src/promise.ts',
      `
        export interface Promise<T> {
          value: T;
        }
        export function identity(value: Promise<number>): Promise<number> {
          return value;
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/types', '/workspace');
    const identity = lowered.declarations.find((item) => item.kind === 'function' && item.name === 'identity');

    expect(lowered.diagnostics).toEqual([]);
    expect(identity).toMatchObject({
      parameters: [{ type: { arguments: [{ kind: 'primitive', name: 'Float' }], kind: 'named', name: 'Promise' } }],
      returns: { arguments: [{ kind: 'primitive', name: 'Float' }], kind: 'named', name: 'Promise' },
    });
  });

  it('distinguishes lexical type parameters from nominal types with generic-looking names', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/types/src/intersections.ts',
      `
        export interface Type {
          nominal: string;
        }
        export interface Node {
          id: number;
        }
        export type ConcreteIntersection = Type & { other: number };
        export type GenericIntersection<Type> = Type & { other: number };
        export type NodeWithTraits<Traits> = Node & Traits;
        export type EntityWithoutRuntime<T> = T;
        export type PortableEntity = EntityWithoutRuntime<Type>;
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/types', '/workspace');
    const type = (name: string) => {
      const declaration = lowered.declarations.find((item) => item.kind === 'type' && item.name === name);
      expect(declaration?.kind).toBe('type');
      return declaration?.kind === 'type' ? declaration.type : undefined;
    };

    expect(lowered.diagnostics).toEqual([]);
    expect(type('ConcreteIntersection')?.kind).toBe('anonymous');
    expect(type('GenericIntersection')).toEqual({ arguments: [], kind: 'named', name: 'Type' });
    expect(type('NodeWithTraits')).toEqual({ arguments: [], kind: 'named', name: 'Node' });
    expect(type('PortableEntity')).toEqual({ arguments: [], kind: 'named', name: 'Type' });
    expect(
      [
        ...portConfig.typeLowering.genericIntersectionBaseOverrides,
        ...portConfig.typeLowering.transparentTypeWrappers,
      ].every((exception) => exception.reason.length > 0),
    ).toBe(true);
  });

  it('lowers declared native constructors to typed capability IR without capturing shadowed values', () => {
    const source = ts.createSourceFile(
      '/workspace/upstream/packages/image/src/constructors.ts',
      `
        export function createImage(data: Uint8ClampedArray, width: number, height: number) {
          return new globalThis.ImageData(data, width, height);
        }
        export function createCanvas(width: number, height: number) {
          return new OffscreenCanvas(width, height);
        }
        export function createUrl(value: string, base: string) {
          return new URL(value, base);
        }
        export function createShadowed(ImageData: any): any {
          return new ImageData();
        }
        export function createShadowedGlobal(globalThis: any): any {
          return new globalThis.ImageData();
        }
      `,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );
    const lowered = lowerTypeScriptSource(source, '@flighthq/image', '/workspace');
    const functions = lowered.declarations.filter((declaration) => declaration.kind === 'function');
    const returnExpression = (name: string) => {
      const declaration = functions.find((candidate) => candidate.name === name);
      const statement = declaration?.body.find((candidate) => candidate.kind === 'return');
      return statement?.kind === 'return' ? statement.expression : undefined;
    };

    expect(lowered.diagnostics).toEqual([]);
    expect(returnExpression('createImage')).toMatchObject({
      capability: 'ImageData',
      kind: 'hostConstruct',
      resultType: 'FlightImageData',
    });
    expect(returnExpression('createCanvas')).toMatchObject({
      capability: 'OffscreenCanvas',
      kind: 'hostConstruct',
      resultType: 'FlightOffscreenCanvas',
    });
    expect(returnExpression('createUrl')).toMatchObject({
      capability: 'URL',
      kind: 'hostConstruct',
      resultType: 'FlightUrl',
    });
    expect(returnExpression('createShadowed')).toMatchObject({
      callee: { kind: 'identifier', name: 'ImageData' },
      kind: 'new',
    });
    expect(returnExpression('createShadowedGlobal')).toMatchObject({ kind: 'new' });
    expect(functions.find((candidate) => candidate.name === 'createImage')?.returns).toEqual({
      arguments: [],
      kind: 'named',
      name: 'FlightImageData',
    });
    expect(portConfig.typeLowering.nativeHostConstructors.every((constructor) => constructor.reason.length > 0)).toBe(
      true,
    );
  });
});
