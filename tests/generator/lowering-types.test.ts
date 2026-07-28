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
});
