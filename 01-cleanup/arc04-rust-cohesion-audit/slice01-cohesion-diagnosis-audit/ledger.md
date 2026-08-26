# Slice01: Cohesion Diagnosis Audit Ledger

Definition of done: Slice01 performs a diagnosis-only, read-only audit of
project-wide Rust cohesion from the Arc03-repaired codebase, producing workbench
evidence and precise Slice02 handoff items without changing behavior.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| C4-1 | Runtime date is captured and cohesion audit filenames are chosen so Arc03 workbench reports are not overwritten. | `date +%Y.%m.%d` and `test -f workbench/<DATE>-cohesion-audit-index.md` and `test -f workbench/<DATE>-cohesion-audit-results-rust.md` | correctness | project-management | open | | |
| C4-2 | Required repo and planning context are read: README/root guidance, Arc03 closing report, Arc03 fix map, Arc04 plan, and this open set. | Closing report lists the files read and why they mattered. | serious | collaboration-framework | open | | |
| C4-3 | Language/tooling detection covers Rust source, binaries, tests, workflow, and Cargo policy while ignoring generated/vendor/target/worktree outputs. | Audit index records detected languages/tools and exclusions. | correctness | code-audit | open | | |
| C4-4 | Rust-guidelines substrate is loaded and the mandatory anti-pattern scan plus relevant topic guides are named in audit evidence. | `rg -n "rust-guidelines|anti-pattern|error|API|async|test|Cargo" workbench/<DATE>-cohesion-audit-*.md` | serious | rust-guidelines | open | | |
| C4-5 | Cohesion lens compares like-with-like Rust patterns across modules and records inconsistent idiom choices even when each local instance is technically valid. | `rg -n "like-with-like|consistent|inconsistent|cohesion|theme" workbench/<DATE>-cohesion-audit-*.md` | serious | operator-follow-up | open | | |
| C4-6 | Arc03 RUST-007 public parser API cohesion is re-entered with disposition options for Slice02. | `rg -n "RUST-007|public parser API|disposition" workbench/<DATE>-cohesion-audit-*.md` | serious | arc03 | open | | |
| C4-7 | Findings use collaboration-framework shape: severity, evidence, consequence, recommended action, and behavior-preservation baseline need. | Audit results contain those fields for every finding or explicitly state no findings. | serious | code-audit | open | | |
| C4-8 | At least five negative cohesion checks are recorded with searched scope and rationale for no finding. | `rg -n "Negative check|No finding|no finding" workbench/<DATE>-cohesion-audit-*.md` | correctness | code-audit | open | | |
| C4-9 | Potential repair areas identify whether existing tests cover current behavior or whether Slice02 must open a characterization-test slice before repair. | `rg -n "baseline|characterization|existing tests|test-only" workbench/<DATE>-cohesion-audit-*.md` | serious | collaboration-framework | open | | |
| C4-10 | Full quality gate passes on the audited codebase without source changes. | `cargo fmt --check && cargo check --locked --all-targets && RUSTFLAGS="-D rust-2024-compatibility" cargo check --locked --all-targets && cargo clippy --locked --all-targets -- -D warnings && cargo test --locked --all-targets && cargo build --locked --release --bins && ./target/release/hddl_analyzer --help && actionlint .github/workflows/ci.yml && git diff --check && git -C ../planning diff --check` | serious | arc01-arc03 | open | | |
| C4-11 | Slice remains read-only for implementation code: no source, test, manifest, workflow, README, or behavior changes are made. | `git diff --name-only` shows only `workbench/<DATE>-cohesion-audit-*.md`; planning diff shows only Slice01 evidence updates. | serious | operator-follow-up | open | | |
| C4-12 | Closing report walks every ledger row and bubbles up Slice02 planning needs. | `test -f 01-cleanup/arc04-rust-cohesion-audit/slice01-cohesion-diagnosis-audit/closing-report.md` and inspect row walk. | correctness | ledger-discipline | open | | |

## What Worked

Pending Slice01 close.

## Closure

Pending Slice01 close.
