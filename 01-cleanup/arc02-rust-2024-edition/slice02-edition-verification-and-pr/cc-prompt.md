# CC Prompt: Slice02 Edition Verification And PR Hardening

You are working on HDDL-Parser Arc02, Slice02: Rust 2024 edition verification
and PR hardening.

Planning files live in:

`/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup`

Read before editing or running verification:

- `01-cleanup/project-plan.md`
- `01-cleanup/arc02-rust-2024-edition/arc-plan.md`
- `01-cleanup/arc02-rust-2024-edition/slice01-edition-migration/ledger.md`
- `01-cleanup/arc02-rust-2024-edition/slice01-edition-migration/closing-report.md`
- `01-cleanup/arc02-rust-2024-edition/slice02-edition-verification-and-pr/slice-doc.md`
- `01-cleanup/arc02-rust-2024-edition/slice02-edition-verification-and-pr/ledger.md`

Do not start until Slice01 has close evidence. This slice verifies and hardens
the migration; it should not become a broad cleanup pass.

Work the ledger in order:

1. Confirm Slice01 closed and read its evidence. If Slice01 did not close, stop
   and record the blocker instead of guessing.
2. Confirm manifest/toolchain state:
   - `rg -n 'edition = "2024"|rust-version' Cargo.toml`
   - `rustc --version`
   - `cargo --version`
3. Run the full Arc01 workflow-equivalent gate set:
   - `cargo fmt --check`
   - `cargo check --all-targets`
   - `cargo clippy --all-targets -- -D warnings`
   - `cargo test --all-targets`
   - `cargo build --release --bins`
   - `./target/release/hddl_analyzer --help`
4. Check Rust 2024 compatibility evidence:
   - `RUSTFLAGS="-W rust-2024-compatibility" cargo check --all-targets`
   - If this is redundant after the edition bump, record the output or no-op
     rationale explicitly.
5. Search and manually review edition-sensitive source candidates:
   - `rg -n "impl Trait|if let|while let|RefCell|RwLock|Mutex|static mut|unsafe|macro_rules!|\bgen\b|into_iter\(|set_var|remove_var" src tests Cargo.toml`
   - Review RPIT/APIT lifetime capture, temporary drop order, never-type
     fallback, explicit unsafe requirements, `gen`, macro fragments, prelude
     additions, `Box<[T]>::into_iter()`, and environment variable writes.
6. Independently review the Slice01-surfaced `tail_expr_drop_order` warning at
   `src/language_server/request_handler.rs:118`. Record why Rust 2024 drop
   order is behavior-preserving there, or make the smallest necessary fix and
   rerun the affected gates.
7. Review the Slice01 package version bump from `0.1.0` to `0.2.0`. Slice01
   says this was operator-requested; verify whether it belongs in this upstream
   edition PR. Either keep it with explicit rationale in the closing report and
   PR notes, or remove/split it to keep the PR purely about Rust 2024.
8. Confirm PR boundary:
   - `git diff --stat <resolved-base>...HEAD`
   - `git diff --name-only <resolved-base>...HEAD`
   - Keep warning fixes, CI work, and Arc03 best-practices refactors out of
     this PR unless they are already in the agreed base.

If verification finds a concrete Rust 2024 blocker, make the smallest necessary
fix and rerun the affected gate plus the full gate set when done. Do not add
broad `#[allow(...)]` suppressions or refactor unrelated code.

Before closing:

- Update `ledger.md` with command evidence.
- Write `closing-report.md` with a per-row walk and a Bubble-up to Arc02.
- Include a `PR Notes` section that has:
  - Summary
  - Migration method
  - Verification
  - Semantic Review, including the `request_handler.rs:118` drop-order decision
  - Version bump decision
  - Scope boundary / base caveat
