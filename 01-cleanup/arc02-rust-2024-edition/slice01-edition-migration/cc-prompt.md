# CC Prompt: Slice01 Edition Migration

You are working on HDDL-Parser Arc02, Slice01: Rust 2024 edition migration.

Planning files live in:

`/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup`

Read before editing:

- `01-cleanup/project-plan.md`
- `01-cleanup/arc02-rust-2024-edition/arc-plan.md`
- `01-cleanup/arc02-rust-2024-edition/slice01-edition-migration/slice-doc.md`
- `01-cleanup/arc02-rust-2024-edition/slice01-edition-migration/ledger.md`

Implementation guidance:

- Use branch `edition/rust-2024` unless the operator has already created an
  equivalent branch.
- Base it on the resolved CI baseline: either the Arc01 CI branch if this is a
  stacked PR, or updated `main` after the warning-fix and CI PRs land.
- Start from a clean implementation worktree. Record `git status --short` and
  recent commit history in the ledger.
- Record `rustc --version` and `cargo --version`. Rust 2024 requires rustc 1.85
  or newer.
- Run baseline checks before edition edits:
  - `cargo fmt --check`
  - `cargo check --all-targets`
- Run the edition migration before changing `Cargo.toml`:
  - `cargo fix --edition`
  - If needed for target/test coverage: `cargo fix --edition --all-targets`
- Then update `Cargo.toml` to `edition = "2024"`.
- Apply only compile-required or test-required follow-up fixes.

Keep out of this slice:

- Broad Rust best-practices cleanup.
- Dependency/version churn unless required by the edition migration.
- CI workflow changes unless required for Rust 2024 compatibility.
- Parser/analyzer/CLI behavior changes.

Before closing, run at minimum:

- `cargo fmt --check`
- `cargo check --all-targets`

Prefer also running the full Arc01 suite if it is quick enough:

- `cargo clippy --all-targets -- -D warnings`
- `cargo test --all-targets`
- `cargo build --release --bins`
- `./target/release/hddl_analyzer --help`

Update `ledger.md` with command evidence and write `closing-report.md` with a
per-row walk plus a Bubble-up to Arc02. Preserve notes for Slice02 about any
Rust 2024 semantic-sensitive sites, especially RPIT/APIT lifetime capture,
temporary drop order, never-type fallback, explicit unsafe changes, and macro or
`gen` keyword rewrites.
