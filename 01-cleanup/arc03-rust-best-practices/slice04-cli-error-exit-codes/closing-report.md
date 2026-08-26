# Arc03 Slice04 Closing Report: CLI Error Exit Codes

Date: 2026-08-25
Feature branch: `fix/cli-error-exit-codes`
Base commit: `b596714 Add baseline characterization tests`

## Summary

Slice04 fixes RUST-001 by making `hddl_analyzer` command handlers return
`std::process::ExitCode`. Recoverable command failures now print the existing
`[Error]` messages to stderr and return `ExitCode::FAILURE`; successful
commands return `ExitCode::SUCCESS`.

The Slice04-owned implementation diff is limited to:

- `src/bin/hddl_analyzer/main.rs`
- `tests/current_behavior.rs`

The feature worktree also has a pre-existing uncommitted `.gitignore` change
adding `workbench`; Slice04 did not edit that file.

## Final CLI Process Contract

- Missing input: non-zero exit, error on stderr.
- Unsupported extension: non-zero exit, error on stderr.
- Parse or semantic verification failure: non-zero exit, diagnostics on stderr.
- Output write failure: non-zero exit, error on stderr.
- Known-good verification: exit `0`, success on stdout, empty stderr.

## Runtime Probes

Required RUST-001 probes now return non-zero:

```text
./target/release/hddl_analyzer verify /tmp/definitely-missing-hddl-parser-input.hddl
exit code: 1
[Error] No such file or directory (os error 2)

./target/release/hddl_analyzer verify Cargo.toml
exit code: 1
[Error] unrecognized input extension '.toml' (expected .hddl or .json)
```

RUST-002 and RUST-003 remain intentionally unchanged for Slice05:

```text
./target/release/hddl_analyzer verify tests/ipc/Blocksworld-GTOHP/p01.hddl
exit code: 101
thread 'main' panicked at src/lib.rs:37:18:
expected domain, found problem

./target/release/hddl_analyzer convert tests/ipc/Blocksworld-GTOHP/domain.hddl --to json --transform remove-equality-constraints
exit code: 101
thread 'main' panicked at src/transpiler/transformations/remove_equality.rs:17:9:
Compiling inequiality requires a problem instance
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

`git diff --name-only` output includes the pre-existing `.gitignore` dirty
file plus the two Slice04-owned files. The scoped Slice04 diff is
`src/bin/hddl_analyzer/main.rs` and `tests/current_behavior.rs`.

## Ledger Walk

- C4-1: Done. Slice04-owned changes are limited to CLI exit-code repair and directly updated CLI baselines; pre-existing `.gitignore` dirt is disclosed.
- C4-2: Done. Missing input exits non-zero and preserves stderr error output.
- C4-3: Done. Unsupported extension exits non-zero and preserves stderr error output.
- C4-4: Done. Semantic verification failure exits non-zero and preserves stderr diagnostics.
- C4-5: Done. Output write failure exits non-zero and preserves stderr error output.
- C4-6: Done. Successful verification still exits `0` and writes success to stdout.
- C4-7: Done. RUST-002 and RUST-003 panic probes still exit `101`; Slice05 owns those fixes.
- C4-8: Done. Full local workflow-equivalent gate passed.
- C4-9: Done. This report records the row walk and final process contract.

## Bubble-Up

Slice05 can proceed knowing the CLI wrapper now returns non-zero for ordinary
error results. Parser/transpiler and transform panics are still present and
still surface as panic exit `101`; Slice05 should replace those with
structured errors, after which the CLI should report them through the Slice04
error path.
