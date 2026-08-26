# CC Prompt: Arc05 Slice03 Corpus Measurement

You are working in `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/features`.

Create and use branch `measure/corpus-phase-timings`. Base it on the final
Arc05 Slice02 feature state if Slice02 has already closed. If Slice02 is still
in flight, pause before branching and confirm the operator's intended base.

Planning lives in:

`/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc05-skipped-test-repair/slice03-corpus-measurement/`

Read first:

- `slice-doc.md`
- `ledger.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc05-skipped-test-repair/arc-plan.md`
- `workbench/2026.08.26-ignored-test-investigation.md`

## Assignment

Add a measurement utility, not a new correctness gate.

Implement a Rust-native utility as an explicit Cargo example target:

- `tools/corpus_measure.rs`

The utility should deterministically enumerate all IPC corpus domain/problem
cases under `tests/ipc` using sorted paths. Give each case a stable,
human-readable ID such as `<domain-dir>/<problem-file>`.

For each selected case, measure and report:

- HDDL parse time.
- Parse plus verify outcome and verify time.
- JSON export time.
- JSON import time.
- Reimported-program verification time.
- JSON re-export time.
- Exact string equality result/time.
- `serde_json::Value` structural equality result/time.

Use `std::time::Instant` or equivalent standard timing support. Prefer standard
library code and existing dependencies; do not add Rayon, `libtest-mimic`,
`cargo-nextest`, sharding, or a custom test harness in this slice.

Support simple environment variables for safe iteration:

- `HDDL_CORPUS_FILTER`: optional substring filter matched against the stable
  case ID.
- `HDDL_CORPUS_LIMIT`: optional positive integer limit after deterministic
  sorting/filtering.
- `HDDL_CORPUS_REPORT`: optional path where a CSV or similarly structured
  report is written incrementally.

If writing a report file, flush it regularly so an externally bounded run still
leaves usable partial evidence.

## Important Boundaries

Do not remove `#[ignore]` from `ipc_validation_test` or
`json_round_trip_ipc`.

Do not add a new ignored test. The measurement utility should be an example
target, not another skipped test hidden inside `cargo test`. Declare it with
`[[example]]` in `Cargo.toml` so `cargo run --locked --example corpus_measure`
stays available without making the source look like user-facing library sample
code.

Do not change GitHub Actions, introduce corpus sharding, add a checked-in corpus
manifest, or decide the final branch-push/PR/main policy in this slice.

Do not optimize parser, verifier, transpiler, or JSON implementation unless a
compile error in the measurement utility forces a tiny local adjustment. This
slice measures before theorizing and implementing.

## Verification

Run focused measurement commands first:

```bash
HDDL_CORPUS_LIMIT=5 cargo run --locked --example corpus_measure
HDDL_CORPUS_FILTER=Blocksworld cargo run --locked --example corpus_measure
HDDL_CORPUS_REPORT=workbench/2026.08.26-corpus-measurement.csv cargo run --locked --example corpus_measure
```

If the full report command is too slow, rerun it with an external timeout and
record the exact command, timeout duration, completed case count, last completed
case, and partial report path. On this machine, Slice01 used this portable
pattern because GNU `timeout` was unavailable:

```bash
perl -e 'alarm shift; exec @ARGV' 300 cargo run --locked --example corpus_measure
```

Then run the normal gate:

```bash
cargo check --locked --all-targets
cargo test --locked --all-targets
RUSTFLAGS="-D rust-2024-compatibility" cargo check --locked --all-targets
cargo clippy --locked --all-targets -- -D warnings
cargo build --release --locked --bins
./target/release/hddl_analyzer --help
rg -n "#\\[ignore" src tests -g '*.rs'
actionlint .github/workflows/ci.yml
git diff --check
git -C ../planning diff --check
```

Create an ignored workbench summary:

- `workbench/2026.08.26-corpus-measurement.md`

The summary should include:

- Total corpus cases discovered.
- Cases attempted and completed.
- Any failures by phase.
- Slowest cases and slowest phases.
- Whether JSON exact string equality and structural equality disagree anywhere.
- Whether timing evidence supports a later manifest/addressable-case slice.
- Whether timing evidence supports a representative fast sample plus full
  PR/main corpus gate on both Linux and macOS.

## Close Requirements

Update `ledger.md` with attested evidence for every row and add
`closing-report.md` with a row-by-row walk for M5-1 through M5-8.

The Bubble-up to the arc must recommend the next slice boundary from evidence.
It may recommend manifest work, addressable case IDs, structural JSON
assertions, representative fast samples, CI policy, or focused optimization
investigation, but it must not quietly finalize those policies inside this
measurement slice.
