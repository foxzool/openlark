# OpenLark verification map

This directory is the maintained source for verifying OpenLark as a maintainer exercises the SDK. Read the index before driving, then use the matching feature file as the recipe.

## Baseline preconditions

- Repository root is the OpenLark workspace (`Cargo.toml` members include `crates/openlark-core`).
- `rustc` / `cargo` ≥ **1.88**, `python3` on `PATH`.
- Put `control-openlark` on `PATH` from `.cursor/skills/verify-openlark/scripts/`.
- Run `control-openlark doctor` and require `doctor=PASS`.
- Prefer an isolated `CARGO_TARGET_DIR` under `/tmp/openlark-verify-*` when the machine may have a concurrent cargo build.
- For offline example proofs, leave `OPENLARK_USER_SEARCH_NAME` and `OPENLARK_CHAT_SEARCH_NAME` unset.
- Never treat a foreign shell's in-progress cargo job as this run's instance.

## Driving conventions

- Start every recipe from doctor + launch unless the feature is a pure Python harness that only needs doctor.
- Treat every command as literal. Keep package names, example names, and feature lists unchanged.
- Capture transcripts via `control-openlark` so they land under `artifacts/<run-id>/`.
- Restore nothing in git; verification is read-mostly. Do not commit `artifacts/` or `.run-state/`.
- Report an unreachable path with the attempted command and unmet precondition (missing credentials for live lookup, network for `--fetch-docs`).

## Proof and skip reporting

- Capture the command, exit code, and resulting toolchain/test/report state — not only a green summary line.
- Mutation proofs (API model changes) require a second view: re-run the focused `cargo test -p <crate>` or re-run field-verify.
- Record the feature ID and entry point with every artifact.
- Do not report a skipped live Feishu call as a live integration proof.

## Feature entry contract

Each feature file starts with an H1 title and one paragraph describing the user-visible behavior. It then uses exactly four H2 sections in this order.

1. `Sub-features` lists short IDs with one line for each behavior.
2. `How to get to it (user POV)` lists every maintainer entry point.
3. `Driving it with control-openlark` starts with `Preconditions:` and uses labeled bullets that pair each action with an exact command and observable result.
4. `Gotchas` lists traps that can waste or invalidate a verification run.

Keep implementation details out of the map. Name only maintainer paths, stable handles, required state, commands, and observable proof.

## Features

- [Workspace build and test](./workspace-build-test.md) covers warm build and `cargo test` as the primary proof path.
- [Public examples](./public-examples.md) covers CI compile-check and the offline `client_setup` run.
- [API coverage harness](./api-coverage.md) covers `tools/validate_apis.py` against `api_list_export.csv`.
- [API field verify](./api-field-verify.md) covers offline and live-doc field checks via `tools/verify_api_fields.py`.
- [CI lint dual-mode](./ci-lint.md) covers the all-features / no-default-features clippy gate.
