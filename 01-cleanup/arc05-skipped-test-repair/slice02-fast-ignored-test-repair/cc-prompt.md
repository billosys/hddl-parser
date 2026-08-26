# CC Prompt: Arc05 Slice02 Fast Ignored Test Repair

You are working in `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/features`.

Create and use branch `fix/fast-ignored-test-repair`, based on
`audit/ignored-tests` at the Arc05 Slice01 investigation close point
(`7e2d8a7`). The investigation branch has no tracked implementation diff, so
this repair branch should contain only Slice02 code/test changes.

Planning lives in:

`/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc05-skipped-test-repair/slice02-fast-ignored-test-repair/`

Read first:

- `slice-doc.md`
- `ledger.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc05-skipped-test-repair/arc-plan.md`
- `workbench/2026.08.26-ignored-test-investigation.md`

## Assignment

Repair only the three fast ignored tests classified by Slice01:

- `file_type_test`
- `forgotten_dash_validation_test`
- `forgotten_question_mark_validation_test`

For `file_type_test`, rewrite the test so it has a real oracle: the domain
fixture should parse as `AbstractSyntaxTree::Domain`, and the problem fixture
should parse as `AbstractSyntaxTree::Problem`. Remove its `#[ignore]` only once
the rewritten test passes.

For the two flawed-domain tests, repair the narrow malformed-declaration
validation path if needed, then enable the tests. Keep the assertions specific:
they should prove the intended structured diagnostic behavior, not merely prove
that some error happened.

Do not change `ipc_validation_test` or `json_round_trip_ipc`. Do not remove
their `#[ignore]` annotations, do not add corpus infrastructure, and do not
change CI in this slice.

## Verification

Run the focused checks first:

```bash
cargo test --locked syntactic_analyzer::tests::syntax_tests::tests::file_type_test -- --exact
cargo test --locked --test flawed forgotten_dash_validation_test -- --exact
cargo test --locked --test flawed forgotten_question_mark_validation_test -- --exact
rg -n "#\\[ignore" src tests -g '*.rs'
```

Then run the full local gate:

```bash
cargo test --locked --all-targets
RUSTFLAGS="-D rust-2024-compatibility" cargo check --locked --all-targets
cargo clippy --locked --all-targets -- -D warnings
cargo build --release --locked --bins
./target/release/hddl_analyzer --help
actionlint .github/workflows/ci.yml
git diff --check
git -C ../planning diff --check
```

The ignored-test inventory should show only:

- `tests/integration_ipc.rs`: `ipc_validation_test`
- `tests/integration_json.rs`: `json_round_trip_ipc`

## Close Requirements

Update `ledger.md` with attested evidence for every row and add
`closing-report.md` with a row-by-row walk for F5-1 through F5-6.

The Bubble-up to the arc must state:

- The three fast ignored tests are now ordinary enabled tests, or name any row
  that could not close with a concrete re-entry condition.
- The two remaining ignored tests are corpus tests only.
- Slow/corpus measurement and infrastructure remain intentionally out of scope
  for Slice02 and should be planned separately.
