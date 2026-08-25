# Slice03: Test Matrix Ledger

## Ledger

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| F-1 | CI runs `cargo test --all-targets` or an explicitly equivalent command set. | `rg -n "cargo test" .github/workflows/ci.yml` | serious | slice-doc | open | | |
| F-2 | The existing named integration tests remain covered. | `rg -n "ipc|flawed|json|all-targets" .github/workflows/ci.yml Cargo.toml` | correctness | slice-doc | open | | |
| F-3 | Local test execution passes on the implementation worktree. | `cargo test --all-targets` | serious | arc-plan | open | | Run from `.worktrees/features`. |
| F-4 | No test is newly ignored to make CI pass. | `git diff -- tests src Cargo.toml | rg "#\\[ignore\\]|ignore ="` | correctness | slice-doc | open | | This verify should return no matches. |

## What Worked

_(At slice close.)_

## Closure

_(At slice close.)_
