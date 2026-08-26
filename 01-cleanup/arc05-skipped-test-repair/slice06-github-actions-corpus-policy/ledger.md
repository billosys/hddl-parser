# Slice06: GitHub Actions Corpus Policy

## Ledger

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| T6-1 | Slice06 starts from the committed Slice05 corpus-routing base with no hidden skipped-test debt. | `git log --oneline --decorate -5`; `rg -n "#\\[ignore" src tests -g '*.rs'`; `cargo test --locked --all-targets`. | serious | Slice05 CDC | open | | Expected feature base: `af0968b`. |
| T6-2 | CI preserves the established platform/tooling baseline: `ubuntu-24.04`, `macos-15`, and `actions/checkout@v7`. | `rg -n "ubuntu-24.04|macos-15|actions/checkout@v7" .github/workflows/ci.yml`; `actionlint .github/workflows/ci.yml`. | serious | Arc01/Arc02 baseline | open | | Do not switch to third-party Rust setup actions. |
| T6-3 | CI uses locked Cargo commands wherever Cargo resolves dependencies. | `rg -n "cargo (check|clippy|test|build).*--locked|cargo run --locked --example corpus_measure" .github/workflows/ci.yml`; inspect that check, clippy, test, build, and corpus runs include `--locked`. | serious | Arc03 reproducibility | open | | `cargo fmt --check` does not need `--locked`. |
| T6-4 | Branch pushes matching CI branch globs run the ordinary quality gate plus explicit fast corpus measurement, not the full corpus gate. | Inspect `on.push.branches` and the fast/full corpus workflow step conditions; run `actionlint .github/workflows/ci.yml`. | serious | operator CI policy | open | | Include branch families used by this cleanup series where practical. |
| T6-5 | Pull requests run full corpus measurement on both Linux and macOS. | Inspect the full corpus step condition for `pull_request`; confirm the step is inside the existing OS matrix; run `actionlint .github/workflows/ci.yml`. | serious | operator CI policy | open | | Full command: `HDDL_CORPUS_SELECTION=full HDDL_CORPUS_ASSERT=both cargo run --locked --example corpus_measure`. |
| T6-6 | Post-merge pushes to `main` and `master` run full corpus measurement on both Linux and macOS. | Inspect push branch filters and the full corpus step condition for `refs/heads/main` and `refs/heads/master`; run `actionlint .github/workflows/ci.yml`. | serious | operator CI policy | open | | Keep both names because upstream uses `master` while this fork uses `main`. |
| T6-7 | Scheduled full-corpus policy is explicit. | `rg -n "schedule|cron|HDDL_CORPUS_SELECTION=full" .github/workflows/ci.yml`; if not implemented, inspect ledger/closing report for a valid no-op rationale and re-entry condition. | correctness | Slice05 bubble-up | open | | Prefer weekly scheduled full corpus on the default branch. |
| T6-8 | Workflow changes do not alter parser/test semantics or corpus selection data. | `git diff --name-status`; inspect diff for `.github/workflows/ci.yml` and any documentation-only updates. | serious | slice boundary | open | | Any change to Rust source, tests, or `fast.txt` must be justified as a CI-exposed bug fix. |
| T6-9 | Corpus policy remains locally reproducible outside CI. | `HDDL_CORPUS_SELECTION=fast HDDL_CORPUS_ASSERT=both cargo run --locked --example corpus_measure`; `HDDL_CORPUS_SELECTION=full HDDL_CORPUS_ASSERT=both cargo run --locked --example corpus_measure`. | serious | Slice04/Slice05 evidence | open | | Final close should include the full run. |
| T6-10 | Full local gate passes after workflow policy changes. | `cargo fmt --check`; `cargo check --locked --all-targets`; `RUSTFLAGS="-D rust-2024-compatibility" cargo check --locked --all-targets`; `cargo clippy --locked --all-targets -- -D warnings`; `cargo test --locked --all-targets`; `cargo build --release --locked --bins`; `./target/release/hddl_analyzer --help`; `actionlint .github/workflows/ci.yml`; `git diff --check`; `git -C ../planning diff --check`. | serious | Arc05 gate | open | | |
| T6-11 | Slice close reports whether Arc05 can close next or needs a final composition/PR-readiness slice. | Inspect `closing-report.md` for T6-1 through T6-11 and Bubble-up to Arc05. | correctness | project-management | open | | Do not create `closing-report.md` until implementation closes. |

## What Worked

_(At slice close. Patterns that made the slice close cleanly.)_

## Closure

Open.
