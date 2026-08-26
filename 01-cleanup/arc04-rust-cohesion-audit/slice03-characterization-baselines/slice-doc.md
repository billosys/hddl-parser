# Slice03: Characterization Baselines

Version: 1.0
Date: 2026-08-25
Arc: `arc04-rust-cohesion-audit`
Expected branch: `test/arc04-characterization-baselines`

## Goal

Add test-only characterization coverage for the current public API and behavior
that Arc04 repairs may change. This slice creates the safety net for parser API
repair, parser error-boundary repair, public error taxonomy decisions, formula
normalization policy, and public spelling compatibility.

## Scope

In scope:

- Add Rust tests only.
- Pin current Vec-backed `HDDLProgram::from_hddl`, `Transpiler::from_hddl`, and
  `Input::Hddl` behavior.
- Pin representative current public imports that exist through crate-root
  exports.
- Pin the current malformed problem-parser panic boundary with `catch_unwind`
  or an equivalent explicit panic assertion.
- Pin current `ParsingError::Transformation(String)` behavior for
  domain/problem kind mismatch and transform precondition errors.
- Pin current `ParsingError::Lexiacal` and
  `Transformation::QuantifierElimintation` public variant availability.
- Pin current formula normalization panic contracts for equality, non-NNF, and
  probabilistic formula inputs.
- Preserve existing ignored long-running or known-fix tests.

Out of scope:

- Rust production source changes.
- Cargo, workflow, README, or docs changes.
- Adding new dependencies unless the operator explicitly approves them.
- Making `&[u8]` caller tests pass before production signatures change.
- Renaming public API items.
- Fixing parser panic behavior.

## Verification Approach

Run the full local gate after adding tests:

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

## Exit Criteria

- Tests characterize the current public API and behavior surfaces listed above.
- The slice is test-only in the feature worktree.
- No production behavior changes are made.
- Full local gate passes.
