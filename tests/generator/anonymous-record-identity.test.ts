import { readFileSync, readdirSync, statSync } from 'node:fs';
import path from 'node:path';

// A synthesized name for an anonymous record is `<Owner>Record<N>`, and `N` is a counter over one module's walk
// order. Nothing makes that counter agree between modules, so a cross-module reference to such a name is a
// guess — and the dangerous outcome is not the guess that misses. It is the guess that LANDS on a different
// struct of the same name, because `Vec<(OpaqueHostValue, SomeRecord1)>` type-checks whatever `SomeRecord1`
// turns out to be.
//
// That happened. `wgpu_device_runtime.rs` typed a cache of WebGPU pipeline handles — upstream's
// `{ bindGroupLayout, pipeline }` — as `{ height, width }`, because it borrowed `WgpuRenderStateRuntimeRecord1`
// from a module that meant something else by it. It compiled. Byte-parity and idempotence checks do not notice
// either: the output is stable, just wrong. See `agents/anonymous-record-naming.md`.
//
// The emitter no longer synthesizes those names, so this asserts the property rather than a ratchet: every
// crate-root reference to a synthesized record resolves to exactly one declaration, and if several modules
// declare that name they must agree on its fields.

const generated = path.resolve('generated');

interface Declaration {
  module: string;
  fields: string[];
}

function rustSources(root: string): string[] {
  const found: string[] = [];
  const walk = (directory: string): void => {
    for (const entry of readdirSync(directory)) {
      const full = path.join(directory, entry);
      if (statSync(full).isDirectory()) walk(full);
      else if (entry.endsWith('.rs')) found.push(full);
    }
  };
  walk(root);
  return found;
}

const sources = rustSources(generated).map((file) => ({ file, text: readFileSync(file, 'utf8') }));

/** Every `pub struct <Name>Record<N>` in the generated tree, with the field names it declares. */
function declarations(): Map<string, Declaration[]> {
  const found = new Map<string, Declaration[]>();
  for (const { file, text } of sources) {
    for (const match of text.matchAll(/pub struct ([A-Za-z0-9]+Record\d+) \{([\s\S]*?)\n\}/gu)) {
      const name = match[1]!;
      const fields = [...match[2]!.matchAll(/pub ([a-z_0-9]+):/gu)].map((field) => field[1]!).sort();
      found.set(name, [...(found.get(name) ?? []), { module: path.basename(file), fields }]);
    }
  }
  return found;
}

/** Every `crate::<Name>Record<N>` reference, which is a reference across module boundaries. */
function crateRootReferences(): Map<string, string[]> {
  const found = new Map<string, string[]>();
  for (const { file, text } of sources) {
    for (const match of text.matchAll(/crate::([A-Za-z0-9]+Record\d+)\b/gu)) {
      const name = match[1]!;
      found.set(name, [...(found.get(name) ?? []), path.basename(file)]);
    }
  }
  return found;
}

describe('synthesized anonymous record identity', () => {
  it('resolves every cross-module record reference to a declaration', () => {
    const declared = declarations();
    for (const [name, referencedBy] of crateRootReferences()) {
      expect(
        declared.has(name),
        `${name} is referenced by ${referencedBy.join(', ')} but declared nowhere — a module named another ` +
          "module's anonymous record and guessed wrong",
      ).toBe(true);
    }
  });

  it('never lets a cross-module record name mean two different shapes', () => {
    // Only names actually referenced at the crate root can bind to the wrong struct. A name declared in several
    // modules but referenced only locally is shadowed by its own module's declaration, which is why the
    // long-standing `SharedStructuralRecord1` duplication is a glob-ambiguity warning rather than a defect.
    const declared = declarations();
    for (const [name, referencedBy] of crateRootReferences()) {
      // An undeclared name is the previous test's finding, not this one's.
      const sites = declared.get(name);
      if (sites === undefined) continue;
      const shapes = new Set(sites.map((item) => JSON.stringify(item.fields)));
      expect(
        [...shapes],
        `${name} is referenced across modules by ${referencedBy.join(', ')} and declared with ` +
          `${shapes.size} different shapes, so the reference binds to whichever the glob re-export wins`,
      ).toHaveLength(1);
    }
  });

  it('reads a generated tree, so the two assertions above are not vacuous', () => {
    // Both loops iterate references. With no generated sources, or none matching, they would pass trivially.
    expect(sources.length, 'generated Rust sources were found').toBeGreaterThan(100);
    expect(declarations().size, 'synthesized record declarations were found').toBeGreaterThan(50);
  });
});
