# Public examples

Public examples are the root-crate sample programs CI compile-checks and maintainers run to show Client setup and helpers without requiring a GUI.

## Sub-features

- `examples-compile` compile-checks every public example listed in `scripts/check-public-examples.sh`.
- `client-setup-offline` runs `client_setup` with placeholder credentials and no live lookup env vars.
- `client-setup-live-lookup` optionally exercises user/chat helpers when search env vars are set (network + real app credentials).

## How to get to it (user POV)

- Run `bash scripts/check-public-examples.sh` from the repo root.
- Run `cargo run --example client_setup --features "auth,communication"`.
- Read example names and feature lists from `examples/README.md`.

## Driving it with control-openlark

Preconditions:

- `control-openlark doctor` printed `doctor=PASS`.
- Launch has completed for this run, or a warm target dir already exists.
- For offline proof: `OPENLARK_USER_SEARCH_NAME` and `OPENLARK_CHAT_SEARCH_NAME` are unset.

- **Compile gate.** Run the CI example check. Run `control-openlark example check`. Exit code `0` and `example-check.txt` show each `Checking …` step succeeding.
- **Offline client setup.** Run the getting-started example. Run `control-openlark example run client_setup --features "auth,communication"`. Exit code `0`; stdout contains `Client 创建成功` and a line that lookup was skipped because search env vars were unset.
- **Live lookup (optional).** Only when intentionally proving network helpers: export real `OPENLARK_APP_ID` / `OPENLARK_APP_SECRET` and a search name, then run the same `example run` command. Exit code `0` and stdout contain a hit line for the user or chat. If credentials are absent, record `verified-unreachable` with the missing env precondition — do not substitute another example as proof.
- **Proof.** Retain `example-check.txt` and `example-client_setup.txt`. Confirm the offline transcript still shows the skip lines after cleanup.

## Gotchas

- Placeholder `cli_demo` / `demo_secret` values are enough for Client construction; they are not enough for live API calls.
- `control-openlark example run` unsets lookup env vars on purpose for the offline path — use a raw `cargo run` only when you intentionally need live lookup.
- Websocket and docs examples need different `--features` lists; copy them from `examples/README.md`, do not reuse `auth,communication`.
- Compile-check success does not prove runtime behavior; pair it with at least one `example run` when the change touches example or Client prelude code.
