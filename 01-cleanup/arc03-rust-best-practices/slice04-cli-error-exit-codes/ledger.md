# Slice04: CLI Error Exit Codes Ledger

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| C4-1 | The slice diff is limited to CLI exit-code repair and the directly updated CLI baselines. | `git diff --name-only main..HEAD` and inspect changed files. | serious | RUST-001 | open | | Expected files are `src/bin/hddl_analyzer/main.rs` and `tests/current_behavior.rs` unless implementation proves a helper is necessary. |
| C4-2 | `hddl_analyzer` returns non-zero for missing input while preserving stderr error output. | `rg -n "missing_input|non.*zero|stderr" tests/current_behavior.rs` and `cargo test --test current_behavior`. | serious | RUST-001 | open | | Rename the test so it no longer describes current bad behavior. |
| C4-3 | `hddl_analyzer` returns non-zero for unsupported input extension while preserving stderr error output. | `rg -n "unsupported.*extension|non.*zero|stderr" tests/current_behavior.rs` and `cargo test --test current_behavior`. | serious | RUST-001 | open | | |
| C4-4 | `hddl_analyzer verify` returns non-zero for parse/semantic failure while preserving diagnostics on stderr. | `rg -n "semantic_failure|non.*zero|stderr" tests/current_behavior.rs` and `cargo test --test current_behavior`. | serious | RUST-001 | open | | |
| C4-5 | `hddl_analyzer convert` returns non-zero for output write failure while preserving stderr error output. | `rg -n "output_write_failure|non.*zero|stderr" tests/current_behavior.rs` and `cargo test --test current_behavior`. | serious | RUST-001 | open | | |
| C4-6 | Successful verification still exits `0` and writes success to stdout. | `rg -n "known_good|success|stdout" tests/current_behavior.rs` and `cargo test --test current_behavior`. | serious | RUST-001 | open | | |
| C4-7 | RUST-002 and RUST-003 panic baselines are not silently changed in this slice. | Run the two panic runtime probes from `workbench/2026.08.25-audit-results-rust.md` and record whether they still return `101`. | correctness | slice03-fix-map | open | | If the CLI wrapper changes their process exit without fixing the library panic, record the exact observed behavior and bubble it to Slice05. |
| C4-8 | The full local workflow-equivalent gate passes. | `cargo fmt --check`; `cargo check --all-targets`; `RUSTFLAGS="-D rust-2024-compatibility" cargo check --all-targets`; `cargo clippy --all-targets -- -D warnings`; `cargo test --all-targets`; `cargo build --release --bins`; `./target/release/hddl_analyzer --help`; `actionlint .github/workflows/ci.yml`; `git diff --check` | serious | arc-plan | open | | |
| C4-9 | Slice04 closing report walks every row and states the final RUST-001 process contract. | Inspect `closing-report.md` for C4-1 through C4-9. | correctness | ledger-discipline | open | | |

## What Worked

_(At slice close.)_

## Closure

_(At slice close.)_
