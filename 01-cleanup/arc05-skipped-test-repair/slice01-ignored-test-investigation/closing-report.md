# Slice01 Closing Report: Ignored Test Investigation

Date: 2026-08-26
Branch: `audit/ignored-tests`
Base: `7e2d8a7 test: consolidate helper and private naming cohesion`

## Summary

Slice01 is locally closed. The investigation inventories all ignored Rust tests,
records blame provenance for each ignored annotation, probes each test
explicitly, and classifies follow-up routes without changing source, tests,
fixtures, manifests, workflows, README, or `#[ignore]` annotations.

Primary artifact:

- `workbench/2026.08.26-ignored-test-investigation.md`

## Findings Matrix

| Test | Probe Result | Route |
|------|--------------|-------|
| `file_type_test` | fails in 0.22s | test-only fix |
| `forgotten_dash_validation_test` | fails in 0.25s | code-and-test repair |
| `forgotten_question_mark_validation_test` | fails in 0.26s | code-and-test repair |
| `ipc_validation_test` | passes in 42.77s | slow/corpus gate |
| `json_round_trip_ipc` | bounded timeout at 120.03s | slow/corpus gate |

No tests were classified as code-and-test rewrite or valid deferral.

## Row Walk

### I5-1

Done. The ignored-test inventory command found five ignored Rust tests:

```bash
rg -n "#\\[ignore" src tests -g '*.rs'
```

Inventory:

- `src/syntactic_analyzer/tests/syntax_tests.rs:9`:
  `file_type_test`, ignored with `stupid test, rewrite from scratch`.
- `tests/integration_flawed.rs:126`:
  `forgotten_dash_validation_test`, ignored with `fix`.
- `tests/integration_flawed.rs:141`:
  `forgotten_question_mark_validation_test`, ignored with `fix`.
- `tests/integration_ipc.rs:7`:
  `ipc_validation_test`, ignored with `takes too long to run`.
- `tests/integration_json.rs:17`:
  `json_round_trip_ipc`, ignored with `takes a long time`.

The workbench report records file, test name, and ignore reason for each.

### I5-2

Done. Git blame provenance is recorded for every ignored annotation:

- `992c4eb6` for `file_type_test`.
- `1f2977eb` for `forgotten_dash_validation_test`.
- `056af9f1` for `forgotten_question_mark_validation_test`.
- `7ee05160` for `ipc_validation_test`.
- `898a718b` for `json_round_trip_ipc`.

The workbench report includes the exact blame lines.

### I5-3

Done. Each ignored test was explicitly probed:

- `file_type_test`: failed in 0.22s with `parsing error`.
- `forgotten_dash_validation_test`: failed in 0.25s because the observed error
  is `Semantic(InconsistentPredicateArity(...))`, not the expected syntactic
  error.
- `forgotten_question_mark_validation_test`: failed in 0.26s because no
  syntactic error is returned.
- `ipc_validation_test`: passed in 42.77s.
- `json_round_trip_ipc`: exceeded a 120-second bound and was terminated.

Bounded corpus probes used:

```bash
perl -e 'alarm shift; exec @ARGV' 120 cargo test --locked --test <target> <test> -- --ignored --exact
```

This wrapper was used because neither `timeout` nor `gtimeout` is installed.

### I5-4

Done. The workbench report includes intended behavior and implementation code
path notes for every ignored test.

The fast failing tests map to parser classification/parsing and malformed
declaration diagnostics. The slow tests map to full IPC corpus verification and
full IPC JSON round-trip behavior.

### I5-5

Done. Every ignored test has exactly one follow-up route:

- `file_type_test`: test-only fix.
- `forgotten_dash_validation_test`: code-and-test repair.
- `forgotten_question_mark_validation_test`: code-and-test repair.
- `ipc_validation_test`: slow/corpus gate.
- `json_round_trip_ipc`: slow/corpus gate.

The route matrix in the workbench report includes all five route labels used by
the ledger grep: test-only fix, code-and-test repair, code-and-test rewrite,
slow/corpus gate, and valid deferral.

### I5-6

Done. The implementation worktree has no source, test, manifest, workflow,
README, fixture, or ignored-annotation diff from this slice:

```bash
git diff --name-status
```

The command produced no output. The workbench report is allowed by Slice01 and
is ignored by the repository:

```bash
git check-ignore -v workbench/2026.08.26-ignored-test-investigation.md
```

Output:

```text
.gitignore:8:workbench	workbench/2026.08.26-ignored-test-investigation.md
```

### I5-7

Done. No downstream repair-slice open sets were created. The only Slice05
skipped-test repair slice document currently under Arc05 is Slice01:

```bash
find 01-cleanup/arc05-skipped-test-repair -maxdepth 2 -name 'slice-doc.md' -print
```

Output:

```text
01-cleanup/arc05-skipped-test-repair/slice01-ignored-test-investigation/slice-doc.md
```

## Bubble-Up

Recommended subsequent slices:

1. Test-only fix for `file_type_test`.
2. Code-and-test repair for malformed declaration diagnostics:
   `forgotten_dash_validation_test` and
   `forgotten_question_mark_validation_test`.
3. Slow/corpus-test policy and gate for `ipc_validation_test` and
   `json_round_trip_ipc`.

Do not enable exhaustive IPC/JSON corpus tests in the default CI gate until a
slow/corpus policy exists. The IPC validation corpus passed but takes about 43
seconds locally; the JSON corpus round-trip exceeded 120 seconds in this
environment.

## Verification Commands

Commands run from `.worktrees/features` unless noted:

```bash
rg -n "#\\[ignore" src tests -g '*.rs'
cargo test --locked --all-targets
perl -e 'alarm shift; exec @ARGV' 60 cargo test --locked -- --ignored
/usr/bin/time -p cargo test --locked syntactic_analyzer::tests::syntax_tests::tests::file_type_test -- --ignored --exact
/usr/bin/time -p cargo test --locked --test flawed forgotten_dash_validation_test -- --ignored --exact
/usr/bin/time -p cargo test --locked --test flawed forgotten_question_mark_validation_test -- --ignored --exact
/usr/bin/time -p perl -e 'alarm shift; exec @ARGV' 120 cargo test --locked --test ipc ipc_validation_test -- --ignored --exact
/usr/bin/time -p perl -e 'alarm shift; exec @ARGV' 120 cargo test --locked --test json json_round_trip_ipc -- --ignored --exact
git blame -L 7,11 -- src/syntactic_analyzer/tests/syntax_tests.rs
git blame -L 125,143 -- tests/integration_flawed.rs
git blame -L 6,9 -- tests/integration_ipc.rs
git blame -L 16,19 -- tests/integration_json.rs
git diff --name-status
git diff --check
git -C ../planning diff --check
```

The default non-ignored test suite passed. The explicit ignored-test probes
produced the pass/fail/timeout evidence recorded above.
