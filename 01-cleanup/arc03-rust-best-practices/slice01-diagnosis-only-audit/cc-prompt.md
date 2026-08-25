# CC Prompt: Arc03 Slice01 Diagnosis-Only Rust Audit

You are working in HDDL-Parser on Arc03 Slice01:
`arc03-rust-best-practices/slice01-diagnosis-only-audit`.

Read these planning files first:

- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/project-plan.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc03-rust-best-practices/arc-plan.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc03-rust-best-practices/slice01-diagnosis-only-audit/slice-doc.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc03-rust-best-practices/slice01-diagnosis-only-audit/ledger.md`

Use the collaboration-framework code-audit discipline and rust-guidelines
substrate. This slice is diagnosis-only:

- Do create `workbench/<DATE>-audit-index.md`.
- Do create `workbench/<DATE>-audit-results-rust.md`.
- Do update this slice ledger and write the closing report when done.
- Do not edit Rust source, tests, manifests, workflows, README, or behavior.
- Do not add characterization tests in this slice; list the missing tests for
  Slice02 instead.
- Do not fix findings in this slice.

Required audit setup:

1. Run `date +%Y.%m.%d` and use that exact value as `<DATE>`.
2. Read `README.md`, root `CLAUDE.md`/`AGENTS.md` if present, and any current
   architecture/design documents they reference.
3. Detect languages/tools by manifests/configs and extensions while ignoring
   generated/build/worktree trees such as `target/` and `.worktrees/`.
4. Audit Rust with rust-guidelines, starting from `11-anti-patterns.md`, then
   loading topic-specific guides as the code requires.
5. Explicitly review the Arc02 async `RwLock` candidate in the language-server
   diagnostic path.
6. Identify missing characterization tests needed for Slice02, or explicitly
   say none are missing and why.

Run and record:

```bash
cargo fmt --check
cargo check --all-targets
RUSTFLAGS="-D rust-2024-compatibility" cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo build --release --bins
./target/release/hddl_analyzer --help
actionlint .github/workflows/ci.yml
git diff --check
```

Closing report requirements:

- Walk every ledger row `R3-1` through `R3-12`; no silent drops.
- Record exact files created under `workbench/`.
- State the implementation diff boundary and confirm no source/test/manifest/
  workflow/README edits were made.
- Bubble up any Arc03 plan changes required before Slice02 starts.
