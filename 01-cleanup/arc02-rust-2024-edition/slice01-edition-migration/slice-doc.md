# Slice01: Edition Migration

Date: 2026-08-25
Branch: `edition/rust-2024`
Arc: `arc02-rust-2024-edition`

## Goal

Perform the compiler-guided Rust 2024 migration for HDDL-Parser while keeping
the patch focused on edition compatibility. This slice should produce a branch
that builds on Rust 2024 and is ready for independent semantic verification in
Slice02.

## In Scope

- Confirm the implementation branch/base is clean and appropriate for a Rust
  2024 PR.
- Record `rustc --version` and `cargo --version`.
- Run baseline checks before edition edits.
- Run `cargo fix --edition`, and `cargo fix --edition --all-targets` if needed
  for tests/bin coverage.
- Set `edition = "2024"` in `Cargo.toml` after the fix pass.
- Apply only compile-required or test-required follow-up edits.
- Run the local post-migration checks needed to prove Slice01 is ready for
  Slice02.

## Out Of Scope

- Broad Rust best-practices refactors.
- Dependency upgrades unless Cargo or rustc proves they are required for the
  edition migration.
- CI workflow changes beyond what is strictly required for the Rust 2024 branch.
- Parser, analyzer, HDDL/PDDL semantics, output format, or CLI behavior changes.
- README/release automation updates unless the edition migration introduces a
  concrete MSRV note that must be documented in this PR.

## Verification Approach

Run a before/after evidence chain. Baseline commands should run before the
edition migration so regressions can be attributed correctly. After the migration,
run enough of the Arc01 command set to show the branch is healthy; Slice02 will
repeat the full gate set and perform the semantic review.

Rust 2024 review notes to preserve for Slice02:

- RPIT/APIT lifetime capture and any inserted `use<...>` bounds.
- `if let` and tail-expression temporary drop-order changes.
- Never-type fallback warnings requiring explicit type annotations.
- New explicit-unsafe requirements.
- `gen` keyword reservation and macro fragment specifier changes.

## Exit Criteria

- `Cargo.toml` declares `edition = "2024"`.
- `cargo fix --edition` evidence is recorded, including any no-op result.
- `cargo check --all-targets` passes after the edition bump.
- `cargo fmt --check` passes after any formatting updates.
- The implementation diff is limited to edition migration changes and directly
  required follow-up fixes.
- `ledger.md` is updated with evidence and a `closing-report.md` is written.
