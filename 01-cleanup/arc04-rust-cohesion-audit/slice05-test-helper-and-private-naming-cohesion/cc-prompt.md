# CC Prompt: Arc04 Slice05 Test Helper And Private Naming Cohesion

You are working in HDDL-Parser on Arc04 Slice05:
`arc04-rust-cohesion-audit/slice05-test-helper-and-private-naming-cohesion`.

This is a focused cleanup slice for tests and private naming. It depends on
Slice03 characterization baselines and the Slice04 parser repair being closed or
explicitly deferred.

## Required Reading

Read these first:

- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/project-plan.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc04-rust-cohesion-audit/arc-plan.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc04-rust-cohesion-audit/slice02-triage-and-fix-map/fix-map.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc04-rust-cohesion-audit/slice03-characterization-baselines/cdc-verification.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc04-rust-cohesion-audit/slice04-parser-api-and-error-boundary/cdc-verification.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc04-rust-cohesion-audit/slice05-test-helper-and-private-naming-cohesion/slice-doc.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc04-rust-cohesion-audit/slice05-test-helper-and-private-naming-cohesion/ledger.md`

Also apply `$rust-guidelines` test/API judgment and `$collaboration-framework`
ledger discipline.

## Goal

Improve test helper consistency and private/test naming without changing public
API or product behavior.

## Required Work

- Create a focused branch: `fix/test-private-naming-cohesion`.
- Consolidate repeated test assertions only where the helper preserves the same
  variant, line, symbol, or message checks.
- Keep fixture coverage intact.
- Preserve existing ignored long-running or known-fix tests unless the operator
  explicitly asks otherwise.
- Rename private implementation modules with spelling drift where scoped.
- Rename test functions with spelling drift where scoped.

Do not rename public enum variants such as `ParsingError::Lexiacal` or
`Transformation::QuantifierElimintation` without explicit operator GO.

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

Update the Slice05 ledger and add `closing-report.md`. The closing report must
walk rows T4-1 through T4-7, prove the no-public-API-change boundary, and list
remaining public API deferrals.
