import { execFileSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
import path from 'node:path';

import { publishablePackages } from '../../scripts/publishable-packages.ts';

// `docs/flight-compiler-adoption.md` is a register of measurements and pointers, and both kinds of
// claim rot silently. A measurement taken against one Flight pin says nothing about the next one, and
// an index into `agents/architecture.md` survives the deletion of the prose it indexes. These assert
// the three facts the document cannot keep true on its own.

const workspace = path.resolve('.');
const register = readFileSync(path.join(workspace, 'docs/flight-compiler-adoption.md'), 'utf8');

describe('the flight-compiler adoption register', () => {
  it('records the Flight pin its measurement was taken against', () => {
    // The refusal counts are a property of one upstream tree. Moving the pin invalidates them, and
    // this failure is the prompt to re-run the reproduction rather than leave stale numbers reading
    // as current — the same reason publishing.test.ts pins its derived version as a golden value.
    const pin = execFileSync('git', ['-C', 'upstream', 'rev-parse', 'HEAD'], {
      cwd: workspace,
      encoding: 'utf8',
    }).trim();

    expect(register, 'the register names the current upstream pin').toContain(pin);
  });

  it('indexes contract paragraphs that still exist in agents/architecture.md', () => {
    // The register's substance is that these Rust-target semantics must survive the migration. If a
    // paragraph is reworded away, the register is pointing at nothing and nobody finds out from it.
    const architecture = readFileSync(path.join(workspace, 'agents/architecture.md'), 'utf8');
    const contracts = [
      'Rust-target contracts to preserve when portable values move into',
      'representation-driven semantics that should remain explicit when the generic IR moves to',
      'Rust package-planning invariant for',
    ];

    for (const contract of contracts) {
      expect(architecture, `agents/architecture.md still states: ${contract}`).toContain(contract);
    }
  });

  it('holds while nothing here depends on the compiler package', () => {
    // The register opens by stating that no part of this repository installs or calls
    // `@flighthq/tool-compiler`. The day that stops being true, the division of ownership, the
    // measured position, and the phase-6 list all need rewriting — so a dependency appearing is a
    // reason to revisit the document, not a change to absorb silently.
    const manifests = [
      path.join(workspace, 'package.json'),
      ...publishablePackages(workspace).map((facade) => path.join(facade.directory, 'package.json')),
    ];

    for (const manifest of manifests) {
      const parsed = JSON.parse(readFileSync(manifest, 'utf8')) as {
        dependencies?: Record<string, string>;
        devDependencies?: Record<string, string>;
      };
      const dependencies = { ...parsed.dependencies, ...parsed.devDependencies };
      expect(
        Object.keys(dependencies),
        `${path.relative(workspace, manifest)} does not yet depend on the compiler package`,
      ).not.toContain('@flighthq/tool-compiler');
    }

    expect(register).toContain('Nothing in this repository installs or calls `@flighthq/tool-compiler` today');
  });
});
