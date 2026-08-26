# Slice04 CDC Verification: Parser API And Error Boundary

Date: 2026-08-26
Planning branch: `planning`
Implementation branch: `fix/parser-api-error-boundary`
Implementation commit: `d17b41f fix: repair parser API error boundary`
Verifier: CDC

## Verdict

Verified closed.

Slice04 delivers the parser byte-input cohesion repair and the malformed
problem-parser recoverability repair promised by the open set. The feature diff
is focused, the compatibility baseline remains green, and public crate-root
export narrowing did not land.

## Boundary Check

The feature diff from the CDC-verified Slice03 baseline is confined to the
expected Slice04 files:

- `src/bin/hddl_analyzer/main.rs`
- `src/language_server/diagnostic_utils.rs`
- `src/language_server/request_handler.rs`
- `src/lexical_analyzer/tokenizer.rs`
- `src/lib.rs`
- `src/syntactic_analyzer/problem_parser/router.rs`
- `src/transpiler/core.rs`
- `src/transpiler/input.rs`
- `src/transpiler/tests.rs`
- `tests/arc04_characterization.rs`

No Cargo, workflow, README, formula API, public enum spelling, transformation
error-taxonomy, or crate-root public export changes were observed.

## Ledger Verification

- R4-1: Reproduced. `git status --short --branch --untracked-files=all`
  reports `## fix/parser-api-error-boundary` with a clean worktree.
- R4-2: Reproduced. `rg -n "&Vec<u8>|&\\[u8\\]" src tests -g '*.rs'` reports
  `&[u8]` byte-input signatures at parser/transpiler/LSP helper boundaries and
  no remaining `&Vec<u8>` matches. `cargo check --locked --all-targets`
  passed.
- R4-3: Reproduced. Vec-backed callers remain supported by deref coercion;
  `cargo test --locked --all-targets` passed.
- R4-4: Reproduced. `tests/arc04_characterization.rs` includes direct
  `as_slice()` caller coverage for `HDDLProgram::from_hddl`,
  `Transpiler::from_hddl`, and `Input::Hddl`; full tests passed.
- R4-5: Reproduced. The malformed problem top-level unexpected-token test now
  asserts no panic and verifies `ParsingError::Syntactic`; full tests passed.
- R4-6: Reproduced. `src/syntactic_analyzer/problem_parser/router.rs` now
  returns `ParsingError::Syntactic` for the unexpected top-level token path,
  matching the existing router error-construction shape without public error
  type changes.
- R4-7: Reproduced. `git diff e904099..HEAD -- src/lib.rs` shows only
  `HDDLProgram::from_hddl` input signature changes from `&Vec<u8>` to
  `&[u8]`; no crate-root `pub use` line was removed, narrowed, or renamed.
- R4-8: Reproduced. Full local gate passed.
- R4-9: Reproduced. `closing-report.md` exists, walks R4-1 through R4-9, and
  bubbles up the public export-policy disposition.

## Verification Commands

Executed from `.worktrees/features` unless noted:

- `git status --short --branch --untracked-files=all`
- `git diff --name-status e904099..HEAD`
- `git show --name-only --pretty=format: d17b41f`
- `rg -n "&Vec<u8>|&\\[u8\\]" src tests -g '*.rs'`
- `rg -n "byte_slice|as_slice|&\\[u8\\]|from_hddl" tests/arc04_characterization.rs src/transpiler/tests.rs src/lib.rs src/transpiler/core.rs src/transpiler/input.rs`
- `rg -n "malformed problem|Syntactic|catch_unwind|unexpected" tests src -g '*.rs'`
- `git diff e904099..HEAD -- src/lib.rs`
- `cargo fmt --check`
- `cargo check --locked --all-targets`
- `RUSTFLAGS="-D rust-2024-compatibility" cargo check --locked --all-targets`
- `cargo clippy --locked --all-targets -- -D warnings`
- `cargo test --locked --test arc04_characterization`
- `cargo test --locked --test current_behavior`
- `cargo test --locked --all-targets`
- `cargo build --locked --release --bins`
- `./target/release/hddl_analyzer --help`
- `actionlint .github/workflows/ci.yml`
- `git diff --check e904099..HEAD`
- `git -C ../planning diff --check`

All commands passed or produced the expected inspection output.

## Bubble-Up

Slice04 composes with Slice03's characterization baseline: the Vec-backed
caller tests still pass, the new byte-slice caller tests pass, and the
malformed problem-parser path has moved from panic to structured syntactic
error behavior.

No arc-plan rescope is required. Slice05 remains the next Arc04 repair slice.
Public crate-root export narrowing remains deferred behind explicit operator GO
or future public API/error/AST contract work.

## What Worked

The Slice03 characterization file gave this repair a clean before/after target.
The focused diff made it easy to verify the public API improvement separately
from the still-gated public export policy.
