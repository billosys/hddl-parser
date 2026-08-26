# Slice03 Closing Report: Characterization Baselines

Date: 2026-08-25
Planning branch: `planning`
Implementation branch: `test/arc04-characterization-baselines`
Implementation commit: `e904099 Add Arc04 characterization baselines`

## Summary

Slice03 added one test-only characterization file:

- `tests/arc04_characterization.rs`

The tests pin current public API and behavior surfaces before Arc04 production
repairs begin:

- Vec-backed `HDDLProgram::from_hddl`, `Transpiler::from_hddl`, and
  `Input::Hddl`.
- Representative crate-root imports.
- The current malformed problem-parser panic boundary.
- Current `ParsingError::Transformation(String)` messages.
- Current public misspelled variants.
- Current formula normalization panic contracts.

No production source, Cargo, workflow, README, or documentation files changed
in the feature commit.

## Ledger Walk

- B4-1: Done. The branch is `test/arc04-characterization-baselines`, created
  from the Arc04 feature baseline at `d820065`.
- B4-2: Done. `git show --name-only --pretty=format: e904099` and
  `git diff --name-only d820065..HEAD` report only
  `tests/arc04_characterization.rs`.
- B4-3: Done. Vec-backed parser/transpiler behavior is pinned through
  `HDDLProgram::from_hddl`, `Transpiler::from_hddl`, and `Input::Hddl` tests.
- B4-4: Done. Representative crate-root exports are imported from
  `hddl_analyzer` in the integration test.
- B4-5: Done. The malformed problem top-level unexpected-token panic is
  characterized with `catch_unwind`.
- B4-6: Done. Current transformation/classification error behavior is pinned
  by matching `ParsingError::Transformation(String)` and exact messages.
- B4-7: Done. Current public misspelled variants `ParsingError::Lexiacal` and
  `Transformation::QuantifierElimintation` are constructed or matched.
- B4-8: Done. Formula normalization panic contracts are pinned for equality,
  non-NNF negated equality, and probabilistic formulae.
- B4-9: Done. The full local gate passed.
- B4-10: Done. This report walks all rows and bubbles up downstream readiness.

## Verification

Executed from `.worktrees/features`:

- `cargo fmt --check` passed.
- `cargo check --locked --all-targets` passed.
- `RUSTFLAGS="-D rust-2024-compatibility" cargo check --locked --all-targets`
  passed.
- `cargo clippy --locked --all-targets -- -D warnings` passed.
- `cargo test --locked --test arc04_characterization` passed with 9 tests.
- `cargo test --locked --all-targets` passed with the existing ignored tests
  unchanged.
- `cargo build --locked --release --bins` passed.
- `./target/release/hddl_analyzer --help` passed.
- `actionlint .github/workflows/ci.yml` passed.
- `git diff --check d820065..HEAD` passed.
- `git show --name-only --pretty=format: e904099` reported only
  `tests/arc04_characterization.rs`.

## Bubble-Up To Arc04

Slice03 delivers the baseline layer that Slice04 and Slice05 need. Slice04 is
ready to start the parser API/error-boundary repair against a known baseline.
Slice05 remains sequenced after Slice04 because it is a cleanup slice that
should preserve both Slice03's public behavior baselines and Slice04's repair
outcomes.

No arc-plan scope change is required. The existing public API gates remain in
force: export narrowing, public error taxonomy changes, public formula API
changes, and public enum spelling repairs require explicit operator GO or a
future public API/error/AST contract arc.
