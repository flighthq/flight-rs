import { readFileSync, readdirSync } from 'node:fs';
import path from 'node:path';

import { parse } from 'yaml';

// Workflows encode real preconditions, and getting one wrong is invisible until CI runs — which is
// how the submodule-tag fetch ended up in both release workflows and not in CI. These assert the
// preconditions structurally, so the omission fails here instead of in a lane that then reports a
// checkout gap as though it were a problem with the change.

const workflowDirectory = path.resolve('.github/workflows');

interface Step {
  id?: string;
  if?: string;
  name?: string;
  run?: string;
  uses?: string;
  with?: Record<string, unknown>;
}

interface Job {
  env?: Record<string, unknown>;
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

  it('receives every Flight channel without allowing dispatches to race or move latest to a prerelease', () => {
    const file = path.join(workflowDirectory, 'flight-release.yml');
    const workflow = parse(readFileSync(file, 'utf8')) as {
      concurrency: { 'cancel-in-progress': boolean; group: string };
      jobs: Record<string, Job>;
      on: { repository_dispatch: { types: string[] } };
    };
    const publish = workflow.jobs.publish;
    if (publish === undefined) throw new Error('flight-release.yml has no publish job');
    const commands = commandsOf(publish);

    expect(workflow.on.repository_dispatch.types).toEqual(['flight-release', 'flight-snapshot']);
    expect(workflow.concurrency.group).toContain('github.event.client_payload.dist_tag');
    expect(workflow.concurrency['cancel-in-progress']).toBe(false);
    expect(publish.env?.DIST_TAG).toContain('github.event.client_payload.dist_tag');

    expect(commands).toContain('latest|edge|next');
    expect(commands).toContain('unexpected version');
    expect(commands).toContain('*-*:latest)');
    expect(commands).toContain('npm view "@flighthq/bitmap-wasm@${FLIGHT_VERSION}" version');
    expect(commands).toContain('npm run typecheck:published');
    expect(commands).toContain('npx vitest run --config packages/bitmap-wasm/vitest.config.published.ts');
    expect(commands).toContain('npm run release -- --tag "${DIST_TAG}"');
    expect(commands).not.toContain('npm run release -- --tag latest');

    const checkout = publish.steps.find((step) => String(step.uses ?? '').startsWith('actions/checkout'));
    expect(
      checkout?.with?.ref,
      'the Flight commit is informational and must never be checked out here',
    ).toBeUndefined();
    const gate = publish.steps.findIndex((step) => step.id === 'gate');
    expect(gate).toBeGreaterThanOrEqual(0);
    for (const step of publish.steps.slice(gate + 1)) {
      expect(step.if, `${step.name ?? step.uses ?? step.run} skips duplicate versions`).toBe(
        "steps.gate.outputs.skip != 'true'",
      );
    }

    const manual = parse(readFileSync(path.join(workflowDirectory, 'release.yml'), 'utf8')) as {
      concurrency: { 'cancel-in-progress': boolean; group: string };
      jobs: Record<string, Job>;
    };
    expect(manual.concurrency.group).toContain('release-');
    expect(manual.concurrency.group).toContain("'latest'");
    expect(manual.concurrency.group).toContain("'next'");
    expect(manual.concurrency.group).toContain("'edge'");
    expect(manual.concurrency['cancel-in-progress']).toBe(false);
    const manualPublish = manual.jobs.publish;
    if (manualPublish === undefined) throw new Error('release.yml has no publish job');
    const manualCommands = commandsOf(manualPublish);
    expect(manualCommands).toContain("dependencies['@flighthq/bitmap']");
    expect(manualCommands).toContain('npm run typecheck:published');
    expect(manualCommands).toContain('npx vitest run --config packages/bitmap-wasm/vitest.config.published.ts');
  });
});
