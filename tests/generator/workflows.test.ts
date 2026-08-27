import { readFileSync, readdirSync } from 'node:fs';
import path from 'node:path';

import { parse } from 'yaml';

// Workflows encode real preconditions, and getting one wrong is invisible until CI runs — which is
// how the submodule-tag fetch ended up in both release workflows and not in CI. These assert the
// preconditions structurally, so the omission fails here instead of in a lane that then reports a
// checkout gap as though it were a problem with the change.

const workflowDirectory = path.resolve('.github/workflows');

interface Step {
  name?: string;
  run?: string;
  uses?: string;
  with?: Record<string, unknown>;
}

interface Job {
  steps: Step[];
}

function workflows(): Array<{ file: string; jobs: Record<string, Job> }> {
  return readdirSync(workflowDirectory)
    .filter((file) => file.endsWith('.yml'))
    .map((file) => ({
      file,
      jobs: (parse(readFileSync(path.join(workflowDirectory, file), 'utf8')) as { jobs: Record<string, Job> }).jobs,
    }));
}

const commandsOf = (job: Job): string => job.steps.map((step) => step.run ?? '').join('\n');

describe('workflow preconditions', () => {
  it('fetches the submodule history and tags in every job that derives the Flight version', () => {
    // scripts/flight-version.ts resolves the release through `git describe` on the submodule, and
    // `actions/checkout` may leave the submodule shallow. Fetching tag objects is insufficient:
    // `git describe` also needs the history joining a tag to HEAD. Anything that reaches that code
    // needs both before it runs.
    // `npm run test` bare and `test:release` both reach publishing.test.ts; `test:host-winit` is
    // pure cargo and does not, so the bare form is matched only when nothing follows it.
    const needsTags = /npm run test(?![:\w-])|npm run test:release|edge-version|version-packages|flight-version/u;

    for (const { file, jobs } of workflows()) {
      for (const [name, job] of Object.entries(jobs)) {
        const commands = commandsOf(job);
        if (!needsTags.test(commands)) continue;
        expect(commands, `${file}:${name} derives the Flight version, so it must unshallow the submodule`).toMatch(
          /git -C upstream fetch --tags --force --unshallow origin/u,
        );
      }
    }
  });

  it('checks out the submodule in every job that touches upstream', () => {
    // Without it the generator fails on a missing source tree rather than on the change under test.
    for (const { file, jobs } of workflows()) {
      for (const [name, job] of Object.entries(jobs)) {
        const commands = commandsOf(job);
        if (!/npm run (generate|wasm|test|check)|cargo |upstream/u.test(commands)) continue;
        const checkout = job.steps.find((step) => String(step.uses ?? '').startsWith('actions/checkout'));
        expect(checkout?.with?.submodules, `${file}:${name} checks out the submodule`).toBe('recursive');
      }
    }
  });

  it('keeps the CI lanes independent', () => {
    // The lanes answer different questions, and a `needs:` between them would let one being red hide
    // the others — which is the exact failure that kept "the shipped crate does not compile" hidden
    // behind a red generation ratchet.
    const ci = parse(readFileSync(path.join(workflowDirectory, 'ci.yml'), 'utf8')) as {
      jobs: Record<string, { needs?: unknown }>;
    };

    for (const [name, job] of Object.entries(ci.jobs)) {
      expect(job.needs, `ci.yml:${name} runs independently`).toBeUndefined();
    }
  });

  it('never publishes on an untagged push', () => {
    // A snapshot per commit would declare a dependency range npm cannot serve while the pin sits
    // ahead of Flight's newest release, which is the ordinary state under locked versioning.
    for (const file of ['release.yml', 'flight-release.yml']) {
      const on = (parse(readFileSync(path.join(workflowDirectory, file), 'utf8')) as { on: Record<string, unknown> })
        .on;
      const push = on.push as { branches?: string[] } | undefined;
      expect(push?.branches, `${file} does not publish on a branch push`).toBeUndefined();
    }
  });
});
