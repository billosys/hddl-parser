# Slice03 Closing Report: Test Matrix

Date: 2026-08-25
Branch: `feature/add-ci`
Worktree: `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/features`

## Summary

Recorded Slice03 evidence against the current uncommitted workflow. The
workflow runs `cargo test --all-targets`, preserving the crate's named
integration test binaries for `ipc`, `flawed`, and `json`.

No Rust source, test source, or `Cargo.toml` changes were made for this slice.
The existing `cargo test --all-targets` step came from the prior uncommitted
workflow overlap and is now the Slice03 test gate.

## Row Walk

| ID | Result | Evidence |
|----|--------|----------|
| F-1 | Closed. CI runs the test command. | `rg -n "cargo test" .github/workflows/ci.yml` found `cargo test --all-targets` at line 63. |
| F-2 | Closed. Named integration test binaries remain covered. | `rg -n "ipc|flawed|json|all-targets" .github/workflows/ci.yml Cargo.toml` found `ipc`, `flawed`, and `json` test targets in `Cargo.toml` and `cargo test --all-targets` in the workflow. Local test output included `tests/integration_flawed.rs`, `tests/integration_ipc.rs`, and `tests/integration_json.rs`. |
| F-3 | Closed. Local test execution passes. | `cargo test --all-targets` exited 0: lib tests 111 passed / 1 ignored; `flawed` 21 passed / 2 ignored; `ipc` 0 passed / 1 ignored; `json` 8 passed / 1 ignored; bin test targets had 0 tests. |
| F-4 | Closed. No tests were newly ignored to make CI pass. | `git diff -- tests src Cargo.toml | rg "#\\[ignore\\]|ignore ="` exited 1 with no output. |

## Caveat

The repository already has ignored tests before this slice, including the IPC
corpus test and one JSON IPC round-trip test marked as long-running. This slice
did not add, remove, or change ignore markers. The workflow uses the prompt's
primary command, `cargo test --all-targets`, so those pre-existing ignored cases
remain governed by the current repository test policy.

## Bubble-Up To Arc

Arc01 row A-3 is ready for independent verification from the CI test-entry
side. A future policy decision may still be needed if maintainers want the
pre-existing ignored IPC corpus tests to run in CI through a split schedule,
separate job, or opt-in slow-test path.
