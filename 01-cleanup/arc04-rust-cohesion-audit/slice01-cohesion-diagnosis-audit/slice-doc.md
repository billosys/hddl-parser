# Slice01: Cohesion Diagnosis Audit

Version: 1.0
Date: 2026-08-26
Arc: `arc04-rust-cohesion-audit`
Expected branch: `audit/rust-cohesion`
Expected base: `fix/cargo-reproducibility-policy` at `d820065 Fix Cargo reproducibility policy`

## Goal

Run a diagnosis-only, read-only audit that checks whole-codebase Rust cohesion
after Arc03's best-practices repairs. The slice should identify inconsistent
idiom choices, accepted local variations, missing behavior baselines needed
before repair, and disposition options for Arc03's RUST-007 public parser API
cohesion deferral.

This slice opens Arc04 without changing code. It creates the evidence needed to
decide whether the final cleanup work is a small public API cohesion repair, a
larger set of focused consistency PRs, or a documented no-op close.

## Scope

In scope:

- Read current project and repository guidance before auditing.
- Read Arc03 close evidence and fix map so Arc04 starts from the repaired state.
- Detect languages and tooling in the implementation worktree, ignoring
  generated/vendor/target/worktree outputs.
- Audit Rust source, binaries, tests, and integration boundaries using the
  collaboration-framework code-audit shape plus the Arc04 cohesion lens.
- Use `$rust-guidelines` as the Rust substrate, starting with the mandatory
  anti-pattern scan and then applying relevant topic guides.
- Compare like-with-like across the codebase: parser modules with parser
  modules, test helpers with test helpers, CLI/library boundaries with similar
  boundaries, and LSP async paths with LSP async paths.
- Classify inconsistent but individually valid choices when they create
  maintenance or review friction.
- Record accepted variation where a local difference has a clear reason.
- Re-enter RUST-007 and propose Slice02 disposition options.
- Create new workbench reports named:
  - `workbench/<DATE>-cohesion-audit-index.md`
  - `workbench/<DATE>-cohesion-audit-results-rust.md`
- Update this slice's ledger and closing report.

Out of scope:

- Editing Rust source, tests, Cargo files, workflows, README, or behavior.
- Adding characterization tests.
- Fixing any audit finding.
- Closing the project or opening an upstream PR.

## Verification

Slice01 verification must include:

```bash
date +%Y.%m.%d
test -f workbench/<DATE>-cohesion-audit-index.md
test -f workbench/<DATE>-cohesion-audit-results-rust.md
rg -n "cohesion|consistency|consistent|accepted variation|intentional divergence|RUST-007" workbench/<DATE>-cohesion-audit-*.md
cargo fmt --check
cargo check --locked --all-targets
RUSTFLAGS="-D rust-2024-compatibility" cargo check --locked --all-targets
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked --all-targets
cargo build --locked --release --bins
./target/release/hddl_analyzer --help
actionlint .github/workflows/ci.yml
git diff --check
git -C ../planning diff --check
```

Negative diff-boundary verification must include:

```bash
git diff --name-only
git -C ../planning diff --name-only
```

Expected implementation diff boundary: only the two new cohesion workbench
reports.

Expected planning diff boundary: Slice01 ledger and closing report only after CC
executes the slice. This open-set commit may also include Arc04 plan and project
plan updates.

## Exit Criteria

- The cohesion audit index and Rust audit results exist with non-overwriting
  filenames.
- Findings, if any, are grouped by consistency theme and include evidence,
  consequence, recommended action, and behavior-preservation baseline need.
- At least five negative cohesion checks are recorded.
- RUST-007 public parser API cohesion is explicitly re-entered.
- Any accepted variation is documented with a local reason.
- The full quality gate passes against the audited codebase.
- No implementation source, test, manifest, workflow, README, or behavior changes
  are made.
- Slice02 receives concrete planning inputs: repair themes, no-op decisions,
  deferrals, and any required characterization-test needs.
