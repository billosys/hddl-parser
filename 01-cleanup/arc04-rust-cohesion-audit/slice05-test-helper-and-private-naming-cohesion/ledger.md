# Slice05: Test Helper And Private Naming Cohesion Ledger

Definition of done: Slice05 improves test/helper consistency and private naming
cohesion without changing public API or product behavior.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| T4-1 | Slice starts after Slice03 baselines and Slice04 parser repair are closed or explicitly deferred. | `git status --short --branch` and inspect prior slice close evidence. | correctness | project-management | open | | Expected branch: `fix/test-private-naming-cohesion`. |
| T4-2 | Test helper consolidation keeps or improves assertion precision. | `git diff -- tests` and inspect before/after assertions; `cargo test --locked --all-targets` | serious | COHESION-005 | open | | Avoid a helper that hides expected variant/line/message checks. |
| T4-3 | Existing fixture coverage and ignored-test status are preserved unless explicitly justified. | `cargo test --locked --all-targets` and inspect ignored-test output. | correctness | test-cohesion | open | | Do not silently unignore long-running or known-fix tests. |
| T4-4 | Private implementation spelling drift is repaired where scoped. | `rg -n "qunatifier|elimintation|parantheses|satelite" src tests -g '*.rs'` | correctness | COHESION-006 | open | | Public variants may remain due compatibility. |
| T4-5 | Public enum variants are not renamed without operator GO. | `git diff -- src/output/errors/generic.rs src/transpiler/transformations/transform.rs` | serious | public-api | open | | `Lexiacal` and `QuantifierElimintation` are public API. |
| T4-6 | Full local gate passes. | `cargo fmt --check && cargo check --locked --all-targets && RUSTFLAGS="-D rust-2024-compatibility" cargo check --locked --all-targets && cargo clippy --locked --all-targets -- -D warnings && cargo test --locked --all-targets && cargo build --locked --release --bins && ./target/release/hddl_analyzer --help && actionlint .github/workflows/ci.yml && git diff --check` | serious | arc04 | open | | |
| T4-7 | Closing report walks every row and bubbles up remaining public API deferrals. | `test -f 01-cleanup/arc04-rust-cohesion-audit/slice05-test-helper-and-private-naming-cohesion/closing-report.md` and inspect row walk. | correctness | ledger-discipline | open | | |

## What Worked

Pending Slice05 close.

## Closure

Pending Slice05 close.
