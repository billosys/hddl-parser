# Slice06 Closing Report: GitHub Actions Corpus Policy

Date: 2026-08-26
Branch: `ci/corpus-policy`
Base: `af0968b test: route corpus tests through fast selection`
Status: locally closed; CDC verified

## Scope

Slice06 wired the verified Slice04/Slice05 corpus command surface into
GitHub Actions policy.

The feature change is limited to `.github/workflows/ci.yml` and the
corpus-selection README. Parser, verifier, transpiler, JSON behavior, Rust
tests, corpus measurement code, and `tests/ipc/corpus-selections/fast.txt`
were not changed.

## Implementation Summary

- Added CI branch globs for cleanup branch families:
  `audit/*`, `ci/*`, `edition/*`, `measure/*`, `policy/*`, and `test/*`,
  while preserving `main`, `master`, `fix/*`, `feature/*`, and `release/*`.
- Added a weekly scheduled workflow trigger:
  `cron: "17 8 * * 1"`.
- Added `--locked` to Cargo commands that resolve dependencies:
  `cargo check`, Rust 2024 compatibility `cargo check`, `cargo clippy`,
  `cargo test`, release `cargo build`, and both `cargo run --example
  corpus_measure` commands.
- Added an unconditional explicit fast corpus measurement step for every
  workflow run.
- Added a full corpus measurement step inside the existing Linux/macOS matrix,
  conditioned to run only for pull requests, scheduled runs, and pushes to
  `refs/heads/main` or `refs/heads/master`.
- Updated `tests/ipc/corpus-selections/README.md` to describe the CI policy now
  that it is no longer future work.

## Ledger Walk

| ID | Result | Evidence |
|----|--------|----------|
| T6-1 | Done | `git log --oneline --decorate -5` showed `af0968b` as the Slice05 base; `rg -n "#\\[ignore" src tests -g '*.rs'` returned no matches; `cargo test --locked --all-targets` passed with IPC 1 passed/0 ignored and JSON 9 passed/0 ignored. |
| T6-2 | Done | `rg -n "ubuntu-24.04|macos-15|actions/checkout@v7" .github/workflows/ci.yml` found the expected runner labels and checkout action; `actionlint .github/workflows/ci.yml` passed. |
| T6-3 | Done | `rg -n "cargo (check|clippy|test|build).*--locked|cargo run --locked --example corpus_measure" .github/workflows/ci.yml` found locked check, compatibility check, clippy, test, fast corpus, full corpus, and release build commands. |
| T6-4 | Done | Push branch globs include the cleanup branch families; the fast corpus step has no event condition; the full corpus step excludes non-default branch pushes by requiring PR, schedule, `refs/heads/main`, or `refs/heads/master`; `actionlint` passed. |
| T6-5 | Done | The full corpus step is inside the existing `ubuntu-24.04`/`macos-15` matrix and includes `github.event_name == 'pull_request'`; `actionlint` passed. |
| T6-6 | Done | Push branches include `main` and `master`; the full corpus condition includes `refs/heads/main` and `refs/heads/master`; the step remains matrixed across both OS labels; `actionlint` passed. |
| T6-7 | Done | The workflow includes a weekly schedule and the full corpus condition includes `github.event_name == 'schedule'`. |
| T6-8 | Done | `git diff --name-status` showed only `.github/workflows/ci.yml` and `tests/ipc/corpus-selections/README.md`. No Rust source, test, tool, or corpus-selection data changed. |
| T6-9 | Done | Fast corpus passed with 43 selected/43 completed/0 failures/0 JSON assertion failures. Full corpus passed with 900 selected/900 completed/0 failures/0 JSON assertion failures. |
| T6-10 | Done | Passed: `cargo fmt --check`; `cargo check --locked --all-targets`; `RUSTFLAGS="-D rust-2024-compatibility" cargo check --locked --all-targets`; `cargo clippy --locked --all-targets -- -D warnings`; `cargo test --locked --all-targets`; both corpus commands; `cargo build --release --locked --bins`; `./target/release/hddl_analyzer --help`; `actionlint`; `git diff --check`; `git -C ../planning diff --check`. |
| T6-11 | Done | This report records every T6 row and bubbles the Arc05 closure/PR-readiness disposition below. |

## Verification Transcript Summary

```text
git status --short --branch --untracked-files=all --ignored=matching
## ci/corpus-policy
 M .github/workflows/ci.yml
 M tests/ipc/corpus-selections/README.md
!! target/
!! workbench/

git log --oneline --decorate -5
af0968b (HEAD -> ci/corpus-policy, origin/test/corpus-test-routing, test/corpus-test-routing) test: route corpus tests through fast selection
8e36a6a tools: add corpus selection policy
c6e4907 tools: add corpus measurement utility
e534df2 fix: repair fast ignored tests
7e2d8a7 test: consolidate helper and private naming cohesion

rg -n "#\\[ignore" src tests -g '*.rs'
# no matches

actionlint .github/workflows/ci.yml
# passed

cargo test --locked --all-targets
# passed all targets, including IPC 1 passed/0 ignored and JSON 9 passed/0 ignored

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
- `git diff --check`
- `git -C ../planning diff --check`

## Bubble-Up To Arc05

Slice06 wired the corpus CI policy without changing corpus or test semantics.
The only feature changes are workflow policy and documentation. The same
test-local fast selection and explicit full corpus command proven in Slice05
remain the semantic surface.

Arc05 should not open another feature implementation slice on the evidence from
this close. After CDC verifies Slice06, Arc05 should move to an arc-level
composition/PR-readiness close that checks the six implemented slices together:
ignored-test investigation, fast ignored-test repair, corpus measurement,
corpus addressability/policy, corpus test routing, and CI corpus policy.

Upstream PR caveats to carry forward:

- The corpus-testing PR is intentionally broader than a small unit-test patch:
  it changes CI runtime policy by adding explicit fast corpus measurement to
  every workflow run and full corpus measurement to PR, default-branch, and
  scheduled runs.
- Full corpus runs on both Linux and macOS and locally takes several minutes,
  with the largest measured tail concentrated in Minecraft corpus cases.
- The workflow keeps both `main` and `master` because this fork has used
  `main` while upstream uses `master`.
- The PR should be described as CI/test-policy work only; it does not change
  parser, verifier, transpiler, JSON semantics, corpus selection contents, or
  release/publishing behavior.

## CDC Verification

CDC verification landed on 2026-08-26 in `cdc-verification.md`. CDC reproduced
the workflow policy checks, no-ignore check, actionlint, locked Rust gates,
fast corpus command, full 900-case corpus command, release build, binary smoke
test, and diff hygiene against feature commit `4f41000`.
