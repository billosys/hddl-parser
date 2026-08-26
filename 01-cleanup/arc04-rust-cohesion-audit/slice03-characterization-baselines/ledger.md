# Slice03: Characterization Baselines Ledger

Definition of done: Slice03 adds test-only current-behavior coverage for Arc04
cohesion repair targets before production changes begin.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| B4-1 | Slice starts from the Arc04 Slice02 plan and uses a new feature branch. | `git status --short --branch` | correctness | project-management | open | | Expected branch: `test/arc04-characterization-baselines`. |
| B4-2 | Slice remains test-only in the feature worktree. | `git diff --name-only` | serious | slice02-fix-map | open | | Expected implementation diff should be limited to Rust test files plus planning close evidence outside the feature worktree. |
| B4-3 | Current Vec-backed parser/transpiler API behavior is pinned. | `rg -n "from_hddl|Input::Hddl|Vec" tests src -g '*.rs'` and `cargo test --locked --all-targets` | serious | COHESION-001 | open | | Do not add failing `&[u8]` tests here. |
| B4-4 | Current public import surface is pinned with representative imports. | `rg -n "LexicalAnalyzer|Formula|Predicate|Transformation|ParsingError" tests -g '*.rs'` and `cargo test --locked --all-targets` | serious | COHESION-001 | open | | This captures export compatibility risk before any narrowing. |
| B4-5 | Current malformed problem-parser panic behavior is explicitly characterized. | `rg -n "catch_unwind|should_panic|malformed problem|unexpected" tests -g '*.rs'` and `cargo test --locked --all-targets` | serious | COHESION-002 | open | | Slice04 will change this expectation to structured error behavior. |
| B4-6 | Current transformation/classification error variant behavior is pinned. | `rg -n "ParsingError::Transformation|expected domain input|expected problem input|remove-equality-constraints" tests -g '*.rs'` and `cargo test --locked --all-targets` | serious | COHESION-003 | open | | Preserve messages before any future typed error policy. |
| B4-7 | Current public spelling variants are pinned. | `rg -n "Lexiacal|QuantifierElimintation" tests -g '*.rs'` and `cargo test --locked --all-targets` | correctness | COHESION-006 | open | | Public typo fixes require later operator GO. |
| B4-8 | Current formula normalization panic contracts are pinned. | `rg -n "to_dnf|to_nnf|probabilistic|panic|catch_unwind" tests -g '*.rs'` and `cargo test --locked --all-targets` | serious | COHESION-004 | open | | Equality, non-NNF, and probabilistic paths should be represented. |
| B4-9 | Full local gate passes. | `cargo fmt --check && cargo check --locked --all-targets && RUSTFLAGS="-D rust-2024-compatibility" cargo check --locked --all-targets && cargo clippy --locked --all-targets -- -D warnings && cargo test --locked --all-targets && cargo build --locked --release --bins && ./target/release/hddl_analyzer --help && actionlint .github/workflows/ci.yml && git diff --check` | serious | arc04 | open | | |
| B4-10 | Closing report walks every row and bubbles up Slice04/Slice05 readiness. | `test -f 01-cleanup/arc04-rust-cohesion-audit/slice03-characterization-baselines/closing-report.md` and inspect row walk. | correctness | ledger-discipline | open | | |

## What Worked

Pending Slice03 close.

## Closure

Pending Slice03 close.
