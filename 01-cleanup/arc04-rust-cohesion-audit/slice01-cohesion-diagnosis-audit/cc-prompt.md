# CC Prompt: Arc04 Slice01 Cohesion Diagnosis Audit

Please execute Arc04 Slice01 for HDDL-Parser.

This is a diagnosis-only, read-only audit slice. Do not edit Rust source, tests,
Cargo files, workflows, README/docs, or behavior. The only implementation
worktree artifacts expected from this slice are new workbench audit reports.

## Branch / Base

- Start from the final local Arc03 feature state:
  `fix/cargo-reproducibility-policy` at `d820065 Fix Cargo reproducibility policy`.
- Create `audit/rust-cohesion` from that commit unless the operator gives you an
  equivalent current base.
- Leave feature implementation files unchanged.

## Required Reading

In the planning worktree, read:

- `01-cleanup/project-plan.md`
- `01-cleanup/arc03-rust-best-practices/closing-report.md`
- `01-cleanup/arc03-rust-best-practices/slice03-triage-and-fix-map/fix-map.md`
- `01-cleanup/arc04-rust-cohesion-audit/arc-plan.md`
- `01-cleanup/arc04-rust-cohesion-audit/slice01-cohesion-diagnosis-audit/slice-doc.md`
- `01-cleanup/arc04-rust-cohesion-audit/slice01-cohesion-diagnosis-audit/ledger.md`

Also load and apply:

- `$collaboration-framework`, especially `CODE-AUDIT.md` and ledger discipline.
- `$rust-guidelines`, beginning with the skill's mandatory anti-pattern scan and
  then the topic guides relevant to the code you inspect.

## Workbench Files

Create new, non-overwriting workbench reports in the feature worktree:

- `workbench/<DATE>-cohesion-audit-index.md`
- `workbench/<DATE>-cohesion-audit-results-rust.md`

Use `date +%Y.%m.%d` for `<DATE>` from your runtime environment. Do not overwrite
or rename any existing Arc03 `workbench/<DATE>-audit-*` reports.

## Audit Instructions

Use the collaboration-framework code-audit shape, augmented by Arc04's cohesion
lens:

- Compare like-with-like across modules, not just each function in isolation.
- Treat inconsistent but individually-valid Rust choices as findings when they
  make the project harder to understand, extend, or review.
- Prefer the dominant project convention where it is sound.
- Where the dominant convention is weak, propose one replacement convention and
  explain why it should become the project norm.
- Distinguish intentional divergence from accidental drift. If a variation is
  accepted, record the local reason.
- Group findings by consistency theme, not just by file.
- Re-enter Arc03 RUST-007 public parser API cohesion and record disposition
  options for Slice02.

Pay particular attention to these consistency areas:

- Error propagation and recoverability boundaries.
- Parser control flow and parse-error construction.
- AST/domain ownership, clone patterns, boxed formulas, and traversal APIs.
- Iterator/collection idioms and allocation patterns.
- Module organization, public exports, helper visibility, and naming.
- Test helper/assertion/fixture style.
- CLI/library boundary shape.
- LSP async helper boundaries, document-map access, and lock ownership.

## Required Negative Checks

Record at least five negative checks where you looked for a plausible cohesion
issue and found no problem. Include the files/modules searched and the reason no
finding was raised.

## Baseline/Test Gate

For every finding that could require production code changes later, say whether
existing tests already preserve current behavior or whether Slice02 should open a
test-only characterization slice before repair.

## Verification Commands

Run and record:

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

## Close

Update the Slice01 ledger and add a closing report. The closing report must walk
C4-1 through C4-12, identify the exact workbench report paths, prove the
diagnosis-only diff boundary, and bubble up what Slice02 must plan next.
