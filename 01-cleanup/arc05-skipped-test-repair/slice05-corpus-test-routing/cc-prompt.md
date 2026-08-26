# CC Prompt: Arc05 Slice05 Corpus Test Routing

You are working in `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/features`.

Create and use branch `test/corpus-test-routing`. Base it on the committed
Slice04 feature state:

`8e36a6a tools: add corpus selection policy`

Planning lives in:

`/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc05-skipped-test-repair/slice05-corpus-test-routing/`

Read first:

- `slice-doc.md`
- `ledger.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc05-skipped-test-repair/arc-plan.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc05-skipped-test-repair/slice04-corpus-addressability-and-policy/closing-report.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc05-skipped-test-repair/slice04-corpus-addressability-and-policy/cdc-verification.md`
- `tests/ipc/corpus-selections/README.md`

## Assignment

Remove the remaining inherited skipped-test debt by routing the IPC and JSON
corpus integration tests through the Slice04 corpus policy surface.

Default tests should gain fast corpus coverage. Full corpus validation should
remain explicit and addressable through:

```bash
HDDL_CORPUS_SELECTION=full HDDL_CORPUS_ASSERT=both cargo run --locked --example corpus_measure
```

Do not wire CI in this slice.

## Required Behavior

Handle both remaining ignored tests:

- `tests/integration_ipc.rs::ipc_validation_test`
- `tests/integration_json.rs::json_round_trip_ipc`

Replace or route them so the default locked test gate runs fast corpus
coverage over `tests/ipc/corpus-selections/fast.txt`.

IPC coverage must parse and verify selected domain/problem pairs.

JSON coverage must parse, export to JSON, reimport, verify, re-export, and
assert both:

- exact string equality;
- structural `serde_json::Value` equality.

Failure messages must include the stable corpus case ID
`<domain-dir>/<problem-file>` so regressions are directly addressable.

Prefer a small shared helper if needed to keep corpus discovery/selection
coherent between the tests and the measurement tool. Do not expose new public
crate API unless there is no reasonable test/tool-local alternative; if you
believe a public API change is necessary, stop and ask before implementing it.

## Boundaries

Do not change `.github/workflows`.

Do not add or retain any Rust `#[ignore]` annotation in `src` or `tests`.

Do not make the default test suite run all 900 corpus cases.

Do not change the checked-in fast corpus selection unless you discover an
invalid entry. If you do, record the exact reason in the ledger.

Do not add Rayon, `cargo-nextest`, `libtest-mimic`, GNU-only shell assumptions,
or a custom test framework.

Do not optimize parser, verifier, transpiler, or JSON implementation in this
slice.

Do not decide or implement branch-push, PR, scheduled, or post-merge CI
policy. That is the next slice.

## Verification

Run:

```bash
git status --short --branch
git log --oneline --decorate -5
test -f tools/corpus_measure.rs
test -f tests/ipc/corpus-selections/fast.txt
rg -n "name = \"corpus_measure\"|path = \"tools/corpus_measure.rs\"" Cargo.toml
rg -n "#\\[ignore" src tests -g '*.rs'
cargo test --locked --test ipc
cargo test --locked --test json
cargo test --locked --all-targets
HDDL_CORPUS_SELECTION=fast HDDL_CORPUS_ASSERT=both cargo run --locked --example corpus_measure
HDDL_CORPUS_SELECTION=full HDDL_CORPUS_ASSERT=both cargo run --locked --example corpus_measure
cargo fmt --check
cargo check --locked --all-targets
RUSTFLAGS="-D rust-2024-compatibility" cargo check --locked --all-targets
cargo clippy --locked --all-targets -- -D warnings
cargo build --release --locked --bins
./target/release/hddl_analyzer --help
git diff -- .github/workflows
actionlint .github/workflows/ci.yml
git diff --check
git -C ../planning diff --check
```

If you need faster inner-loop iteration, use the fast command and focused
tests while developing. The final close evidence should include the full corpus
command unless there is a concrete blocked reason and re-entry condition.

## Close Requirements

Update `ledger.md` with attested evidence for every T5 row and add
`closing-report.md` with a row-by-row walk for T5-1 through T5-8.

The Bubble-up to Arc05 must answer:

- Did Slice05 eliminate the remaining inherited `#[ignore]` debt?
- Did the default test suite gain fast corpus coverage without becoming the
  full corpus gate?
- What exact CI policy slice should come next for branch-push, PR, and
  post-merge/main coverage?
