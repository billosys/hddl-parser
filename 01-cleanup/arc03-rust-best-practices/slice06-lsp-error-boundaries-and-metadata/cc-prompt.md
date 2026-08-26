# CC Prompt: Arc03 Slice06 LSP Error Boundaries And Metadata

You are working in HDDL-Parser on Arc03 Slice06:
`arc03-rust-best-practices/slice06-lsp-error-boundaries-and-metadata`.

This is a focused repair slice for RUST-005 and RUST-008.

Start from the verified Slice05 base,
`fix/structured-parser-transform-errors` at `6bd1b0a`, unless Slice05 has
already been merged upstream. Create `fix/lsp-error-boundaries-and-metadata`
from that base.

Read first:

- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/project-plan.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc03-rust-best-practices/arc-plan.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc03-rust-best-practices/slice03-triage-and-fix-map/fix-map.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc03-rust-best-practices/slice06-lsp-error-boundaries-and-metadata/slice-doc.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc03-rust-best-practices/slice06-lsp-error-boundaries-and-metadata/ledger.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc03-rust-best-practices/slice02-baseline-characterization-tests/cdc-verification.md`
- `tests/lsp_current_behavior.rs`
- `workbench/2026.08.25-audit-results-rust.md`

Goal:

Make ordinary LSP request/runtime failures non-panicking and update initialize metadata to the package version.

Constraints:

- Keep RUST-004 diagnostic lock-scope repair separate unless you have a concrete reason to promote it.
- Do not change CLI/parser/transform behavior, Cargo policy, or public API cohesion.
- Do not make private internals public only for tests.
- Use the existing stdio LSP harness where possible.
- Treat Slice04/Slice05 behavior as settled baseline; do not include those
  files in the Slice06 diff unless an LSP-specific change truly requires it.

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
git diff --name-only 6bd1b0a..HEAD
```

Closing report requirements:

- Walk C6-1 through C6-10.
- State the final behavior for initialize version, unsynced diagnostics, non-file URI diagnostics, `didSave`, sibling-domain discovery, and no-domain-found diagnostics.
- State any remaining harness limitations with precise re-entry conditions.
