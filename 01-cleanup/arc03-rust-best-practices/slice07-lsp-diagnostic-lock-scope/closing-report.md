# Arc03 Slice07 Closing Report: LSP Diagnostic Lock Scope

Date: 2026-08-26
Feature branch: `fix/lsp-diagnostic-lock-scope`
Base commit: `fbb27d7 Fix LSP error boundaries and metadata`
Feature commit: `e14078c Fix LSP diagnostic lock scope`

## Summary

Slice07 fixes RUST-004 by narrowing the diagnostic document-map read lock. The handler now clones the synced document bytes inside a short block and performs awaited logging, URI/path handling, sibling-domain filesystem work, and diagnostic generation after the read guard has dropped.

Changed feature file:

- `src/language_server/request_handler.rs`

No CLI, parser, transform, Cargo, public API, metadata, or Slice06 error-boundary behavior changes were mixed into this slice.

## Lock-Scope Proof

Proof type: source inspection plus existing behavior regression tests. No runtime contention test was added.

Relevant source shape:

```rust
let document = {
    self.documents
        .read()
        .await
        .get(&params.text_document.uri)
        .cloned()
};

match document {
    Some(document) => {
        self.client
            .log_message(MessageType::LOG, "Diagnostic Request Recieved.")
            .await;
```

The map lookup borrow is immediately cloned into an owned `Vec<u8>` while the read guard is confined to the block. Later calls borrow from the owned local with `&document`, so no value borrowed from the document map crosses later awaits.

## Runtime Test Re-entry

Deterministic contention coverage is deferred. The current stdio LSP harness does not provide a controlled way to suspend diagnostic inside later awaited work while concurrently proving a document write can complete, without relying on timing sleeps or making private internals public only for tests.

Re-enter C7-5 when there is either a controlled test hook/dependency injection around awaited diagnostic work, or an in-crate direct handler harness that can pause diagnostic after the document clone. The regression should then assert a concurrent `did_change`/sync write completes while the diagnostic request is paused.

## Verification

Commands run locally:

- `cargo fmt --check` -> passed
- `cargo check --all-targets` -> passed
- `RUSTFLAGS="-D rust-2024-compatibility" cargo check --all-targets` -> passed
- `cargo clippy --all-targets -- -D warnings` -> passed
- `cargo test --test lsp_current_behavior` -> passed, 6 tests
- `cargo test --all-targets` -> passed
- `cargo build --release --bins` -> passed
- `./target/release/hddl_analyzer --help` -> passed
- `actionlint .github/workflows/ci.yml` -> passed
- `git diff --check` -> passed

Scope checks:

- `git rev-parse --short HEAD` -> `e14078c`
- `git diff --name-only fbb27d7..HEAD` -> `src/language_server/request_handler.rs`

Source inspection:

- `src/language_server/request_handler.rs:224` starts the scoped document clone block.
- `src/language_server/request_handler.rs:230` ends the block, dropping the read guard.
- `src/language_server/request_handler.rs:236`, `:248`, `:255`, and `:267` are later awaited operations that now occur after the guard has dropped.

## Ledger Walk

- C7-1: Done. Feature commit diff is limited to `src/language_server/request_handler.rs` on base `fbb27d7`.
- C7-2: Done. Diagnostic owns cloned document bytes before later awaited work.
- C7-3: Done. Manual scope inspection confirms no borrowed map value crosses an await.
- C7-4: Done. Focused LSP behavior tests pass.
- C7-5: Deferred. Runtime contention regression coverage has a concrete re-entry condition above.
- C7-6: Done. Slice06 RUST-005 error-boundary behavior was not changed.
- C7-7: Done. Full local workflow-equivalent gate passes.
- C7-8: Done. This report walks all rows and states the proof type.
