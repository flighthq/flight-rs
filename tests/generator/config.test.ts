import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import ts from 'typescript';

import { portConfig } from '../../tools/generator/port.config.ts';
import { lowerTypeScriptSource } from '../../tools/generator/src/lower/typescript.ts';

describe('cultivated generator configuration', () => {
  it('keeps selected sources, declarations, and wasm exports aligned with the pinned upstream tree', () => {
    const workspace = path.resolve('.');
    const declarationsByCrate = new Map<string, Set<string>>();

    for (const target of portConfig.targets) {
      if (!target.sourceSelection) continue;
      const packageDirectory = target.package.replace(/^@flighthq\//u, '');
      const sourceDirectory = path.join(workspace, portConfig.upstreamDirectory, 'packages', packageDirectory, 'src');
      const selected = new Set(target.sourceSelection.sources);
      const declarations = new Set<string>();
      const declarationSelection = target.declarationSelection as
        | Readonly<Record<string, { names: readonly string[] } | undefined>>
        | undefined;

      for (const source of selected) {
        const file = path.join(sourceDirectory, source);
        expect(existsSync(file), `${target.package} selected source ${source}`).toBe(true);
        const sourceFile = ts.createSourceFile(
          file,
          readFileSync(file, 'utf8'),
          ts.ScriptTarget.Latest,
          true,
          file.endsWith('.tsx') ? ts.ScriptKind.TSX : ts.ScriptKind.TS,
        );
        const lowered = lowerTypeScriptSource(sourceFile, target.package, workspace);
        const sourceDeclarations = new Set(lowered.declarations.map((declaration) => declaration.name));
        for (const name of sourceDeclarations) declarations.add(name);

        const configured = declarationSelection?.[source];
        for (const name of configured?.names ?? []) {
          expect(sourceDeclarations.has(name), `${target.package} selected declaration ${source}:${name}`).toBe(true);
        }
      }

      for (const source of Object.keys(declarationSelection ?? {})) {
        expect(selected.has(source), `${target.package} declaration selection ${source}`).toBe(true);
      }
      declarationsByCrate.set(target.crate, declarations);
    }

    for (const facade of portConfig.wasmFacades) {
      if (facade.coreCrate === undefined) continue;
      const declarations = declarationsByCrate.get(facade.coreCrate);
      expect(declarations, `${facade.crate} core target ${facade.coreCrate}`).toBeDefined();
      for (const name of facade.exports) {
        expect(declarations?.has(name), `${facade.crate} wasm export ${name}`).toBe(true);
      }
    }
  });

  it('makes full promotion explicit and permits only source-level reasoned exclusions', () => {
    const workspace = path.resolve('.');
    const fullyPromoted = portConfig.targets.filter((target) => target.fullyPromoted);

    expect(fullyPromoted.map((target) => target.package)).toEqual(['@flighthq/types', '@flighthq/easing']);
    for (const target of fullyPromoted) {
      expect(target.sourceSelection, target.package).toBeUndefined();
      expect(target.declarationSelection, target.package).toBeUndefined();
      const packageDirectory = target.package.replace(/^@flighthq\//u, '');
      for (const exclusion of target.sourceExclusions) {
        expect(exclusion.reason.trim().length, `${target.package}:${exclusion.source} reason`).toBeGreaterThan(0);
        expect(
          existsSync(
            path.join(workspace, portConfig.upstreamDirectory, 'packages', packageDirectory, 'src', exclusion.source),
          ),
          `${target.package} excluded source ${exclusion.source}`,
        ).toBe(true);
      }
    }
  });
});
