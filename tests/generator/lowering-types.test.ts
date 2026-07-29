import ts from 'typescript';

import { portConfig } from '../../tools/generator/port.config.ts';
import { lowerTypeScriptSource } from '../../tools/generator/src/lower/typescript.ts';

describe('configured type lowering exceptions', () => {
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
