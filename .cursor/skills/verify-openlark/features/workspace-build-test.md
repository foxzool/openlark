# Workspace build and test

Workspace build and test is how a maintainer proves the SDK still compiles and its automated tests pass after a code change.

## Sub-features

- `build-all-features` compiles every workspace member with `--all-features`.
- `test-workspace` runs the full workspace test suite with `--all-features`.
- `test-focused` runs a single crate's lib tests when the change is localized.

## How to get to it (user POV)

- From the repo root, run `cargo build --workspace --all-features`.
- From the repo root, run `cargo test --workspace --all-features`.
- From the repo root, run `cargo test -p <crate> --lib` for a focused crate.

## Driving it with control-openlark

Preconditions:

- `control-openlark doctor` printed `doctor=PASS`.
- Optional: `CARGO_TARGET_DIR` points at an isolated `/tmp/openlark-verify-*` directory.

- **Warm build.** Launch the workspace build. Run `control-openlark launch --target-dir "$CARGO_TARGET_DIR"` (omit `--target-dir` to use the default `target/`). The transcript ends with `READY workspace build finished` and exit code `0`.
- **Full test suite.** Run the maintainer test command. Run `control-openlark test`. Exit code `0` and `artifacts/<run-id>/test.txt` contain `test result: ok` (or crate-level ok summaries) with no failed tests.
- **Focused crate.** When only one crate changed, run `control-openlark test --package openlark-core --lib`. Exit code `0` and the transcript names `openlark-core`.
- **Proof.** Keep `doctor.txt`, `launch-build.txt`, and `test.txt` under `artifacts/<run-id>/`. Re-list that directory after cleanup and confirm the files remain.

## Gotchas

- Full workspace tests take on the order of a few minutes; do not truncate the log early.
- A green compile without tests is incomplete for behavioral library changes.
- Feature-gated modules need `--all-features` or the explicit feature list that exercises the change; default features alone miss most crates.
- Do not claim proof from a different `CARGO_TARGET_DIR` than doctor/launch used.
