# Slice01: Diagnosis-Only Rust Audit

Version: 1.0
Date: 2026-08-25
Arc: `arc03-rust-best-practices`
Expected branch: `audit/rust-best-practices`

## Goal

Produce a diagnosis-only Rust audit of HDDL-Parser using the
collaboration-framework code-audit discipline and the rust-guidelines substrate.
The audit must identify concrete findings, missing behavior baselines for
Slice02, and any issues that should be routed to Arc04's later cohesion pass.

## Scope

In scope:

- Read repository context from `README.md`, root assistant guidance files if
  present, and any current architecture/design documents referenced there.
- Detect project languages and tools while ignoring generated, vendored, and
  worktree output.
- Audit Rust source, binaries, test modules, and integration tests.
- Use rust-guidelines, starting with `11-anti-patterns.md`, then loading
  topic-specific guides for CLI, errors, async/concurrency, Cargo/lints,
  project structure, API design, documentation, and tests as the code requires.
- Review the Arc02 follow-up candidate around async `RwLock` read guards held
  across awaits in the language-server diagnostics path.
- Create `workbench/<DATE>-audit-index.md` and
  `workbench/<DATE>-audit-results-rust.md` in the implementation worktree.
- Update this slice's ledger and closing report when complete.

Out of scope:

- Any Rust source, test, manifest, workflow, README, or behavior changes.
- Adding characterization tests; that is Slice02.
- Fixing audit findings; those are Slice04+ after Slice03 opens focused repair
  slices.
- Final whole-codebase idiom unification; that is Arc04 unless the audit finds
  a concrete correctness or maintainability issue that belongs in Arc03.

## Verification Approach

This slice verifies the quality and boundaries of the audit, not a code change.
CDC should inspect the generated workbench reports, run the baseline quality
gate against the audited branch state, and confirm that the implementation
diff contains only audit reports plus planning close artifacts.

Expected verification commands:

```bash
date +%Y.%m.%d
test -f workbench/<DATE>-audit-index.md
test -f workbench/<DATE>-audit-results-rust.md
rg -n "RwLock|await|characterization|Things I looked for and did not find" workbench/<DATE>-audit-results-rust.md
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

CDC should also inspect the implementation diff and confirm that any changed
files outside `workbench/` are limited to planning close artifacts in the
planning worktree.

## Exit Criteria

- The audit index and Rust audit report exist under `workbench/` using the
  captured date prefix.
- Findings use the required severity/location/what/why/fix shape and cite
  concrete file lines.
- The report includes at least five negative checks.
- Missing characterization tests needed for Slice02 are identified, or the
  report explicitly states that no baseline tests are missing and why.
- The full local workflow-equivalent gate passes on the audited code state.
- No Rust source, test, manifest, workflow, or README edits are made by this
  slice.
