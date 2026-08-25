# CC Prompt: Arc03 Slice07 LSP Diagnostic Lock Scope

You are working in HDDL-Parser on Arc03 Slice07:
`arc03-rust-best-practices/slice07-lsp-diagnostic-lock-scope`.

This is a focused repair slice for RUST-004.

Read first:

- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/project-plan.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc03-rust-best-practices/arc-plan.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc03-rust-best-practices/slice03-triage-and-fix-map/fix-map.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc03-rust-best-practices/slice07-lsp-diagnostic-lock-scope/slice-doc.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc03-rust-best-practices/slice07-lsp-diagnostic-lock-scope/ledger.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc03-rust-best-practices/slice02-baseline-characterization-tests/cdc-verification.md`
- `src/language_server/request_handler.rs`
- `tests/lsp_current_behavior.rs`

Goal:

Drop the diagnostic document-map read guard before any awaited work.

Constraints:

- Keep this slice narrowly scoped to RUST-004.
- Do not add timing-fragile tests.
- Do not make private internals public only for tests.
- If a deterministic runtime contention test is not feasible, record source-level evidence and a precise re-entry condition.

Run and record:

```bash
cargo fmt --check
cargo check --all-targets
RUSTFLAGS="-D rust-2024-compatibility" cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test --test lsp_current_behavior
cargo test --all-targets
cargo build --release --bins
./target/release/hddl_analyzer --help
actionlint .github/workflows/ci.yml
git diff --check
git diff --name-only
rg -n "documents\\.read\\(\\)\\.await|get\\(|\\.await" src/language_server/request_handler.rs
```

Closing report requirements:

- Walk C7-1 through C7-8.
- State whether the proof is runtime-test based, source-inspection based, or both.
- If runtime contention coverage is deferred, state the exact re-entry condition.
