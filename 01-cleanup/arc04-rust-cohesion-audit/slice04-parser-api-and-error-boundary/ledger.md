# Slice04: Parser API And Error Boundary Ledger

Definition of done: Slice04 repairs parser byte-input cohesion and the malformed
problem parser recoverability boundary after characterization tests exist.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| R4-1 | Slice starts from the CDC-verified Slice03 baseline on a focused branch. | `git status --short --branch` | correctness | project-management | done | `git status --short --branch --untracked-files=all` reports `## fix/parser-api-error-boundary` with only Slice04 source/test edits. | Started from clean Slice03 feature and clean planning worktrees; branch creation required an escalated `git switch -c` because the shared worktree lock was outside the writable sandbox. |
| R4-2 | Public parser/transpiler/LSP byte-input APIs accept `&[u8]` where Vec behavior is not required. | `rg -n "&Vec<u8>|&\\[u8\\]" src tests -g '*.rs'` and `cargo check --locked --all-targets` | serious | COHESION-001 | done | `rg -n "&Vec<u8>|&\\[u8\\]" src tests -g '*.rs'` now reports byte-slice signatures at parser/transpiler/LSP helpers and no `&Vec<u8>` matches; `cargo check --locked --all-targets` passed. | Changed borrowed byte input boundaries to slices where no Vec-specific behavior is used. |
| R4-3 | Vec-backed callers remain supported through normal deref coercion. | `cargo test --locked --all-targets` | serious | compatibility | done | `cargo test --locked --all-targets` passed: library tests 111 passed/1 ignored; Arc04 characterization 11 passed; current behavior 10 passed; integration suites green with existing ignored tests preserved. | Existing Vec-backed characterization calls still compile and pass through deref coercion. |
| R4-4 | New tests prove byte-slice callers work. | `rg -n "as_slice|&\\[u8\\]|from_hddl" tests -g '*.rs'` and `cargo test --locked --all-targets` | serious | RUST-007 | done | `tests/arc04_characterization.rs` adds `byte_slice_hddl_program_api_parses_domain_and_problem` and `byte_slice_transpiler_and_input_hddl_apis_parse_domain_and_problem`; full locked test gate passed. | Tests cover `HDDLProgram::from_hddl`, `Transpiler::from_hddl`, and `Input::Hddl` with `as_slice()` callers. |
| R4-5 | Malformed problem parser no longer panics on the Slice03 characterized path. | `rg -n "malformed problem|Syntactic|catch_unwind|unexpected" tests src -g '*.rs'` and `cargo test --locked --all-targets` | serious | COHESION-002 | done | `malformed_problem_top_level_token_returns_syntactic_error` catches unwind, asserts no panic, and verifies `ParsingError::Syntactic`; `cargo test --locked --all-targets` passed. | Replaced the problem router top-level unexpected-token panic with a structured syntactic error. |
| R4-6 | Parser syntactic error construction is made more consistent without changing public error types. | `rg -n "SyntacticError|syntax_error|ParsingError::Syntactic" src/syntactic_analyzer -g '*.rs'` | correctness | COHESION-002 | done | `rg -n "SyntacticError|ParsingError::Syntactic" src/syntactic_analyzer/problem_parser/router.rs src/syntactic_analyzer/domain_parser/router.rs` shows the problem router now follows the same existing `SyntacticError`/`ParsingError::Syntactic` pattern as the domain router. | Helper extraction was intentionally skipped because the local pattern was small and extraction would widen this repair. |
| R4-7 | Crate-root export narrowing does not land without explicit operator GO. | `git diff -- src/lib.rs` and inspect public `pub use` changes. | serious | public-api | done | `git diff -- src/lib.rs` shows only `HDDLProgram::from_hddl` input signature changes from `&Vec<u8>` to `&[u8]`; no crate-root `pub use` lines were removed, narrowed, or renamed. | Public export narrowing remains gated/deferred. |
| R4-8 | Full local gate passes. | `cargo fmt --check && cargo check --locked --all-targets && RUSTFLAGS="-D rust-2024-compatibility" cargo check --locked --all-targets && cargo clippy --locked --all-targets -- -D warnings && cargo test --locked --all-targets && cargo build --locked --release --bins && ./target/release/hddl_analyzer --help && actionlint .github/workflows/ci.yml && git diff --check` | serious | arc04 | done | Passed: `cargo fmt --check`; `cargo check --locked --all-targets`; `RUSTFLAGS="-D rust-2024-compatibility" cargo check --locked --all-targets`; `cargo clippy --locked --all-targets -- -D warnings`; `cargo test --locked --all-targets`; `cargo build --locked --release --bins`; `./target/release/hddl_analyzer --help`; `actionlint .github/workflows/ci.yml`; `git diff --check`. | `git -C ../planning diff --check` also passed after planning edits landed. |
| R4-9 | Closing report walks every row and bubbles up export-policy disposition. | `test -f 01-cleanup/arc04-rust-cohesion-audit/slice04-parser-api-and-error-boundary/closing-report.md` and inspect row walk. | correctness | ledger-discipline | done | `slice04-parser-api-and-error-boundary/closing-report.md` added with row-by-row closure and public export-policy disposition; `cdc-verification.md` verifies the close. | Slice04 is CDC-verified. |

## What Worked

Changing parser/transpiler/LSP borrowed byte-input boundaries from `&Vec<u8>`
to `&[u8]` stayed narrow and preserved existing Vec-backed callers through
normal coercion. The new byte-slice characterization tests prove the intended
caller shape directly.

The malformed problem-parser repair also stayed inside the existing public
error taxonomy: the top-level unexpected-token path now returns
`ParsingError::Syntactic` instead of panicking, matching the established parser
router convention.

## Closure

Slice04 is CDC-verified. Public crate-root export narrowing was skipped and
remains gated behind explicit operator GO or a later public API/error/AST
contract arc.
