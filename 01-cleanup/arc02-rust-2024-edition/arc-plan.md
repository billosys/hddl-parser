# Arc02: Rust 2024 Edition

Version: 1.4
Date: 2026-08-25
Expected branch: `edition/rust-2024`
Implementation base: Arc01 CI branch after the warning-fix baseline is resolved upstream

## Capability

Migrate HDDL-Parser from Rust 2021 to Rust 2024 with a focused upstream PR. The
arc uses the standard edition workflow, keeps the diff reviewable, and preserves
current parser/analyzer behavior.

This arc is intentionally narrower than the later Rust best-practices audit. The
edition PR may contain compiler-guided compatibility rewrites and directly
required compile/test fixes, but it should not absorb broad API redesign,
idiomatic cleanup, dependency churn, or behavioral parser changes.

## Dependencies

- PR #5 warning-fix baseline merged upstream or present in the local branch base.
- Arc01 CI branch merged, stacked, or otherwise available as the base so Rust
  2024 changes are protected by format, check, clippy, test, release build, and
  binary smoke gates.
- Toolchain must support Rust 2024. Rust 2024 implies rustc 1.85 or newer; record
  `rustc --version` and `cargo --version` evidence in Slice01.

## Slice Breakdown

| Slice | Goal | Status |
|-------|------|--------|
| slice01-edition-migration | Run the compiler-guided migration, bump Cargo.toml to edition 2024, and land only mechanical/required fixes. | CDC-verified |
| slice02-edition-verification-and-pr | Independently audit edition-sensitive semantic risks, rerun the full Arc01 gate set, and prepare the upstream PR notes. | CDC-verified |

## Edition Migration Rules

- Start from a clean implementation worktree on `edition/rust-2024` or an
  equivalent branch name agreed by the operator.
- Run baseline checks before changing the edition so any failure is not
  attributed to Rust 2024.
- Run `cargo fix --edition` before editing `Cargo.toml` to `edition = "2024"`.
- If the crate has target/test-specific code that the first pass does not cover,
  also run `cargo fix --edition --all-targets`.
- After the compatibility rewrites, manually set `edition = "2024"` in
  `Cargo.toml`.
- Review for Rust 2024 behavior risks called out by the Rust guide: RPIT/APIT
  lifetime capture, `if let` and tail-expression temporary scope, never-type
  fallback, explicit unsafe requirements, `gen` keyword reservation, and macro
  fragment specifier changes.
- Do not add `#![deny(warnings)]`, broad `#[allow(...)]` suppressions, unrelated
  refactors, or dependency/version churn unless required to compile on Rust
  2024.

## Arc Ledger

Definition of done: HDDL-Parser builds and tests on Rust 2024 with a focused,
reviewable upstream PR boundary and no intended planning-language behavior
change.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| A2-1 | Slice01 mechanical edition migration closes with Cargo.toml on Rust 2024 and only scoped source changes. | `test -f 01-cleanup/arc02-rust-2024-edition/slice01-edition-migration/closing-report.md` and inspect row walk. | serious | arc-plan | done | `slice01-edition-migration/cdc-verification.md` verifies rows E2-1 through E2-7, with the historical `cargo fix` ordering evidence boundary explicitly named. | |
| A2-2 | Slice02 independently reviews edition-sensitive semantics and closes. | `test -f 01-cleanup/arc02-rust-2024-edition/slice02-edition-verification-and-pr/closing-report.md` and inspect row walk. | serious | rust-guidelines | done | `slice02-edition-verification-and-pr/cdc-verification.md` verifies all Slice02 rows, including tail-expression drop-order and package-version review items. | |
| A2-3 | Full Arc01 workflow-equivalent gate set passes on the Rust 2024 branch. | `cargo fmt --check && cargo check --all-targets && cargo clippy --all-targets -- -D warnings && cargo test --all-targets && cargo build --release --bins && ./target/release/hddl_analyzer --help` | serious | arc01 | done | CDC reproduced the full gate set on `edition/rust-2024` at `d6de756`; strict `RUSTFLAGS="-D rust-2024-compatibility" cargo check --all-targets` also passed. | |
| A2-4 | PR boundary remains separate from warning-fix, CI, and best-practices audit work. | `git diff --stat <resolved-base>...HEAD` plus PR description review. | serious | issue-4 | open | | Base depends on PR #5/CI merge order. |

## Version History

### v1.4 - 2026-08-25

Slice01 CDC verification landed. Arc ledger row A2-1 is done, with the
historical `cargo fix --edition` ordering evidence boundary explicitly recorded
and reconciled by the scoped diff plus Slice02's strict compatibility gate.

### v1.3 - 2026-08-25

Slice02 CDC verification landed. Arc ledger rows A2-2 and A2-3 are done after
independent reproduction of the full gate set and the strict Rust 2024
compatibility lint gate. The pre-existing async `RwLock` guard-across-await
pattern is preserved as Arc03 audit scope, not Arc02 edition scope.

### v1.2 - 2026-08-25

Slice01 close report surfaced two concrete Slice02 obligations: independently
review the `tail_expr_drop_order` warning at
`src/language_server/request_handler.rs:118`, and verify whether the package
version bump to `0.2.0` belongs in the edition PR or should be split/reverted.

### v1.1 - 2026-08-25

Slice02 open set created while Slice01 is underway. Arc ledger rows A2-2 and
A2-3 moved from planned to open because the verification/PR-hardening slice is
now ready to execute after Slice01 closes.

### v1.0 - 2026-08-25

Arc02 promoted from placeholder to active arc. Slice01 opened for the mechanical
Rust 2024 migration; Slice02 planned for independent semantic verification and
PR hardening.

### v0.1 - 2026-08-25

Placeholder opened to preserve the project roadmap without planning too far
ahead of the CI baseline.
