# Slice03: Test Matrix Ledger

## Ledger

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| F-1 | CI runs `cargo test --all-targets` or an explicitly equivalent command set. | `rg -n "cargo test" .github/workflows/ci.yml` | serious | slice-doc | closed | `rg` found `cargo test --all-targets` at `.github/workflows/ci.yml:63`. | |
| F-2 | The existing named integration tests remain covered. | `rg -n "ipc|flawed|json|all-targets" .github/workflows/ci.yml Cargo.toml` | correctness | slice-doc | closed | `rg` found named test targets `ipc`, `flawed`, and `json` in `Cargo.toml`, plus `cargo test --all-targets` in `.github/workflows/ci.yml:63`; local `cargo test --all-targets` ran the `flawed`, `ipc`, and `json` integration test binaries. | Pre-existing ignored tests remain ignored; no coverage was removed in this slice. |
| F-3 | Local test execution passes on the implementation worktree. | `cargo test --all-targets` | serious | arc-plan | closed | `cargo test --all-targets` exited 0: lib tests 111 passed / 1 ignored; `flawed` 21 passed / 2 ignored; `ipc` 0 passed / 1 ignored; `json` 8 passed / 1 ignored; bin test targets had 0 tests. | Run from `.worktrees/features`. |
| F-4 | No test is newly ignored to make CI pass. | `git diff -- tests src Cargo.toml | rg "#\\[ignore\\]|ignore ="` | correctness | slice-doc | closed | `git diff -- tests src Cargo.toml | rg "#\\[ignore\\]|ignore ="` exited 1 with no output, indicating no new ignore markers or ignore settings in the implementation diff. | This verify should return no matches. |

## What Worked

The existing `cargo test --all-targets` workflow step cleanly covers Slice03's
CI test-entry requirement, and the local test run passed without source or test
edits. The command includes the named integration test binaries declared in
`Cargo.toml`.

## Closure

Slice03 is implementation-complete from the CC side. The current workflow still
contains setup and quality-gate changes from earlier uncommitted work; this
slice records only the test-matrix evidence and does not alter Rust source,
ignore attributes, or test corpus files.
