# Arc01: GitHub Actions CI

Version: 1.0
Date: 2026-08-25
Implementation branch: `feature/add-ci`
Expected upstream PR: second cleanup PR after warning fixes

## Capability

Add the repository's first GitHub Actions CI workflow so every PR gets automatic
evidence for Rust formatting, linting, builds, tests, and binary-level smoke.
The workflow should be small, conventional, and friendly to upstream review.

## Slice Breakdown

| Slice | Scope | Load-bearing for |
|-------|-------|------------------|
| slice01-workflow-scaffold | Add the workflow file, trigger policy, checkout/toolchain setup, and cache strategy. | All later CI gates run inside this workflow. |
| slice02-rust-quality-gates | Add `cargo fmt --check`, `cargo check --all-targets`, and `cargo clippy --all-targets -- -D warnings`. | Blocks edition migration and audit fixes from drifting below Rust quality floor. |
| slice03-test-matrix | Add test coverage for the crate's named integration tests and normal test entrypoint. | Establishes regression signal for parser, semantic analyzer, and JSON round trips. |
| slice04-binary-smoke-and-status | Fix strict Clippy failures, then add release build and CLI help smoke checks; optionally add a README badge if upstream style permits. | Makes the CI PR mergeably green and confirms the two shipped binaries keep compiling and the command-line entry remains invokable. |

## Dependencies

- Consumes the warning-fix baseline from PR #5 or an equivalent local branch.
- Leaves Arc02 with a CI baseline that should catch edition-migration regressions.
- Leaves Arc03 with test and lint gates that make audit fixes reviewable.

## Arc Ledger

Capability: GitHub Actions CI exists and gives maintainers reproducible signal
for the Rust crate.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| A-1 | slice01 workflow scaffold closes. | `test -f 01-cleanup/arc01-github-actions-ci/slice01-workflow-scaffold/cdc-verification.md` | correctness | arc-plan | done | `slice01-workflow-scaffold/cdc-verification.md` verifies all 5 slice rows with reproduced evidence. | attested by child close |
| A-2 | slice02 quality gates close. | `test -f 01-cleanup/arc01-github-actions-ci/slice02-rust-quality-gates/cdc-verification.md` | correctness | arc-plan | done | `slice02-rust-quality-gates/cdc-verification.md` is closed after Slice04 discharged strict Clippy; workflow rows are reproduced and strict Clippy now exits 0. | attested by child close |
| A-3 | slice03 test matrix closes. | `test -f 01-cleanup/arc01-github-actions-ci/slice03-test-matrix/cdc-verification.md` | correctness | arc-plan | done | `slice03-test-matrix/cdc-verification.md` verifies all 4 slice rows with reproduced evidence. | attested by child close |
| A-4 | slice04 Clippy remediation and binary smoke/status closes. | `test -f 01-cleanup/arc01-github-actions-ci/slice04-binary-smoke-and-status/cdc-verification.md` | correctness | arc-plan | done | `slice04-binary-smoke-and-status/cdc-verification.md` verifies all 8 rows with reproduced evidence or no-op rationale. | attested by child close |
| A-5 | CI workflow composes into one reviewable upstream PR. | `git -C ../.worktrees/features diff --stat main...feature/add-ci` plus inspect `.github/workflows/ci.yml` | serious | arc-plan | done | `feature/add-ci` is clean at `c7b4828`; diff against local `main` is one CI workflow plus mechanical Clippy cleanup needed by the strict gate. PR should target the PR #5 baseline or be opened after PR #5 merges. | reproduce at arc scale |
| A-6 | Local verification command set mirrors workflow behavior before PR handoff. | `cargo fmt --check && cargo check --all-targets && cargo clippy --all-targets -- -D warnings && cargo test --all-targets && cargo build --release --bins` from `../.worktrees/features` | serious | arc-plan | done | Full workflow-equivalent command exited 0 from `.worktrees/features`, including release `hddl_analyzer --help`; `actionlint` and `git diff --check` also passed in Slice04 CDC verification. | reproduce at arc scale |

## Version History

### v1.0 - 2026-08-25

Initial arc breakdown opened with all CI slices because the effort is tightly
scoped and the expected workflow surface is already visible from the crate.

### v1.1 - 2026-08-25

Expanded Slice04 after CDC reproduced the Slice02 blocker: strict
`cargo clippy --all-targets -- -D warnings` fails with existing warning debt.
Slice04 now owns the mechanical Clippy remediation required to make the CI PR
mergeably green, while preserving its original binary smoke/status work.
