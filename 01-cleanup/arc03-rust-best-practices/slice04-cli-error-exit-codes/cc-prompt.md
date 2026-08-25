# CC Prompt: Arc03 Slice04 CLI Error Exit Codes

You are working in HDDL-Parser on Arc03 Slice04:
`arc03-rust-best-practices/slice04-cli-error-exit-codes`.

This is a focused repair slice for RUST-001 only.

Read first:

- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/project-plan.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc03-rust-best-practices/arc-plan.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc03-rust-best-practices/slice03-triage-and-fix-map/fix-map.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc03-rust-best-practices/slice04-cli-error-exit-codes/slice-doc.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc03-rust-best-practices/slice04-cli-error-exit-codes/ledger.md`
- `workbench/2026.08.25-audit-results-rust.md`
- `tests/current_behavior.rs`

Goal:

Make recoverable `hddl_analyzer` CLI failures exit non-zero while preserving successful command behavior and output routing.

Constraints:

- Keep this slice narrowly scoped to RUST-001.
- Do not fix parser/transpiler panic paths, LSP behavior, Cargo policy, or public API cleanup.
- Update only the CLI portions of `tests/current_behavior.rs`.
- Preserve user-facing stderr diagnostics and stdout success/data behavior.

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

Also rerun the missing-input and unsupported-extension runtime probes and show their new non-zero exit codes.

Closing report requirements:

- Walk C4-1 through C4-9.
- State the final CLI process contract.
- State whether RUST-002/RUST-003 panic probes were unchanged or affected only at the process wrapper boundary.
