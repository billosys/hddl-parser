# CC Prompt: Arc05 Slice06 GitHub Actions Corpus Policy

You are working in `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/features`.

Create and use branch `ci/corpus-policy`. Base it on the committed Slice05
feature state:

`af0968b test: route corpus tests through fast selection`

Planning lives in:

`/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc05-skipped-test-repair/slice06-github-actions-corpus-policy/`

Read first:

- `slice-doc.md`
- `ledger.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc05-skipped-test-repair/arc-plan.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc05-skipped-test-repair/slice05-corpus-test-routing/closing-report.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc05-skipped-test-repair/slice05-corpus-test-routing/cdc-verification.md`
- `.github/workflows/ci.yml`
- `tests/ipc/corpus-selections/README.md`

## Assignment

Wire the verified corpus command surface into GitHub Actions policy.

Branch pushes should keep a fast feedback loop: the existing quality gate plus
the default fast corpus tests from Slice05 and an explicit fast corpus
measurement command.

Pull requests and post-merge pushes to the default branch should run the full
900-case corpus command on both Linux and macOS.

Make scheduled full-corpus policy explicit. Prefer a weekly scheduled full
corpus run. If you find a concrete reason not to add a schedule in this
upstream-facing PR, record it as a no-op with rationale and re-entry condition
in the ledger and closing report.

## Required CI Policy

Preserve:

- `ubuntu-24.04`
- `macos-15`
- `actions/checkout@v7`
- direct `rustup` setup
- the existing release binary build and `hddl_analyzer --help` smoke test

Use `--locked` for Cargo commands that resolve dependencies:

- `cargo check --locked --all-targets`
- `cargo clippy --locked --all-targets -- -D warnings`
- `cargo test --locked --all-targets`
- `cargo build --release --locked --bins`
- `cargo run --locked --example corpus_measure`

Add the explicit fast corpus command in CI:

```bash
HDDL_CORPUS_SELECTION=fast HDDL_CORPUS_ASSERT=both cargo run --locked --example corpus_measure
```

Add the explicit full corpus command for qualifying events:

```bash
HDDL_CORPUS_SELECTION=full HDDL_CORPUS_ASSERT=both cargo run --locked --example corpus_measure
```

The full corpus step should run for:

- `pull_request`
- push to `refs/heads/main`
- push to `refs/heads/master`
- scheduled runs, if you implement the preferred schedule

The full corpus step should not run for every non-default branch push.

Review the `on.push.branches` globs. Keep existing useful globs and add the
branch families this cleanup series actually uses where practical, such as
`edition/*`, `audit/*`, `test/*`, `policy/*`, and `measure/*`.

## Boundaries

Prefer limiting feature changes to `.github/workflows/ci.yml` and, if needed,
`tests/ipc/corpus-selections/README.md`.

Do not change parser, verifier, transpiler, AST, or JSON behavior.

Do not change `tools/corpus_measure.rs`, `tests/common/mod.rs`, or
`tests/ipc/corpus-selections/fast.txt` unless CI exposes a real bug in the
command surface. If that happens, record the bug and keep the fix minimal.

Do not add Rayon, `cargo-nextest`, `libtest-mimic`, shell-specific test
harnessing, or a custom CI orchestration script.

Do not reintroduce any Rust `#[ignore]` annotation.

Do not add release artifacts, upload/download actions, or publishing behavior.

## Verification

Run:

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

If the full corpus command is too slow during inner-loop work, use the fast
corpus command while editing. Final close evidence must include the full
corpus command or a concrete blocked reason and re-entry condition.

## Close Requirements

Update `ledger.md` with attested evidence for every T6 row and add
`closing-report.md` with a row-by-row walk for T6-1 through T6-11.

The Bubble-up to Arc05 must answer:

- Did Slice06 wire the corpus CI policy without changing corpus/test
  semantics?
- Is Arc05 now ready for arc-level close, or is a final composition or
  PR-readiness slice needed?
- What exact upstream PR branch/body caveats should be carried forward for
  this corpus-testing work?
