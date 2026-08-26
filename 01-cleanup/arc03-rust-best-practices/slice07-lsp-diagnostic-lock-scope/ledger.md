# Slice07: LSP Diagnostic Lock Scope Ledger

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| C7-1 | The slice diff is limited to diagnostic lock-scope repair and directly related tests/evidence, on top of the verified Slice06 base. | `git diff --name-only fbb27d7..HEAD` unless Slice06 has merged upstream, then inspect changed files. | serious | RUST-004 | open | | Slice06 settled LSP error-boundary and metadata behavior; do not count those changes as Slice07 scope. |
| C7-2 | The diagnostic path owns or clones document bytes before any awaited logging/filesystem work. | Inspect `src/language_server/request_handler.rs` around `documents.read().await` and later `.await` sites. | serious | RUST-004 | open | | |
| C7-3 | No borrowed value from the document-map read guard crosses an `.await` in `diagnostic`. | `rg -n "documents\\.read\\(\\)\\.await|get\\(|\\.await" src/language_server/request_handler.rs` plus manual scope inspection. | serious | RUST-004 | open | | |
| C7-4 | Existing LSP behavior tests remain green after the lock-scope change. | `cargo test --test lsp_current_behavior` | correctness | slice02-baseline | open | | |
| C7-5 | A deterministic contention regression test is added or explicitly deferred with re-entry. | Inspect tests and closing report for `RwLock`, `contention`, `re-entry`, or source-level proof. | correctness | slice02-cdc | open | | Avoid timing-only tests. |
| C7-6 | RUST-005 error-boundary behavior is not silently changed in this slice unless explicitly inherited from Slice06. | `git diff fbb27d7..HEAD -- src/language_server/request_handler.rs` and inspect non-lock error paths. | correctness | slice03-fix-map | open | | |
| C7-7 | The full local workflow-equivalent gate passes. | `cargo fmt --check`; `cargo check --all-targets`; `RUSTFLAGS="-D rust-2024-compatibility" cargo check --all-targets`; `cargo clippy --all-targets -- -D warnings`; `cargo test --all-targets`; `cargo build --release --bins`; `./target/release/hddl_analyzer --help`; `actionlint .github/workflows/ci.yml`; `git diff --check` | serious | arc-plan | open | | |
| C7-8 | Slice07 closing report walks every row and states the final lock-scope proof. | Inspect `closing-report.md` for C7-1 through C7-8. | correctness | ledger-discipline | open | | |

## What Worked

_(At slice close.)_

## Closure

_(At slice close.)_
