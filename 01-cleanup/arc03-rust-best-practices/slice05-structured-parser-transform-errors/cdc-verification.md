# CDC Verification: Arc03 Slice05 Structured Parser And Transform Errors

Date: 2026-08-26
Verifier: CDC
Feature branch: `fix/structured-parser-transform-errors`
Feature base: `f3a3f8d Fix CLI error exit codes`
Feature commit verified: `6bd1b0a Fix structured parser transform errors`
Planning branch: `planning`

## Verdict

Verified. Slice05 fixes RUST-002 and RUST-003 by replacing the mapped
recoverable parser/transpiler/transform panic paths with structured
`ParsingError::Transformation` returns on existing `Result` surfaces.

All C5-1 through C5-9 rows are verified. No LSP, Cargo policy, dependency,
edition, public API re-export, or unrelated source work is mixed into the
feature diff.

## Artifact Boundary

`git diff --name-only f3a3f8d..HEAD`:

```text
src/lib.rs
src/transpiler/transformations/remove_equality.rs
tests/current_behavior.rs
```

CDC inspected the three-file diff. `HDDLProgram::from_hddl` now returns
`Err(ParsingError::Transformation(...))` for domain/problem slot mismatches.
`Transpiler::from_hddl` propagates those errors. `RemoveEqualityConstraints`
checks for a problem before calling the mutating helper. The relevant
`tests/current_behavior.rs` `catch_unwind` baselines were replaced with
ordinary `Err` assertions.

## Runtime Probes

The two Slice01 panic probes no longer exit with panic code `101`:

| Probe | Expected | Observed |
|-------|----------|----------|
| `./target/release/hddl_analyzer verify tests/ipc/Blocksworld-GTOHP/p01.hddl` | non-zero through `[Error]`, not panic | exit `1`, `[Error] expected domain input, found problem` |
| `./target/release/hddl_analyzer convert tests/ipc/Blocksworld-GTOHP/domain.hddl --to json --transform remove-equality-constraints` | non-zero through `[Error]`, not panic | exit `1`, `[Error] remove-equality-constraints requires a problem input` |

## Quality Gate

CDC reproduced the Slice05 gate:

- `cargo fmt --check`
- `cargo check --all-targets`
- `RUSTFLAGS="-D rust-2024-compatibility" cargo check --all-targets`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test --test current_behavior`
- `cargo test --all-targets`
- `cargo build --release --bins`
- `./target/release/hddl_analyzer --help`
- `actionlint .github/workflows/ci.yml`
- `git diff --check`
- `git diff --check f3a3f8d..HEAD`
- `git -C ../planning diff --check`

All commands exited `0`. `cargo test --test current_behavior` passed with 10
tests, 0 failures, and 0 ignored tests. `cargo test --all-targets` passed with
the existing ignored legacy tests unchanged.

## Ledger Walk

- C5-1: Verified. Feature commit `6bd1b0a` changes only `src/lib.rs`,
  `src/transpiler/transformations/remove_equality.rs`, and
  `tests/current_behavior.rs` relative to `f3a3f8d`.
- C5-2: Verified. `HDDLProgram::from_hddl` returns `Err` for problem-as-domain
  input.
- C5-3: Verified. `HDDLProgram::from_hddl` returns `Err` for domain-as-problem
  input.
- C5-4: Verified. `Transpiler::from_hddl` propagates both mismatch errors.
- C5-5: Verified. Domain-only `RemoveEqualityConstraints` returns `Err` before
  calling the mutating helper.
- C5-6: Verified. The three public error messages distinguish unexpected
  domain input, unexpected problem input, and missing problem input.
- C5-7: Verified. Both former panic probes exit `1` through Slice04's ordinary
  CLI `[Error]` path, not panic exit `101`.
- C5-8: Verified. Full local workflow-equivalent gate passed.
- C5-9: Verified. Closing report walks every row and states the final
  RUST-002/RUST-003 error contracts.

## Bubble-Up

No arc-plan re-slicing is required. Slice05 delivers its assigned structured
error repair and strengthens Slice04's CLI contract by giving those failures
ordinary `Result` errors to print. Slice06 can proceed with LSP error
boundaries and package metadata.
