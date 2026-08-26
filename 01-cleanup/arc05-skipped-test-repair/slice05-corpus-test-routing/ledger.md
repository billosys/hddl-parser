# Slice05: Corpus Test Routing

## Ledger

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| T5-1 | Slice05 starts from the committed Slice04 corpus policy base and preserves the corpus measurement command surface. | `git log --oneline --decorate -5`; `test -f tools/corpus_measure.rs`; `test -f tests/ipc/corpus-selections/fast.txt`; `rg -n "name = \"corpus_measure\"|path = \"tools/corpus_measure.rs\"" Cargo.toml`. | serious | Slice04 close | open | | Expected base is `8e36a6a tools: add corpus selection policy`. |
| T5-2 | The two inherited corpus ignored tests are enabled or replaced by enabled tests, leaving no Rust `#[ignore]` annotations in `src` or `tests`. | `rg -n "#\\[ignore" src tests -g '*.rs'`; inspect `tests/integration_ipc.rs` and `tests/integration_json.rs`. | serious | Arc05 goal | open | | Removing a skip is only valid if replacement coverage is present. |
| T5-3 | Default IPC corpus coverage uses the checked-in fast selection and reports stable case IDs on failure. | `cargo test --locked --test ipc`; inspect the test/helper code for use of `tests/ipc/corpus-selections/fast.txt` or the shared fast-selection policy and case-ID diagnostics. | correctness | Slice04 policy | open | | The default IPC test must not run all 900 cases. |
| T5-4 | Default JSON corpus coverage uses the checked-in fast selection and checks both structural and exact string round-trip equality. | `cargo test --locked --test json`; inspect the test/helper code for structural and exact string assertions over selected case IDs. | correctness | Slice03/Slice04 evidence | open | | Slice03 found zero disagreements, so both is the expected validation mode. |
| T5-5 | Full corpus validation remains explicitly addressable and passes through the Slice04 policy command rather than an ignored test. | `HDDL_CORPUS_SELECTION=full HDDL_CORPUS_ASSERT=both cargo run --locked --example corpus_measure`; inspect docs for the full command. | serious | operator CI discussion | open | | If this is too expensive to complete, record a concrete blocked reason and re-entry condition. |
| T5-6 | CI workflow policy remains unchanged by this routing slice. | `git diff -- .github/workflows`; `actionlint .github/workflows/ci.yml`. | serious | Slice05 boundary | open | | CI branch/PR/main scheduling is the next slice. |
| T5-7 | Full local quality gates pass with no hidden skip debt. | `cargo fmt --check`; `cargo check --locked --all-targets`; `cargo test --locked --all-targets`; `RUSTFLAGS="-D rust-2024-compatibility" cargo check --locked --all-targets`; `cargo clippy --locked --all-targets -- -D warnings`; `cargo build --release --locked --bins`; `./target/release/hddl_analyzer --help`; `git diff --check`; `git -C ../planning diff --check`. | serious | Arc05 gate | open | | `cargo test --locked --all-targets` should now run the fast corpus routes with zero ignored tests. |
| T5-8 | Slice close bubbles the CI-policy boundary to Arc05 without silently wiring it. | Inspect `closing-report.md` for T5-1 through T5-8 and Bubble-up to Arc05. | correctness | project-management | open | | Likely next slice: GitHub Actions branch/PR/main corpus policy. |

## What Worked

_(At slice close. Patterns that made the slice close cleanly.)_

## Closure

Open.
