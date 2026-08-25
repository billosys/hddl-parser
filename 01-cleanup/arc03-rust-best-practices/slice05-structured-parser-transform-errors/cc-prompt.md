# CC Prompt: Arc03 Slice05 Structured Parser And Transform Errors

You are working in HDDL-Parser on Arc03 Slice05:
`arc03-rust-best-practices/slice05-structured-parser-transform-errors`.

This is a focused repair slice for RUST-002 and RUST-003.

Read first:

- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/project-plan.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc03-rust-best-practices/arc-plan.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc03-rust-best-practices/slice03-triage-and-fix-map/fix-map.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc03-rust-best-practices/slice05-structured-parser-transform-errors/slice-doc.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc03-rust-best-practices/slice05-structured-parser-transform-errors/ledger.md`
- `workbench/2026.08.25-audit-results-rust.md`
- `tests/current_behavior.rs`

Goal:

Replace recoverable parser/transpiler/transform panics with structured errors.

Constraints:

- Keep the slice narrowly scoped to RUST-002 and RUST-003.
- Do not change LSP behavior, dependency policy, or public API re-export design.
- Prefer the existing `ParsingError` channel unless the codebase clearly needs a narrower new error type.
- Update the current `catch_unwind` tests into ordinary `Err` assertions.

Run and record:

```bash
cargo fmt --check
cargo check --all-targets
RUSTFLAGS="-D rust-2024-compatibility" cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test --test current_behavior
cargo test --all-targets
cargo build --release --bins
./target/release/hddl_analyzer --help
actionlint .github/workflows/ci.yml
git diff --check
git diff --name-only
```

Also rerun the two audit panic probes and confirm they no longer exit via panic code `101`.

Closing report requirements:

- Walk C5-1 through C5-9.
- State the final public error behavior for `HDDLProgram::from_hddl`, `Transpiler::from_hddl`, and `RemoveEqualityConstraints`.
- Note any CLI behavior dependency on Slice04.
