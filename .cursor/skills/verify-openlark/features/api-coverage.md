# API coverage harness

API coverage compares `api_list_export.csv` to on-disk typed API implementations and reports coverage gaps the way release tooling does.

## Sub-features

- `coverage-one-crate` validates a single crate's typed API files against the CSV catalog.
- `coverage-all-crates` validates every mapped crate and writes reports under `reports/api_validation/`.

## How to get to it (user POV)

- Run `python3 tools/validate_apis.py --crate openlark-docs`.
- Run `python3 tools/validate_apis.py --all-crates`.
- Use the project skill `openlark-api-validation` for interpretation of reports.

## Driving it with control-openlark

Preconditions:

- `control-openlark doctor` printed `doctor=PASS`.
- `api_list_export.csv` exists at the repo root.
- No cargo build is required for this harness.

- **Single crate.** Validate one business crate. Run `control-openlark api-coverage --crate openlark-docs`. Exit code `0` and `api-coverage.txt` summarize matches/gaps for that crate.
- **All crates.** Run the release-style sweep. Run `control-openlark api-coverage --all-crates`. Exit code `0`; evidence directory contains a copied `api_validation/` report tree when the tool wrote `reports/api_validation/`.
- **Proof.** Keep `api-coverage.txt` and any copied `api_validation/` files. A coverage percentage alone is insufficient if the transcript shows the wrong crate name.

## Gotchas

- This harness answers “is the API file present / path-matched?”, not “are the fields correct?” — use the field-verify feature for field correctness.
- Path layout noise (flat vs nested project segments) can mark soft matches; read the report classes before claiming a true gap.
- Do not edit `api_list_export.csv` during verification to force a green result.
