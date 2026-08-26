# CDC Verification: Arc03 Slice07 LSP Diagnostic Lock Scope

Date: 2026-08-26
Verifier: CDC
Feature branch: `fix/lsp-diagnostic-lock-scope`
Feature base: `fbb27d7 Fix LSP error boundaries and metadata`
Feature commit verified: `e14078c Fix LSP diagnostic lock scope`
Planning branch: `planning`

## Verdict

Verified with one valid deferred row. Slice07 fixes RUST-004 by ensuring the
language-server diagnostic path does not hold the document-map `RwLock` read
guard across awaited logging, filesystem work, or diagnostic generation.

C7-1 through C7-4 and C7-6 through C7-8 are verified done. C7-5 is validly
deferred with a concrete re-entry condition because deterministic contention
coverage through the public stdio LSP harness would require timing-fragile
sleeps or exposing private internals only for tests.

## Artifact Boundary

`git diff --name-only fbb27d7..HEAD`:

```text
src/language_server/request_handler.rs
```

CDC inspected the one-file diff. No tests, CLI, parser, transform, Cargo,
public API, metadata, or Slice06 error-boundary behavior changes are mixed into
the feature commit.

## Lock-Scope Proof

Source shape at `src/language_server/request_handler.rs:224`:

```rust
let document = {
    self.documents
        .read()
        .await
        .get(&params.text_document.uri)
        .cloned()
};
```

The document-map read guard is confined to the block ending at line 230. Later
awaited work starts at line 236 and borrows from the owned `Vec<u8>` local,
not from the document map.

CDC reproduced the source proof with:

```text
rg -n "documents\\.read\\(\\)\\.await|get\\(|\\.await" src/language_server/request_handler.rs
```

Relevant observations:

- `documents.read().await` / `get` occur at lines 227-228.
- Later awaits occur at lines 236, 248, 255, and 267.
- Manual scope inspection confirms the read guard drops at line 230 before
  those later awaits.

## Quality Gate

CDC reproduced the Slice07 gate:

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
- `git diff --check fbb27d7..HEAD`

All commands exited `0`. `cargo test --test lsp_current_behavior` passed with
6 tests, 0 failures, and 0 ignored tests. `cargo test --all-targets` passed
with the existing ignored legacy tests unchanged.

## Ledger Walk

- C7-1: Verified. Feature commit `e14078c` changes only
  `src/language_server/request_handler.rs` relative to `fbb27d7`.
- C7-2: Verified. `diagnostic` clones the synced document bytes before later
  awaited logging/filesystem/diagnostic work.
- C7-3: Verified. No borrowed value from the document-map read guard crosses a
  later `.await` in `diagnostic`.
- C7-4: Verified. Existing LSP behavior tests remain green.
- C7-5: Deferred. Deterministic runtime contention coverage needs either a
  controlled test hook/dependency injection around awaited diagnostic work, or
  an in-crate direct handler harness that can pause diagnostic after document
  clone and assert a concurrent `did_change`/sync write completes.
- C7-6: Verified. Slice06 RUST-005 error-boundary behavior is not changed.
- C7-7: Verified. Full local workflow-equivalent gate passed.
- C7-8: Verified. Closing report walks every row and states the final
  lock-scope proof.

## Bubble-Up

No arc-plan re-slicing is required. Slice07 delivers the RUST-004 lock-scope
repair while leaving its deferred runtime-contention proof with a concrete
future re-entry condition. Slice08 remains correctly scoped to Cargo
reproducibility policy.
