# Slice03 CDC Verification: Test Matrix

Date: 2026-08-25
Verifier: CDC / Sofie
Implementation worktree: `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/features`
Planning worktree: `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning`
Implementation commit observed: `9474df5` (`ci: add Rust quality and test gates`)

## Verdict

Verified. Slice03 satisfies its test-matrix ledger. The workflow contains the
test gate, the declared integration-test targets remain covered by
`cargo test --all-targets`, local test execution passes, and no new ignored-test
markers were introduced in the implementation diff.

Process note: the Slice03 ledger used `closed` as the row status. CDC normalized
those statuses to `done`, matching the ledger-discipline final-status
vocabulary. Evidence was not weakened by that terminology fix.

## Row Count

Opening ledger rows: 4
Closing report rows: 4
CDC verification rows: 4

No silent drops found.

## Row Walk

| ID | CDC result | Evidence |
|----|------------|----------|
| F-1 | done / reproduced | `rg -n "cargo test" .github/workflows/ci.yml` found `cargo test --all-targets` at line 63. |
| F-2 | done / reproduced | `rg -n "ipc|flawed|json|all-targets" .github/workflows/ci.yml Cargo.toml` found the `ipc`, `flawed`, and `json` integration-test targets in `Cargo.toml`, plus the workflow's `cargo test --all-targets` gate. |
| F-3 | done / reproduced | `cargo test --all-targets` exited 0. Results: lib tests 111 passed / 1 ignored; `flawed` 21 passed / 2 ignored; `ipc` 0 passed / 1 ignored; `json` 8 passed / 1 ignored; binary test targets had 0 tests. |
| F-4 | done / reproduced | `git diff -- tests src Cargo.toml | rg "#\\[ignore\\]|ignore ="` exited 1 with no output, so no ignored-test marker or ignore setting was added in the implementation diff. |

## Caveat

The repository already has ignored tests: one lib test, two `flawed`
integration tests, the long-running `ipc` integration test, and the
long-running JSON IPC round-trip test. Slice03 did not add, remove, or change
those markers. The current CI test command preserves the repository's existing
default test policy.

## Bubble-Up Check

Arc01 row A-3 may be treated as verified. No arc-plan change is required.

The later audit arc may revisit whether the pre-existing ignored IPC/JSON
corpus tests should have an explicit slow-test CI path.
