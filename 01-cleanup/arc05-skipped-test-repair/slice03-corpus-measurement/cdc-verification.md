# Slice03 CDC Verification: Corpus Measurement

Date: 2026-08-26
Feature branch: `measure/corpus-phase-timings`
Feature base: `e534df2 fix: repair fast ignored tests`
Planning branch: `planning`

## Verdict

CDC verification passes. Slice03 adds a Rust-native corpus measurement example,
keeps default tests and CI policy unchanged, and produces reproducible timing
evidence for the remaining IPC/JSON corpus-policy work.

The feature implementation remains uncommitted at verification time and is
limited to:

- `Cargo.toml`
- `tools/corpus_measure.rs`

The generated measurement artifacts are intentionally ignored:

- `workbench/2026.08.26-corpus-measurement.csv`
- `workbench/2026.08.26-corpus-measurement.md`

## Commands Reproduced

Feature worktree:

```bash
git status --short --branch --untracked-files=all
git log --oneline --decorate -5
sed -n '1,180p' Cargo.toml
sed -n '1,620p' tools/corpus_measure.rs
sed -n '1,240p' workbench/2026.08.26-corpus-measurement.md
sed -n '1,5p' workbench/2026.08.26-corpus-measurement.csv
wc -l workbench/2026.08.26-corpus-measurement.csv
tail -5 workbench/2026.08.26-corpus-measurement.csv
git check-ignore -v workbench/2026.08.26-corpus-measurement.md workbench/2026.08.26-corpus-measurement.csv
rg -n "#\\[ignore" src tests -g '*.rs'
rg -n "name = \"corpus_measure\"|path = \"tools/corpus_measure.rs\"" Cargo.toml
rg -n "corpus_measure" Cargo.toml tools tests
awk -F, 'NR>1 && ($12!="ok" || $13!="ok" || $14!="ok" || $15!="ok" || $16!="ok" || $17!="ok" || $18!="true" || $19!="true") { bad++ } END { print bad+0 }' workbench/2026.08.26-corpus-measurement.csv
git diff -- .github/workflows tests/integration_ipc.rs tests/integration_json.rs
git diff -- Cargo.toml
cargo fmt --check
HDDL_CORPUS_LIMIT=5 cargo run --locked --example corpus_measure
HDDL_CORPUS_FILTER=Blocksworld cargo run --locked --example corpus_measure
HDDL_CORPUS_REPORT=/private/tmp/hddl-corpus-measurement-cdc.csv cargo run --locked --example corpus_measure
wc -l /private/tmp/hddl-corpus-measurement-cdc.csv
awk -F, 'NR>1 && ($12!="ok" || $13!="ok" || $14!="ok" || $15!="ok" || $16!="ok" || $17!="ok" || $18!="true" || $19!="true") { bad++ } END { print bad+0 }' /private/tmp/hddl-corpus-measurement-cdc.csv
cargo check --locked --all-targets
cargo test --locked --all-targets
RUSTFLAGS="-D rust-2024-compatibility" cargo check --locked --all-targets
cargo clippy --locked --all-targets -- -D warnings
cargo build --release --locked --bins
./target/release/hddl_analyzer --help
actionlint .github/workflows/ci.yml
git diff --check
```

Planning worktree:

```bash
git diff --check
```

## Reproduced Results

- `tools/corpus_measure.rs` exists and compiles under locked all-target
  checks.
- `Cargo.toml` declares an explicit `[[example]]` target named
  `corpus_measure` with path `tools/corpus_measure.rs`, so the command remains
  `cargo run --locked --example corpus_measure`.
- The example deterministically discovers 900 IPC problem cases and reports
  stable IDs as `<domain-dir>/<problem-file>`.
- `HDDL_CORPUS_LIMIT=5 cargo run --locked --example corpus_measure` reproduced
  `discovered_cases=900, selected_cases=5`, with the first five sorted
  `AssemblyHierarchical` cases completed and zero failures.
- `HDDL_CORPUS_FILTER=Blocksworld cargo run --locked --example corpus_measure`
  reproduced `selected_cases=134`, `completed=134`, `failures=0`, and
  `json_equality_disagreements=0`.
- The full independent report rerun to
  `/private/tmp/hddl-corpus-measurement-cdc.csv` completed all 900 selected
  cases with `attempted=900 completed=900 failures=0` and
  `json_equality_disagreements=0`.
- Both CC's workbench CSV and CDC's temp CSV contain 901 lines: one header plus
  900 case rows.
- CSV scans over parse, verify, JSON import/export/reverify/reexport, string
  equality, and structural equality columns found 0 bad rows in both reports.
- The workbench summary records full-run phase totals, distribution buckets,
  slowest cases, slowest domains, and design implications for later slices.
- `git check-ignore -v` confirms both workbench measurement artifacts are
  ignored by the existing `workbench` rule.
- `rg -n "#\\[ignore" src tests -g '*.rs'` reports only
  `tests/integration_ipc.rs:7` and `tests/integration_json.rs:17`.
- `git diff -- .github/workflows tests/integration_ipc.rs
  tests/integration_json.rs` is empty.
- `git diff -- Cargo.toml` shows only the explicit `corpus_measure` example
  target declaration.
- `cargo fmt --check`, locked all-target check, locked all-target test, strict
  Rust 2024 compatibility check, strict Clippy, release binary build, binary
  help smoke, `actionlint`, feature whitespace check, and planning whitespace
  check all passed.

## Row Walk

| Row | CDC disposition | Evidence |
|-----|-----------------|----------|
| M5-1 | reproduced | `tools/corpus_measure.rs` exists; `Cargo.toml` declares `name = "corpus_measure"` and `path = "tools/corpus_measure.rs"`; `cargo check --locked --all-targets` passed; ignored-test scan still reports only the two corpus tests. |
| M5-2 | reproduced | Limit run reported `discovered_cases=900, selected_cases=5` and completed the first five sorted `AssemblyHierarchical` IDs with zero failures. |
| M5-3 | reproduced | Full independent report run to `/private/tmp/hddl-corpus-measurement-cdc.csv` completed `attempted=900 completed=900 failures=0`; temp CSV has 901 lines and zero bad outcome rows. |
| M5-4 | reproduced | Source/report inspection confirms all requested phase timing columns; Blocksworld filtered run completed 134 cases with zero failures and JSON timing output. |
| M5-5 | reproduced | CSV header separates `json_string_equal` and `json_value_equal`; both CC workbench and CDC temp report scans found zero equality failures or disagreements across 900 rows. |
| M5-6 | reproduced | Workbench summary exists and records bounded, filtered, and full runs; totals, failure count, slowest cases/phases, timeout note, and recommended next-slice options are present. |
| M5-7 | reproduced | Default locked all-target tests passed; the only remaining ignored tests are the two corpus tests; CI and corpus integration test diffs are empty; the Cargo manifest diff is limited to the explicit example target. |
| M5-8 | reproduced | Full local gate passed: fmt, locked check, locked tests, Rust 2024 compatibility, Clippy `-D warnings`, release build, help smoke, actionlint, and whitespace checks; closing report walks M5-1 through M5-8 and includes Bubble-Up. |

## Bubble-Up Check

Slice03 delivers its assigned Arc05 piece: measurement-first evidence for the
remaining corpus ignored tests, without silently choosing a final execution
policy or changing CI/default tests.

The silent-drop diff is clean. The utility was added as an explicit Cargo
example target with source at `tools/corpus_measure.rs`; it records
deterministic case IDs, per-phase timings, per-phase outcomes, exact string
equality, and structural JSON equality; it supports filter, limit, and report
path controls; and it writes ignored workbench evidence. The two corpus
`#[ignore]` annotations remain intentionally in place for later policy work.

This verification does not close Arc05. It does, however, change the next slice
boundary: the next Arc05 slice should make the corpus addressable and
policy-ready from the measured case inventory before wiring CI. The measurement
evidence supports a representative fast sample plus a full PR/post-merge corpus
gate, with Linux and macOS policy decided from explicit commands rather than
the inherited ignored tests.

## What Worked

Keeping the measurement harness as a Cargo example gave us ordinary Rust
portability, all-target compilation, and easy local iteration without adding a
second test framework or another ignored test.

The separate structural JSON comparison was worth the cost because it turns the
next policy discussion into a precise choice: exact text equality, semantic JSON
equality, or both.
