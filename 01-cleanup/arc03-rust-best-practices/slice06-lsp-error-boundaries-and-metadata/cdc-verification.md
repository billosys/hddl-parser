# CDC Verification: Arc03 Slice06 LSP Error Boundaries And Metadata

Date: 2026-08-26
Verifier: CDC
Feature branch: `fix/lsp-error-boundaries-and-metadata`
Feature base: `6bd1b0a Fix structured parser transform errors`
Feature commit verified: `fbb27d7 Fix LSP error boundaries and metadata`
Planning branch: `planning`

## Verdict

Verified. Slice06 fixes RUST-005 and RUST-008 for the scoped LSP
request-boundary failures: reachable request-handler unwrap paths are replaced
with explicit error handling, and `initialize` reports the package version.

All C6-1 through C6-10 rows are verified. No CLI, parser, transform, Cargo
policy, dependency, edition, public API cohesion, or RUST-004 lock-scope repair
is mixed into the feature diff.

## Artifact Boundary

`git diff --name-only 6bd1b0a..HEAD`:

```text
src/language_server/request_handler.rs
tests/lsp_current_behavior.rs
```

CDC inspected the two-file diff. `request_handler.rs` now handles non-file
`didSave` URIs, saved-file read failures, diagnostic URI conversion failures,
sibling-directory read failures, directory iteration failures, and unreadable
sibling-domain candidates without `unwrap` panics. `initialize` now uses
`env!("CARGO_PKG_VERSION")`. `tests/lsp_current_behavior.rs` expands the stdio
LSP harness from three tests to six tests.

## Behavior Evidence

The focused LSP test run passed with 6 tests, 0 failures, and 0 ignored tests:

- `lsp_initialize_reports_package_version`
- `lsp_diagnostic_unsynced_document_returns_jsonrpc_error`
- `lsp_diagnostic_non_file_uri_returns_jsonrpc_error_without_panic`
- `lsp_did_save_missing_file_keeps_server_alive_and_document_unsynced`
- `lsp_diagnostic_missing_sibling_directory_returns_empty_report`
- `lsp_diagnostic_problem_without_domain_returns_empty_report`

Grep evidence:

- `rg -n "CARGO_PKG_VERSION|0\\.1\\.0|ServerInfo" src tests/lsp_current_behavior.rs`
  finds `CARGO_PKG_VERSION` in the handler and test, with no `0.1.0` match.
- `rg -n "unsynced|JSON-RPC|32602|invalid|non_file|file URI|did_save|missing_sibling|without_domain|CARGO_PKG_VERSION" tests/lsp_current_behavior.rs`
  finds the expected coverage points.
- `rg -n "unwrap\\(|expect\\(|to_file_path\\(\\)\\.unwrap|read_dir\\(.*\\.await\\.unwrap|next_entry\\(\\).*unwrap|tokio::fs::read\\(.*\\.await\\.unwrap" src/language_server tests/lsp_current_behavior.rs`
  shows no request-handler unwraps for the Slice06 failure paths. Remaining
  matches are test harness assertions or pre-existing `diagnostic_utils.rs`
  sites outside the Slice06 diff.

## Quality Gate

CDC reproduced the Slice06 gate:

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
- `git diff --check 6bd1b0a..HEAD`
- `git -C ../planning diff --check`

All commands exited `0`. `cargo test --all-targets` passed with the existing
ignored legacy tests unchanged.

## Ledger Walk

- C6-1: Verified. Feature commit `fbb27d7` changes only
  `src/language_server/request_handler.rs` and `tests/lsp_current_behavior.rs`
  relative to `6bd1b0a`.
- C6-2: Verified. `initialize` reports `env!("CARGO_PKG_VERSION")` and the
  stale `0.1.0` literal is gone from the LSP metadata path.
- C6-3: Verified. Unsynced diagnostics still return JSON-RPC `-32602` without
  server panic.
- C6-4: Verified. Non-file diagnostic URIs return JSON-RPC `-32602` with
  `diagnostic URI must be a file URI`.
- C6-5: Verified. Missing-file `didSave` leaves the server alive and the
  document unsynced; non-file and read-error branches log and return.
- C6-6: Verified. Missing sibling directories are harness-tested; unreadable
  sibling candidates are source-inspected through the same `tokio::fs::read`
  error branch, with a portable permission fixture re-entry condition recorded
  in the closing report.
- C6-7: Verified. Problem diagnostics with no discoverable domain preserve the
  zero-item full report behavior.
- C6-8: Verified. RUST-004 lock-scope repair is not mixed in; `diagnostic`
  still borrows the document from `documents.read().await.get(...)` through
  awaited work, leaving the lock-scope repair for Slice07.
- C6-9: Verified. Full local workflow-equivalent gate passed.
- C6-10: Verified. Closing report walks every row and states the final LSP
  behavior plus harness limits.

## Bubble-Up

No arc-plan re-slicing is required. Slice06 delivers RUST-005 and RUST-008.
Slice07 remains correctly scoped to RUST-004: drop the diagnostic document-map
read guard before awaited diagnostic work.
