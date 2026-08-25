# Slice02: Edition Verification And PR Hardening

Date: 2026-08-25
Branch: `edition/rust-2024`
Arc: `arc02-rust-2024-edition`
Depends on: `slice01-edition-migration` closing evidence

## Goal

Independently verify the Rust 2024 migration produced by Slice01, audit the
edition-sensitive semantic risks, rerun the full Arc01 workflow-equivalent gate
set, and prepare the branch for an upstream PR.

This slice should avoid new migration work unless verification finds a concrete
blocking issue. Its main job is to prove that the edition migration is scoped,
behavior-preserving, and reviewable.

## In Scope

- Read Slice01 `ledger.md` and `closing-report.md` before touching code.
- Confirm `Cargo.toml` declares `edition = "2024"` and the Rust toolchain is
  compatible.
- Rerun the full Arc01 workflow-equivalent gate set on the Rust 2024 branch.
- Run or inspect Rust 2024 compatibility evidence, including any Slice01 notes
  about lints or manual fixes.
- Audit edition-sensitive source sites called out by the Rust guide:
  RPIT/APIT lifetime capture, `if let` and tail-expression temporary scope,
  never-type fallback, explicit unsafe requirements, `gen`, macro fragments,
  prelude additions, `Box<[T]>::into_iter()`, and environment variable writes.
- Independently review the Slice01-surfaced `tail_expr_drop_order` warning at
  `src/language_server/request_handler.rs:118`.
- Verify whether the package version bump from `0.1.0` to `0.2.0` belongs in
  the edition PR because it was operator-requested, or should be split/reverted
  to keep the PR purely about Rust 2024.
- Confirm the diff stays limited to edition migration and directly required
  follow-up fixes, plus any intentionally retained operator-requested version
  metadata.
- Prepare PR-ready notes: summary, verification commands, migration method,
  scope boundary, and any semantic-risk findings.

## Out Of Scope

- Running `cargo fix --edition` as routine new work. That belongs to Slice01;
  rerun it only if Slice01 evidence shows a missed target that blocks closure.
- Broad Rust best-practices cleanup, API redesign, dependency modernization, or
  CLI UX changes.
- CI workflow redesign, release automation, or README polish unless needed to
  make the Rust 2024 PR honest.
- Changes to parser/analyzer semantics or HDDL/PDDL output behavior.

## Verification Approach

Start with evidence review, then reproduce the full gate set locally. Treat the
Rust 2024 semantic checklist as a manual audit, not just a grep exercise: grep
identifies candidate sites, but the row closes only after the candidate set is
reviewed and the result is recorded.

The full local gate set is:

```bash
cargo fmt --check
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo build --release --bins
./target/release/hddl_analyzer --help
```

If `actionlint` is available and the branch includes CI changes from its base,
run `actionlint .github/workflows/ci.yml` as an additional confidence check.

## Exit Criteria

- Slice01 close evidence has been reviewed and any blockers are resolved or
  explicitly deferred with rationale.
- The full Arc01 workflow-equivalent gate set passes on the Rust 2024 branch.
- Rust 2024 semantic-risk candidates are searched, reviewed, and recorded.
- The `request_handler.rs:118` tail-expression drop-order warning is resolved by
  either reproduced safety rationale or a smallest-necessary code fix.
- The `Cargo.toml` version bump is explicitly kept with rationale or removed from
  the edition PR.
- The PR boundary is confirmed separate from warning-fix, CI, and Arc03 audit
  work.
- PR-ready notes are present in the closing report.
- `ledger.md` is updated with evidence and `closing-report.md` is written.
