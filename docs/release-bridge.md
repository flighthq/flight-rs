# Release bridge

`@flighthq/bitmap-rs` is a drop-in for `@flighthq/bitmap@X` and declares `^X` as a dependency. It is therefore only installable once Flight has published X, and only _correct_ when it was generated from the commit X was cut from. Releasing on an independent schedule cannot satisfy either condition, so Flight drives the release and this repository verifies and publishes.

The dependency stays one-directional. Flight does not pin, clone, or build this repository — it sends one notification. Everything about how the Rust port is built stays here, where the toolchain, the submodule, and the parity suite already live.

## Sending side — add to Flight's release workflow

After the npm publish step in `.github/workflows/release.yml`, in the same job:

```yaml
- name: Trigger the Rust port release
  env:
    # A PAT or GitHub App token with `contents: write` on flighthq/flight-rs.
    # The default GITHUB_TOKEN cannot dispatch to another repository.
    GH_TOKEN: ${{ secrets.FLIGHT_RS_DISPATCH_TOKEN }}
    VERSION: ${{ github.ref_name }}
    COMMIT: ${{ github.sha }}
  run: |
    jq -n --arg version "$VERSION" --arg commit "$COMMIT" \
      '{event_type: "flight-release", client_payload: {version: $version, commit: $commit}}' \
      | gh api repos/flighthq/flight-rs/dispatches --method POST --input -
```

`jq` builds the body rather than a here-doc so the values are JSON-encoded rather than interpolated.

The call returns as soon as GitHub accepts it — Flight's release does not wait for, and is not failed by, the Rust publish. That is deliberate: a Flight release should not be held open by a downstream port, and a failure here is fixed and re-run without touching Flight.

## Receiving side — `.github/workflows/flight-release.yml`

Four gates before anything is published:

1. **The pin matches the released commit.** Flight names the commit; if `upstream/` points elsewhere, the port under this tree is of a different source. The run fails and names both commits.
2. **The derived version matches the release.** `scripts/flight-version.ts` works out which Flight release the pin belongs to. If that disagrees with what Flight says it published, one of the two is wrong and either number would misdescribe the tarball.
3. **The suite passes** — differential parity against the TypeScript implementation, packaging invariants, version logic.
4. **`prepack` rebuilds the wasm** from this commit, so the tarball never carries a stale module.

Gate 1 is why this lane verifies rather than moves the pin. Moving it regenerates every crate and report, which needs the full check suite and human review — a pull request, not an unattended release.

## What that implies for sequencing

The pin move is ordinary reviewed work that happens **before** Flight's release, not as part of it:

1. Flight cuts a release branch or otherwise settles the commit it will release from.
2. Here: a pull request moves `upstream/` to that commit, regenerates, and updates the `@flighthq/*` dependency ranges to `^<new version>`. CI proves the port still holds. `tests/generator/publishing.test.ts` fails until the ranges and the golden version agree with the new pin, so this cannot be forgotten.
3. Flight publishes and dispatches. This repository verifies its pin already matches, and publishes.

Between steps 2 and 3 the declared dependency names a version npm does not have yet. That is expected and does not break anything here: nothing in this repository installs its own facade, and pushes to `main` continue to publish `edge` snapshots. It resolves the moment Flight publishes.

## Manual paths

- **`workflow_dispatch`** on the same workflow, taking `version` and `commit` — re-runs a failed bridge without another Flight release.
- **A numeric tag** (`release.yml`) publishes that version to `latest` directly. The escape hatch for a port-only fix that must ship between Flight releases.
- **Pushes to `main`** (`release.yml`) publish `<version>-edge.<count>.<sha>` to the `edge` tag, which is how the port stays continuously installable between releases.

## Secrets and variables

| Name | Where | Purpose |
| --- | --- | --- |
| `FLIGHT_RS_DISPATCH_TOKEN` | Flight | Cross-repo dispatch; needs `contents: write` here |
| `NPM_TOKEN` | flight-rs | Publishing. Absent → the lane dry-runs and stays green |
| `NPM_PROVENANCE` | flight-rs (variable) | Set to `false` if this repository is private; npm rejects provenance there |
