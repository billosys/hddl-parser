# Slice05: Corpus Test Routing

Version: 1.0
Date: 2026-08-26
Arc: `arc05-skipped-test-repair`
Expected branch: `test/corpus-test-routing`
Expected base: `8e36a6a tools: add corpus selection policy`

## Goal

Route the two remaining inherited ignored corpus tests through the verified
Slice04 corpus policy surface so skipped test debt is no longer hidden behind
`#[ignore]`.

After this slice, default `cargo test --locked --all-targets` should include
fast corpus coverage for IPC verification and JSON round-tripping, while full
corpus validation remains explicitly addressable through the policy command
surface created in Slice04. CI workflow scheduling remains a later slice.

## Scope

In scope:

- Start from the committed Slice04 corpus policy commit.
- Preserve `cargo run --locked --example corpus_measure` as the explicit
  full-corpus policy surface.
- Replace or route `tests/integration_ipc.rs::ipc_validation_test` so it is an
  enabled default test over the checked-in fast corpus selection.
- Replace or route `tests/integration_json.rs::json_round_trip_ipc` so it is
  an enabled default test over the checked-in fast corpus selection.
- Remove the remaining corpus `#[ignore]` annotations only after the tests run
  quickly enough for the default locked all-target gate.
- Keep corpus failures diagnosable with stable case IDs in assertion or panic
  messages.
- Share selection/discovery helpers where that keeps the tool and tests
  coherent, without exposing new public library API unless absolutely required.
- Update corpus-selection documentation if the local commands or test routing
  story changes.

Out of scope:

- GitHub Actions workflow changes.
- Final branch-push, PR, scheduled, or post-merge CI policy.
- Parser, verifier, transpiler, or JSON performance optimization.
- Changing the checked-in fast corpus selection policy except to fix a
  discovered invalid entry.
- Adding Rayon, `cargo-nextest`, `libtest-mimic`, GNU-only shell assumptions,
  or a custom test framework.
- Public API changes to the `hddl_analyzer` crate unless the operator gives an
  explicit GO.

## Routing Direction

The default Rust test suite should not run all 900 corpus cases. Use the
Slice04 fast selection as the ordinary test route so `cargo test --locked
--all-targets` gains meaningful corpus coverage without turning every local
test run into a full corpus gate.

The full corpus remains the explicit policy command:

```bash
HDDL_CORPUS_SELECTION=full HDDL_CORPUS_ASSERT=both cargo run --locked --example corpus_measure
```

This keeps three lanes distinct:

- default tests: fast representative corpus coverage;
- local/full validation: explicit full corpus command;
- CI scheduling: later slice, using the same commands.

## Verification Approach

Suggested commands:

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
actionlint .github/workflows/ci.yml
git diff -- .github/workflows
git diff --check
git -C ../planning diff --check
```

If the full corpus command is too expensive during an implementation iteration,
run a bounded smoke command while iterating, but the final close evidence should
include the full command or a concrete blocked reason and re-entry condition.

## Exit Criteria

- Both inherited corpus tests are enabled or replaced by enabled tests.
- `rg -n "#\\[ignore" src tests -g '*.rs'` returns no matches.
- Default locked all-target tests include fast IPC corpus verification and fast
  JSON round-trip coverage.
- Corpus test failures include stable case IDs.
- The explicit full-corpus validation command remains documented and passes.
- No GitHub Actions workflow changes land in this slice.
- Full local quality gates pass.
- The close report recommends the next CI-policy slice boundary without wiring
  CI inside Slice05.
