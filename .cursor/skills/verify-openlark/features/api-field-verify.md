# API field verify

API field verify scans Rust request/response structs for suspicious patterns and optionally compares them to Feishu official docs.

## Sub-features

- `fields-offline` runs the fast crate scan without fetching docs.
- `fields-fetch-docs` enables `--fetch-docs` for live comparison (network).
- `fields-one-api` gates a single API id with `--api-id`.

## How to get to it (user POV)

- Run `python3 tools/verify_api_fields.py --crate openlark-workflow`.
- Run `python3 tools/verify_api_fields.py --crate openlark-workflow --fetch-docs`.
- Run `python3 tools/verify_api_fields.py --api-id <id> --fetch-docs`.
- For SPA document rendering failures, switch to `.agents/skills/openlark-api-field-verify/`.

## Driving it with control-openlark

Preconditions:

- `control-openlark doctor` printed `doctor=PASS`.
- Offline mode needs no Feishu credentials.
- `--fetch-docs` needs network egress to Feishu documentation hosts.

- **Offline scan.** Run the fast gate on a crate touched by the change. Run `control-openlark field-verify --crate openlark-workflow`. Exit code `0` (or the tool's documented non-zero only when real field issues exist — treat issues as product findings, not harness failure). Transcript is `field-verify.txt`.
- **Live docs.** When field correctness against official docs is required, run `control-openlark field-verify --crate openlark-workflow --fetch-docs`. Observe that the transcript shows fetch/compare activity, not only the offline pattern pass.
- **Single API.** When validating one endpoint, run `control-openlark field-verify --api-id <id> --fetch-docs`. The transcript names that API id.
- **Proof.** Retain `field-verify.txt`. If live fetch is blocked by network, record `verified-unreachable` with the network precondition; do not claim doc parity from offline mode alone.

## Gotchas

- Offline success does not prove parity with Feishu docs.
- Feishu docs are SPAs; when the CLI fetch returns empty shells, use `openlark-api-field-verify` (playwright) rather than inventing fields.
- Canonical doc URLs come from CSV `fullPath`, not hand-built paths.
- Do not “fix” verification by deleting failing APIs from the scan set.
