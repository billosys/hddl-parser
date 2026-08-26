# Slice05 Closing Report: Corpus Test Routing

Date: 2026-08-26
Branch: `test/corpus-test-routing`
Base: `8e36a6a tools: add corpus selection policy`
Status: locally closed; CDC verification pending

## Scope

Slice05 removed the remaining inherited corpus `#[ignore]` annotations by
routing the IPC and JSON corpus integration tests through the checked-in fast
selection policy from Slice04.

The slice did not wire GitHub Actions, did not change
`.github/workflows`, did not change the checked-in fast selection, did not add
a custom test framework, and did not expose new public crate API.

## Implementation Summary

- Added `tests/common/mod.rs` as a test-local helper for loading
  `tests/ipc/corpus-selections/fast.txt`, validating stable
  `<domain-dir>/<problem-file>` IDs, rejecting duplicate IDs, and resolving
  domain/problem paths.
- Updated `tests/integration_ipc.rs::ipc_validation_test` to run by default
  over the fast corpus selection and parse plus verify each selected
  domain/problem pair.
- Updated `tests/integration_json.rs::json_round_trip_ipc` to run by default
  over the fast corpus selection and parse, export, reimport, verify,
  re-export, and assert both exact JSON string equality and structural
  `serde_json::Value` equality.
- Updated `tests/ipc/corpus-selections/README.md` to document the default
  fast-test routing and preserve the explicit full-corpus command.

## Ledger Walk

| ID | Result | Evidence |
|----|--------|----------|
| T5-1 | Done | `git log --oneline --decorate -5` showed the Slice04 base at `8e36a6a`; `tools/corpus_measure.rs` and `tests/ipc/corpus-selections/fast.txt` exist; `Cargo.toml` still declares `name = "corpus_measure"` and `path = "tools/corpus_measure.rs"`. |
| T5-2 | Done | `rg -n "#\\[ignore" src tests -g '*.rs'` returned no matches. The IPC and JSON corpus tests no longer carry `#[ignore]`. |
| T5-3 | Done | `cargo test --locked --test ipc` passed: 1 passed, 0 failed, 0 ignored. The test uses the shared fast-selection helper and reports `case.id` in read/parse/verify failures. |
| T5-4 | Done | `cargo test --locked --test json` passed: 9 passed, 0 failed, 0 ignored. The corpus route uses the shared fast-selection helper and checks exact plus structural JSON equality with stable case IDs. |
| T5-5 | Done | `HDDL_CORPUS_SELECTION=full HDDL_CORPUS_ASSERT=both cargo run --locked --example corpus_measure` passed: `selection=full, assertion=both, discovered_cases=900, selected_cases=900`; `summary attempted=900 completed=900 failures=0`; `json_equality_disagreements=0`; `json_assertion_failures=0`. |
| T5-6 | Done | `git diff -- .github/workflows` produced no output; `actionlint .github/workflows/ci.yml` passed. |
| T5-7 | Done | Passed: `cargo fmt --check`; `cargo check --locked --all-targets`; `cargo test --locked --all-targets`; `RUSTFLAGS="-D rust-2024-compatibility" cargo check --locked --all-targets`; `cargo clippy --locked --all-targets -- -D warnings`; `cargo build --release --locked --bins`; `./target/release/hddl_analyzer --help`; `git diff --check`; `git diff --cached --check`; `git -C ../planning diff --check`. |
| T5-8 | Done | This close report records the row walk and bubbles the remaining CI-policy decision to Arc05 without wiring CI in this slice. |

## Verification Transcript Summary

```text
git status --short --branch --untracked-files=all --ignored=matching
## test/corpus-test-routing
A  tests/common/mod.rs
M  tests/integration_ipc.rs
M  tests/integration_json.rs
M  tests/ipc/corpus-selections/README.md
!! target/
!! workbench/

git log --oneline --decorate -5
8e36a6a (HEAD -> test/corpus-test-routing, origin/main, main) tools: add corpus selection policy
c6e4907 tools: add corpus measurement utility
e534df2 fix: repair fast ignored tests
7e2d8a7 test: consolidate helper and private naming cohesion
d17b41f fix: repair parser API error boundary

rg -n "#\\[ignore" src tests -g '*.rs'
# no matches

cargo test --locked --test ipc
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.49s

cargo test --locked --test json
test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.31s

cargo test --locked --all-targets
# Passed all targets, including IPC 1 passed/0 ignored and JSON 9 passed/0 ignored.

HDDL_CORPUS_SELECTION=fast HDDL_CORPUS_ASSERT=both cargo run --locked --example corpus_measure
selection=fast, assertion=both, discovered_cases=900, selected_cases=43
summary attempted=43 completed=43 failures=0
json_equality_disagreements=0
json_assertion_failures=0

HDDL_CORPUS_SELECTION=full HDDL_CORPUS_ASSERT=both cargo run --locked --example corpus_measure
selection=full, assertion=both, discovered_cases=900, selected_cases=900
summary attempted=900 completed=900 failures=0
json_equality_disagreements=0
json_assertion_failures=0
```

Additional gates passed:

- `cargo fmt --check`
- `cargo check --locked --all-targets`
- `RUSTFLAGS="-D rust-2024-compatibility" cargo check --locked --all-targets`
- `cargo clippy --locked --all-targets -- -D warnings`
- `cargo build --release --locked --bins`
- `./target/release/hddl_analyzer --help`
- `git diff -- .github/workflows`
- `actionlint .github/workflows/ci.yml`
- `git diff --check`
- `git diff --cached --check`
- `git -C ../planning diff --check`

## Bubble-Up To Arc05

Slice05 eliminated the remaining inherited `#[ignore]` debt. No Rust
`#[ignore]` annotations remain under `src` or `tests`, and the two former
corpus skips now run through enabled default integration tests.

The default test suite gained fast corpus coverage without becoming the full
corpus gate. IPC validation now covers the checked-in 43-case fast selection,
and JSON round-trip validation covers the same fast selection with exact and
structural equality checks. Full corpus validation remains the explicit
Slice04 policy command:

```bash
HDDL_CORPUS_SELECTION=full HDDL_CORPUS_ASSERT=both cargo run --locked --example corpus_measure
```

The next exact CI policy slice should be
`slice06-github-actions-corpus-policy`. Its scope should decide and wire the
branch-push, PR, scheduled, and post-merge/main corpus gates against the now
verified command surface. A reasonable policy candidate for that slice is:
run default locked tests on branch pushes, run fast explicit corpus measurement
where CI time is constrained, and place the full
`HDDL_CORPUS_SELECTION=full HDDL_CORPUS_ASSERT=both` command on the PR and/or
post-merge/main boundary after the operator confirms runtime and platform
coverage expectations.
