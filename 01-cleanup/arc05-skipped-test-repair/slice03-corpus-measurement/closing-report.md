# Slice03: Corpus Measurement Closing Report

Date: 2026-08-26
Feature branch: `measure/corpus-phase-timings`
Feature base: final Slice02 repair state at `e534df2`
Status: locally closed; CDC verification pending

## Summary

Slice03 added a Rust-native measurement example for the IPC corpus. The utility
discovers stable case IDs, applies optional substring and limit selectors,
measures parse/verify and JSON round-trip phases per case, and can flush an
incremental CSV report while it runs.

The full 900-case measurement completed normally. No parse, verification,
JSON import/export, re-verification, string equality, or structural JSON
equality failures were observed. Default tests and CI policy were deliberately
left unchanged.

## Row Walk

### M5-1

Done. `tools/corpus_measure.rs` exists and compiles under
`cargo check --locked --all-targets`. `Cargo.toml` declares it as an explicit
`[[example]]` target named `corpus_measure`, preserving
`cargo run --locked --example corpus_measure` while avoiding a user-facing
`examples/` source directory. The slice added no ignored test and no custom
test harness.

### M5-2

Done. `HDDL_CORPUS_LIMIT=5 cargo run --locked --example corpus_measure`
reported `discovered_cases=900, selected_cases=5`, then measured the first
five sorted case IDs:

- `AssemblyHierarchical/genericLinearProblem_depth01.hddl`
- `AssemblyHierarchical/genericLinearProblem_depth02.hddl`
- `AssemblyHierarchical/genericLinearProblem_depth03.hddl`
- `AssemblyHierarchical/genericLinearProblem_depth04.hddl`
- `AssemblyHierarchical/genericLinearProblem_depth05.hddl`

The run ended with `attempted=5 completed=5 failures=0`.

### M5-3

Done. The full report run:

`HDDL_CORPUS_REPORT=workbench/2026.08.26-corpus-measurement.csv cargo run --locked --example corpus_measure`

reported `discovered_cases=900, selected_cases=900` and
`attempted=900 completed=900 failures=0`. The CSV has 901 lines including the
header and records per-case parse and verify timing/outcome fields.

### M5-4

Done. The CSV header records all requested phase timings:

- `hddl_parse_ms`
- `verify_ms`
- `json_export_ms`
- `json_import_ms`
- `reimport_verify_ms`
- `json_reexport_ms`
- `json_string_compare_ms`
- `json_value_compare_ms`

The filtered run
`HDDL_CORPUS_FILTER=Blocksworld cargo run --locked --example corpus_measure`
completed 134 selected cases with zero failures.

### M5-5

Done. The report separates exact string equality from structural JSON equality
using `json_string_equal` and `json_value_equal`. The full run found zero JSON
equality disagreements across 900 completed cases.

### M5-6

Done. `workbench/2026.08.26-corpus-measurement.md` summarizes the bounded,
filtered, and full measurement runs. The full run completed without an external
timeout and recorded:

- 900 attempted cases
- 900 completed cases
- 0 failures
- 0 JSON equality disagreements
- p50 total 16.719 ms
- p90 total 311.651 ms
- p95 total 1096.562 ms
- p99 total 14656.655 ms

The report identifies Minecraft cases as the slowest tail and JSON structural
comparison as the largest aggregate phase.

### M5-7

Done. Default behavior is unchanged. `cargo test --locked --all-targets`
passed, and the only remaining ignored Rust tests are:

- `tests/integration_json.rs:17`
- `tests/integration_ipc.rs:7`

`git diff -- .github/workflows tests/integration_ipc.rs tests/integration_json.rs`
is empty. `Cargo.toml` changes only to declare the explicit `corpus_measure`
example target.

### M5-8

Done. The full local gate passed:

- `cargo fmt --check`
- `cargo check --locked --all-targets`
- `cargo test --locked --all-targets`
- `RUSTFLAGS="-D rust-2024-compatibility" cargo check --locked --all-targets`
- `cargo clippy --locked --all-targets -- -D warnings`
- `cargo build --release --locked --bins`
- `./target/release/hddl_analyzer --help`
- `rg -n "#\\[ignore" src tests -g '*.rs'`
- `actionlint .github/workflows/ci.yml`
- `git diff --check`
- `git -C ../planning diff --check`

## Measurement Notes

The full report CSV is intentionally ignored under `workbench/`. It contains
one row per discovered IPC case and is suitable for local policy analysis
without committing generated measurement output.

Aggregate phase totals from the full run:

| Phase | Total ms | Share |
|-------|----------|-------|
| JSON structural comparison | 150003.572 | 36.35% |
| JSON export | 77559.915 | 18.80% |
| JSON re-export | 76489.879 | 18.54% |
| JSON import | 43112.913 | 10.45% |
| Reimport verify | 23008.473 | 5.58% |
| Verify | 23008.236 | 5.58% |
| HDDL parse | 19218.768 | 4.66% |
| String compare | 249.088 | 0.06% |

The slowest cases are concentrated in `Minecraft-Regular` and
`Minecraft-Player`, with the largest individual totals above 20 seconds.
Most cases remain fast: 596 of 900 completed in 25 ms or less.

## Bubble-Up

Slice03 does not choose the final slow/corpus CI policy. The evidence supports
opening the next slice around corpus policy design and addressability:

- define stable named commands for full corpus and representative corpus runs;
- consider a checked-in manifest or generated inventory for addressable case
  selection;
- choose representative fast samples using measured timings rather than domain
  names alone;
- decide whether JSON round-trip assertions should use exact string equality,
  structural JSON equality, or both;
- decide where full corpus runs belong in branch, PR, post-merge, and platform
  matrix policy.

The current data suggests branch-push CI should avoid the Minecraft-heavy tail
unless sharded or moved to a slower gate. Full PR/post-merge coverage remains
plausible, but it should be designed explicitly from this measurement rather
than by simply unignoring both inherited corpus tests.
