import { spawnSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
import path from 'node:path';

import { parse } from 'yaml';

// The receiving half of the release bridge is only correct relative to what Flight actually sends, and
// that sender lives in another repository. Asserting the receiver against itself cannot catch a payload
// mismatch — requiring `dist_tag` passed every structural check here while making every real dispatch
// fail before checkout.
//
// So this pins the sender's payload as a fixture and RUNS the workflow's own validation script against
// it, rather than matching its text. The fixture is the contract to re-check when the sender changes;
// the execution is what proves the receiver accepts it.
//
// Sender of record, `flighthq/flight` `.github/workflows/release.yml`, job `publish`:
//
//     gh api --method POST repos/flighthq/flight-rs/dispatches \
//       --raw-field event_type=flight-release \
//       --raw-field "client_payload[version]=${GITHUB_REF_NAME}" \
//       --raw-field "client_payload[commit]=${GITHUB_SHA}"
//
// No `dist_tag`, and a comment above it treats the payload as fixed across both receivers.
const SENDER_SENDS = ['version', 'commit'] as const;

const workflowFile = path.resolve('.github/workflows/flight-release.yml');
const workflowText = readFileSync(workflowFile, 'utf8');
const workflow = parse(workflowText) as {
  concurrency: { group: string };
  jobs: Record<string, { env?: Record<string, string>; steps: { name?: string; run?: string }[] }>;
};
const publish = workflow.jobs.publish;

/** The `Validate payload` step's script, run in isolation with an explicit environment. */
function validate(environment: Record<string, string>): { status: number; stderr: string; stdout: string } {
  const step = publish?.steps.find((candidate) => candidate.name === 'Validate payload');
  if (step?.run === undefined) throw new Error('flight-release.yml has no Validate payload step');

  const result = spawnSync('bash', ['-c', step.run], {
    encoding: 'utf8',
    env: { PATH: process.env.PATH ?? '', ...environment },
  });
  return { status: result.status ?? -1, stderr: result.stderr, stdout: result.stdout };
}

describe('the Flight release dispatch this repository receives', () => {
  it('defaults the channel, because the sender does not send one', () => {
    // Both places that read the channel must default identically. The env block decides what is
    // published; the concurrency group decides which pointer is locked while it happens. A default in
    // one and not the other publishes to `latest` under an empty lock shared with every other channel.
    expect(publish?.env?.DIST_TAG, 'the job defaults the channel to latest').toBe(
      "${{ github.event.client_payload.dist_tag || inputs.dist_tag || 'latest' }}",
    );
    expect(workflow.concurrency.group, 'the concurrency group defaults the same way').toBe(
      "release-${{ github.event.client_payload.dist_tag || inputs.dist_tag || 'latest' }}",
    );

    // The fixture is only meaningful while it names fields the receiver actually reads.
    for (const field of SENDER_SENDS) {
      expect(workflowText, `the receiver reads client_payload.${field}`).toContain(
        `github.event.client_payload.${field}`,
      );
    }
  });

  it('accepts a stable release on the defaulted channel and refuses the empty one', () => {
    // Together these are the defect and its fix. `latest` is what the default resolves the sender's
    // channel-less payload to, and the empty string is what it resolved to before — which failed.
    expect(validate({ DIST_TAG: 'latest', FLIGHT_COMMIT: 'a'.repeat(40), FLIGHT_VERSION: '0.5.0' }).status).toBe(0);

    const empty = validate({ DIST_TAG: '', FLIGHT_COMMIT: 'a'.repeat(40), FLIGHT_VERSION: '0.5.0' });
    expect(empty.status, 'an unresolved channel is still a hard failure').not.toBe(0);
    expect(empty.stdout + empty.stderr).toContain('unexpected dist_tag');
  });

  it('still refuses a prerelease on latest, which is what makes the default safe', () => {
    // Defaulting to `latest` would be reckless if it silently promoted snapshots. Flight's snapshot
    // versions are hyphenated, so a snapshot dispatched without a channel is REFUSED here rather than
    // moving the pointer ordinary installs read.
    const snapshot = validate({
      DIST_TAG: 'latest',
      FLIGHT_COMMIT: 'a'.repeat(40),
      FLIGHT_VERSION: '0.5.1-next.1685.dc0e23a',
    });

    expect(snapshot.status, 'a prerelease may not move latest').not.toBe(0);
    expect(snapshot.stdout + snapshot.stderr).toContain('refusing prerelease');

    // The same version is fine on a prerelease channel.
    expect(
      validate({ DIST_TAG: 'next', FLIGHT_COMMIT: 'a'.repeat(40), FLIGHT_VERSION: '0.5.1-next.1685.dc0e23a' }).status,
    ).toBe(0);
  });

  it('refuses a version or channel it does not recognise', () => {
    for (const version of ['', 'latest', '0.5', '0.5.0; rm -rf /', 'v0.5.0']) {
      expect(
        validate({ DIST_TAG: 'latest', FLIGHT_COMMIT: '', FLIGHT_VERSION: version }).status,
        `version "${version}" is refused`,
      ).not.toBe(0);
    }

    for (const tag of ['beta', 'LATEST', 'latest extra']) {
      expect(
        validate({ DIST_TAG: tag, FLIGHT_COMMIT: '', FLIGHT_VERSION: '0.5.0' }).status,
        `channel "${tag}" is refused`,
      ).not.toBe(0);
    }
  });
});
