# Arc03: Rust Best-Practices Audit And Fixes

Version: 0.1 placeholder
Date: 2026-08-25
Expected branch: `audit/rust-quality-fixes` or smaller focused branches

## Capability

Run a full evidence-based Rust audit using the collaboration framework and
Rust-guidelines substrate, then contribute focused fixes upstream.

## Deferred Breakdown

Detailed slice planning is intentionally deferred until Arc01 closes and Arc02
settles the edition target. Expected audit areas:

- Parser correctness and diagnostics.
- JSON round-trip invariants and test coverage.
- CLI behavior, exit status, stdout/stderr separation, and pipe friendliness.
- Error handling and public API design.
- Dependency, feature, and workspace hygiene.
- Tests that protect the behavior most important to downstream Lykn, Chengdu,
  Wolong, and CCD protocol use cases.

If the audit finds unrelated issues, this arc should split fixes into multiple
upstream PRs rather than forcing one broad cleanup diff.

## Placeholder Ledger

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| A3-1 | Arc03 remains audit-only until Arc01 and Arc02 provide the quality baseline. | `rg -n "deferred until Arc01 closes and Arc02 settles" 01-cleanup/arc03-rust-best-practices/arc-plan.md` | correctness | project-plan | open | | |

## Version History

### v0.1 - 2026-08-25

Placeholder opened to preserve the expected third PR family while keeping the
audit/fix breakdown out of scope until CI and edition work settle.
