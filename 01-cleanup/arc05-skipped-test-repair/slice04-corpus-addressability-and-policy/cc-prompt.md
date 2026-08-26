# CC Prompt: Arc05 Slice04 Corpus Addressability And Policy

You are working in `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/features`.

Create and use branch `policy/corpus-addressability`. Base it on the committed
Slice03 feature state:

`c6e4907 tools: add corpus measurement utility`

Planning lives in:

`/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc05-skipped-test-repair/slice04-corpus-addressability-and-policy/`

Read first:

- `slice-doc.md`
- `ledger.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc05-skipped-test-repair/arc-plan.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc05-skipped-test-repair/slice03-corpus-measurement/closing-report.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc05-skipped-test-repair/slice03-corpus-measurement/cdc-verification.md`
- `workbench/2026.08.26-corpus-measurement.md`

## Assignment

Make the Slice03 corpus measurement tool policy-ready without changing CI.

Keep the invocation:

```bash
cargo run --locked --example corpus_measure
```

Keep the implementation source at:

```text
tools/corpus_measure.rs
```

The utility is a Cargo example target for convenient `cargo run --example`
execution, but the source belongs under `tools/` because this is maintainer
infrastructure, not user-facing library sample code.

## Required Behavior

Add named corpus selections:

- `HDDL_CORPUS_SELECTION=full`: all discovered corpus cases, sorted by stable
  case ID. This should also be the default if no selection or manifest is
  provided.
- `HDDL_CORPUS_SELECTION=fast`: a checked-in representative fast selection.

Add custom manifest support:

- `HDDL_CORPUS_MANIFEST=<path>` loads a newline-oriented selection file of
  stable case IDs.
- Allow blank lines and `#` comments.
- Reject duplicate case IDs.
- Reject unknown case IDs.
- Reject unreadable manifest paths.
- Document selection precedence if `HDDL_CORPUS_MANIFEST`,
  `HDDL_CORPUS_SELECTION`, `HDDL_CORPUS_FILTER`, and `HDDL_CORPUS_LIMIT` are
  combined. Prefer: discover all cases, apply manifest or named selection, then
  apply filter/limit only as local iteration controls.

Add a checked-in fast selection under:

```text
tests/ipc/corpus-selections/fast.txt
tests/ipc/corpus-selections/README.md
```

Use Slice03 timing evidence, not only domain names, when choosing the fast
selection. The fast selection should be broad enough to catch cross-domain
regressions and small enough for branch-push use later. Avoid the measured slow
tail from the Slice03 summary.

Add explicit JSON assertion policy:

- `HDDL_CORPUS_ASSERT=none`: measure and report only; do not fail for equality
  mismatches.
- `HDDL_CORPUS_ASSERT=structural`: fail if `serde_json::Value` equality fails.
- `HDDL_CORPUS_ASSERT=string`: fail if exact string equality fails.
- `HDDL_CORPUS_ASSERT=both`: fail if either structural or exact string equality
  fails.

Recommended local validation commands should use `both`, because Slice03 found
zero disagreements across all 900 cases and the inherited JSON corpus test
currently asserts exact string equality.

Expected input/policy errors must return non-zero with useful messages. Do not
panic for bad selection names, bad manifest paths, unknown case IDs, duplicate
case IDs, or invalid assertion modes.

## Boundaries

Do not change `.github/workflows`.

Do not remove or weaken the two remaining corpus `#[ignore]` annotations:

- `tests/integration_ipc.rs:7`
- `tests/integration_json.rs:17`

Do not add Rayon, `cargo-nextest`, `libtest-mimic`, GNU-only shell assumptions,
or a custom test framework.

Do not optimize parser, verifier, transpiler, or JSON implementation in this
slice.

Do not wire branch/PR/main CI policy yet. This slice creates the policy
substrate; the next slice can decide how to route the inherited ignored tests
and CI gates through it.

## Verification

Run:

```bash
git status --short --branch
test -f tools/corpus_measure.rs
rg -n "name = \"corpus_measure\"|path = \"tools/corpus_measure.rs\"" Cargo.toml
test -f tests/ipc/corpus-selections/fast.txt
test -f tests/ipc/corpus-selections/README.md
HDDL_CORPUS_SELECTION=fast HDDL_CORPUS_ASSERT=both cargo run --locked --example corpus_measure
HDDL_CORPUS_SELECTION=full HDDL_CORPUS_LIMIT=5 HDDL_CORPUS_ASSERT=both cargo run --locked --example corpus_measure
HDDL_CORPUS_MANIFEST=tests/ipc/corpus-selections/fast.txt HDDL_CORPUS_ASSERT=both cargo run --locked --example corpus_measure
HDDL_CORPUS_SELECTION=fast HDDL_CORPUS_ASSERT=none cargo run --locked --example corpus_measure
HDDL_CORPUS_SELECTION=fast HDDL_CORPUS_ASSERT=structural cargo run --locked --example corpus_measure
HDDL_CORPUS_SELECTION=fast HDDL_CORPUS_ASSERT=string cargo run --locked --example corpus_measure
```

Run negative probes and record non-zero exits plus useful messages:

```bash
HDDL_CORPUS_SELECTION=wat cargo run --locked --example corpus_measure
HDDL_CORPUS_ASSERT=wat cargo run --locked --example corpus_measure
HDDL_CORPUS_MANIFEST=/tmp/hddl-parser-bad-corpus-selection.txt cargo run --locked --example corpus_measure
```

Use temporary bad manifest files to exercise duplicate and unknown case IDs.

Run the full local gate:

```bash
cargo fmt --check
cargo check --locked --all-targets
cargo test --locked --all-targets
RUSTFLAGS="-D rust-2024-compatibility" cargo check --locked --all-targets
cargo clippy --locked --all-targets -- -D warnings
cargo build --release --locked --bins
./target/release/hddl_analyzer --help
rg -n "#\\[ignore" src tests -g '*.rs'
git diff -- .github/workflows tests/integration_ipc.rs tests/integration_json.rs
actionlint .github/workflows/ci.yml
git diff --check
git -C ../planning diff --check
```

## Close Requirements

Update `ledger.md` with attested evidence for every CP5 row and add
`closing-report.md` with a row-by-row walk for CP5-1 through CP5-8.

The Bubble-up to Arc05 must recommend the next slice boundary. Likely options:

- Replace or route the inherited ignored corpus tests through the named
  fast/full commands.
- Add CI workflow policy for branch, PR, and post-merge gates from the named
  commands.
- Split those two if the implementation diff is large enough to deserve two
  reviewable PRs.
