# Slice04: Binary Smoke And Status Ledger

## Ledger

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| F-1 | CI runs `cargo build --release --bins`. | `rg -n "cargo build --release --bins" .github/workflows/ci.yml` | correctness | slice-doc | open | | |
| F-2 | CI invokes `hddl_analyzer --help`. | `rg -n "hddl_analyzer(.exe)? --help|target/release/hddl_analyzer" .github/workflows/ci.yml` | correctness | slice-doc | open | | Windows is out of scope unless added deliberately. |
| F-3 | Local release binary smoke passes. | `cargo build --release --bins && ./target/release/hddl_analyzer --help` | serious | arc-plan | open | | Run from `.worktrees/features`. |
| F-4 | README badge, if added, points to the upstream workflow path. | `rg -n "actions/workflows|badge.svg" README.md .github/workflows/ci.yml` | polish | slice-doc | open | | If no badge is added, close as no-op with rationale. |

## What Worked

_(At slice close.)_

## Closure

_(At slice close.)_
