# CC Prompt: Arc04 Slice04 Parser API And Error Boundary

You are working in HDDL-Parser on Arc04 Slice04:
`arc04-rust-cohesion-audit/slice04-parser-api-and-error-boundary`.

This is a focused production repair slice. It depends on Slice03
characterization tests being closed and CDC-verified.

## Required Reading

Read these first:

- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/project-plan.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc04-rust-cohesion-audit/arc-plan.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc04-rust-cohesion-audit/slice02-triage-and-fix-map/fix-map.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc04-rust-cohesion-audit/slice03-characterization-baselines/cdc-verification.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc04-rust-cohesion-audit/slice04-parser-api-and-error-boundary/slice-doc.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc04-rust-cohesion-audit/slice04-parser-api-and-error-boundary/ledger.md`

Also apply `$rust-guidelines` API/error guidance and `$collaboration-framework`
ledger discipline.

## Goal

Repair the parser API and parser recoverability issues from Arc04 without
silently approving broader public API breaks.

## Required Work

- Create a focused branch: `fix/parser-api-error-boundary`.
- Change borrowed byte-input APIs from `&Vec<u8>` to `&[u8]` where no
  Vec-specific operations are used.
- Preserve existing Vec-backed callers through deref coercion.
- Add or update tests proving `&[u8]` callers work for public parser/transpiler
  entry points.
- Replace the malformed problem-parser panic path with a structured
  `ParsingError::Syntactic`.
- Consider a small parser-local helper for repeated syntactic error construction
  only if it keeps the diff narrower and clearer.

Stop and ask the operator before removing, narrowing, or renaming public
crate-root exports. That part of COHESION-001 is not pre-approved.

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

Update the Slice04 ledger and add `closing-report.md`. The closing report must
walk rows R4-1 through R4-9, prove the focused diff boundary, and state whether
public export narrowing was skipped, gated, or explicitly approved.
