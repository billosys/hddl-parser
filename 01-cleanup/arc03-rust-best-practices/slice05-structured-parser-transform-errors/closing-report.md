# Arc03 Slice05 Closing Report: Structured Parser And Transform Errors

Date: 2026-08-25
Feature branch: `fix/structured-parser-transform-errors`
Base commit: `f3a3f8d Fix CLI error exit codes`
Feature commit: `6bd1b0a Fix structured parser transform errors`

## Summary

Slice05 fixes RUST-002 and RUST-003 by replacing recoverable panic paths with
structured `ParsingError` returns on existing `Result` surfaces.

The verified feature commit diff contains:

- `src/lib.rs`
- `src/transpiler/transformations/remove_equality.rs`
- `tests/current_behavior.rs`

No LSP, Cargo policy, dependency, edition, or public API re-export work is
mixed into this slice.

## Final Public Error Behavior

- `HDDLProgram::from_hddl`: problem input in the domain slot returns
  `Err(ParsingError::Transformation("expected domain input, found problem"))`.
- `HDDLProgram::from_hddl`: domain input in the optional problem slot returns
  `Err(ParsingError::Transformation("expected problem input, found domain"))`.
- `Transpiler::from_hddl`: propagates the same `ParsingError` values from
  `HDDLProgram::from_hddl`.
- `RemoveEqualityConstraints`: domain-only transformation returns
  `Err(ParsingError::Transformation("remove-equality-constraints requires a problem input"))`
  before calling the mutating helper.

CLI invocations depend on Slice04's `ExitCode::FAILURE` handling to surface
these structured errors as ordinary non-zero `[Error]` results instead of
panic exit `101`.

## Runtime Probes

The two Slice01 panic probes no longer return panic exit `101`:

```text
./target/release/hddl_analyzer verify tests/ipc/Blocksworld-GTOHP/p01.hddl
exit code: 1
[Error] expected domain input, found problem

./target/release/hddl_analyzer convert tests/ipc/Blocksworld-GTOHP/domain.hddl --to json --transform remove-equality-constraints
exit code: 1
[Error] remove-equality-constraints requires a problem input
```

## Verification

All required commands passed:

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

`cargo test --test current_behavior` passed with 10 tests, 0 failures, and 0
ignored tests. `cargo test --all-targets` passed with the existing ignored
legacy tests unchanged.

`git diff --name-only f3a3f8d..HEAD` outputs:

```text
src/lib.rs
src/transpiler/transformations/remove_equality.rs
tests/current_behavior.rs
```

## Ledger Walk

- C5-1: Done. The feature commit diff is limited to parser/transpiler/transform
  error handling and directly updated baselines on top of `f3a3f8d`.
- C5-2: Done. `hddl_program_problem_as_domain_returns_error` passed in
  `cargo test --test current_behavior`.
- C5-3: Done. `hddl_program_domain_as_problem_returns_error` passed in
  `cargo test --test current_behavior`.
- C5-4: Done. `transpiler_domain_as_problem_returns_error` and
  `transpiler_problem_as_domain_returns_error` passed in
  `cargo test --test current_behavior`.
- C5-5: Done. `remove_equality_constraints_domain_only_returns_error` passed
  in `cargo test --test current_behavior`; the method returns before calling
  the mutating helper when `problem` is absent.
- C5-6: Done. Error construction and test assertions distinguish unexpected
  domain input, unexpected problem input, and missing problem input.
- C5-7: Done. Both former panic probes exit `1` with `[Error]` output, not
  panic exit `101`.
- C5-8: Done. Full local workflow-equivalent gate passed.
- C5-9: Done. This report records C5-1 through C5-9 and the final RUST-002
  and RUST-003 error contracts.

## Bubble-Up

Slice05 delivers its assigned Arc03 repair: RUST-002 and RUST-003 no longer
panic for the mapped recoverable parser/transpiler/transform failures. No
arc-plan re-slicing is required. Slice06 can proceed with LSP error boundaries
and package metadata on top of the settled structured-error behavior.
