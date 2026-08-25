# Slice05: Structured Parser And Transform Errors Ledger

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| C5-1 | The slice diff is limited to parser/transpiler/transform error handling and directly updated baselines. | `git diff --name-only main..HEAD` and inspect changed files. | serious | RUST-002/RUST-003 | open | | |
| C5-2 | `HDDLProgram::from_hddl` returns `Err` for problem-as-domain input instead of panicking. | Updated `tests/current_behavior.rs` test plus focused `cargo test --test current_behavior`. | serious | RUST-002 | open | | |
| C5-3 | `HDDLProgram::from_hddl` returns `Err` for domain-as-problem input instead of panicking. | Updated `tests/current_behavior.rs` test plus focused `cargo test --test current_behavior`. | serious | RUST-002 | open | | |
| C5-4 | `Transpiler::from_hddl` returns `Err` for both domain/problem mismatch directions instead of panicking. | Updated `tests/current_behavior.rs` tests plus focused `cargo test --test current_behavior`. | serious | RUST-002 | open | | |
| C5-5 | Domain-only `RemoveEqualityConstraints` returns `Err` before mutation and does not panic. | Updated `tests/current_behavior.rs` transform test plus focused `cargo test --test current_behavior`. | serious | RUST-003 | open | | |
| C5-6 | Public error messages are specific enough to distinguish unexpected domain/problem variants and missing problem input. | Inspect updated `ParsingError` or equivalent error construction and test assertions. | correctness | rust-guidelines | open | | Avoid string-only ambiguity if a structured variant is practical. |
| C5-7 | The two Slice01 panic runtime probes no longer exit with panic code `101`. | Rerun the problem-as-domain and domain-only remove-equality runtime probes from the audit report. | serious | slice01-audit | open | | Expected final process exit depends on Slice04, but it must not be `101`. |
| C5-8 | The full local workflow-equivalent gate passes. | `cargo fmt --check`; `cargo check --all-targets`; `RUSTFLAGS="-D rust-2024-compatibility" cargo check --all-targets`; `cargo clippy --all-targets -- -D warnings`; `cargo test --all-targets`; `cargo build --release --bins`; `./target/release/hddl_analyzer --help`; `actionlint .github/workflows/ci.yml`; `git diff --check` | serious | arc-plan | open | | |
| C5-9 | Slice05 closing report walks every row and states the final RUST-002/RUST-003 error contracts. | Inspect `closing-report.md` for C5-1 through C5-9. | correctness | ledger-discipline | open | | |

## What Worked

_(At slice close.)_

## Closure

_(At slice close.)_
