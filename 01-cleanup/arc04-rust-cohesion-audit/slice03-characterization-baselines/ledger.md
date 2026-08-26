# Slice03: Characterization Baselines Ledger

Definition of done: Slice03 adds test-only current-behavior coverage for Arc04
cohesion repair targets before production changes begin.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| B4-1 | Slice starts from the Arc04 Slice02 plan and uses a new feature branch. | `git status --short --branch` | correctness | project-management | done | `git status --short --branch` reported `## test/arc04-characterization-baselines`; implementation commit `e904099`. | Branch created from Arc04 feature baseline `d820065`. |
| B4-2 | Slice remains test-only in the feature worktree. | `git diff --name-only` | serious | slice02-fix-map | done | `git show --name-only --pretty=format: e904099` and `git diff --name-only d820065..HEAD` both reported only `tests/arc04_characterization.rs`. | Ignored `workbench/` files were left untracked/ignored, not included in the slice commit. |
| B4-3 | Current Vec-backed parser/transpiler API behavior is pinned. | `rg -n "from_hddl|Input::Hddl|Vec" tests src -g '*.rs'` and `cargo test --locked --all-targets` | serious | COHESION-001 | done | `tests/arc04_characterization.rs` pins `HDDLProgram::from_hddl`, `Transpiler::from_hddl`, and `Input::Hddl` with Vec-backed fixtures; full tests passed. | No failing `&[u8]` tests were added. |
| B4-4 | Current public import surface is pinned with representative imports. | `rg -n "LexicalAnalyzer|Formula|Predicate|Transformation|ParsingError" tests -g '*.rs'` and `cargo test --locked --all-targets` | serious | COHESION-001 | done | `tests/arc04_characterization.rs` imports crate-root `LexicalAnalyzer`, `Parser`, `Formula`, `Predicate`, `Transformation`, and `ParsingError`; full tests passed. | Captures representative export compatibility risk before narrowing. |
| B4-5 | Current malformed problem-parser panic behavior is explicitly characterized. | `rg -n "catch_unwind|should_panic|malformed problem|unexpected" tests -g '*.rs'` and `cargo test --locked --all-targets` | serious | COHESION-002 | done | `malformed_problem_top_level_token_currently_panics` uses `catch_unwind` on an unexpected top-level problem token; full tests passed. | Slice04 can change this to structured error behavior. |
| B4-6 | Current transformation/classification error variant behavior is pinned. | `rg -n "ParsingError::Transformation|expected domain input|expected problem input|remove-equality-constraints" tests -g '*.rs'` and `cargo test --locked --all-targets` | serious | COHESION-003 | done | `current_transformation_error_variant_and_messages_are_pinned` matches `ParsingError::Transformation(String)` and exact messages for kind mismatch and remove-equality precondition paths; full tests passed. | Preserves messages before any future typed error policy. |
| B4-7 | Current public spelling variants are pinned. | `rg -n "Lexiacal|QuantifierElimintation" tests -g '*.rs'` and `cargo test --locked --all-targets` | correctness | COHESION-006 | done | `current_public_misspelled_variants_are_available` constructs `ParsingError::Lexiacal` and matches `Transformation::QuantifierElimintation`; full tests passed. | Public typo fixes remain later-arc/operator-GO work. |
| B4-8 | Current formula normalization panic contracts are pinned. | `rg -n "to_dnf|to_nnf|probabilistic|panic|catch_unwind" tests -g '*.rs'` and `cargo test --locked --all-targets` | serious | COHESION-004 | done | Three `catch_unwind` tests pin `to_dnf` equality, `to_dnf` non-NNF negated equality, and `to_nnf` probabilistic panic contracts; full tests passed. | Equality, non-NNF, and probabilistic paths are represented. |
| B4-9 | Full local gate passes. | `cargo fmt --check && cargo check --locked --all-targets && RUSTFLAGS="-D rust-2024-compatibility" cargo check --locked --all-targets && cargo clippy --locked --all-targets -- -D warnings && cargo test --locked --all-targets && cargo build --locked --release --bins && ./target/release/hddl_analyzer --help && actionlint .github/workflows/ci.yml && git diff --check` | serious | arc04 | done | All listed commands passed. `cargo test --locked --all-targets` reported 111 lib tests passed, 1 lib ignored, 9 Arc04 tests passed, 10 current-behavior tests passed, 21 flawed integration tests passed with 2 ignored, IPC ignored, JSON 8 passed with 1 ignored, and 6 LSP tests passed. | `git diff --check d820065..HEAD` also passed for the committed feature diff. |
| B4-10 | Closing report walks every row and bubbles up Slice04/Slice05 readiness. | `test -f 01-cleanup/arc04-rust-cohesion-audit/slice03-characterization-baselines/closing-report.md` and inspect row walk. | correctness | ledger-discipline | done | `closing-report.md` added with B4-1 through B4-10 row walk and Slice04/Slice05 readiness bubble-up. | |

## What Worked

The characterization targets fit well in one public integration test file. The
focused test target caught fixture-metadata assumptions before the full gate,
which kept the slice test-only and small.

## Closure

Closed locally at feature commit `e904099` on 2026-08-25.
Rows: 10. Done: 10. Deferred: 0. No-op: 0.
