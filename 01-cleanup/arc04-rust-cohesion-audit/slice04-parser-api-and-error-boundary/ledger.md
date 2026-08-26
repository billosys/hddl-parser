# Slice04: Parser API And Error Boundary Ledger

Definition of done: Slice04 repairs parser byte-input cohesion and the malformed
problem parser recoverability boundary after characterization tests exist.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| R4-1 | Slice starts from the CDC-verified Slice03 baseline on a focused branch. | `git status --short --branch` | correctness | project-management | open | | Expected branch: `fix/parser-api-error-boundary`. |
| R4-2 | Public parser/transpiler/LSP byte-input APIs accept `&[u8]` where Vec behavior is not required. | `rg -n "&Vec<u8>|&\\[u8\\]" src tests -g '*.rs'` and `cargo check --locked --all-targets` | serious | COHESION-001 | open | | Remaining `&Vec<u8>` must be justified or removed. |
| R4-3 | Vec-backed callers remain supported through normal deref coercion. | `cargo test --locked --all-targets` | serious | compatibility | open | | Existing tests from earlier slices should continue to pass. |
| R4-4 | New tests prove byte-slice callers work. | `rg -n "as_slice|&\\[u8\\]|from_hddl" tests -g '*.rs'` and `cargo test --locked --all-targets` | serious | RUST-007 | open | | These tests belong in this repair slice because they cannot pass before the signature change. |
| R4-5 | Malformed problem parser no longer panics on the Slice03 characterized path. | `rg -n "malformed problem|Syntactic|catch_unwind|unexpected" tests src -g '*.rs'` and `cargo test --locked --all-targets` | serious | COHESION-002 | open | | Update the characterization from panic to structured error. |
| R4-6 | Parser syntactic error construction is made more consistent without changing public error types. | `rg -n "SyntacticError|syntax_error|ParsingError::Syntactic" src/syntactic_analyzer -g '*.rs'` | correctness | COHESION-002 | open | | Helper extraction is optional if it would widen the slice. |
| R4-7 | Crate-root export narrowing does not land without explicit operator GO. | `git diff -- src/lib.rs` and inspect public `pub use` changes. | serious | public-api | open | | If GO is not given, record accepted variation/no-op for this slice. |
| R4-8 | Full local gate passes. | `cargo fmt --check && cargo check --locked --all-targets && RUSTFLAGS="-D rust-2024-compatibility" cargo check --locked --all-targets && cargo clippy --locked --all-targets -- -D warnings && cargo test --locked --all-targets && cargo build --locked --release --bins && ./target/release/hddl_analyzer --help && actionlint .github/workflows/ci.yml && git diff --check` | serious | arc04 | open | | |
| R4-9 | Closing report walks every row and bubbles up export-policy disposition. | `test -f 01-cleanup/arc04-rust-cohesion-audit/slice04-parser-api-and-error-boundary/closing-report.md` and inspect row walk. | correctness | ledger-discipline | open | | |

## What Worked

Pending Slice04 close.

## Closure

Pending Slice04 close.
