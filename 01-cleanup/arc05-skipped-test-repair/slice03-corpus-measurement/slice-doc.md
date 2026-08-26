# Slice03: Corpus Measurement

Version: 1.0
Date: 2026-08-26
Arc: `arc05-skipped-test-repair`
Expected branch: `measure/corpus-phase-timings`

## Goal

Measure the IPC corpus tests before changing their execution policy.

This slice should produce deterministic evidence about where corpus time is
spent, especially in `json_round_trip_ipc`, so the next slice can decide from
data whether to add a manifest, addressable case IDs, representative fast
samples, structural JSON assertions, sharding, or CI policy changes.

## Scope

In scope:

- Add a Rust-native measurement utility under `examples/`, preferably
  `examples/corpus_measure.rs`.
- Deterministically enumerate all `tests/ipc` domain/problem cases using sorted
  paths and stable case IDs.
- Measure parse/verify behavior for each case.
- Measure JSON round-trip phases separately for each case:
  HDDL parse, JSON export, JSON import, reimport verification, JSON re-export,
  string equality comparison, and `serde_json::Value` structural comparison.
- Support simple environment-variable controls for safe local iteration:
  optional case substring filter, optional case limit, and optional report path.
- Write an ignored workbench report summarizing totals, slowest cases, failures,
  partial-run behavior if bounded externally, and design implications for later
  corpus slices.
- Keep Linux and macOS portability in view by using standard Rust/Cargo
  behavior and avoiding GNU-only shell assumptions in the implementation.

Out of scope:

- Removing `#[ignore]` from `ipc_validation_test` or `json_round_trip_ipc`.
- Adding a new ignored Rust test.
- Adding sharding, Rayon/internal parallelism, `libtest-mimic`, `cargo-nextest`,
  a checked-in corpus manifest, or CI workflow changes.
- Replacing the existing corpus correctness tests.
- Optimizing parser, verifier, transpiler, or JSON behavior beyond what is
  necessary to compile the measurement utility.
- Deciding the final push/PR/main corpus policy.

## Verification Approach

Run the utility on a small bounded subset first, then run the largest practical
measurement pass with an external bound so partial output is still useful if
the JSON corpus remains too slow.

Suggested commands:

```bash
HDDL_CORPUS_LIMIT=5 cargo run --locked --example corpus_measure
HDDL_CORPUS_FILTER=Blocksworld cargo run --locked --example corpus_measure
HDDL_CORPUS_REPORT=workbench/2026.08.26-corpus-measurement.csv cargo run --locked --example corpus_measure
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

If a full unbounded JSON measurement is still too slow, use a bounded external
runner and record the exact command, timeout duration, completed case count,
last completed case, partial report path, and slowest phases observed before
termination.

## Exit Criteria

- The corpus measurement utility compiles under `cargo check --locked
  --all-targets`.
- Corpus case enumeration is deterministic and records total domain/problem
  case count.
- Parse/verify and JSON round-trip phase timings are recorded per case.
- Structural JSON comparison evidence is recorded separately from string
  equality evidence.
- The default locked test gate remains unchanged and passing.
- The only ignored Rust tests remain the intended corpus tests, assuming
  Slice02 has already enabled the three fast ignored tests.
- The closing report bubbles up concrete next-slice recommendations without
  finalizing corpus CI policy in this slice.
