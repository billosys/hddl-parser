# Slice06: GitHub Actions Corpus Policy

Version: 1.0
Date: 2026-08-26
Arc: `arc05-skipped-test-repair`
Expected branch: `ci/corpus-policy`
Expected base: `af0968b test: route corpus tests through fast selection`

## Goal

Wire the verified corpus test policy into GitHub Actions so the project gets
fast representative corpus coverage on ordinary branch pushes and full corpus
coverage at the PR and post-merge boundaries.

Slice05 proved the command surface and removed the inherited `#[ignore]` debt.
This slice decides the CI routing policy and makes it executable in the
workflow without changing parser, JSON, corpus-selection, or test semantics.

## Scope

In scope:

- Start from committed Slice05 feature state `af0968b`.
- Preserve the existing Linux/macOS matrix with `ubuntu-24.04` and `macos-15`.
- Preserve `actions/checkout@v7`.
- Use locked Cargo commands in CI wherever Cargo reads the dependency graph.
- Keep the default Rust test gate in CI, now including the fast IPC/JSON corpus
  tests enabled by Slice05.
- Add an explicit fast corpus measurement command for branch-push confidence:

  ```bash
  HDDL_CORPUS_SELECTION=fast HDDL_CORPUS_ASSERT=both cargo run --locked --example corpus_measure
  ```

- Add full corpus measurement for PR and post-merge default-branch coverage:

  ```bash
  HDDL_CORPUS_SELECTION=full HDDL_CORPUS_ASSERT=both cargo run --locked --example corpus_measure
  ```

- Ensure full corpus coverage runs on both Linux and macOS when the workflow
  event qualifies.
- Make the scheduled-run policy explicit. The expected default is a weekly
  scheduled full-corpus run on the default branch, unless implementing it
  exposes a concrete upstream-maintainer concern that should be recorded.
- Keep failures diagnosable from GitHub Actions logs by using named workflow
  steps and the existing corpus command summaries.
- Update corpus-selection documentation if the CI policy becomes user-facing
  enough that the existing README would otherwise be stale.

Out of scope:

- Parser, verifier, transpiler, AST, or JSON behavior changes.
- Changes to `tools/corpus_measure.rs` or `tests/common/mod.rs`, unless a small
  fix is required because CI exposes a command-surface bug.
- Changing `tests/ipc/corpus-selections/fast.txt`.
- Adding new test runners such as `cargo-nextest`, `libtest-mimic`, Rayon, or
  a custom harness.
- Making the full corpus run on every non-default branch push.
- Release automation, artifacts, or upload/download behavior.
- README badge work unless the workflow name or badge target changes.

## Policy Direction

Use three lanes:

- Branch pushes matching CI branch globs run the normal quality gate plus the
  explicit fast corpus measurement.
- Pull requests run the normal quality gate, fast corpus measurement, and full
  corpus measurement.
- Pushes to `main` or `master` run the normal quality gate, fast corpus
  measurement, and full corpus measurement as the post-merge gate.

Scheduled policy should be explicit. Prefer adding a weekly schedule that runs
the same full-corpus condition on the default branch. If this is rejected during
implementation, record the no-op rationale and re-entry condition in the
ledger rather than silently omitting it.

## Verification Approach

Suggested commands:

```bash
git status --short --branch
git log --oneline --decorate -5
git diff -- .github/workflows
rg -n "ubuntu-24.04|macos-15|actions/checkout@v7" .github/workflows/ci.yml
rg -n "cargo (check|clippy|test|build).*--locked|cargo run --locked --example corpus_measure" .github/workflows/ci.yml
rg -n "HDDL_CORPUS_SELECTION=fast|HDDL_CORPUS_SELECTION=full|HDDL_CORPUS_ASSERT=both" .github/workflows/ci.yml
rg -n "pull_request|refs/heads/main|refs/heads/master|schedule|cron" .github/workflows/ci.yml
rg -n "#\\[ignore" src tests -g '*.rs'
actionlint .github/workflows/ci.yml
cargo fmt --check
cargo check --locked --all-targets
RUSTFLAGS="-D rust-2024-compatibility" cargo check --locked --all-targets
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked --all-targets
HDDL_CORPUS_SELECTION=fast HDDL_CORPUS_ASSERT=both cargo run --locked --example corpus_measure
HDDL_CORPUS_SELECTION=full HDDL_CORPUS_ASSERT=both cargo run --locked --example corpus_measure
cargo build --release --locked --bins
./target/release/hddl_analyzer --help
git diff --check
git -C ../planning diff --check
```

If the full corpus command is too expensive during iteration, use the fast
command while editing. The final close evidence must include the full command
or a concrete blocked reason and re-entry condition.

## Exit Criteria

- CI branch push behavior includes fast/default corpus coverage and does not
  run the full 900-case corpus on every non-default branch push.
- CI pull-request behavior runs the full corpus command on both Linux and
  macOS.
- CI post-merge default-branch behavior runs the full corpus command on both
  Linux and macOS.
- Scheduled full-corpus behavior is either implemented or explicitly
  dispositioned with a reason and re-entry condition.
- CI Cargo commands that depend on the lockfile use `--locked`.
- Existing CI platform and checkout decisions are preserved.
- No Rust `#[ignore]` annotations are reintroduced.
- Full local quality gates and corpus commands pass.
- The close report bubbles whether Arc05 can close next or needs one more
  composition slice.
