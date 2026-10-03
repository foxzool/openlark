---
name: verify-openlark
description: "Prove OpenLark (Feishu/Lark Rust SDK) changes the way a maintainer does — cargo build/test, public examples, API coverage and field-verify harnesses. Use when verifying a library change, before claiming an SDK fix works, or when asked to drive OpenLark verification."
---

# Verify OpenLark

OpenLark is a **Rust workspace library SDK**, not a GUI or long-lived server. Verification means compiling and exercising the SDK the way maintainers do: `cargo` build/test, public examples (usually offline), and the Python harnesses under `tools/`. Read this skill cold mid-task; follow it literally.

Put `control-openlark` on `PATH` for the session:

```bash
export PATH="$(pwd)/.cursor/skills/verify-openlark/scripts:$PATH"
# or from anywhere:
export PATH="/absolute/path/to/repo/.cursor/skills/verify-openlark/scripts:$PATH"
```

Evidence lives under `.cursor/skills/verify-openlark/artifacts/<run-id>/` (override with `VERIFY_OPENLARK_ARTIFACTS`). Cleanup never deletes that tree.

Feature recipes: [features/README.md](features/README.md). A proof that drives one convenient entry point is incomplete when the map lists others for the change under test.

## Launch

There is no daemon to keep alive. Launch = toolchain ready + one workspace build so later drives share a warm `target/`.

```bash
# Optional isolation (recommended when sharing a machine):
export CARGO_TARGET_DIR="/tmp/openlark-verify-$USER-$$"
control-openlark launch --target-dir "$CARGO_TARGET_DIR"
# Ready when the transcript ends with: READY workspace build finished
# Equivalent raw command:
#   cargo build --workspace --all-features
```

Teardown is `control-openlark cleanup` (see Cleanup). Do not leave isolated `CARGO_TARGET_DIR` trees forever; purge only paths under `/tmp/openlark-verify-*` via `cleanup --purge-target`.

If `rustc` is older than **1.88**, upgrade before generating proofs (`rustup update stable && rustup default stable`). Edition 2024 requires a modern Cargo.

## Doctor

Run first whenever anything looks off, and before the first drive of a run:

```bash
control-openlark doctor
```

Doctor passes only when all of these hold:

- Repo root contains `Cargo.toml` and `crates/openlark-core`
- `rustc` / `cargo` / `python3` on `PATH`
- `rustc` major.minor ≥ **1.88**
- `cargo metadata --no-deps` succeeds (manifest parses)
- Prints `doctor=PASS` and writes `artifacts/<run-id>/doctor.txt`

If `OPENLARK_USER_SEARCH_NAME` or `OPENLARK_CHAT_SEARCH_NAME` is set, doctor warns that examples may hit the live Feishu API — unset them for offline proofs.

Never drive a shared interactive cargo session you did not start for this verification run.

## Drive

Prefer `control-openlark` over raw commands so transcripts land in the run evidence directory.

| Intent | Harness command | Maintainer equivalent |
|--------|-----------------|------------------------|
| Unit/integration tests | `control-openlark test` or `control-openlark test --package openlark-core --lib` | `cargo test --workspace --all-features` |
| Public example compile gate | `control-openlark example check` | `bash scripts/check-public-examples.sh` |
| Offline example run | `control-openlark example run client_setup --features "auth,communication"` | `cargo run --example client_setup --features "auth,communication"` |
| Typed API coverage | `control-openlark api-coverage --crate openlark-docs` | `python3 tools/validate_apis.py --crate openlark-docs` |
| Field sanity (offline) | `control-openlark field-verify --crate openlark-workflow` | `python3 tools/verify_api_fields.py --crate openlark-workflow` |
| Field vs live docs | `control-openlark field-verify --crate openlark-workflow --fetch-docs` | same + `--fetch-docs` (needs network; use project skill `openlark-api-field-verify` for SPA docs) |
| CI-style clippy | `control-openlark lint` | dual `cargo clippy` all-features + no-default-features `-Dwarnings` |

Stable handles for this repo: crate names (`openlark-core`, `openlark-docs`, …), example names from root `Cargo.toml` `[[example]]`, script paths `scripts/check-public-examples.sh`, `tools/validate_apis.py`, `tools/verify_api_fields.py`. Do not invent alternate entry points.

Deep field verification against Feishu SPA docs is owned by `.agents/skills/openlark-api-field-verify/` — invoke that skill when `--fetch-docs` is insufficient; this verify skill only wraps the CLI gate.

## Evidence

Proof standards for a library SDK:

- Exercise the **maintainer path** (cargo/examples/tools), not private test hooks that skip the public API surface you claim to fix.
- Capture **command + exit code + full stdout/stderr** in `artifacts/<run-id>/`.
- For mutations to API models, prove with `cargo test` on the affected crate **and** (when the change is coverage/fields) the matching Python harness.
- Side effects: compiled binaries under `CARGO_TARGET_DIR`, reports under `reports/api_validation/` (copied into evidence by `api-coverage`).
- Mocks/wiremock inside unit tests are fine — they are the production-isolated boundary. Do **not** claim a live Feishu round-trip unless credentials and network were intentionally used and the transcript shows the response.
- `client_setup` without `OPENLARK_USER_SEARCH_NAME` / `OPENLARK_CHAT_SEARCH_NAME` is an **offline** proof; observe that the transcript says those lookups were skipped — do not trust the example name alone if those env vars might be set.

Locate the current evidence directory:

```bash
control-openlark evidence-path
```

Minimum artifact set for a claimed fix: `doctor.txt`, the drive transcript for the mapped feature you ran (`test.txt`, `example-client_setup.txt`, …), and exit status 0 on that drive.

## Cleanup

```bash
control-openlark cleanup                 # drop .run-state readiness markers only
control-openlark cleanup --purge-target  # also rm isolated /tmp/openlark-verify-* target dir
```

Rules:

- Never `pkill` / kill by process name. This skill starts no long-lived processes; cargo/python children exit with the drive.
- Never delete `artifacts/` proof trees.
- Do not `cargo clean` the developer's default `target/` unless this run used an isolated `CARGO_TARGET_DIR` and `--purge-target` is set.

After cleanup, confirm evidence still exists at the path printed by `evidence-path` during the run (re-create path from `artifacts/<run-id>/` listing if state was cleared).

## Helpers

Executable: `.cursor/skills/verify-openlark/scripts/control-openlark`

```bash
control-openlark doctor
control-openlark launch [--target-dir DIR]
control-openlark test [--package PKG] [--lib] [-- -- FILTER]
control-openlark example run NAME [--features FEATS]
control-openlark example check
control-openlark api-coverage [--crate NAME|--all-crates]
control-openlark field-verify --crate NAME [--fetch-docs] [--api-id ID]
control-openlark lint
control-openlark evidence-init [RUN_ID]
control-openlark evidence-path
control-openlark cleanup [--purge-target]
```

## Maintenance

Keep the feature map honest with `/maintain-verification-skill` as the SDK surface changes (new examples, harness flags, MSRV bumps).
