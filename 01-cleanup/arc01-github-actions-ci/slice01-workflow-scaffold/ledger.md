# Slice01: Workflow Scaffold Ledger

## Ledger

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| F-1 | `.github/workflows/ci.yml` exists in the implementation worktree. | `test -f .github/workflows/ci.yml` | serious | slice-doc | closed | `test -f .github/workflows/ci.yml` exited 0. | Run from `.worktrees/features`. |
| F-2 | CI runs for pull requests and pushes to `main`. | `rg -n "pull_request|push|main" .github/workflows/ci.yml` | correctness | slice-doc | closed | `rg` found `pull_request` at line 4, `push` at line 5, and `main` at line 7. | |
| F-3 | CI uses a Rust stable toolchain setup with rustfmt and clippy components available. | `rg -n "stable|rustfmt|clippy" .github/workflows/ci.yml` | correctness | slice-doc | closed | `rg` found stable Rust installation with `rustfmt` and `clippy` at line 41, `rustup default stable` at line 42, and `cargo clippy --version` at line 49. | |
| F-4 | CI covers Linux and macOS or records a concrete deferral reason. | `rg -n "ubuntu|macos" .github/workflows/ci.yml` | correctness | arc-plan | closed | `rg` found `ubuntu-latest` at line 21 and `macos-latest` at line 22. | No runner deferral needed. |
| F-5 | Workflow syntax is checked locally when tooling is available. | `actionlint .github/workflows/ci.yml` | polish | slice-doc | closed | `actionlint .github/workflows/ci.yml` exited 0. | `actionlint` was available at `/opt/homebrew/bin/actionlint`. |

## What Worked

The workflow scaffold stayed narrow: it adds triggers, concurrency, checkout,
Cargo cache setup, the Linux/macOS runner matrix, stable Rust installation with
`rustfmt` and `clippy`, and a toolchain version sanity step. Later slices can
append format, lint, test, and smoke gates without restructuring the job.

## Closure

Slice01 is implementation-complete from the CC side. Local static verification
passed, including `actionlint`; no code, edition, release, or audit changes were
made.
