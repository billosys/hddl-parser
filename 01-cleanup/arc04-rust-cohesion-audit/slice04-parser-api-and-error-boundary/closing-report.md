# Slice04 Closing Report: Parser API And Error Boundary

Date: 2026-08-25
Branch: `fix/parser-api-error-boundary`

## Summary

Slice04 is locally closed. The repair changes borrowed byte-input boundaries
from `&Vec<u8>` to `&[u8]` where no Vec-specific behavior is required, preserves
Vec-backed callers through deref coercion, and replaces the characterized
malformed problem-parser panic path with a structured `ParsingError::Syntactic`.

No crate-root public exports were removed, narrowed, or renamed. Public export
narrowing remains gated behind explicit operator GO or a later public
API/error/AST contract arc.

## Focused Diff Boundary

Feature worktree diff is confined to:

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

The implementation stays inside the Slice04 parser/transpiler/LSP byte-input
boundary and problem-parser recoverability scope. It does not change Cargo,
workflow, README, formula APIs, public enum spellings, transformation error
taxonomy, or crate-root public exports.

## Row Walk

### R4-1

Done. `git status --short --branch --untracked-files=all` reports branch
`fix/parser-api-error-boundary` with only Slice04 source/test edits. The branch
was created from the clean Slice03 feature baseline; branch creation required an
escalated `git switch -c` because the shared worktree lock was outside the
writable sandbox.

### R4-2

Done. Borrowed byte-input APIs now accept `&[u8]` where Vec-specific behavior is
not used:

- `LexicalAnalyzer::new`
- `HDDLProgram::from_hddl`
- `Transpiler::from_hddl`
- `Input::Hddl`
- LSP diagnostic helper boundaries

Evidence:

- `rg -n "&Vec<u8>|&\\[u8\\]" src tests -g '*.rs'` reports the expected
  byte-slice signatures and no remaining `&Vec<u8>` matches.
- `cargo check --locked --all-targets` passed.

### R4-3

Done. Vec-backed callers remain supported through deref coercion. The existing
Vec-backed characterization tests still compile and pass under
`cargo test --locked --all-targets`.

### R4-4

Done. `tests/arc04_characterization.rs` adds direct byte-slice coverage for:

- `HDDLProgram::from_hddl(domain.as_slice(), Some(problem.as_slice()))`
- `Transpiler::from_hddl(domain.as_slice(), Some(problem.as_slice()))`
- `Input::Hddl { domain: domain.as_slice(), problem: Some(problem.as_slice()) }`

Evidence:

- `rg -n "byte_slice|as_slice|&\\[u8\\]|from_hddl" tests/arc04_characterization.rs src/transpiler/tests.rs src/lib.rs src/transpiler/core.rs src/transpiler/input.rs`
- `cargo test --locked --all-targets` passed.

### R4-5

Done. The Slice03 malformed-problem panic characterization was updated to prove
the path no longer panics and now returns `ParsingError::Syntactic`.

Evidence:

- `malformed_problem_top_level_token_returns_syntactic_error` uses
  `panic::catch_unwind`, asserts the call did not panic, and checks the expected
  syntactic error details.
- `src/syntactic_analyzer/problem_parser/router.rs` returns
  `Err(ParsingError::Syntactic(error))` for the top-level unexpected-token path.
- `cargo test --locked --all-targets` passed.

### R4-6

Done. The problem parser now follows the same existing router error-construction
shape as the domain parser by building `SyntacticError` and returning
`ParsingError::Syntactic`. No public error type changed.

Helper extraction was skipped intentionally. The repair is small, and extracting
a new helper would have widened the patch without improving the Slice04 review
boundary.

### R4-7

Done. Public crate-root export narrowing did not land.

Evidence:

- `git diff -- src/lib.rs` shows only the `HDDLProgram::from_hddl` byte-input
  signature changing from `&Vec<u8>` to `&[u8]`.
- No crate-root `pub use` line was removed, narrowed, or renamed.

Disposition: skipped/gated. Public export narrowing requires explicit operator
GO or future public API/error/AST contract work.

### R4-8

Done. Full local gate passed:

```bash
cargo fmt --check
cargo check --locked --all-targets
RUSTFLAGS="-D rust-2024-compatibility" cargo check --locked --all-targets
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked --all-targets
cargo build --locked --release --bins
./target/release/hddl_analyzer --help
actionlint .github/workflows/ci.yml
git diff --check
```

Additional planning whitespace gate:

```bash
git -C ../planning diff --check
```

### R4-9

Done. This closing report walks rows R4-1 through R4-9, records the focused diff
boundary, and bubbles up the public export-policy disposition.

## Verification Evidence

Commands run and passing:

```bash
cargo fmt --check
cargo check --locked --all-targets
RUSTFLAGS="-D rust-2024-compatibility" cargo check --locked --all-targets
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked --all-targets
cargo build --locked --release --bins
./target/release/hddl_analyzer --help
actionlint .github/workflows/ci.yml
git diff --check
git -C ../planning diff --check
```

Observed test summary included:

- Library tests: 111 passed, 1 ignored.
- `tests/arc04_characterization.rs`: 11 passed.
- `current_behavior`: 10 passed.
- `integration_flawed`: 21 passed, 2 ignored.
- `integration_ipc`: 1 ignored.
- `integration_json`: 8 passed, 1 ignored.
- `lsp_current_behavior`: 6 passed.

## Bubble-Up

Slice04 is locally complete and ready for CDC verification. Slice05 remains the
next Arc04 repair slice after CDC accepts this parser API/error-boundary repair.

Public crate-root export narrowing was not performed. That topic remains behind
the already-recorded public API gate and should not be treated as implicitly
approved by this slice.
