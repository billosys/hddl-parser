# Slice02 Closing Report: Fast Ignored Test Repair

Date: 2026-08-26
Branch: `fix/fast-ignored-test-repair`
Base: `7e2d8a7 test: consolidate helper and private naming cohesion`
Feature commit: pending
Planning commit: pending
CDC verification: pending

## Summary

Slice02 repaired and enabled the three fast ignored tests identified by
Slice01:

| Test | Final state | Evidence |
|------|-------------|----------|
| `file_type_test` | enabled; meaningful AST oracle | focused exact test passed |
| `forgotten_dash_validation_test` | enabled; syntactic diagnostic pinned | focused exact test passed |
| `forgotten_question_mark_validation_test` | enabled; syntactic diagnostic pinned | focused exact test passed |

The only remaining ignored Rust tests are the two corpus tests:

- `tests/integration_ipc.rs`: `ipc_validation_test`
- `tests/integration_json.rs`: `json_round_trip_ipc`

No corpus test body or ignore annotation changed in this slice. CI and
slow/corpus infrastructure remain out of scope.

## Implementation

The parser now distinguishes domain predicate declarations from predicate atoms:

- `parse_predicate_definitions` parses the domain `:predicates` block.
- `parse_predicates` continues to parse predicate atoms for problem `:init`.
- `parse_variable_args` validates declaration parameters with a narrow
  `symbol.starts_with('?')` check.

That keeps bare constants valid in atom/term contexts while rejecting malformed
predicate declarations such as `(at-segment ?a airplane ?s - segment)` and
`(occupied s - segment)` before semantic verification.

Two pre-existing semantic unit fixtures used bare names in predicate
declarations even though they were testing duplicate/undefined predicate
behavior. Those fixture declarations were corrected to use `?` variable names
so the semantic tests still reach their intended assertions.

## Ledger Walk

### F5-1

Status: done.

`file_type_test` no longer has `#[ignore]`. It now parses a valid minimal
domain fixture and a valid minimal problem fixture, then asserts the returned
`AbstractSyntaxTree::Domain` and `AbstractSyntaxTree::Problem` variants and
their identifying names.

Verification:

```text
cargo test --locked syntactic_analyzer::tests::syntax_tests::tests::file_type_test -- --exact
test syntactic_analyzer::tests::syntax_tests::tests::file_type_test ... ok
test result: ok. 1 passed; 0 failed; 0 ignored
```

### F5-2

Status: done.

`forgotten_dash_validation_test` no longer has `#[ignore]`. It now asserts the
specific syntactic diagnostic for the malformed predicate declaration:

- `expected == "a variable name starting with '?'"`
- `found == "Identifier airplane"`
- `position.line == 33`

Verification:

```text
cargo test --locked --test flawed forgotten_dash_validation_test -- --exact
test forgotten_dash_validation_test ... ok
test result: ok. 1 passed; 0 failed; 0 ignored
```

### F5-3

Status: done.

`forgotten_question_mark_validation_test` no longer has `#[ignore]`. It now
asserts the specific syntactic diagnostic for the malformed predicate
declaration:

- `expected == "a variable name starting with '?'"`
- `found == "Identifier s"`
- `position.line == 35`

Verification:

```text
cargo test --locked --test flawed forgotten_question_mark_validation_test -- --exact
test forgotten_question_mark_validation_test ... ok
test result: ok. 1 passed; 0 failed; 0 ignored
```

### F5-4

Status: done.

The ignored-test inventory now contains only the two corpus tests.

Verification:

```text
rg -n "#\\[ignore" src tests -g '*.rs'
tests/integration_json.rs:17:#[ignore = "takes a long time"]
tests/integration_ipc.rs:7:#[ignore = "takes too long to run"]
```

`git diff -- tests/integration_ipc.rs tests/integration_json.rs` is empty, so
neither corpus test body nor ignore annotation was modified.

### F5-5

Status: done.

The full locked local gate passed with the repaired tests enabled and without
warning suppressions.

Verification:

```text
cargo fmt --check
passed

cargo test --locked --all-targets
lib: 112 passed; 0 failed; 0 ignored
integration_flawed: 23 passed; 0 failed; 0 ignored
integration_ipc: 1 ignored
integration_json: 8 passed; 1 ignored
lsp_current_behavior: 6 passed

RUSTFLAGS="-D rust-2024-compatibility" cargo check --locked --all-targets
Finished `dev` profile

cargo clippy --locked --all-targets -- -D warnings
Finished `dev` profile

cargo build --release --locked --bins
Finished `release` profile

./target/release/hddl_analyzer --help
exited 0 and printed command help

actionlint .github/workflows/ci.yml
passed

git diff --check
passed
```

### F5-6

Status: done.

This report walks F5-1 through F5-6. The bubble-up below states the remaining
ignored-test disposition and preserves the measurement-first corpus path.

Verification:

```text
git -C ../planning diff --check
passed
```

## Bubble-Up To Arc

The three fast ignored tests are now ordinary enabled tests:

- `file_type_test`
- `forgotten_dash_validation_test`
- `forgotten_question_mark_validation_test`

The two remaining ignored Rust tests are corpus tests only:

- `ipc_validation_test`
- `json_round_trip_ipc`

Slow/corpus measurement and infrastructure remain intentionally out of scope for
Slice02. The planning worktree already contains Slice03 corpus-measurement
planning, so later corpus policy should proceed from that measurement-first
evidence rather than from Slice02.
