# Slice06: LSP Error Boundaries And Metadata Ledger

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| C6-1 | The slice diff is limited to LSP error-boundary and metadata repair plus directly updated LSP tests, on top of the verified Slice05 base. | `git diff --name-only 6bd1b0a..HEAD` unless Slice05 has merged upstream, then inspect changed files. | serious | RUST-005/RUST-008 | open | | Slice05 settled CLI and structured parser/transform error behavior; do not count those files as Slice06 scope. |
| C6-2 | `initialize` reports `CARGO_PKG_VERSION` instead of hard-coded `0.1.0`. | Updated `tests/lsp_current_behavior.rs` initialize test and `rg -n "CARGO_PKG_VERSION|0\\.1\\.0|ServerInfo" src tests`. | correctness | RUST-008 | open | | |
| C6-3 | Unsynced diagnostics keep returning a structured JSON-RPC error without server panic. | `rg -n "unsynced|JSON-RPC|32602|invalid" tests/lsp_current_behavior.rs` and `cargo test --test lsp_current_behavior`. | correctness | RUST-005 | open | | |
| C6-4 | Non-file or malformed diagnostic URIs are handled without `unwrap` panic. | New or updated LSP harness test plus source inspection for URI conversion. | serious | RUST-005 | open | | |
| C6-5 | `didSave` missing or unreadable file behavior is handled without server panic or is explicitly deferred with a re-entry condition. | LSP harness test where deterministic; otherwise closing-report deferral with source rationale. | serious | RUST-005 | open | | |
| C6-6 | Missing or unreadable sibling-domain discovery is handled without server panic or is explicitly deferred with a re-entry condition. | LSP harness test where deterministic; otherwise closing-report deferral with source rationale. | serious | RUST-005 | open | | |
| C6-7 | No-domain-found diagnostics are intentionally preserved or intentionally changed and tested. | Updated `tests/lsp_current_behavior.rs` no-domain test. | correctness | RUST-005 | open | | |
| C6-8 | RUST-004 lock-scope repair is not silently mixed into this slice unless explicitly promoted and recorded. | `git diff 6bd1b0a..HEAD -- src/language_server/request_handler.rs` and inspect lock scope. | correctness | slice03-fix-map | open | | Prefer leaving RUST-004 to Slice07. |
| C6-9 | The full local workflow-equivalent gate passes. | `cargo fmt --check`; `cargo check --all-targets`; `RUSTFLAGS="-D rust-2024-compatibility" cargo check --all-targets`; `cargo clippy --all-targets -- -D warnings`; `cargo test --all-targets`; `cargo build --release --bins`; `./target/release/hddl_analyzer --help`; `actionlint .github/workflows/ci.yml`; `git diff --check` | serious | arc-plan | open | | |
| C6-10 | Slice06 closing report walks every row and states all LSP edge-case deferrals, if any. | Inspect `closing-report.md` for C6-1 through C6-10 and `re-entry`. | correctness | ledger-discipline | open | | |

## What Worked

_(At slice close.)_

## Closure

_(At slice close.)_
