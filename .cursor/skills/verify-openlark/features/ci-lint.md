# CI lint dual-mode

CI lint dual-mode is the clippy gate that must pass both with all features enabled and with default features disabled, matching `.github/workflows` / `just lint`.

## Sub-features

- `clippy-all-features` runs clippy on the workspace with `--all-features -- -Dwarnings`.
- `clippy-no-default-features` runs clippy with `--no-default-features -- -Dwarnings`.

## How to get to it (user POV)

- Run `cargo clippy --workspace --all-targets --all-features -- -Dwarnings`.
- Run `cargo clippy --workspace --all-targets --no-default-features -- -Dwarnings`.

## Driving it with control-openlark

Preconditions:

- `control-openlark doctor` printed `doctor=PASS`.
- Prefer after `control-openlark launch` so dependencies are warm.

- **Dual clippy.** Run the CI lint pair. Run `control-openlark lint`. Exit code `0` and `lint.txt` contains both clippy invocations finishing without warnings-as-errors failures.
- **Proof.** Retain `lint.txt`. One mode passing while the other fails is not a pass — both must succeed.

## Gotchas

- Feature-gated code can be clippy-clean under default features and dirty under `--all-features` (and the reverse for `cfg` exclusions). Always run both.
- This is a static analysis proof; it does not replace `cargo test` for behavioral changes.
- Full-workspace clippy is slow; do not kill the process by name if it appears hung — wait or stop the specific PID you started.
