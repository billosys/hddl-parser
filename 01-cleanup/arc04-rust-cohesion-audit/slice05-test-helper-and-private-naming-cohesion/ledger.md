# Slice05: Test Helper And Private Naming Cohesion Ledger

Definition of done: Slice05 improves test/helper consistency and private naming
cohesion without changing public API or product behavior.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| T4-1 | Slice starts after Slice03 baselines and Slice04 parser repair are closed or explicitly deferred. | `git status --short --branch` and inspect prior slice close evidence. | correctness | project-management | done | `git status --short --branch` reports `## fix/test-private-naming-cohesion`; `slice03-characterization-baselines/cdc-verification.md` and `slice04-parser-api-and-error-boundary/cdc-verification.md` both exist and report verified closure. | Branch creation required an escalated `git switch -c` because the shared worktree ref lock is outside the writable sandbox. |
| T4-2 | Test helper consolidation keeps or improves assertion precision. | `git diff -- tests` and inspect before/after assertions; `cargo test --locked --all-targets` | serious | COHESION-005 | done | `tests/integration_flawed.rs` now centralizes parse/verify result handling in `expect_semantic_error`, `expect_syntactic_error`, and `expect_warnings`; call sites still assert the exact expected semantic/syntactic variant plus prior line, symbol, var-name, found-token, and warning-count checks. `cargo test --locked --all-targets` passed. | Helper consolidation is limited to the repeated flawed-domain integration-test assertion shell. |
| T4-3 | Existing fixture coverage and ignored-test status are preserved unless explicitly justified. | `cargo test --locked --all-targets` and inspect ignored-test output. | correctness | test-cohesion | done | `cargo test --locked --all-targets` passed with the existing ignored-test pattern preserved: 1 ignored unit test, 2 ignored flawed integration tests, 1 ignored IPC integration test, and 1 ignored JSON integration test. | No ignored test was removed or added. |
| T4-4 | Private implementation spelling drift is repaired where scoped. | `rg -n "qunatifier|elimintation|parantheses|satelite" src tests -g '*.rs'` | correctness | COHESION-006 | done | `rg -n "qunatifier|elimintation|parantheses|satelite" src tests -g '*.rs'` returned no matches. The private transformation module is now `quantifier_elimination`, the private parser comment says `parentheses`, and test names use `extra_parentheses` and `satellite`. | Public uppercase spellings are covered by T4-5 and remain deferred. |
| T4-5 | Public enum variants are not renamed without operator GO. | `git diff -- src/output/errors/generic.rs src/transpiler/transformations/transform.rs` | serious | public-api | done | `git diff -- src/output/errors/generic.rs src/transpiler/transformations/transform.rs` produced no output. `rg -n "QuantifierElimintation|Lexiacal" src tests -g '*.rs'` confirms the public spellings remain present in the existing public enum definitions and characterization tests. | `Lexiacal` and `QuantifierElimintation` remain public API deferrals. |
| T4-6 | Full local gate passes. | `cargo fmt --check && cargo check --locked --all-targets && RUSTFLAGS="-D rust-2024-compatibility" cargo check --locked --all-targets && cargo clippy --locked --all-targets -- -D warnings && cargo test --locked --all-targets && cargo build --locked --release --bins && ./target/release/hddl_analyzer --help && actionlint .github/workflows/ci.yml && git diff --check` | serious | arc04 | done | Passed: `cargo fmt --check`; `cargo check --locked --all-targets`; `RUSTFLAGS="-D rust-2024-compatibility" cargo check --locked --all-targets`; `cargo clippy --locked --all-targets -- -D warnings`; `cargo test --locked --all-targets`; `cargo build --locked --release --bins`; `./target/release/hddl_analyzer --help`; `actionlint .github/workflows/ci.yml`; `git diff --name-only`; `git diff --check`; `git -C ../planning diff --check`. | Release smoke showed the expected `convert`, `verify`, `metadata`, `format`, and `help` commands. |
| T4-7 | Closing report walks every row and bubbles up remaining public API deferrals. | `test -f 01-cleanup/arc04-rust-cohesion-audit/slice05-test-helper-and-private-naming-cohesion/closing-report.md` and inspect row walk. | correctness | ledger-discipline | done | `slice05-test-helper-and-private-naming-cohesion/closing-report.md` added with T4-1 through T4-7 row walk, no-public-API-change boundary, and remaining public API deferrals. | Slice05 is locally closed; CDC verification remains external. |

## What Worked

The flawed-domain integration tests had a single repeated assertion shell, so
centralizing only parse/verify result handling reduced duplication without
hiding the exact expected variant and field checks at each test site.

The naming cleanup stayed low risk by separating private/test-only typos from
public enum variant spellings that remain part of the compatibility policy.

## Closure

Slice05 is locally closed. Public enum spelling repairs for
`ParsingError::Lexiacal` and `Transformation::QuantifierElimintation` remain
deferred behind explicit operator GO or a future public API/error/AST contract
arc.
