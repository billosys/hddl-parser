# CC Prompt: Arc04 Slice03 Characterization Baselines

You are working in HDDL-Parser on Arc04 Slice03:
`arc04-rust-cohesion-audit/slice03-characterization-baselines`.

This is a test-only characterization slice. Do not edit Rust production source,
Cargo files, workflows, README/docs, or runtime behavior.

## Required Reading

Read these first:

- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/project-plan.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc04-rust-cohesion-audit/arc-plan.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc04-rust-cohesion-audit/slice02-triage-and-fix-map/fix-map.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc04-rust-cohesion-audit/slice03-characterization-baselines/slice-doc.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc04-rust-cohesion-audit/slice03-characterization-baselines/ledger.md`
- `workbench/2026.08.25-cohesion-audit-results-rust.md`

Also apply `$collaboration-framework` ledger discipline and `$rust-guidelines`
test/API/error judgment.

## Goal

Add passing tests that pin current behavior before Arc04 production repairs.

## Required Work

- Create a new branch from the current Arc04 feature baseline:
  `test/arc04-characterization-baselines`.
- Add tests only.
- Characterize current Vec-backed parser/transpiler APIs:
  `HDDLProgram::from_hddl`, `Transpiler::from_hddl`, and `Input::Hddl`.
- Characterize representative public imports reachable from the crate root.
- Characterize current malformed problem-parser panic behavior with an explicit
  panic assertion.
- Characterize current `ParsingError::Transformation(String)` behavior for
  domain/problem kind mismatch and transform precondition errors.
- Characterize current public misspelled variants:
  `ParsingError::Lexiacal` and `Transformation::QuantifierElimintation`.
- Characterize formula normalization panic contracts for equality, non-NNF, and
  probabilistic inputs.

Do not add `&[u8]` caller tests that fail against the current implementation.
Those belong in Slice04 as red-to-green repair evidence.

## Verification Commands

Run and record:

```bash
cargo fmt --check
cargo check --locked --all-targets
RUSTFLAGS="-D rust-2024-compatibility" cargo check --locked --all-targets
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked --all-targets
cargo build --locked --release --bins
./target/release/hddl_analyzer --help
actionlint .github/workflows/ci.yml
git diff --name-only
git diff --check
git -C ../planning diff --check
```

## Closing

Update the Slice03 ledger and add `closing-report.md`. The closing report must
walk rows B4-1 through B4-10, prove the test-only boundary, and state whether
Slice04 is ready to start.
