# Slice04: Clippy Remediation And Binary Smoke

Date: 2026-08-25
Branch: `feature/add-ci`
Arc: `arc01-github-actions-ci`

## Goal

Make the strict CI quality gate genuinely mergeable by fixing every issue
reported by `cargo clippy --all-targets -- -D warnings`, then add CI evidence
that the shipped binaries build and that the command-line tool can be invoked.
Add minimal project status polish if it remains review-friendly.

## In Scope

- Mechanical Clippy fixes required for
  `cargo clippy --all-targets -- -D warnings` to exit 0
- `cargo build --release --bins`
- `hddl_analyzer --help` smoke check
- Optional README CI badge if it points to the new workflow and does not
  distract from the CI PR

## Out Of Scope

- Language-server integration testing
- Packaging, release uploads, installers, or binary artifacts
- CLI behavior fixes beyond mechanical Clippy remediation
- `#[allow(...)]` or workflow weakening to hide warnings
- Semantic parser, analyzer, JSON, or LSP behavior changes unless required to
  preserve existing behavior after a mechanical cleanup

## Verification Approach

Run strict Clippy locally and confirm it exits 0 without weakening the workflow.
Then run the test suite, release build, and release `hddl_analyzer --help`.
Inspect any README badge/link for the upstream repository path.

## Exit Criteria

- `cargo clippy --all-targets -- -D warnings` passes locally.
- Clippy cleanup is mechanical and does not use `#[allow(...)]` as a substitute
  for fixing warnings.
- `cargo test --all-targets` still passes after the cleanup.
- CI builds both binaries in release mode.
- CI invokes `hddl_analyzer --help`.
- Any README status change is minimal and accurate.
