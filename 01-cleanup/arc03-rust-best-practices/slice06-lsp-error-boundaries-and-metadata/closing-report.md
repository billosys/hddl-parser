# Arc03 Slice06 Closing Report: LSP Error Boundaries And Metadata

Date: 2026-08-26
Feature branch: `fix/lsp-error-boundaries-and-metadata`
Base commit: `6bd1b0a Fix structured parser transform errors`
Feature commit: `fbb27d7 Fix LSP error boundaries and metadata`

## Summary

Slice06 fixes RUST-005 and RUST-008 by replacing reachable LSP request/runtime
unwraps with explicit error handling and by reporting the package version in
the LSP `initialize` response.

The verified feature commit diff contains:

- `src/language_server/request_handler.rs`
- `tests/lsp_current_behavior.rs`

No CLI, parser, transform, Cargo policy, dependency, edition, or public API
cohesion work is mixed into this slice.

## Final LSP Behavior

- Initialize version: `serverInfo.version` is `env!("CARGO_PKG_VERSION")`.
- Unsynced diagnostics: `textDocument/diagnostic` returns JSON-RPC
  `invalid_params` (`-32602`) with `is not synced`; the server stays alive.
- Non-file diagnostic URI: synced non-file diagnostic requests return JSON-RPC
  `invalid_params` (`-32602`) with `diagnostic URI must be a file URI`.
- `didSave`: inline text still syncs normally. Without inline text, non-file
  URIs and filesystem read failures are logged as LSP error messages and the
  notification returns without syncing or panicking.
- Sibling-domain discovery: missing parent directories, directory iteration
  errors, and unreadable sibling candidates are logged and handled without
  panic. Missing domain discovery falls back to problem-only diagnostics.
- No-domain-found diagnostics: preserved as a full diagnostic report with zero
  items.

RUST-004 diagnostic lock-scope behavior is intentionally unchanged. The
diagnostic path still borrows the synced document from
`self.documents.read().await.get(...)` through awaited work; Slice07 owns the
lock-scope repair.

## Verification

All required commands passed:

- `cargo fmt --check`
- `cargo check --all-targets`
- `RUSTFLAGS="-D rust-2024-compatibility" cargo check --all-targets`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test --test lsp_current_behavior`
- `cargo test --all-targets`
- `cargo build --release --bins`
- `./target/release/hddl_analyzer --help`
- `actionlint .github/workflows/ci.yml`
- `git diff --check`

`cargo test --test lsp_current_behavior` passed with 6 tests, 0 failures, and
0 ignored tests. `cargo test --all-targets` passed with the existing ignored
legacy tests unchanged.

`git diff --name-only 6bd1b0a..HEAD` outputs:

```text
src/language_server/request_handler.rs
tests/lsp_current_behavior.rs
```

## LSP Harness Limits

No Slice06 behavior is deferred. One edge is source-inspected rather than
directly harness-tested: unreadable sibling-domain candidates use the same
`tokio::fs::read` error branch that now logs and continues, but a portable
permission-denied fixture is not deterministic across all runner contexts. If
CDC wants direct runtime evidence for that subcase, the re-entry condition is
a Unix-gated permission fixture or a narrow filesystem test hook in a future
LSP test slice.

## Ledger Walk

- C6-1: Done. The feature commit diff is limited to
  `src/language_server/request_handler.rs` and
  `tests/lsp_current_behavior.rs` on top of `6bd1b0a`.
- C6-2: Done. `lsp_initialize_reports_package_version` passed, and source
  grep shows `CARGO_PKG_VERSION` in the handler and test with no remaining
  `0.1.0` match.
- C6-3: Done. `lsp_diagnostic_unsynced_document_returns_jsonrpc_error` passed
  and asserts `-32602` plus `is not synced`.
- C6-4: Done. `lsp_diagnostic_non_file_uri_returns_jsonrpc_error_without_panic`
  passed and asserts `-32602` plus `diagnostic URI must be a file URI`.
- C6-5: Done. `lsp_did_save_missing_file_keeps_server_alive_and_document_unsynced`
  passed; source inspection shows non-file and read-error `didSave` cases log
  and return without panic.
- C6-6: Done. `lsp_diagnostic_missing_sibling_directory_returns_empty_report`
  passed; source inspection shows sibling `read_dir`, `next_entry`, and file
  read errors no longer unwrap.
- C6-7: Done. `lsp_diagnostic_problem_without_domain_returns_empty_report`
  passed, preserving the zero-item full report behavior.
- C6-8: Done. RUST-004 lock-scope repair is not mixed in; the document borrow
  from the read guard still spans awaited diagnostic work.
- C6-9: Done. Full local workflow-equivalent gate passed.
- C6-10: Done. This report records C6-1 through C6-10, final LSP behavior,
  and the one source-inspected harness limitation with re-entry condition.

## Bubble-Up

Slice06 delivers its assigned Arc03 repair for RUST-005 and RUST-008. No
arc-plan re-slicing is required. Slice07 should still address RUST-004 by
dropping the document-map read guard before awaited diagnostic work.
