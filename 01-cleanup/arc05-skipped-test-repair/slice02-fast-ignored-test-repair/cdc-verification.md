# Slice02 CDC Verification: Fast Ignored Test Repair

Date: 2026-08-26
Feature branch: `fix/fast-ignored-test-repair`
Feature commit: `e534df2 fix: repair fast ignored tests`
Planning branch: `planning`

## Verdict

CDC verification passes. Slice02 is verified as a focused repair of the three
fast ignored tests, with the two corpus ignored tests left unchanged for the
measurement-first Slice03 path.

The implementation diff is limited to the expected parser/test surface:

- `src/syntactic_analyzer/domain_parser/predicate_parser.rs`
- `src/syntactic_analyzer/domain_parser/router.rs`
- `src/syntactic_analyzer/problem_parser/list_parser.rs`
- `src/syntactic_analyzer/tests/syntax_tests.rs`
- `tests/integration_flawed.rs`
- `src/semantic_analyzer/tests/duplicate_tests.rs`
- `src/semantic_analyzer/tests/undefined_tests.rs`

The current feature worktree is checked out on `measure/corpus-phase-timings`,
but `measure/corpus-phase-timings` and `fix/fast-ignored-test-repair` both
point at the verified Slice02 commit `e534df2`.

Because the shared feature worktree already had untracked Slice03 `examples/`
work in progress, CDC created a clean detached verification worktree at
`/private/tmp/hddl-parser-slice02-verify` from commit `e534df2` and reran the
Cargo gate there. This keeps the reproduced evidence scoped to Slice02 alone.

## Commands Reproduced

Feature worktree:

```bash
git status --short --branch
git log --oneline --decorate -8
git show --stat --oneline --decorate HEAD
git show -- src/syntactic_analyzer/domain_parser/predicate_parser.rs src/syntactic_analyzer/problem_parser/list_parser.rs src/syntactic_analyzer/tests/syntax_tests.rs tests/integration_flawed.rs
git show -- src/semantic_analyzer/tests/duplicate_tests.rs src/semantic_analyzer/tests/undefined_tests.rs src/syntactic_analyzer/domain_parser/router.rs
```

Clean detached verification worktree at `e534df2`:

```bash
git status --short --branch
rg -n "#\\[ignore" src tests -g '*.rs'
git diff -- tests/integration_ipc.rs tests/integration_json.rs
cargo fmt --check
cargo test --locked syntactic_analyzer::tests::syntax_tests::tests::file_type_test -- --exact
cargo test --locked --test flawed forgotten_dash_validation_test -- --exact
cargo test --locked --test flawed forgotten_question_mark_validation_test -- --exact
cargo test --locked --all-targets
cargo check --locked --all-targets
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

The detached verification worktree is under `/private/tmp`, so the clean Cargo
rerun did not depend on the shared feature worktree's in-progress Slice03
state.

## Reproduced Results

- Feature commit `e534df2` is present on both `fix/fast-ignored-test-repair`
  and `measure/corpus-phase-timings`.
- `git show --stat HEAD` shows the expected seven-file implementation diff.
- Clean detached verification worktree status was `## HEAD (no branch)`.
- The parser now routes domain `:predicates` through
  `parse_predicate_definitions`, which requires variable names, while
  predicate atoms still use `parse_predicates` / `parse_args`.
- `file_type_test` has no `#[ignore]` and asserts
  `AbstractSyntaxTree::Domain` plus `AbstractSyntaxTree::Problem`.
- `forgotten_dash_validation_test` has no `#[ignore]` and pins the syntactic
  diagnostic to `expected == "a variable name starting with '?'"`,
  `found == "Identifier airplane"`, line 33.
- `forgotten_question_mark_validation_test` has no `#[ignore]` and pins the
  syntactic diagnostic to `expected == "a variable name starting with '?'"`,
  `found == "Identifier s"`, line 35.
- `rg -n "#\\[ignore" src tests -g '*.rs'` reports only
  `tests/integration_ipc.rs:7` and `tests/integration_json.rs:17`.
- `git diff -- tests/integration_ipc.rs tests/integration_json.rs` is empty.
- Focused tests reproduced:
  - `file_type_test`: 1 passed, 0 ignored.
  - `forgotten_dash_validation_test`: 1 passed, 0 ignored.
  - `forgotten_question_mark_validation_test`: 1 passed, 0 ignored.
- Full locked all-target tests passed:
  - library: 112 passed, 0 ignored.
  - `integration_flawed`: 23 passed, 0 ignored.
  - `integration_ipc`: 1 ignored.
  - `integration_json`: 8 passed, 1 ignored.
  - `lsp_current_behavior`: 6 passed.
- `cargo fmt --check`, ordinary locked check, Rust 2024 compatibility check,
  strict Clippy, release binary build, binary help smoke, `actionlint`, feature
  whitespace check, and planning whitespace check all passed.

## Row Walk

| Row | CDC disposition | Evidence |
|-----|-----------------|----------|
| F5-1 | reproduced | Focused exact `file_type_test` passed with `1 passed; 0 ignored`; source inspection confirms no `#[ignore]` and explicit domain/problem AST assertions. |
| F5-2 | reproduced | Focused exact `forgotten_dash_validation_test` passed with `1 passed; 0 ignored`; source inspection confirms the specific structured syntactic diagnostic assertion and narrow parser path split. |
| F5-3 | reproduced | Focused exact `forgotten_question_mark_validation_test` passed with `1 passed; 0 ignored`; source inspection confirms the specific structured syntactic diagnostic assertion and variable-name validation path. |
| F5-4 | reproduced | Ignored-test grep reports only the IPC and JSON corpus tests; corpus test files have no diff. |
| F5-5 | reproduced | Full locked local gate passed: fmt, tests, ordinary check, Rust 2024 compatibility, Clippy `-D warnings`, release build, help smoke, actionlint, and whitespace checks. |
| F5-6 | reproduced | Closing report walks F5-1 through F5-6 and bubbles remaining ignored-test work to the Slice03 measurement-first corpus path. |

## Bubble-Up Check

Slice02 delivers its assigned Arc05 piece: the three fast non-corpus ignored
tests are now ordinary enabled tests, and their assertions are stronger than
the inherited skipped forms.

The silent-drop diff is clean for Slice02. The test-only parser classification
repair landed; the two malformed declaration diagnostics landed with focused
production parser changes; the corpus tests were not changed; and no CI policy
or corpus infrastructure work was smuggled into this slice.

This verification does not close Arc05. The remaining ignored-test debt is now
corpus-only and should proceed through Slice03 corpus measurement before final
slow/corpus execution policy is chosen.

## What Worked

Splitting declaration parsing from predicate atom parsing was the right narrow
move: it validates malformed domain predicate declarations without forbidding
bare constants in problem/init predicate atoms.

The full all-target test run was valuable because it confirmed the stricter
parser still lets semantic tests reach their intended semantic assertions after
updating two stale predicate-declaration fixtures.
