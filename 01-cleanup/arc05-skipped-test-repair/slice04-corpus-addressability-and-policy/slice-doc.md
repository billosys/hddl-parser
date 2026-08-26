# Slice04: Corpus Addressability And Policy

Version: 1.0
Date: 2026-08-26
Arc: `arc05-skipped-test-repair`
Expected branch: `policy/corpus-addressability`
Expected base: `c6e4907 tools: add corpus measurement utility`

## Goal

Turn the Slice03 measurement utility into a policy-ready corpus runner without
wiring CI yet.

After this slice, the project should have stable, checked-in corpus selection
policy and named local commands for fast and full corpus validation. The next
slice can then replace or route the two inherited ignored corpus tests and wire
branch/PR/main behavior from a concrete substrate rather than from an ad hoc
choice.

## Scope

In scope:

- Keep `cargo run --locked --example corpus_measure` as the invocation surface.
- Keep the implementation source at `tools/corpus_measure.rs` with the explicit
  `[[example]]` target in `Cargo.toml`.
- Add addressable corpus selections using the stable Slice03 case IDs
  (`<domain-dir>/<problem-file>`).
- Add a checked-in fast selection policy under the test/corpus area, with
  enough comments or companion documentation that reviewers can see why the
  cases were selected.
- Add support for a named full selection and a named fast selection.
- Add support for a custom manifest/selection file for focused regression
  iteration.
- Add an explicit JSON assertion mode so later gates can choose measurement
  only, structural JSON equality, exact string equality, or both.
- Document named local commands for fast and full corpus validation.
- Preserve the two inherited corpus `#[ignore]` annotations until a later slice
  replaces or routes them deliberately.

Out of scope:

- GitHub Actions or CI workflow changes.
- Removing `#[ignore]` from `ipc_validation_test` or `json_round_trip_ipc`.
- Adding Rayon, `cargo-nextest`, `libtest-mimic`, GNU-only shell assumptions,
  or a custom test framework.
- Optimizing parser, verifier, transpiler, or JSON implementation.
- Rewriting the inherited corpus integration tests.
- Deciding final branch/PR/post-merge matrix policy.

## Policy Direction

Use Slice03's measurements as the selection basis:

- The full corpus remains all 900 discovered cases.
- The fast corpus should be checked in and reviewable, not inferred from a
  brittle substring filter.
- The fast corpus should be representative enough for branch-push confidence
  while avoiding the measured slow tail that belongs in full PR/main gates.
- The JSON policy should distinguish semantic JSON equality from exact string
  determinism. The recommended default for validation commands is to assert
  both, because Slice03 found zero disagreements across the full corpus and the
  inherited JSON corpus test currently used exact string equality.

## Verification Approach

Suggested commands:

```bash
git status --short --branch
test -f tools/corpus_measure.rs
rg -n "name = \"corpus_measure\"|path = \"tools/corpus_measure.rs\"" Cargo.toml
test -f tests/ipc/corpus-selections/fast.txt
test -f tests/ipc/corpus-selections/README.md
HDDL_CORPUS_SELECTION=fast HDDL_CORPUS_ASSERT=both cargo run --locked --example corpus_measure
HDDL_CORPUS_SELECTION=full HDDL_CORPUS_LIMIT=5 HDDL_CORPUS_ASSERT=both cargo run --locked --example corpus_measure
HDDL_CORPUS_MANIFEST=tests/ipc/corpus-selections/fast.txt HDDL_CORPUS_ASSERT=both cargo run --locked --example corpus_measure
rg -n "#\\[ignore" src tests -g '*.rs'
git diff -- .github/workflows tests/integration_ipc.rs tests/integration_json.rs
cargo fmt --check
cargo check --locked --all-targets
cargo test --locked --all-targets
RUSTFLAGS="-D rust-2024-compatibility" cargo check --locked --all-targets
cargo clippy --locked --all-targets -- -D warnings
cargo build --release --locked --bins
./target/release/hddl_analyzer --help
actionlint .github/workflows/ci.yml
git diff --check
git -C ../planning diff --check
```

Also run at least one negative manifest/assertion probe and record the exact
command and non-zero failure:

```bash
HDDL_CORPUS_MANIFEST=/tmp/hddl-parser-bad-corpus-selection.txt cargo run --locked --example corpus_measure
HDDL_CORPUS_ASSERT=wat cargo run --locked --example corpus_measure
```

## Exit Criteria

- The corpus runner exposes named `fast` and `full` selections.
- A checked-in fast selection file exists, uses stable case IDs, has no
  duplicates, and resolves only to known corpus cases.
- A custom manifest path can run a selected subset by stable case ID.
- Unknown or duplicate custom manifest entries fail clearly and non-zero.
- JSON assertion mode is explicit and supports measurement-only,
  structural-only, string-only, and both-equality validation.
- Fast and full validation commands are documented for local and later CI use.
- Default `cargo test --locked --all-targets` remains unchanged and passing.
- The two corpus ignored tests remain in place until a later slice replaces or
  routes them.
- No GitHub Actions workflow changes land in this slice.
- The close report recommends the next slice boundary without silently wiring
  CI inside Slice04.
