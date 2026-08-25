# Slice03: Test Matrix

Date: 2026-08-25
Branch: `feature/add-ci`
Arc: `arc01-github-actions-ci`

## Goal

Add test execution to CI in a way that protects the parser, flawed-domain
checks, IPC corpus checks, and JSON round-trip behavior already present in the
crate.

## In Scope

- `cargo test --all-targets`
- Explicit attention to named integration tests: `ipc`, `flawed`, and `json`
- Runner-matrix behavior that avoids accidentally dropping macOS or Linux
  coverage

## Out Of Scope

- Adding new tests
- Rewriting long-running tests
- Marking tests ignored

## Verification Approach

Run the test command locally from the implementation worktree and inspect the CI
workflow to ensure the same coverage is present. If the IPC corpus is too slow
for every matrix cell, propose a documented split rather than silently removing
it.

## Exit Criteria

- CI runs the crate's tests.
- Named integration tests remain covered.
- Any runtime-related split is explicit in the workflow and the close report.
