# Release bridge

Flight uses locked versioning: every package ships at the family version whether or not it changed. `@flighthq/bitmap-wasm` belongs to that family — it is a drop-in for `@flighthq/bitmap@X` and declares `^X` — so it ships when Flight ships, at the version Flight names. Releasing on an independent schedule would publish a package depending on a version that does not exist yet, so Flight drives the release and this repository verifies and publishes.

The dependency stays one-directional. Flight does not pin, clone, or build this repository — it sends one notification. Everything about how the Rust port is built stays here, where the toolchain, the submodule, and the parity suite already live.

## Sending side — add to Flight's release workflow

In `flighthq/flight`, `.github/workflows/release.yml`, job `publish`. Add one step **immediately after `- name: Publish packages to npm`** and before the examples-site steps:

```yaml
- name: Trigger the Rust port release
  env:
    # A PAT or GitHub App token with Contents: write on flighthq/flight-rs.
    # The default GITHUB_TOKEN cannot dispatch to another repository.
    GH_TOKEN: ${{ secrets.FLIGHT_RS_DISPATCH_TOKEN }}
    VERSION: ${{ github.ref_name }}
    COMMIT: ${{ github.sha }}
  run: |
    jq -n --arg version "$VERSION" --arg commit "$COMMIT" \
      '{event_type: "flight-release", client_payload: {version: $version, commit: $commit}}' \
      | gh api repos/flighthq/flight-rs/dispatches --method POST --input -
```

Placement matters: the npm publish above it is the precondition this port needs, so the dispatch fires as soon as `@flighthq/bitmap@<version>` exists. Putting it after the examples-site build would let an unrelated asset failure block the port release.

No `permissions:` change is needed — the step authenticates with `FLIGHT_RS_DISPATCH_TOKEN`, not the job's `GITHUB_TOKEN`. `gh` and `jq` are both present on the runner, and `jq` builds the body so the values are JSON-encoded rather than interpolated.

The call returns as soon as GitHub accepts it. Flight's release does not wait for, and is not failed by, the Rust publish — a failure there is fixed and re-run here without touching Flight. A failure of the dispatch _call itself_ does fail the step, which is deliberate: it is the one condition nobody else would notice. Re-running Flight's job afterwards is safe, since the npm publish skips versions already on the registry.

## Receiving side — `.github/workflows/flight-release.yml`

The gate is **behavioral, not identity-based**. It has two layers: all 376 unchanged tests from the pinned upstream bitmap package run with every Rust override substituted, then the workflow installs the `@flighthq/bitmap` Flight just published and runs a focused differential suite against it using `packages/bitmap-wasm/vitest.config.published.ts`.

That answers the question a consumer actually has: are the Rust kernels still indistinguishable from the package this claims to substitute? Comparing commits only ever answered it by proxy, and answered it wrongly — under locked versioning the pin routinely lags the released commit by commits that never touched `bitmap`, which is a difference with no consequence.

In order:

1. **Report the relationship.** Released commit, pinned commit, and the version derived from the pin go into the run summary. Differences are `::notice::`, never failures.
2. **Install the released packages** at the version Flight named, retrying while the registry propagates.
3. **Full upstream conformance plus released-package parity.** A failure blocks: a drop-in that silently computes different pixels is worse for a consumer than a version briefly missing from the family. Fix and re-run through `workflow_dispatch`.
4. **Stamp version and dependency range** to the released version — the range moves only because step 3 just demonstrated compatibility with exactly those packages.
5. **Packaging invariants**, then publish. `prepack` rebuilds the wasm from this commit, so the tarball never carries a stale module.

The pin is never moved here. Moving it regenerates every crate and report, which needs the full check suite and human review — a pull request, not an unattended release.

## What a release gates on, and what it does not

CI verifies the **repository**. A release verifies the **artifact**. Both lanes run `npm run test:release`, not the whole suite, and the difference is the point.

The full suite also carries generator bookkeeping — how many upstream packages compile, lowering coverage, the conformance harvest shape. Those move when **upstream** changes, and they say nothing about whether this tarball works. Gating a release on them means an upstream package this port does not touch can block shipping a fix. That is not hypothetical: the pin move to `181dea5e` added seven packages and immediately failed the lowering coverage gate and three golden counts, none of which involve `bitmap`.

So the release gate is exactly what determines whether the tarball is fit to publish:

| Checked | Why |
| --- | --- |
| Unchanged upstream suite | All 42 bitmap test files and 376 cases exercise the facade, with every Rust override routed through them |
| Differential parity | The whole claim — the Rust kernels behave identically to the TypeScript they substitute |
| Packaging invariants | The tarball shadows the right names, ships the wasm glue, and has a publishable manifest |
| Version and dependency logic | The number and the range it declares are correct |
| The build itself | `prepack` rebuilds the wasm from this commit |

Deliberately **not** checked at release time: compiled-candidate counts, lowering coverage, conformance harvest, lint, formatting. Those are repository health, they belong on every push and pull request, and CI is where they gate.

One consequence worth stating plainly: a release can succeed while `npm run check` is red. That is intended. The question a release asks is "is this artifact correct", and an unrelated upstream package arriving is not evidence that it is not.

## What that implies for sequencing

Moving the pin is ordinary reviewed work on its own schedule, **not** a prerequisite for a release. A release publishes whatever the port currently is, at Flight's version, provided it still behaves as a drop-in — exactly as an unchanged Flight package ships at the family version.

So the pin moves when there is a reason: new upstream sources worth generating, a lowering fix that needs newer input, or drift the parity run has started to warn about. `tests/generator/publishing.test.ts` holds the in-repo dependency range and the golden derived version to whatever the pin currently is, so a pin move updates them in the same reviewed pull request.

The release lane does not read those in-repo values. It stamps both from the version Flight released, after proving parity against it.

## Testing it manually

Four levels, cheapest first. Each exercises the same code the workflow runs.

**1. Rehearse the lane locally.** No GitHub, no token. `VERSION` must already exist on npm.

The stamp mutates a tracked file, so the cleanup runs from a `trap` — an interrupted rehearsal that leaves a stamped range committed is exactly the failure this is guarding against, and a manual `git checkout` at the end is too easy to skip:

```sh
VERSION=0.3.0
trap 'git checkout -- packages/bitmap-wasm/package.json; npm ci' EXIT

npm install --no-save "@flighthq/bitmap@$VERSION" "@flighthq/types@$VERSION"
npx vitest run --config packages/bitmap-wasm/vitest.config.published.ts   # parity vs the release
npm run test:release                                                    # artifact gate
npx tsx scripts/version-packages.ts "$VERSION" --flight "$VERSION"      # stamp
npx vitest run tests/generator/facade-packaging.test.ts                 # post-stamp check
npm run release -- --dry-run --tag latest                               # pack + report, no upload
```

The `npm ci` in the trap matters as much as the checkout: `--no-save` leaves the released `@flighthq/*` packages in `node_modules`, and anything that regenerates afterwards resolves upstream's own imports through them.

**The committed range must be installable.** Most Flight releases are prereleases (`0.4.0-next.<count>.<sha>`), and under semver a plain `^0.4.0` does **not** satisfy `0.4.0-next.…` — so committing the stable form while npm only carries prereleases produces a package that cannot resolve. Commit the range that actually resolves today; `publishing.test.ts` checks only that it is a caret range in the same major.minor family as the pin, which is the part that must not drift.

**2. Drive the real workflow without publishing.** Actions → **Flight release bridge** → Run workflow, supplying `version` and `commit`. With no `NPM_TOKEN` configured it dry-runs and reports what it would have published, so checkout, submodule, toolchain, parity, stamp and pack all run for real. Nothing is committed, so this cannot leave a stamped manifest behind — it is the safer way to rehearse.

**3. Test the dispatch itself**, without cutting a Flight release:

```sh
jq -n --arg version 0.3.0 --arg commit "$(git -C upstream rev-parse HEAD)" \
  '{event_type: "flight-release", client_payload: {version: $version, commit: $commit}}' \
  | gh api repos/flighthq/flight-rs/dispatches --method POST --input -
```

Run it with the token Flight will use, to confirm its permissions: `404` means the token cannot see the repository, `403` means it lacks Contents: write.

**4. Publish for real.** Add `NPM_TOKEN` and repeat step 2 or 3. Worth doing by dispatch rather than waiting on a Flight release, so a permissions or provenance problem surfaces while you are watching.

## Manual paths

- **`workflow_dispatch`** on `flight-release.yml`, taking `version` and `commit` — re-runs a failed bridge without another Flight release.
- **A numeric tag** (`release.yml`) publishes that version to `latest` directly. The escape hatch for a port-only fix that must ship between Flight releases.
- **`workflow_dispatch`** on `release.yml` publishes an `<version>-edge.<count>.<sha>` snapshot to the `edge` tag, for deliberately putting a pre-release build in front of someone.

Nothing publishes on a push to `main`. Under locked versioning the pin usually sits ahead of Flight's newest _release_, so the dependency range an untagged build declares names a version npm does not have yet — a snapshot per commit would be a stream of packages nobody can install, each paid for with a full wasm build. Releases happen when Flight releases; everything else is deliberate.

## Secrets and variables

| Name | Where | Purpose |
| --- | --- | --- |
| `FLIGHT_RS_DISPATCH_TOKEN` | Flight | Cross-repo dispatch; needs `contents: write` here |
| `NPM_TOKEN` | flight-rs | Publishing. Absent → the lane dry-runs and stays green |
| `NPM_PROVENANCE` | flight-rs (variable) | Set to `false` if this repository is private; npm rejects provenance there |
