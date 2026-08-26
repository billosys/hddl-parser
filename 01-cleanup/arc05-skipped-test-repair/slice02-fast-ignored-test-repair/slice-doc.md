# Slice02: Fast Ignored Test Repair

Version: 1.0
Date: 2026-08-26
Arc: `arc05-skipped-test-repair`
Expected branch: `fix/fast-ignored-test-repair`

## Goal

Repair the three fast ignored tests identified by Slice01 so they become
ordinary, default-gate tests again.

This slice intentionally excludes the IPC and JSON corpus tests. Those tests
need a separate measurement-first slow/corpus strategy before their execution
policy changes.

## Scope

In scope:

- Rewrite `file_type_test` so it asserts parser classification behavior rather
  than merely accepting any successful parse.
- Repair `forgotten_dash_validation_test` and its underlying parser/diagnostic
  behavior if the current failure exposes a real code bug.
- Repair `forgotten_question_mark_validation_test` and its underlying
  parser/diagnostic behavior if the current success exposes a real code bug.
- Remove only the three `#[ignore]` annotations for the repaired fast tests.
- Keep test assertions specific enough to prevent the current regressions from
  returning silently.
- Preserve or improve default locked test coverage.

Out of scope:

- Changing `ipc_validation_test` or `json_round_trip_ipc` ignore annotations.
- Adding slow/corpus test policy, sharding, sampling, manifests, timing
  reports, or CI matrix changes.
- Rewriting the parser architecture beyond the narrow malformed-declaration
  validation needed for the two flawed-domain tests.
- Broad public API redesign or JSON schema work.
- Suppressing warnings or weakening existing CI-quality gates.

## Verification Approach

Run focused repaired-test probes first, then the full local quality gate:

```bash
rg -n "#\\[ignore" src tests -g '*.rs'
cargo test --locked syntactic_analyzer::tests::syntax_tests::tests::file_type_test -- --exact
cargo test --locked --test flawed forgotten_dash_validation_test -- --exact
cargo test --locked --test flawed forgotten_question_mark_validation_test -- --exact
cargo test --locked --all-targets
RUSTFLAGS="-D rust-2024-compatibility" cargo check --locked --all-targets
cargo clippy --locked --all-targets -- -D warnings
cargo build --release --locked --bins
./target/release/hddl_analyzer --help
actionlint .github/workflows/ci.yml
git diff --check
git -C ../planning diff --check
```

The final ignored-test inventory should show only the two corpus tests:
`ipc_validation_test` and `json_round_trip_ipc`.

## Exit Criteria

- `file_type_test` is rewritten as a meaningful, enabled parser-classification
  test and passes without `#[ignore]`.
- `forgotten_dash_validation_test` is enabled and passes with an assertion that
  pins the intended malformed-declaration diagnostic behavior.
- `forgotten_question_mark_validation_test` is enabled and passes with an
  assertion that pins the intended malformed-declaration diagnostic behavior.
- The only remaining ignored Rust tests are the two corpus tests classified by
  Slice01 as slow/corpus-gate work.
- The full locked local gate passes without warning suppressions or weakened
  assertions.
