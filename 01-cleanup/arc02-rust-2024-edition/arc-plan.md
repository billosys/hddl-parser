# Arc02: Rust 2024 Edition

Version: 0.1 placeholder
Date: 2026-08-25
Expected branch: `edition/rust-2024`

## Capability

Migrate HDDL-Parser from Rust 2021 to Rust 2024 with a focused upstream PR that
separates mechanical edition rewrites from semantic fixes.

## Deferred Breakdown

Detailed slice planning is intentionally deferred until Arc01 closes. Expected
work:

- Confirm the target toolchain and MSRV implication for Rust 2024.
- Run `cargo fix --edition` on a clean worktree and review the mechanical diff.
- Bump `edition = "2024"` only after the fix pass.
- Re-run the Arc01 CI command set locally.
- Record any semantic adjustments required by Rust 2024 behavior changes.

## Placeholder Ledger

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| A2-1 | Arc02 remains unplanned in detail until Arc01 CI closes. | `rg -n "deferred until Arc01 closes" 01-cleanup/arc02-rust-2024-edition/arc-plan.md` | correctness | project-plan | open | | |

## Version History

### v0.1 - 2026-08-25

Placeholder opened to preserve the project roadmap without planning too far
ahead of the CI baseline.
