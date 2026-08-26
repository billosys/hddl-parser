# Slice07: LSP Diagnostic Lock Scope Ledger

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| C7-1 | The slice diff is limited to diagnostic lock-scope repair and directly related tests/evidence, on top of the verified Slice06 base. | `git diff --name-only fbb27d7..HEAD` unless Slice06 has merged upstream, then inspect changed files. | serious | RUST-004 | done | Feature commit `e14078c` changes only `src/language_server/request_handler.rs` relative to `fbb27d7`. | Feature diff is limited to the diagnostic lock-scope file. |
| C7-2 | The diagnostic path owns or clones document bytes before any awaited logging/filesystem work. | Inspect `src/language_server/request_handler.rs` around `documents.read().await` and later `.await` sites. | serious | RUST-004 | done | Feature commit `e14078c`; `src/language_server/request_handler.rs:224` clones the map value inside a short block; first later awaited log is at line 236. | The synced document is owned as `Vec<u8>` before diagnostic logging, path parsing, classification, or sibling-domain reads. |
| C7-3 | No borrowed value from the document-map read guard crosses an `.await` in `diagnostic`. | `rg -n "documents\\.read\\(\\)\\.await|get\\(|\\.await" src/language_server/request_handler.rs` plus manual scope inspection. | serious | RUST-004 | done | Feature commit `e14078c`; `rg` shows `documents.read().await`/`.get` at lines 227-228 and later awaits at lines 236, 248, 255, and 267; manual scope inspection confirms the guard is dropped at line 230. | The later `&document` borrows are from the owned cloned local, not from the map guard. |
| C7-4 | Existing LSP behavior tests remain green after the lock-scope change. | `cargo test --test lsp_current_behavior` | correctness | slice02-baseline | done | Feature commit `e14078c`; CDC reproduced `cargo test --test lsp_current_behavior` -> 6 passed, 0 failed. | Existing RUST-005 and metadata baseline behavior stayed green. |
| C7-5 | A deterministic contention regression test is added or explicitly deferred with re-entry. | Inspect tests and closing report for `RwLock`, `contention`, `re-entry`, or source-level proof. | correctness | slice02-cdc | deferred | Runtime contention test deferred; closing report records source-level proof and re-entry condition. | Avoided timing-only tests and avoided making private internals public only for tests. |
| C7-6 | RUST-005 error-boundary behavior is not silently changed in this slice unless explicitly inherited from Slice06. | `git diff fbb27d7..HEAD -- src/language_server/request_handler.rs` and inspect non-lock error paths. | correctness | slice03-fix-map | done | Feature commit `e14078c`; diff only changes `diagnostic` document lookup from borrowed map value to cloned owned bytes, plus adjusted `&document` calls. | No non-file URI, unsynced document, missing-file, metadata, CLI, parser, transform, Cargo, or public API changes. |
| C7-7 | The full local workflow-equivalent gate passes. | `cargo fmt --check`; `cargo check --all-targets`; `RUSTFLAGS="-D rust-2024-compatibility" cargo check --all-targets`; `cargo clippy --all-targets -- -D warnings`; `cargo test --all-targets`; `cargo build --release --bins`; `./target/release/hddl_analyzer --help`; `actionlint .github/workflows/ci.yml`; `git diff --check` | serious | arc-plan | done | Feature commit `e14078c`; CDC reproduced all requested commands with exit `0` on 2026-08-26. | `cargo test --all-targets` reported lib 111 passed/1 ignored; current behavior 10 passed; flawed 21 passed/2 ignored; ipc 1 ignored; json 8 passed/1 ignored; LSP 6 passed. |
| C7-8 | Slice07 closing report walks every row and states the final lock-scope proof. | Inspect `closing-report.md` for C7-1 through C7-8. | correctness | ledger-discipline | done | Feature commit `e14078c`; `closing-report.md` records C7-1 through C7-8, proof type, and C7-5 re-entry condition. | Proof is source-inspection based plus behavior-regression gate coverage; no runtime contention test in this slice. |

## What Worked

The fix was a small lexical-scope change: clone the synced document bytes while the document-map read guard is live, then perform awaited logging/filesystem/diagnostic work after the guard has dropped. Existing LSP behavior tests were sufficient to protect the already-settled Slice06 error-boundary behavior while source inspection proved the RUST-004 lock-scope change.

## Closure

Slice07 is closed at feature commit `e14078c` with one deferred test-evidence
row. C7-1 through C7-4 and C7-6 through C7-8 are done. C7-5 is explicitly
deferred because a deterministic runtime contention test is not currently
feasible through the public stdio LSP harness without timing sleeps or exposing
private internals only for tests. Re-enter C7-5 if a controlled test hook or
in-crate direct handler harness is introduced that can pause diagnostic after
document clone and assert a concurrent document write completes.
